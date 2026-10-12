<?php
/**
 * Skrypt instalacyjny portfolio
 * Tworzy bazę danych, tabele i podstawowe dane
 */

// Sprawdź czy instalacja została już wykonana
if (file_exists(__DIR__ . '/../config.php') && 
    file_exists(__DIR__ . '/../.htaccess')) {
    header('Location: /');
    exit;
}

// Obsługa formularza instalacji
if ($_SERVER['REQUEST_METHOD'] === 'POST') {
    // Sprawdź token CSRF (jeśli istnieje)
    session_start();
    
    if (!isset($_POST['csrf_token']) || !isset($_SESSION['csrf_token']) || 
        $_POST['csrf_token'] !== $_SESSION['csrf_token']) {
        die('Nieważny token CSRF');
    }
    
    // Pobierz dane z formularza
    $dbHost = isset($_POST['db_host']) ? trim($_POST['db_host']) : 'localhost';
    $dbName = isset($_POST['db_name']) ? trim($_POST['db_name']) : 'mijsyspl_nowe';
    $dbUser = isset($_POST['db_user']) ? trim($_POST['db_user']) : 'mijsyspl_nowe';
    $dbPass = isset($_POST['db_pass']) ? $_POST['db_pass'] : 'FTCCn3wNNBRTEj86sW8e';
    $adminUsername = isset($_POST['admin_username']) ? trim($_POST['admin_username']) : 'admin';
    $adminPassword = isset($_POST['admin_password']) ? $_POST['admin_password'] : 'Patryk1991!/';
    $adminEmail = isset($_POST['admin_email']) ? trim($_POST['admin_email']) : 'admin@mijsys.pl';
    $siteTitle = isset($_POST['site_title']) ? trim($_POST['site_title']) : 'Patryk Szkudlarek | Portfolio';
    $siteDescription = isset($_POST['site_description']) ? trim($_POST['site_description']) : 'Portfolio projektów webowych i fotografii';
    
    // Walidacja
    $errors = [];
    
    if (empty($dbName)) {
        $errors[] = 'Nazwa bazy danych jest wymagana';
    }
    
    if (empty($adminUsername)) {
        $errors[] = 'Nazwa użytkownika admina jest wymagana';
    }
    
    if (empty($adminPassword) || strlen($adminPassword) < 8) {
        $errors[] = 'Hasło admina musi mieć co najmniej 8 znaków';
    }
    
    if (empty($adminEmail) || !filter_var($adminEmail, FILTER_VALIDATE_EMAIL)) {
        $errors[] = 'Poprawny adres email jest wymagany';
    }
    
    if (empty($errors)) {
        try {
            // 1. Utwórz plik config.php
            $configContent = <<<CONFIG
<?php
/**
 * Konfiguracja główna portfolio
 */

// USTAWIENIA CIASTECZEK SESJI (MUSZĄ BYĆ PRZED session_start!)
ini_set('session.cookie_httponly', 1);
ini_set('session.cookie_secure', 0);
ini_set('session.cookie_samesite', 'Strict');

// USTAWIENIA BAZY DANYCH
define('DB_HOST', '{$dbHost}');
define('DB_NAME', '{$dbName}');
define('DB_USER', '{$dbUser}');
define('DB_PASS', '{$dbPass}');
define('DB_CHARSET', 'utf8mb4');

// USTAWIENIA STRONY
define('SITE_TITLE', '{$siteTitle}');
define('SITE_DESCRIPTION', '{$siteDescription}');
define('SITE_URL', 'http://' . \$_SERVER['HTTP_HOST'] . str_replace('/install', '', dirname(\$_SERVER['SCRIPT_NAME'])));

// USTAWIENIA BEZPIECZEŃSTWA
define('SESSION_NAME', 'PAG_PORTFOLIO_SESS');
define('CSRF_TOKEN_NAME', 'csrf_token');
define('MAX_LOGIN_ATTEMPTS', 5);
define('LOGIN_LOCKOUT_TIME', 15 * 60);

// USTAWIENIA UPLOADU
define('MAX_UPLOAD_SIZE', 10 * 1024 * 1024);
define('ALLOWED_IMAGE_TYPES', ['image/jpeg', 'image/png', 'image/webp']);
define('ALLOWED_EXTENSIONS', ['jpg', 'jpeg', 'png', 'webp']);

define('THUMB_WIDTH', 400);
define('THUMB_HEIGHT', 300);

// USTAWIENIA MAIL
define('MAIL_TO', '{$adminEmail}');
define('MAIL_FROM', 'noreply@' . \$_SERVER['HTTP_HOST']);
define('MAIL_SUBJECT_PREFIX', '[Portfolio] ');

// USTAWIENIA DEBUG
define('DEBUG', false);

// ROZPOCZĘCIE SESJI
session_name(SESSION_NAME);
session_start();

// Regeneracja ID sesji
if (!isset(\$_SESSION['last_regeneration']) || 
    time() - \$_SESSION['last_regeneration'] > 1800) {
    session_regenerate_id(true);
    \$_SESSION['last_regeneration'] = time();
}
CONFIG;
            
            file_put_contents(__DIR__ . '/../config.php', $configContent);
            
            // 2. Połącz z bazą danych i utwórz tabele
            $dsn = "mysql:host=$dbHost;charset=utf8mb4";
            $pdo = new PDO($dsn, $dbUser, $dbPass, [
                PDO::ATTR_ERRMODE => PDO::ERRMODE_EXCEPTION,
                PDO::ATTR_DEFAULT_FETCH_MODE => PDO::FETCH_ASSOC
            ]);
            
            // Utwórz bazę danych
            $pdo->exec("CREATE DATABASE IF NOT EXISTS `$dbName` CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci");
            
            // Wybierz bazę danych
            $pdo->exec("USE `$dbName`");
            
            // Utwórz tabele
            $tables = [
                "CREATE TABLE IF NOT EXISTS `users` (
                    `id` INT AUTO_INCREMENT PRIMARY KEY,
                    `username` VARCHAR(50) NOT NULL UNIQUE,
                    `password_hash` VARCHAR(255) NOT NULL,
                    `email` VARCHAR(100) NOT NULL UNIQUE,
                    `last_login` DATETIME NULL,
                    `failed_attempts` INT DEFAULT 0,
                    `lock_until` DATETIME NULL,
                    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                ) ENGINE=InnoDB",
                
                "CREATE TABLE IF NOT EXISTS `settings` (
                    `id` INT AUTO_INCREMENT PRIMARY KEY,
                    `site_title` VARCHAR(100) NOT NULL DEFAULT '{$siteTitle}',
                    `site_description` TEXT NOT NULL DEFAULT '{$siteDescription}',
                    `theme` ENUM('light', 'dark') NOT NULL DEFAULT 'light',
                    `github_url` VARCHAR(255) NULL,
                    `linkedin_url` VARCHAR(255) NULL,
                    `instagram_url` VARCHAR(255) NULL,
                    `email` VARCHAR(100) NOT NULL DEFAULT '{$adminEmail}',
                    `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
                ) ENGINE=InnoDB",
                
                "CREATE TABLE IF NOT EXISTS `about` (
                    `id` INT AUTO_INCREMENT PRIMARY KEY,
                    `title` VARCHAR(100) NOT NULL DEFAULT 'O mnie',
                    `content` TEXT NOT NULL DEFAULT 'Tworzę unikalne strony internetowe i dokumentuję świat przez obiektyw.',
                    `image_path` VARCHAR(255) NULL,
                    `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
                ) ENGINE=InnoDB",
                
                "CREATE TABLE IF NOT EXISTS `project_categories` (
                    `id` INT AUTO_INCREMENT PRIMARY KEY,
                    `name` VARCHAR(50) NOT NULL UNIQUE,
                    `slug` VARCHAR(50) NOT NULL UNIQUE,
                    `icon` VARCHAR(20) NULL,
                    `order_index` INT DEFAULT 0
                ) ENGINE=InnoDB",
                
                "CREATE TABLE IF NOT EXISTS `projects` (
                    `id` INT AUTO_INCREMENT PRIMARY KEY,
                    `title` VARCHAR(100) NOT NULL,
                    `slug` VARCHAR(100) NOT NULL UNIQUE,
                    `description` TEXT NOT NULL,
                    `short_description` VARCHAR(255) NULL,
                    `category_id` INT NULL,
                    `technologies` TEXT NULL,
                    `live_url` VARCHAR(255) NULL,
                    `github_url` VARCHAR(255) NULL,
                    `cover_image` VARCHAR(255) NULL,
                    `gallery_images` TEXT NULL,
                    `order_index` INT DEFAULT 0,
                    `is_published` BOOLEAN DEFAULT TRUE,
                    `views` INT DEFAULT 0,
                    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                    FOREIGN KEY (`category_id`) REFERENCES `project_categories`(`id`) ON DELETE SET NULL
                ) ENGINE=InnoDB",
                
                "CREATE TABLE IF NOT EXISTS `gallery_albums` (
                    `id` INT AUTO_INCREMENT PRIMARY KEY,
                    `name` VARCHAR(100) NOT NULL,
                    `slug` VARCHAR(100) NOT NULL UNIQUE,
                    `description` TEXT NULL,
                    `cover_image` VARCHAR(255) NULL,
                    `order_index` INT DEFAULT 0,
                    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                ) ENGINE=InnoDB",
                
                "CREATE TABLE IF NOT EXISTS `gallery_images` (
                    `id` INT AUTO_INCREMENT PRIMARY KEY,
                    `album_id` INT NULL,
                    `title` VARCHAR(100) NULL,
                    `description` TEXT NULL,
                    `image_path` VARCHAR(255) NOT NULL,
                    `thumbnail_path` VARCHAR(255) NOT NULL,
                    `webp_path` VARCHAR(255) NULL,
                    `alt_text` VARCHAR(255) NULL,
                    `order_index` INT DEFAULT 0,
                    `is_published` BOOLEAN DEFAULT TRUE,
                    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY (`album_id`) REFERENCES `gallery_albums`(`id`) ON DELETE SET NULL
                ) ENGINE=InnoDB",
                
                "CREATE TABLE IF NOT EXISTS `messages` (
                    `id` INT AUTO_INCREMENT PRIMARY KEY,
                    `name` VARCHAR(100) NOT NULL,
                    `email` VARCHAR(100) NOT NULL,
                    `subject` VARCHAR(255) NULL,
                    `message` TEXT NOT NULL,
                    `honeypot` VARCHAR(100) NULL,
                    `is_read` BOOLEAN DEFAULT FALSE,
                    `ip_address` VARCHAR(45) NULL,
                    `user_agent` VARCHAR(255) NULL,
                    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                ) ENGINE=InnoDB",
                
                "CREATE TABLE IF NOT EXISTS `activity_logs` (
                    `id` INT AUTO_INCREMENT PRIMARY KEY,
                    `user_id` INT NULL,
                    `action` VARCHAR(100) NOT NULL,
                    `details` TEXT NULL,
                    `ip_address` VARCHAR(45) NULL,
                    `user_agent` VARCHAR(255) NULL,
                    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY (`user_id`) REFERENCES `users`(`id`) ON DELETE SET NULL
                ) ENGINE=InnoDB"
            ];
            
            foreach ($tables as $table) {
                $pdo->exec($table);
            }
            
            // 3. Utwórz użytkownika admin
            $passwordHash = password_hash($adminPassword, PASSWORD_BCRYPT);
            $pdo->exec("INSERT INTO `users` (username, password_hash, email) 
                        VALUES ('{$adminUsername}', '{$passwordHash}', '{$adminEmail}')");
            
            // 4. Utwórz domyślne ustawienia
            $pdo->exec("INSERT INTO `settings` (site_title, site_description, theme, email) 
                        VALUES ('{$siteTitle}', '{$siteDescription}', 'light', '{$adminEmail}')");
            
            // 5. Utwórz domyślną treść "O mnie"
            $pdo->exec("INSERT INTO `about` (title, content) 
                        VALUES ('O mnie', 'Od 10 lat łączę precyzję kodu z magią analogowej fotografii. Specjalizuję się w tworzeniu unikalnych doświadczeń webowych i dokumentowaniu świata przez obiektyw.')");
            
            // 6. Utwórz domyślne kategorie projektów
            $categories = [
                ['Web Development', 'web-development', '💻'],
                ['Frontend', 'frontend', '⚡'],
                ['Backend', 'backend', '🔧'],
                ['Fullstack', 'fullstack', '🏗️']
            ];
            
            foreach ($categories as $category) {
                $pdo->exec("INSERT INTO `project_categories` (name, slug, icon, order_index) 
                            VALUES ('{$category[0]}', '{$category[1]}', '{$category[2]}', 0)");
            }
            
            // 7. Utwórz domyślne albumy
            $albums = [
                ['Portrety', 'portrety'],
                ['Krajobraz', 'krajobraz'],
                ['Street', 'street']
            ];
            
            foreach ($albums as $album) {
                $pdo->exec("INSERT INTO `gallery_albums` (name, slug) 
                            VALUES ('{$album[0]}', '{$album[1]}')");
            }
            
            // 8. Utwórz katalog uploads
            if (!file_exists(__DIR__ . '/../uploads')) {
                mkdir(__DIR__ . '/../uploads', 0755, true);
                mkdir(__DIR__ . '/../uploads/projects', 0755, true);
                mkdir(__DIR__ . '/../uploads/gallery', 0755, true);
                mkdir(__DIR__ . '/../uploads/gallery/thumbs', 0755, true);
                mkdir(__DIR__ . '/../uploads/gallery/webp', 0755, true);
            }
            
            // 9. Utwórz plik .htaccess
            $htaccessContent = <<<HTACCESS
# PORTFOLIO - KONFIGURACJA APACHE

<IfModule mod_rewrite.c>
    RewriteEngine On
    RewriteBase /

    # Przyjazne URL-e
    RewriteCond %{REQUEST_FILENAME} !-d
    RewriteCond %{REQUEST_FILENAME}\.php -f
    RewriteRule ^(.*)$ $1.php [L]

    RewriteCond %{REQUEST_FILENAME} !-f
    RewriteCond %{REQUEST_FILENAME} !-d
    RewriteRule ^([^/]+)/?$ index.php?page=$1 [L,QSA]

    RewriteCond %{REQUEST_FILENAME} !-f
    RewriteCond %{REQUEST_FILENAME} !-d
    RewriteRule ^([^/]+)/([^/]+)/?$ index.php?page=$1&slug=$2 [L,QSA]

    RewriteCond %{REQUEST_FILENAME} !-f
    RewriteCond %{REQUEST_FILENAME} !-d
    RewriteRule ^admin/(.*)$ admin/index.php?admin_page=$1 [L,QSA]

    RewriteCond %{REQUEST_URI} ^/$
    RewriteRule ^(.*)$ index.php [L]
</IfModule>

# OCHRONA KATALOGU UPLOADS
<FilesMatch "\.(php|php5|phtml|phar)$">
    Deny from all
</FilesMatch>

<Directory "/uploads">
    php_flag engine off
    <FilesMatch "^\.ht">
        Order allow,deny
        Deny from all
    </FilesMatch>
</Directory>

# NAGŁÓWKI BEZPIECZEŃSTWA
<IfModule mod_headers.c>
    Header always set X-XSS-Protection "1; mode=block"
    Header always set X-Content-Type-Options "nosniff"
    Header always set X-Frame-Options "SAMEORIGIN"
    Header always set Referrer-Policy "strict-origin-when-cross-origin"
    Header always unset X-Powered-By
</IfModule>

# CACHE
<IfModule mod_expires.c>
    ExpiresActive On
    ExpiresByType image/jpg "access plus 1 year"
    ExpiresByType image/jpeg "access plus 1 year"
    ExpiresByType image/png "access plus 1 year"
    ExpiresByType image/webp "access plus 1 year"
    ExpiresByType text/css "access plus 1 month"
    ExpiresByType application/javascript "access plus 1 month"
</IfModule>

<IfModule mod_mime.c>
    AddType image/webp .webp
</IfModule>
HTACCESS;
            
            file_put_contents(__DIR__ . '/../.htaccess', $htaccessContent);
            
            // 10. Utwórz plik .htaccess dla uploads
            $uploadsHtaccess = <<<HTACCESS
# OCHRONA KATALOGU UPLOADS
<FilesMatch "\.(php|php5|phtml|phar|pl|py|jsp|asp|sh|cgi)$">
    Require all denied
</FilesMatch>

<FilesMatch "^\..*">
    Require all denied
</FilesMatch>

<FilesMatch "\.(jpg|jpeg|png|gif|webp|svg)$">
    Require all granted
</FilesMatch>

Require all denied
php_flag engine off
HTACCESS;
            
            file_put_contents(__DIR__ . '/../uploads/.htaccess', $uploadsHtaccess);
            
            // 11. Zakończenie - przekieruj do strony głównej
            header('Location: /');
            exit;
            
        } catch (PDOException $e) {
            $errors[] = 'Błąd połączenia z bazą danych: ' . $e->getMessage();
        }
    }
}

// Generuj token CSRF
session_start();
if (empty($_SESSION['csrf_token'])) {
    $_SESSION['csrf_token'] = bin2hex(random_bytes(32));
}
$csrfToken = $_SESSION['csrf_token'];

// Domyslne wartosci formularza
$defaults = [
    'db_host' => 'localhost',
    'db_name' => 'mijsyspl_nowe',
    'db_user' => 'mijsyspl_nowe',
    'db_pass' => 'FTCCn3wNNBRTEj86sW8e',
    'admin_username' => 'admin',
    'admin_email' => 'admin@mijsys.pl',
    'site_title' => 'Patryk Szkudlarek | Portfolio',
    'site_description' => 'Portfolio projektów webowych i fotografii'
];
?>

<!DOCTYPE html>
<html lang="pl">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Instalacja Portfolio</title>
    <link href="https://fonts.googleapis.com/css2?family=Bebas+Neue&family=Roboto+Mono:wght@300;400;700&display=swap" rel="stylesheet">
    <style>
        :root {
            --color-wood: #5C4033;
            --color-wood-dark: #3A2E28;
            --color-paper: #F8F4E6;
            --color-ink: #1A1A1A;
            --color-highlight: #4A90E2;
            --color-warning: #D32F2F;
            --spacing-xs: 0.5rem;
            --spacing-sm: 0.5rem;
            --spacing-md: 1rem;
            --spacing-lg: 1.5rem;
            --spacing-xl: 2rem;
            --radius-sm: 4px;
            --radius-md: 8px;
        }
        
        * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }
        
        body {
            font-family: 'Roboto Mono', monospace;
            background: linear-gradient(135deg, var(--color-wood-dark) 0%, var(--color-wood) 100%);
            color: var(--color-paper);
            min-height: 100vh;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            padding: var(--spacing-lg);
        }
        
        .install-container {
            width: 100%;
            max-width: 600px;
            background: var(--color-paper);
            color: var(--color-ink);
            border-radius: var(--radius-md);
            box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
            padding: var(--spacing-xl);
        }
        
        .install-header {
            text-align: center;
            margin-bottom: var(--spacing-xl);
        }
        
        .install-logo {
            font-family: 'Bebas Neue', sans-serif;
            font-size: 2rem;
            color: var(--color-wood-dark);
            margin-bottom: var(--spacing-sm);
        }
        
        .install-title {
            font-family: 'Bebas Neue', sans-serif;
            font-size: 1.5rem;
            color: var(--color-ink);
            margin-bottom: var(--spacing-sm);
        }
        
        .install-description {
            color: var(--color-ink);
            opacity: 0.8;
            font-size: 0.875rem;
        }
        
        .install-form {
            display: flex;
            flex-direction: column;
            gap: var(--spacing-lg);
        }
        
        .install-section {
            padding-bottom: var(--spacing-lg);
            border-bottom: 1px solid #ddd;
        }
        
        .install-section:last-child {
            border-bottom: none;
            padding-bottom: 0;
        }
        
        .install-section-title {
            font-family: 'Bebas Neue', sans-serif;
            font-size: 1.125rem;
            color: var(--color-wood-dark);
            margin-bottom: var(--spacing-md);
        }
        
        .install-form-group {
            margin-bottom: var(--spacing-md);
        }
        
        .install-form-label {
            display: block;
            font-size: 0.875rem;
            color: var(--color-ink);
            margin-bottom: var(--spacing-xs);
        }
        
        .install-form-input,
        .install-form-textarea,
        .install-form-select {
            width: 100%;
            padding: var(--spacing-sm) var(--spacing-md);
            border: 2px solid #ddd;
            border-radius: var(--radius-sm);
            font-family: 'Roboto Mono', monospace;
            font-size: 1rem;
            color: var(--color-ink);
            background: white;
        }
        
        .install-form-input:focus,
        .install-form-textarea:focus,
        .install-form-select:focus {
            outline: none;
            border-color: var(--color-highlight);
        }
        
        .install-form-textarea {
            resize: vertical;
            min-height: 80px;
        }
        
        .install-form-row {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: var(--spacing-md);
        }
        
        .install-form-error {
            display: block;
            font-size: 0.75rem;
            color: var(--color-warning);
            margin-top: var(--spacing-xs);
        }
        
        .install-form-actions {
            display: flex;
            gap: var(--spacing-md);
            justify-content: flex-end;
            padding-top: var(--spacing-lg);
        }
        
        .install-btn {
            padding: var(--spacing-md) var(--spacing-lg);
            border: none;
            border-radius: var(--radius-sm);
            font-family: 'Bebas Neue', sans-serif;
            font-size: 1rem;
            letter-spacing: 0.1em;
            text-transform: uppercase;
            cursor: pointer;
            transition: all 0.2s ease;
        }
        
        .install-btn-primary {
            background: var(--color-wood-dark);
            color: var(--color-paper);
        }
        
        .install-btn-primary:hover {
            background: var(--color-wood);
        }
        
        .install-btn-secondary {
            background: #f0f0f0;
            color: var(--color-ink);
        }
        
        .install-btn-secondary:hover {
            background: #e0e0e0;
        }
        
        .install-success {
            text-align: center;
            padding: var(--spacing-xl);
        }
        
        .success-icon {
            font-size: 4rem;
            color: var(--color-highlight);
            margin-bottom: var(--spacing-md);
        }
        
        .success-title {
            font-family: 'Bebas Neue', sans-serif;
            font-size: 1.5rem;
            color: var(--color-ink);
            margin-bottom: var(--spacing-sm);
        }
        
        .success-text {
            color: var(--color-ink);
            opacity: 0.8;
            margin-bottom: var(--spacing-lg);
        }
        
        @media (max-width: 600px) {
            .install-form-row {
                grid-template-columns: 1fr;
            }
        }
    </style>
</head>
<body>
    <div class="install-container">
        <div class="install-header">
            <div class="install-logo">🔨 PATRYK SZKUDLAREK</div>
            <h1 class="install-title">INSTALACJA PORTFOLIO</h1>
            <p class="install-description">Wypełnij formularz, aby zainstalować system</p>
        </div>
        
        <?php if (!empty($errors)): ?>
        <div style="background: rgba(211, 47, 47, 0.1); border: 1px solid var(--color-warning); border-radius: var(--radius-sm); padding: var(--spacing-md); margin-bottom: var(--spacing-lg);">
            <?php foreach ($errors as $error): ?>
            <p style="color: var(--color-warning); margin: 0;"><?php echo htmlspecialchars($error); ?></p>
            <?php endforeach; ?>
        </div>
        <?php endif; ?>
        
        <form class="install-form" method="POST">
            <input type="hidden" name="csrf_token" value="<?php echo $csrfToken; ?>">
            
            <!-- Baza danych -->
            <div class="install-section">
                <h2 class="install-section-title">Konfiguracja bazy danych</h2>
                
                <div class="install-form-row">
                    <div class="install-form-group">
                        <label for="db_host" class="install-form-label">Host</label>
                        <input type="text" 
                               id="db_host" 
                               name="db_host"
                               class="install-form-input"
                               value="<?php echo htmlspecialchars($defaults['db_host']); ?>"
                               placeholder="localhost">
                    </div>
                    
                    <div class="install-form-group">
                        <label for="db_name" class="install-form-label">Nazwa bazy *</label>
                        <input type="text" 
                               id="db_name" 
                               name="db_name"
                               class="install-form-input"
                               value="<?php echo htmlspecialchars($defaults['db_name']); ?>"
                               placeholder="pag_portfolio"
                               required>
                    </div>
                </div>
                
                <div class="install-form-row">
                    <div class="install-form-group">
                        <label for="db_user" class="install-form-label">Użytkownik</label>
                        <input type="text" 
                               id="db_user" 
                               name="db_user"
                               class="install-form-input"
                               value="<?php echo htmlspecialchars($defaults['db_user']); ?>"
                               placeholder="root">
                    </div>
                    
                    <div class="install-form-group">
                        <label for="db_pass" class="install-form-label">Hasło</label>
                        <input type="password" 
                               id="db_pass" 
                               name="db_pass"
                               class="install-form-input"
                               value="<?php echo htmlspecialchars($defaults['db_pass']); ?>"
                               placeholder="Hasło do bazy">
                    </div>
                </div>
            </div>
            
            <!-- Konto administratora -->
            <div class="install-section">
                <h2 class="install-section-title">Konto administratora</h2>
                
                <div class="install-form-row">
                    <div class="install-form-group">
                        <label for="admin_username" class="install-form-label">Nazwa użytkownika *</label>
                        <input type="text" 
                               id="admin_username" 
                               name="admin_username"
                               class="install-form-input"
                               value="<?php echo htmlspecialchars($defaults['admin_username']); ?>"
                               placeholder="admin"
                               required>
                    </div>
                    
                    <div class="install-form-group">
                        <label for="admin_email" class="install-form-label">Email *</label>
                        <input type="email" 
                               id="admin_email" 
                               name="admin_email"
                               class="install-form-input"
                               value="<?php echo htmlspecialchars($defaults['admin_email']); ?>"
                               placeholder="twój@email.com"
                               required>
                    </div>
                </div>
                
                <div class="install-form-row">
                    <div class="install-form-group">
                        <label for="admin_password" class="install-form-label">Hasło *</label>
                        <input type="password" 
                               id="admin_password" 
                               name="admin_password"
                               class="install-form-input"
                               placeholder="Hasło (min. 8 znaków)"
                               required
                               minlength="8">
                    </div>
                    
                    <div class="install-form-group">
                        <label for="admin_password_confirm" class="install-form-label">Powtórz hasło *</label>
                        <input type="password" 
                               id="admin_password_confirm" 
                               name="admin_password_confirm"
                               class="install-form-input"
                               placeholder="Powtórz hasło"
                               required
                               minlength="8">
                    </div>
                </div>
            </div>
            
            <!-- Ustawienia strony -->
            <div class="install-section">
                <h2 class="install-section-title">Ustawienia strony</h2>
                
                <div class="install-form-group">
                    <label for="site_title" class="install-form-label">Tytuł strony *</label>
                    <input type="text" 
                           id="site_title" 
                           name="site_title"
                           class="install-form-input"
                           value="<?php echo htmlspecialchars($defaults['site_title']); ?>"
                           placeholder="Tytuł strony"
                           required>
                </div>
                
                <div class="install-form-group">
                    <label for="site_description" class="install-form-label">Opis strony</label>
                    <textarea id="site_description" 
                              name="site_description"
                              class="install-form-textarea"
                              placeholder="Opis strony (meta description)"><?php echo htmlspecialchars($defaults['site_description']); ?></textarea>
                </div>
            </div>
            
            <!-- Akcje -->
            <div class="install-form-actions">
                <button type="button" class="install-btn install-btn-secondary" onclick="window.location.reload()">
                    WYCZYŚĆ
                </button>
                <button type="submit" class="install-btn install-btn-primary">
                    ZAINSTALUJ
                </button>
            </div>
        </form>
    </div>
    
    <script>
    // Walidacja formularza
    const form = document.querySelector('.install-form');
    const passwordInput = document.getElementById('admin_password');
    const passwordConfirmInput = document.getElementById('admin_password_confirm');
    
    if (form && passwordInput && passwordConfirmInput) {
        form.addEventListener('submit', (e) => {
            if (passwordInput.value !== passwordConfirmInput.value) {
                e.preventDefault();
                alert('Hasła nie są identyczne');
            }
        });
    }
    </script>
</body>
</html>
