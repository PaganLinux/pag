<?php
/**
 * Panel administracyjny - Logowanie
 * Strona logowania dla administratorów
 */

require_once '../config.php';
require_once '../includes/db.php';
require_once '../includes/functions.php';
require_once '../includes/csrf.php';
require_once '../includes/auth.php';

// Jeśli użytkownik jest już zalogowany, przekieruj do dashboard
if (is_admin_logged_in()) {
    header('Location: /admin');
    exit;
}

// Inicjalizacja zmiennych
$error = '';
$username = '';

// Obsługa formularza logowania
if ($_SERVER['REQUEST_METHOD'] === 'POST') {
    $username = isset($_POST['username']) ? trim($_POST['username']) : '';
    $password = isset($_POST['password']) ? $_POST['password'] : '';
    
    // Sprawdź token CSRF
    if (!isset($_POST[CSRF_TOKEN_NAME]) || !validate_csrf_token($_POST[CSRF_TOKEN_NAME])) {
        $error = 'Nieważny token bezpieczeństwa. Odśwież stronę i spróbuj ponownie.';
    } else {
        // Próba logowania
        $result = admin_login($username, $password);
        
        if ($result['success']) {
            // Zalogowano pomyślnie, przekieruj do dashboard
            header('Location: /admin');
            exit;
        } else {
            $error = $result['message'];
        }
    }
}

// Ustaw tytuł strony
$pageTitle = 'Logowanie - Panel Administracyjny';
?>
<!DOCTYPE html>
<html lang="pl">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="robots" content="noindex, nofollow">
    <title><?php echo escape_output($pageTitle); ?></title>
    
    <!-- Google Fonts -->
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Bebas+Neue&family=Roboto+Mono:wght@300;400;700&display=swap" rel="stylesheet">
    
    <!-- Main CSS -->
    <link rel="stylesheet" href="<?php echo ASSETS_URL; ?>/css/style.css">
    
    <!-- Admin CSS -->
    <style>
        body {
            background: linear-gradient(135deg, var(--color-wood-dark) 0%, var(--color-wood) 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            padding: var(--spacing-xl);
        }
        
        .login-container {
            width: 100%;
            max-width: 400px;
            background: var(--color-paper);
            border-radius: var(--radius-lg);
            box-shadow: var(--shadow-xl);
            padding: var(--spacing-2xl);
        }
        
        .login-header {
            text-align: center;
            margin-bottom: var(--spacing-xl);
        }
        
        .login-logo {
            font-family: var(--font-primary);
            font-size: 2rem;
            color: var(--color-wood-dark);
            margin-bottom: var(--spacing-sm);
        }
        
        .login-title {
            font-family: var(--font-primary);
            font-size: 1.5rem;
            color: var(--color-ink);
        }
        
        .login-form {
            display: flex;
            flex-direction: column;
            gap: var(--spacing-lg);
        }
        
        .form-group {
            position: relative;
        }
        
        .form-label {
            display: block;
            font-family: var(--font-primary);
            font-size: 0.875rem;
            margin-bottom: var(--spacing-xs);
            color: var(--color-ink);
        }
        
        .form-input {
            width: 100%;
            padding: var(--spacing-sm) var(--spacing-md);
            border: 2px solid var(--color-metal);
            border-radius: var(--radius-sm);
            font-family: var(--font-secondary);
            font-size: 1rem;
            color: var(--color-ink);
            background: var(--color-paper-dark);
            transition: all var(--transition-fast);
        }
        
        .form-input:focus {
            outline: none;
            border-color: var(--color-highlight);
            box-shadow: 0 0 0 3px rgba(74, 144, 226, 0.1);
        }
        
        .form-input::placeholder {
            color: var(--color-ink-light);
        }
        
        .form-error {
            display: block;
            font-size: 0.75rem;
            color: var(--color-warning);
            margin-top: var(--spacing-xs);
        }
        
        .login-actions {
            display: flex;
            flex-direction: column;
            gap: var(--spacing-md);
        }
        
        .login-btn {
            width: 100%;
            padding: var(--spacing-md) var(--spacing-lg);
            background: var(--color-wood-dark);
            color: var(--color-paper);
            border: none;
            border-radius: var(--radius-sm);
            font-family: var(--font-primary);
            font-size: 1rem;
            letter-spacing: 0.1em;
            text-transform: uppercase;
            cursor: pointer;
            transition: all var(--transition-fast);
        }
        
        .login-btn:hover {
            background: var(--color-wood);
        }
        
        .login-footer {
            text-align: center;
            margin-top: var(--spacing-lg);
            padding-top: var(--spacing-lg);
            border-top: 1px solid var(--color-metal-light);
        }
        
        .login-footer a {
            color: var(--color-highlight);
            font-size: 0.875rem;
        }
        
        .login-footer a:hover {
            text-decoration: underline;
        }
        
        .login-decoration {
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            pointer-events: none;
            overflow: hidden;
        }
        
        .login-decoration::before,
        .login-decoration::after {
            content: '';
            position: absolute;
            width: 200px;
            height: 200px;
            background: var(--color-wood);
            border-radius: 50%;
            opacity: 0.1;
        }
        
        .login-decoration::before {
            top: -50px;
            right: -50px;
        }
        
        .login-decoration::after {
            bottom: -50px;
            left: -50px;
        }
        
        /* Animacje */
        .login-container {
            animation: fadeInUp 0.6s ease-out;
        }
        
        @keyframes fadeInUp {
            from {
                opacity: 0;
                transform: translateY(20px);
            }
            to {
                opacity: 1;
                transform: translateY(0);
            }
        }
        
        /* Responsive */
        @media (max-width: 480px) {
            body {
                padding: var(--spacing-md);
            }
            
            .login-container {
                padding: var(--spacing-lg);
            }
        }
    </style>
    
    <!-- CSRF Token -->
    <?php echo csrf_meta_tag(); ?>
</head>
<body>
    <div class="login-decoration"></div>
    
    <div class="login-container">
        <div class="login-header">
            <div class="login-logo">🔨 PAWEŁ GAJDA</div>
            <h1 class="login-title">PANEL ADMINISTRACYJNY</h1>
        </div>
        
        <form class="login-form" method="POST" action="">
            <?php echo csrf_input(); ?>
            
            <?php if (!empty($error)): ?>
            <div class="form-error" style="text-align: center; margin-bottom: var(--spacing-md);">
                <?php echo escape_output($error); ?>
            </div>
            <?php endif; ?>
            
            <div class="form-group">
                <label for="username" class="form-label">Nazwa użytkownika</label>
                <input type="text" 
                       id="username" 
                       name="username"
                       class="form-input"
                       value="<?php echo escape_output($username); ?>"
                       placeholder="Wprowadź nazwę użytkownika"
                       required
                       autocomplete="username">
            </div>
            
            <div class="form-group">
                <label for="password" class="form-label">Hasło</label>
                <input type="password" 
                       id="password" 
                       name="password"
                       class="form-input"
                       placeholder="Wprowadź hasło"
                       required
                       autocomplete="current-password">
            </div>
            
            <div class="login-actions">
                <button type="submit" class="login-btn">
                    ZALOGUJ SIĘ
                </button>
            </div>
        </form>
        
        <div class="login-footer">
            <a href="<?php echo SITE_URL; ?>">Powrót do strony głównej</a>
        </div>
    </div>
</body>
</html>
