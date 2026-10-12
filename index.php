<?php
/**
 * Główny plik wejściowy - Router
 * Obsługuje przyjazne URL-e i wyświetla odpowiednie strony
 */

require_once 'config.php';
require_once INCLUDES_PATH . 'db.php';
require_once INCLUDES_PATH . 'functions.php';
require_once INCLUDES_PATH . 'csrf.php';

// Pobierz parametr strony z URL
$page = isset($_GET['page']) ? trim($_GET['page']) : 'home';
$slug = isset($_GET['slug']) ? trim($_GET['slug']) : null;

// Obsługa AJAX (dla dynamicznego ładowania treści)
if (isset($_SERVER['HTTP_X_REQUESTED_WITH']) && 
    strtolower($_SERVER['HTTP_X_REQUESTED_WITH']) === 'xmlhttprequest') {
    
    // Sprawdź token CSRF dla żądań POST
    if ($_SERVER['REQUEST_METHOD'] === 'POST') {
        if (!isset($_POST[CSRF_TOKEN_NAME]) || !validate_csrf_token($_POST[CSRF_TOKEN_NAME])) {
            header('Content-Type: application/json');
            echo json_encode(['success' => false, 'error' => 'Nieważny token CSRF']);
            exit;
        }
    }
    
    // Obsługa różnych typów żądań AJAX
    switch ($page) {
        case 'load-projects':
            load_projects_ajax($slug);
            exit;
            
        case 'load-gallery':
            load_gallery_ajax($slug);
            exit;
            
        case 'load-about':
            load_about_ajax();
            exit;
            
        case 'send-message':
            send_contact_message();
            exit;
            
        default:
            header('HTTP/1.1 404 Not Found');
            echo json_encode(['success' => false, 'error' => 'Nieznane żądanie']);
            exit;
    }
}

// Pobierz ustawienia strony z bazy
$settings = get_settings();

// Ustaw tytuły stron
$pageTitles = [
    'home' => $settings['site_title'] ?? SITE_TITLE,
    'projekty' => 'Projekty - ' . ($settings['site_title'] ?? SITE_TITLE),
    'galeria' => 'Galeria - ' . ($settings['site_title'] ?? SITE_TITLE),
    'o-mnie' => 'O mnie - ' . ($settings['site_title'] ?? SITE_TITLE),
    'kontakt' => 'Kontakt - ' . ($settings['site_title'] ?? SITE_TITLE),
];

$currentPageTitle = $pageTitles[$page] ?? ($settings['site_title'] ?? SITE_TITLE);
$currentPageDescription = $settings['site_description'] ?? SITE_DESCRIPTION;

// Wczytanie nagłówka
require_once INCLUDES_PATH . 'header.php';

// Wczytanie odpowiedniej strony
switch ($page) {
    case 'projekty':
        require_once TEMPLATES_PATH . 'projects.php';
        break;
        
    case 'galeria':
        require_once TEMPLATES_PATH . 'gallery.php';
        break;
        
    case 'o-mnie':
        require_once TEMPLATES_PATH . 'about.php';
        break;
        
    case 'kontakt':
        require_once TEMPLATES_PATH . 'contact.php';
        break;
        
    case 'projekt':
        if ($slug) {
            require_once TEMPLATES_PATH . 'project-detail.php';
        } else {
            header("HTTP/1.1 404 Not Found");
            require_once TEMPLATES_PATH . '404.php';
        }
        break;
        
    case 'album':
        if ($slug) {
            require_once TEMPLATES_PATH . 'album-detail.php';
        } else {
            header("HTTP/1.1 404 Not Found");
            require_once TEMPLATES_PATH . '404.php';
        }
        break;
        
    default:
        require_once TEMPLATES_PATH . 'hero.php';
        require_once TEMPLATES_PATH . 'projects.php';
        require_once TEMPLATES_PATH . 'gallery.php';
        require_once TEMPLATES_PATH . 'about.php';
        require_once TEMPLATES_PATH . 'contact.php';
        break;
}

// Wczytanie stopki
require_once INCLUDES_PATH . 'footer.php';

/**
 * Funkcje obsługi AJAX
 */

/**
 * Ładowanie projektów przez AJAX
 */
function load_projects_ajax($categorySlug = null) {
    $db = get_db_connection();
    
    $categoryId = null;
    if ($categorySlug) {
        $stmt = $db->prepare("SELECT id FROM project_categories WHERE slug = ?");
        $stmt->execute([$categorySlug]);
        $category = $stmt->fetch(PDO::FETCH_ASSOC);
        if ($category) {
            $categoryId = $category['id'];
        }
    }
    
    $query = "SELECT p.*, pc.name as category_name, pc.slug as category_slug 
              FROM projects p 
              LEFT JOIN project_categories pc ON p.category_id = pc.id 
              WHERE p.is_published = 1";
    $params = [];
    
    if ($categoryId) {
        $query .= " AND p.category_id = ?";
        $params[] = $categoryId;
    }
    
    $query .= " ORDER BY p.order_index ASC";
    
    $stmt = $db->prepare($query);
    $stmt->execute($params);
    $projects = $stmt->fetchAll(PDO::FETCH_ASSOC);
    
    header('Content-Type: application/json');
    echo json_encode(['success' => true, 'projects' => $projects]);
}

