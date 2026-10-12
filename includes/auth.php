<?php
/**
 * Autoryzacja i uwierzytelnianie dla panelu admina
 */

/**
 * Sprawdza czy użytkownik jest zalogowany
 * Jeśli nie, przekierowuje do strony logowania
 * @param bool $redirect
 * @return bool
 */
function check_admin_auth($redirect = true) {
    if (!is_admin_logged_in()) {
        if ($redirect) {
            header('Location: /admin/login');
            exit;
        }
        return false;
    }
    return true;
}

/**
 * Logowanie admina
 * @param string $username
 * @param string $password
 * @return array (success, message, user)
 */
function admin_login($username, $password) {
    $db = get_db_connection();
    
    // Sprawdź czy użytkownik istnieje
    $stmt = $db->prepare("SELECT * FROM users WHERE username = ? LIMIT 1");
    $stmt->execute([$username]);
    $user = $stmt->fetch(PDO::FETCH_ASSOC);
    
    if (!$user) {
        // Loguj nieudaną próbę
        log_login_attempt($username, false, 'User not found');
        return ['success' => false, 'message' => 'Nieprawidłowa nazwa użytkownika lub hasło'];
    }
    
    // Sprawdź czy konto jest zablokowane
    if ($user['failed_attempts'] >= MAX_LOGIN_ATTEMPTS && 
        strtotime($user['lock_until']) > time()) {
        
        $remainingTime = ceil((strtotime($user['lock_until']) - time()) / 60);
        log_login_attempt($username, false, 'Account locked');
        return ['success' => false, 'message' => "Konto zablokowane. Spróbuj ponownie za $remainingTime minut."];
    }
    
    // Weryfikuj hasło
    if (password_verify($password, $user['password_hash'])) {
        // Zresetuj licznik nieudanych prób
        $stmt = $db->prepare("UPDATE users SET failed_attempts = 0, lock_until = NULL, last_login = NOW() WHERE id = ?");
        $stmt->execute([$user['id']]);
        
        // Utwórz sesję
        $_SESSION['admin_logged_in'] = true;
        $_SESSION['admin_user'] = [
            'id' => $user['id'],
            'username' => $user['username'],
            'email' => $user['email']
        ];
        
        // Zaloguj udaną próbę
        log_login_attempt($username, true);
        
        return ['success' => true, 'message' => 'Zalogowano pomyślnie', 'user' => $user];
    } else {
        // Zwiększ licznik nieudanych prób
        $failedAttempts = $user['failed_attempts'] + 1;
        $lockUntil = null;
        
        if ($failedAttempts >= MAX_LOGIN_ATTEMPTS) {
            $lockUntil = date('Y-m-d H:i:s', time() + LOGIN_LOCKOUT_TIME);
        }
        
        $stmt = $db->prepare("UPDATE users SET failed_attempts = ?, lock_until = ? WHERE id = ?");
        $stmt->execute([$failedAttempts, $lockUntil, $user['id']]);
        
        // Loguj nieudaną próbę
        log_login_attempt($username, false, 'Invalid password');
        
        $remainingAttempts = MAX_LOGIN_ATTEMPTS - $failedAttempts;
        $message = 'Nieprawidłowa nazwa użytkownika lub hasło';
        
        if ($remainingAttempts > 0) {
            $message .= ". Pozostało $remainingAttempts prób.";
        } else {
            $message = "Konto zablokowane na " . (LOGIN_LOCKOUT_TIME / 60) . " minut.";
        }
        
        return ['success' => false, 'message' => $message];
    }
}

/**
 * Wylogowanie admina
 */
function admin_logout() {
    // Zapisanie aktywności
    if (isset($_SESSION['admin_user']['id'])) {
        log_activity(
            $_SESSION['admin_user']['id'],
            'logout',
            'User logged out'
        );
    }
    
    // Usunięcie sesji
    $_SESSION = [];
    session_destroy();
    
    // Usunięcie ciasteczka sesji
    if (ini_get("session.use_cookies")) {
        $params = session_get_cookie_params();
        setcookie(
            session_name(),
            '',
            time() - 42000,
            $params["path"],
            $params["domain"],
            $params["secure"],
            $params["httponly"]
        );
    }
}

/**
 * Logowanie prób logowania
 * @param string $username
 * @param bool $success
 * @param string|null $details
 */
