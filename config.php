<?php
/**
 * Konfiguracja główna portfolio
 * Plik powinien być chroniony przed dostępem z zewnątrz
 */

// ============================================
// USTAWIENIA BAZY DANYCH
// ============================================
define('DB_HOST', 'localhost');      // Host bazy danych
define('DB_NAME', 'mijsyspl_pag');   // Nazwa bazy danych
define('DB_USER', 'mijsyspl_pag');    // Użytkownik bazy danych
define('DB_PASS', 'TwojeHasloTu');    // Hasło do bazy danych - ZMIEŃ NA SWOJE!
define('DB_CHARSET', 'utf8mb4');     // Kodowanie (obsługa emoji)

// ============================================
// USTAWIENIA STRONY
// ============================================
define('SITE_TITLE', 'Paweł Gajda | Web Developer + Photographer');
define('SITE_DESCRIPTION', 'Portfolio projektów webowych i fotografii - Paweł Gajda');
define('SITE_URL', 'https://mijsys.pl/nowe'); // Zmień na swoją domenę

// ============================================
// USTAWIENIA ŚCIEŻEK
// ============================================
define('ROOT_PATH', __DIR__ . DIRECTORY_SEPARATOR);
define('ASSETS_PATH', ROOT_PATH . 'assets' . DIRECTORY_SEPARATOR);
define('UPLOADS_PATH', ROOT_PATH . 'uploads' . DIRECTORY_SEPARATOR);
define('INCLUDES_PATH', ROOT_PATH . 'includes' . DIRECTORY_SEPARATOR);
define('TEMPLATES_PATH', ROOT_PATH . 'templates' . DIRECTORY_SEPARATOR);

// URL do assetów
define('ASSETS_URL', SITE_URL . '/assets');
define('UPLOADS_URL', SITE_URL . '/uploads');

// ============================================
// USTAWIENIA BEZPIECZEŃSTWA
// ============================================
define('SESSION_NAME', 'PAG_PORTFOLIO_SESS');
define('CSRF_TOKEN_NAME', 'csrf_token');
define('MAX_LOGIN_ATTEMPTS', 5);            // Maksymalna liczba prób logowania
define('LOGIN_LOCKOUT_TIME', 15 * 60);      // Czas blokady w sekundach (15 minut)

// ============================================
// USTAWIENIA UPLOADU
// ============================================
define('MAX_UPLOAD_SIZE', 10 * 1024 * 1024); // 10MB
define('ALLOWED_IMAGE_TYPES', ['image/jpeg', 'image/png', 'image/webp']);
define('ALLOWED_EXTENSIONS', ['jpg', 'jpeg', 'png', 'webp']);

// Wymiary miniatur
define('THUMB_WIDTH', 400);
define('THUMB_HEIGHT', 300);

// ============================================
// USTAWIENIA MAIL (dla formularza kontaktowego)
// ============================================
define('MAIL_TO', 'twoj@email.com');         // Adres email, na który będą wysyłane wiadomości
define('MAIL_FROM', 'noreply@pag-portfolio.com');
define('MAIL_SUBJECT_PREFIX', '[Portfolio] ');

// ============================================
// USTAWIENIA PAGINACJI
// ============================================
define('PROJECTS_PER_PAGE', 6);
define('GALLERY_IMAGES_PER_PAGE', 12);

// ============================================
// USTAWIENIA DEBUG
// ============================================
define('DEBUG', true); // Wyłącz w produkcji!

if (DEBUG) {
    ini_set('display_errors', 1);
    ini_set('display_startup_errors', 1);
    error_reporting(E_ALL);
} else {
    ini_set('display_errors', 0);
    ini_set('display_startup_errors', 0);
    error_reporting(0);
}

// ============================================
// AUTO-LOAD KLAS (opcjonalnie, jeśli będziemy używać)
// ============================================
spl_autoload_register(function ($class) {
    $file = ROOT_PATH . 'classes/' . str_replace('\\', '/', $class) . '.php';
    if (file_exists($file)) {
        require $file;
    }
});

// ============================================
// USTAWIENIA CIASTECZEK SESJI (MUSZĄ BYĆ PRZED session_start()!)
// ============================================
ini_set('session.cookie_httponly', 1);
ini_set('session.cookie_secure', 0);     // Zmień na 1 w produkcji (HTTPS)
ini_set('session.cookie_samesite', 'Strict');

// ============================================
// ROZPOCZĘCIE SESJI
// ============================================
session_name(SESSION_NAME);
session_start();

// Regeneracja ID sesji co 30 minut dla bezpieczeństwa
if (!isset($_SESSION['last_regeneration']) || 
    time() - $_SESSION['last_regeneration'] > 1800) {
    session_regenerate_id(true);
    $_SESSION['last_regeneration'] = time();
}