/**
 * Ładowanie galerii przez AJAX
 */
function load_gallery_ajax($albumSlug = null) {
    $db = get_db_connection();
    
    $albumId = null;
    if ($albumSlug) {
        $stmt = $db->prepare("SELECT id FROM gallery_albums WHERE slug = ?");
        $stmt->execute([$albumSlug]);
        $album = $stmt->fetch(PDO::FETCH_ASSOC);
        if ($album) {
            $albumId = $album['id'];
        }
    }
    
    $query = "SELECT * FROM gallery_images WHERE is_published = 1";
    $params = [];
    
    if ($albumId) {
        $query .= " AND album_id = ?";
        $params[] = $albumId;
    }
    
    $query .= " ORDER BY order_index ASC";
    
    $stmt = $db->prepare($query);
    $stmt->execute($params);
    $images = $stmt->fetchAll(PDO::FETCH_ASSOC);
    
    header('Content-Type: application/json');
    echo json_encode(['success' => true, 'images' => $images]);
}

/**
 * Ładowanie sekcji "O mnie" przez AJAX
 */
function load_about_ajax() {
    $db = get_db_connection();
    
    $stmt = $db->prepare("SELECT * FROM about LIMIT 1");
    $stmt->execute();
    $about = $stmt->fetch(PDO::FETCH_ASSOC);
    
    header('Content-Type: application/json');
    echo json_encode(['success' => true, 'about' => $about]);
}

/**
 * Wysyłanie wiadomości kontaktowej
 */
function send_contact_message() {
    $db = get_db_connection();
    
    // Walidacja danych
    $errors = [];
    
    $name = isset($_POST['name']) ? trim($_POST['name']) : '';
    $email = isset($_POST['email']) ? trim($_POST['email']) : '';
    $message = isset($_POST['message']) ? trim($_POST['message']) : '';
    $honeypot = isset($_POST['honeypot']) ? trim($_POST['honeypot']) : '';
    
    // Sprawdź honeypot (powinien być pusty)
    if (!empty($honeypot)) {
        header('Content-Type: application/json');
        echo json_encode(['success' => false, 'error' => 'Spam detected']);
        exit;
    }
    
    // Walidacja
    if (empty($name)) {
        $errors[] = 'Imię jest wymagane';
    }
    
    if (empty($email) || !filter_var($email, FILTER_VALIDATE_EMAIL)) {
        $errors[] = 'Poprawny adres email jest wymagany';
    }
    
    if (empty($message) || strlen($message) < 10) {
        $errors[] = 'Wiadomość musi mieć co najmniej 10 znaków';
    }
    
    if (!empty($errors)) {
        header('Content-Type: application/json');
        echo json_encode(['success' => false, 'errors' => $errors]);
        exit;
    }
    
    // Sanityzacja
    $name = sanitize_input($name);
    $email = sanitize_input($email);
    $message = sanitize_input($message);
    
    // Zapisz do bazy
    $stmt = $db->prepare("INSERT INTO messages (name, email, message, ip_address, user_agent) 
                          VALUES (?, ?, ?, ?, ?)");
    $stmt->execute([
        $name,
        $email,
        $message,
        $_SERVER['REMOTE_ADDR'] ?? null,
        $_SERVER['HTTP_USER_AGENT'] ?? null
    ]);
    
    // Opcjonalnie: Wyślij email
    $mailSent = false;
    if (defined('MAIL_TO') && MAIL_TO) {
        $subject = MAIL_SUBJECT_PREFIX . 'Nowa wiadomość od ' . $name;
        $headers = "From: " . MAIL_FROM . "\r\n";
        $headers .= "Reply-To: " . $email . "\r\n";
        $headers .= "Content-Type: text/plain; charset=UTF-8\r\n";
        
        $mailBody = "Imię: " . $name . "\n";
        $mailBody .= "Email: " . $email . "\n";
        $mailBody .= "Wiadomość:\n" . $message . "\n";
        $mailBody .= "\n---\n";
        $mailBody .= "IP: " . ($_SERVER['REMOTE_ADDR'] ?? 'unknown') . "\n";
        $mailBody .= "User Agent: " . ($_SERVER['HTTP_USER_AGENT'] ?? 'unknown') . "\n";
        
        $mailSent = mail(MAIL_TO, $subject, $mailBody, $headers);
    }
    
    header('Content-Type: application/json');
    echo json_encode([
        'success' => true,
        'message' => 'Wiadomość została wysłana pomyślnie!',
        'mail_sent' => $mailSent
    ]);
}