function log_login_attempt($username, $success, $details = null) {
    $db = get_db_connection();
    
    try {
        $action = $success ? 'login_success' : 'login_failed';
        $ip = $_SERVER['REMOTE_ADDR'] ?? 'unknown';
        $userAgent = $_SERVER['HTTP_USER_AGENT'] ?? 'unknown';
        
        $stmt = $db->prepare("INSERT INTO activity_logs (action, details, ip_address, user_agent, created_at) 
                              VALUES (?, ?, ?, ?, NOW())");
        $stmt->execute([$action, $details, $ip, $userAgent]);
    } catch (PDOException $e) {
        if (DEBUG) {
            error_log('Błąd logowania próby logowania: ' . $e->getMessage());
        }
    }
}

/**
 * Logowanie aktywności admina
 * @param int $userId
 * @param string $action
 * @param string|null $details
 */
function log_activity($userId, $action, $details = null) {
    $db = get_db_connection();
    
    try {
        $ip = $_SERVER['REMOTE_ADDR'] ?? 'unknown';
        $userAgent = $_SERVER['HTTP_USER_AGENT'] ?? 'unknown';
        
        $stmt = $db->prepare("INSERT INTO activity_logs (user_id, action, details, ip_address, user_agent, created_at) 
                              VALUES (?, ?, ?, ?, ?, NOW())");
        $stmt->execute([$userId, $action, $details, $ip, $userAgent]);
    } catch (PDOException $e) {
        if (DEBUG) {
            error_log('Błąd logowania aktywności: ' . $e->getMessage());
        }
    }
}

/**
 * Sprawdza uprawnienia użytkownika
 * @param int $userId
 * @param string $requiredPermission
 * @return bool
 */
function has_permission($userId, $requiredPermission = 'admin') {
    // W tej wersji wszystkie konta admin mają pełne uprawnienia
    // Można rozbudować o system ról
    return true;
}

/**
 * Pobiera listę użytkowników admin
 * @return array
 */
function get_admin_users() {
    $db = get_db_connection();
    
    try {
        $stmt = $db->prepare("SELECT * FROM users ORDER BY username ASC");
        $stmt->execute();
        return $stmt->fetchAll(PDO::FETCH_ASSOC);
    } catch (PDOException $e) {
        if (DEBUG) {
            error_log('Błąd pobierania użytkowników: ' . $e->getMessage());
        }
        return [];
    }
}

/**
 * Pobiera dane użytkownika po ID
 * @param int $userId
 * @return array|null
 */
function get_admin_user_by_id($userId) {
    $db = get_db_connection();
    
    try {
        $stmt = $db->prepare("SELECT * FROM users WHERE id = ? LIMIT 1");
        $stmt->execute([$userId]);
        return $stmt->fetch(PDO::FETCH_ASSOC);
    } catch (PDOException $e) {
        if (DEBUG) {
            error_log('Błąd pobierania użytkownika: ' . $e->getMessage());
        }
        return null;
    }
}

/**
 * Aktualizuje dane użytkownika
 * @param int $userId
 * @param array $data
 * @return bool
 */
function update_admin_user($userId, $data) {
    $db = get_db_connection();
    
    try {
        $updates = [];
        $params = [];
        
        if (isset($data['username'])) {
            $updates[] = "username = ?";
            $params[] = $data['username'];
        }
        
        if (isset($data['email'])) {
            $updates[] = "email = ?";
            $params[] = $data['email'];
        }
        
        if (isset($data['password']) && !empty($data['password'])) {
            $updates[] = "password_hash = ?";
            $params[] = password_hash($data['password'], PASSWORD_BCRYPT);
        }
        
        if (empty($updates)) {
            return false;
        }
        
        $params[] = $userId;
        $sql = "UPDATE users SET " . implode(', ', $updates) . " WHERE id = ?";
        
        $stmt = $db->prepare($sql);
        $stmt->execute($params);
        
        return $stmt->rowCount() > 0;
    } catch (PDOException $e) {
        if (DEBUG) {
            error_log('Błąd aktualizacji użytkownika: ' . $e->getMessage());
        }
        return false;
    }
}

/**
 * Tworzy nowego użytkownika admin
 * @param array $data
 * @return array (success, message, userId)
 */
function create_admin_user($data) {
    $db = get_db_connection();
    
    // Walidacja
    if (empty($data['username']) || empty($data['email']) || empty($data['password'])) {
        return ['success' => false, 'message' => 'Wszystkie pola są wymagane'];
    }
    
    // Sprawdź czy użytkownik lub email już istnieje
    $stmt = $db->prepare("SELECT id FROM users WHERE username = ? OR email = ?");
    $stmt->execute([$data['username'], $data['email']]);
    
    if ($stmt->rowCount() > 0) {
        return ['success' => false, 'message' => 'Nazwa użytkownika lub email już istnieje'];
    }
    
    try {
        $passwordHash = password_hash($data['password'], PASSWORD_BCRYPT);
        
        $stmt = $db->prepare("INSERT INTO users (username, password_hash, email, created_at) 
                              VALUES (?, ?, ?, NOW())");
        $stmt->execute([$data['username'], $passwordHash, $data['email']]);
        
        $userId = $db->lastInsertId();
        
        log_activity($userId, 'user_created', 'New admin user created');
        
        return ['success' => true, 'message' => 'Użytkownik utworzony pomyślnie', 'userId' => $userId];
    } catch (PDOException $e) {
        if (DEBUG) {
            error_log('Błąd tworzenia użytkownika: ' . $e->getMessage());
        }
        return ['success' => false, 'message' => 'Wystąpił błąd podczas tworzenia użytkownika'];
    }
}

/**
 * Usuwa użytkownika
 * @param int $userId
 * @return bool
 */
function delete_admin_user($userId) {
    $db = get_db_connection();
    
    // Nie można usunąć własnego konta
    if (isset($_SESSION['admin_user']['id']) && $_SESSION['admin_user']['id'] == $userId) {
        return false;
    }
    
    try {
        $stmt = $db->prepare("DELETE FROM users WHERE id = ?");
        $stmt->execute([$userId]);
        
        log_activity(
            $_SESSION['admin_user']['id'] ?? null,
            'user_deleted',
            "Deleted user ID: $userId"
        );
        
        return $stmt->rowCount() > 0;
    } catch (PDOException $e) {
        if (DEBUG) {
            error_log('Błąd usuwania użytkownika: ' . $e->getMessage());
        }
        return false;
    }
}
