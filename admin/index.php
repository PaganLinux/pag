<?php
/**
 * Panel administracyjny - Główny router
 * Obsługuje różne podstrony panelu admina
 */

require_once '../config.php';
require_once '../includes/db.php';
require_once '../includes/functions.php';
require_once '../includes/csrf.php';
require_once '../includes/auth.php';

// Sprawdź czy użytkownik jest zalogowany
check_admin_auth();

// Pobierz parametr podstrony
$adminPage = isset($_GET['admin_page']) ? trim($_GET['admin_page']) : 'dashboard';

// Obsługa AJAX
if (isset($_SERVER['HTTP_X_REQUESTED_WITH']) && 
    strtolower($_SERVER['HTTP_X_REQUESTED_WITH']) === 'xmlhttprequest') {
    
    // Sprawdź token CSRF
    if ($_SERVER['REQUEST_METHOD'] === 'POST') {
        if (!isset($_POST[CSRF_TOKEN_NAME]) || !validate_csrf_token($_POST[CSRF_TOKEN_NAME])) {
            header('Content-Type: application/json');
            echo json_encode(['success' => false, 'error' => 'Nieważny token CSRF']);
            exit;
        }
    }
    
    // Obsługa różnych typów żądań
    switch ($adminPage) {
        case 'login':
            // Nie powinno się tu dostać
            break;
            
        case 'get-stats':
            get_dashboard_stats();
            exit;
            
        case 'get-projects':
            get_projects_list();
            exit;
            
        case 'save-project':
            save_project();
            exit;
            
        case 'delete-project':
            delete_project();
            exit;
            
        case 'reorder-projects':
            reorder_projects();
            exit;
            
        case 'get-gallery-images':
            get_gallery_images();
            exit;
            
        case 'upload-image':
            upload_gallery_image();
            exit;
            
        case 'delete-image':
            delete_gallery_image();
            exit;
            
        case 'reorder-images':
            reorder_gallery_images();
            exit;
            
        case 'get-about':
            get_about_content_admin();
            exit;
            
        case 'save-about':
            save_about_content();
            exit;
            
        case 'get-messages':
            get_messages();
            exit;
            
        case 'mark-message-read':
            mark_message_read();
            exit;
            
        case 'delete-message':
            delete_message();
            exit;
            
        case 'get-settings':
            get_settings_admin();
            exit;
            
        case 'save-settings':
            save_settings();
            exit;
            
        case 'get-users':
            get_users();
            exit;
            
        case 'save-user':
            save_user();
            exit;
            
        case 'delete-user':
            delete_user_admin();
            exit;
            
        default:
            header('HTTP/1.1 404 Not Found');
            echo json_encode(['success' => false, 'error' => 'Nieznane żądanie']);
            exit;
    }
}

// Pobierz ustawienia strony
$settings = get_settings();
$adminUser = get_admin_user();

// Wczytanie nagłówka panelu admina
require_once 'includes/header.php';

// Wczytanie odpowiedniej podstrony
switch ($adminPage) {
    case 'dashboard':
        require_once 'dashboard.php';
        break;
        
    case 'projekty':
        require_once 'projects.php';
        break;
        
    case 'galeria':
        require_once 'gallery.php';
        break;
        
    case 'o-mnie':
        require_once 'about.php';
        break;
        
    case 'wiadomosci':
        require_once 'messages.php';
        break;
        
    case 'ustawienia':
        require_once 'settings.php';
        break;
        
    case 'uzytkownicy':
        require_once 'users.php';
        break;
        
    case 'login':
        // Przekieruj do dashboard jeśli zalogowany
        header('Location: /admin');
        exit;
        break;
        
    default:
        require_once 'dashboard.php';
        break;
}

// Wczytanie stopki panelu admina
require_once 'includes/footer.php';

/**
 * Funkcje obsługi AJAX
 */

/**
 * Pobiera statystyki dla dashboard
 */
function get_dashboard_stats() {
    $db = get_db_connection();
    
    try {
        // Liczba projektów
        $stmt = $db->prepare("SELECT COUNT(*) as count FROM projects");
        $stmt->execute();
        $projectsCount = $stmt->fetch(PDO::FETCH_ASSOC)['count'];
        
        // Liczba opublikowanych projektów
        $stmt = $db->prepare("SELECT COUNT(*) as count FROM projects WHERE is_published = 1");
        $stmt->execute();
        $publishedProjectsCount = $stmt->fetch(PDO::FETCH_ASSOC)['count'];
        
        // Liczba zdjęć
        $stmt = $db->prepare("SELECT COUNT(*) as count FROM gallery_images");
        $stmt->execute();
        $imagesCount = $stmt->fetch(PDO::FETCH_ASSOC)['count'];
        
        // Liczba albumów
        $stmt = $db->prepare("SELECT COUNT(*) as count FROM gallery_albums");
        $stmt->execute();
        $albumsCount = $stmt->fetch(PDO::FETCH_ASSOC)['count'];
        
        // Liczba wiadomości
        $stmt = $db->prepare("SELECT COUNT(*) as count FROM messages");
        $stmt->execute();
        $messagesCount = $stmt->fetch(PDO::FETCH_ASSOC)['count'];
        
        // Liczba nieprzeczytanych wiadomości
        $stmt = $db->prepare("SELECT COUNT(*) as count FROM messages WHERE is_read = 0");
        $stmt->execute();
        $unreadMessagesCount = $stmt->fetch(PDO::FETCH_ASSOC)['count'];
        
        header('Content-Type: application/json');
        echo json_encode([
            'success' => true,
            'stats' => [
                'projects' => $projectsCount,
                'published_projects' => $publishedProjectsCount,
                'images' => $imagesCount,
                'albums' => $albumsCount,
                'messages' => $messagesCount,
                'unread_messages' => $unreadMessagesCount
            ]
        ]);
    } catch (PDOException $e) {
        header('Content-Type: application/json');
        echo json_encode(['success' => false, 'error' => 'Błąd pobierania statystyk']);
    }
}

/**
 * Pobiera listę projektów
 */
function get_projects_list() {
    $db = get_db_connection();
    
    try {
        $stmt = $db->prepare("SELECT p.*, pc.name as category_name 
                              FROM projects p 
                              LEFT JOIN project_categories pc ON p.category_id = pc.id 
                              ORDER BY p.order_index ASC");
        $stmt->execute();
        $projects = $stmt->fetchAll(PDO::FETCH_ASSOC);
        
        header('Content-Type: application/json');
        echo json_encode(['success' => true, 'projects' => $projects]);
    } catch (PDOException $e) {
        header('Content-Type: application/json');
        echo json_encode(['success' => false, 'error' => 'Błąd pobierania projektów']);
    }
}

/**
 * Zapisz projekt
 */
function save_project() {
    $db = get_db_connection();
    
    try {
        $id = isset($_POST['id']) ? (int)$_POST['id'] : 0;
        $title = isset($_POST['title']) ? trim($_POST['title']) : '';
        $slug = isset($_POST['slug']) ? trim($_POST['slug']) : '';
        $description = isset($_POST['description']) ? $_POST['description'] : '';
        $shortDescription = isset($_POST['short_description']) ? trim($_POST['short_description']) : '';
        $categoryId = isset($_POST['category_id']) ? (int)$_POST['category_id'] : null;
        $technologies = isset($_POST['technologies']) ? json_encode(explode(',', $_POST['technologies'])) : '[]';
        $liveUrl = isset($_POST['live_url']) ? trim($_POST['live_url']) : '';
        $githubUrl = isset($_POST['github_url']) ? trim($_POST['github_url']) : '';
        $isPublished = isset($_POST['is_published']) ? (bool)$_POST['is_published'] : true;
        $orderIndex = isset($_POST['order_index']) ? (int)$_POST['order_index'] : 0;
        
        // Walidacja
        if (empty($title)) {
            header('Content-Type: application/json');
            echo json_encode(['success' => false, 'error' => 'Tytuł jest wymagany']);
            exit;
        }
        
        // Generuj slug jeśli nie podano
        if (empty($slug)) {
            $slug = sanitize_slug($title);
        }
        
        // Sprawdź czy slug już istnieje (jeśli to nowy projekt)
        if ($id === 0) {
            $stmt = $db->prepare("SELECT id FROM projects WHERE slug = ?");
            $stmt->execute([$slug]);
            if ($stmt->rowCount() > 0) {
                $slug = $slug . '-' . time();
            }
        }
        
        // Obsługa uploadu okładki
        $coverImage = null;
        if (isset($_FILES['cover_image']) && $_FILES['cover_image']['error'] === UPLOAD_ERR_OK) {
            $coverImage = handle_image_upload($_FILES['cover_image'], 'projects');
        }
        
        // Obsługa uploadu galerii
        $galleryImages = [];
        if (!empty($_POST['existing_gallery_images'])) {
            $galleryImages = array_filter(explode(',', $_POST['existing_gallery_images']));
        }
        
        if (isset($_FILES['gallery_images']) && !empty($_FILES['gallery_images']['name'][0])) {
            foreach ($_FILES['gallery_images']['tmp_name'] as $index => $tmpName) {
                if ($_FILES['gallery_images']['error'][$index] === UPLOAD_ERR_OK) {
                    $file = [
                        'name' => $_FILES['gallery_images']['name'][$index],
                        'type' => $_FILES['gallery_images']['type'][$index],
                        'tmp_name' => $tmpName,
                        'error' => $_FILES['gallery_images']['error'][$index],
                        'size' => $_FILES['gallery_images']['size'][$index]
                    ];
                    $imageName = handle_image_upload($file, 'projects');
                    if ($imageName) {
                        $galleryImages[] = $imageName;
                    }
                }
            }
        }
        
        // Zapisz projekt
        if ($id > 0) {
            // Aktualizuj istniejący projekt
            $galleryImagesJson = json_encode($galleryImages);
            
            $query = "UPDATE projects SET 
                      title = ?, 
                      slug = ?, 
                      description = ?, 
                      short_description = ?, 
                      category_id = ?, 
                      technologies = ?, 
                      live_url = ?, 
                      github_url = ?, 
                      cover_image = ?, 
                      gallery_images = ?, 
                      is_published = ?, 
                      order_index = ?, 
                      updated_at = NOW() 
                      WHERE id = ?";
            
            $stmt = $db->prepare($query);
            $stmt->execute([
                $title, $slug, $description, $shortDescription, $categoryId,
                $technologies, $liveUrl, $githubUrl, $coverImage,
                $galleryImagesJson, $isPublished ? 1 : 0, $orderIndex, $id
            ]);
            
            $projectId = $id;
        } else {
            // Utwórz nowy projekt
            $galleryImagesJson = json_encode($galleryImages);
            
            $stmt = $db->prepare("INSERT INTO projects 
                                  (title, slug, description, short_description, category_id, 
                                   technologies, live_url, github_url, cover_image, 
                                   gallery_images, is_published, order_index, created_at, updated_at) 
                                  VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NOW(), NOW())");
            $stmt->execute([
                $title, $slug, $description, $shortDescription, $categoryId,
                $technologies, $liveUrl, $githubUrl, $coverImage,
                $galleryImagesJson, $isPublished ? 1 : 0, $orderIndex
            ]);
            
            $projectId = $db->lastInsertId();
        }
        
        header('Content-Type: application/json');
        echo json_encode([
            'success' => true,
            'message' => $id > 0 ? 'Projekt zaktualizowany pomyślnie' : 'Projekt utworzony pomyślnie',
            'projectId' => $projectId
        ]);
    } catch (PDOException $e) {
        header('Content-Type: application/json');
        echo json_encode(['success' => false, 'error' => 'Błąd zapisu projektu: ' . $e->getMessage()]);
    }
}

/**
 * Usuwa projekt
 */
function delete_project() {
    $db = get_db_connection();
    
    try {
        $id = isset($_POST['id']) ? (int)$_POST['id'] : 0;
        
        if ($id <= 0) {
            header('Content-Type: application/json');
            echo json_encode(['success' => false, 'error' => 'Nieprawidłowe ID projektu']);
            exit;
        }
        
        // Pobierz projekt (żeby usunąć pliki)
        $stmt = $db->prepare("SELECT cover_image, gallery_images FROM projects WHERE id = ?");
        $stmt->execute([$id]);
        $project = $stmt->fetch(PDO::FETCH_ASSOC);
        
        if ($project) {
            // Usuń okładkę
            if (!empty($project['cover_image'])) {
                unlink(__DIR__ . "/../uploads/" . 'projects/' . $project['cover_image']);
            }
            
            // Usuń zdjęcia z galerii
            if (!empty($project['gallery_images'])) {
                $galleryImages = json_decode($project['gallery_images'], true);
                if (is_array($galleryImages)) {
                    foreach ($galleryImages as $image) {
                        unlink(__DIR__ . "/../uploads/" . 'projects/' . $image);
                    }
                }
            }
        }
        
        // Usuń projekt
        $stmt = $db->prepare("DELETE FROM projects WHERE id = ?");
        $stmt->execute([$id]);
        
        header('Content-Type: application/json');
        echo json_encode(['success' => true, 'message' => 'Projekt usunięty pomyślnie']);
    } catch (PDOException $e) {
        header('Content-Type: application/json');
        echo json_encode(['success' => false, 'error' => 'Błąd usuwania projektu']);
    }
}

/**
 * Zmienia kolejność projektów
 */
function reorder_projects() {
    $db = get_db_connection();
    
    try {
        $items = isset($_POST['items']) ? $_POST['items'] : [];
        
        if (empty($items)) {
            header('Content-Type: application/json');
            echo json_encode(['success' => false, 'error' => 'Brak elementów do uporządkowania']);
            exit;
        }
        
        foreach ($items as $index => $item) {
            $id = (int)$item['id'];
            $stmt = $db->prepare("UPDATE projects SET order_index = ? WHERE id = ?");
            $stmt->execute([$index, $id]);
        }
        
        header('Content-Type: application/json');
        echo json_encode(['success' => true, 'message' => 'Kolejność zaktualizowana']);
    } catch (PDOException $e) {
        header('Content-Type: application/json');
        echo json_encode(['success' => false, 'error' => 'Błąd aktualizacji kolejności']);
    }
}

/**
 * Pobiera obrazy galerii
 */
function get_gallery_images() {
    $db = get_db_connection();
    
    try {
        $albumId = isset($_GET['album_id']) ? (int)$_GET['album_id'] : null;
        
        $query = "SELECT * FROM gallery_images";
        $params = [];
        
        if ($albumId) {
            $query .= " WHERE album_id = ?";
            $params[] = $albumId;
        }
        
        $query .= " ORDER BY order_index ASC";
        
        $stmt = $db->prepare($query);
        $stmt->execute($params);
        $images = $stmt->fetchAll(PDO::FETCH_ASSOC);
        
        header('Content-Type: application/json');
        echo json_encode(['success' => true, 'images' => $images]);
    } catch (PDOException $e) {
        header('Content-Type: application/json');
        echo json_encode(['success' => false, 'error' => 'Błąd pobierania obrazów']);
    }
}

/**
 * Upload obrazu do galerii
 */
function upload_gallery_image() {
    try {
        if (!isset($_FILES['image']) || $_FILES['image']['error'] !== UPLOAD_ERR_OK) {
            header('Content-Type: application/json');
            echo json_encode(['success' => false, 'error' => 'Brak pliku do uploadu']);
            exit;
        }
        
        $albumId = isset($_POST['album_id']) ? (int)$_POST['album_id'] : null;
        $title = isset($_POST['title']) ? trim($_POST['title']) : '';
        $description = isset($_POST['description']) ? trim($_POST['description']) : '';
        $altText = isset($_POST['alt_text']) ? trim($_POST['alt_text']) : '';
        
        // Upload obrazu
        $imageName = handle_image_upload($_FILES['image'], 'gallery');
        
        if (!$imageName) {
            header('Content-Type: application/json');
            echo json_encode(['success' => false, 'error' => 'Błąd podczas uploadu obrazu']);
            exit;
        }
        
        // Zapisz do bazy
        $db = get_db_connection();
        $stmt = $db->prepare("INSERT INTO gallery_images 
                              (album_id, title, description, image_path, thumbnail_path, webp_path, alt_text, order_index, created_at) 
                              VALUES (?, ?, ?, ?, ?, ?, ?, 0, NOW())");
        $stmt->execute([
            $albumId,
            $title,
            $description,
            $imageName,
            'thumbs/' . pathinfo($imageName, PATHINFO_FILENAME) . '_thumb.' . pathinfo($imageName, PATHINFO_EXTENSION),
            'webp/' . pathinfo($imageName, PATHINFO_FILENAME) . '.webp',
            $altText
        ]);
        
        $imageId = $db->lastInsertId();
        
        // Generuj miniaturkę i WebP
        generate_thumbnail(
            __DIR__ . "/../uploads/" . 'gallery/' . $imageName,
            __DIR__ . "/../uploads/" . 'gallery/thumbs/' . pathinfo($imageName, PATHINFO_FILENAME) . '_thumb.' . pathinfo($imageName, PATHINFO_EXTENSION)
        );
        
        convert_to_webp(
            __DIR__ . "/../uploads/" . 'gallery/' . $imageName,
            __DIR__ . "/../uploads/" . 'gallery/webp/' . pathinfo($imageName, PATHINFO_FILENAME) . '.webp'
        );
        
        header('Content-Type: application/json');
        echo json_encode([
            'success' => true,
            'message' => 'Obraz dodany pomyślnie',
            'image' => [
                'id' => $imageId,
                'image_path' => $imageName,
                'thumbnail_path' => 'thumbs/' . pathinfo($imageName, PATHINFO_FILENAME) . '_thumb.' . pathinfo($imageName, PATHINFO_EXTENSION),
                'webp_path' => 'webp/' . pathinfo($imageName, PATHINFO_FILENAME) . '.webp'
            ]
        ]);
    } catch (Exception $e) {
        header('Content-Type: application/json');
        echo json_encode(['success' => false, 'error' => 'Błąd: ' . $e->getMessage()]);
    }
}

/**
 * Usuwa obraz z galerii
 */
function delete_gallery_image() {
    $db = get_db_connection();
    
    try {
        $id = isset($_POST['id']) ? (int)$_POST['id'] : 0;
        
        if ($id <= 0) {
            header('Content-Type: application/json');
            echo json_encode(['success' => false, 'error' => 'Nieprawidłowe ID obrazu']);
            exit;
        }
        
        // Pobierz obraz
        $stmt = $db->prepare("SELECT image_path, thumbnail_path, webp_path FROM gallery_images WHERE id = ?");
        $stmt->execute([$id]);
        $image = $stmt->fetch(PDO::FETCH_ASSOC);
        
        if ($image) {
            // Usuń pliki
            if (!empty($image['image_path'])) {
                unlink(__DIR__ . "/../uploads/" . 'gallery/' . $image['image_path']);
            }
            if (!empty($image['thumbnail_path'])) {
                unlink(__DIR__ . "/../uploads/" . 'gallery/' . $image['thumbnail_path']);
            }
            if (!empty($image['webp_path'])) {
                unlink(__DIR__ . "/../uploads/" . 'gallery/' . $image['webp_path']);
            }
        }
        
        // Usuń z bazy
        $stmt = $db->prepare("DELETE FROM gallery_images WHERE id = ?");
        $stmt->execute([$id]);
        
        header('Content-Type: application/json');
        echo json_encode(['success' => true, 'message' => 'Obraz usunięty pomyślnie']);
    } catch (PDOException $e) {
        header('Content-Type: application/json');
        echo json_encode(['success' => false, 'error' => 'Błąd usuwania obrazu']);
    }
}

/**
 * Zmienia kolejność obrazów
 */
function reorder_gallery_images() {
    $db = get_db_connection();
    
    try {
        $items = isset($_POST['items']) ? $_POST['items'] : [];
        
        if (empty($items)) {
            header('Content-Type: application/json');
            echo json_encode(['success' => false, 'error' => 'Brak elementów do uporządkowania']);
            exit;
        }
        
        foreach ($items as $index => $item) {
            $id = (int)$item['id'];
            $stmt = $db->prepare("UPDATE gallery_images SET order_index = ? WHERE id = ?");
            $stmt->execute([$index, $id]);
        }
        
        header('Content-Type: application/json');
        echo json_encode(['success' => true, 'message' => 'Kolejność zaktualizowana']);
    } catch (PDOException $e) {
        header('Content-Type: application/json');
        echo json_encode(['success' => false, 'error' => 'Błąd aktualizacji kolejności']);
    }
}

/**
 * Pobiera treść "O mnie"
 */
function get_about_content_admin() {
    $db = get_db_connection();
    
    try {
        $stmt = $db->prepare("SELECT * FROM about LIMIT 1");
        $stmt->execute();
        $about = $stmt->fetch(PDO::FETCH_ASSOC);
        
        header('Content-Type: application/json');
        echo json_encode(['success' => true, 'about' => $about ?: []]);
    } catch (PDOException $e) {
        header('Content-Type: application/json');
        echo json_encode(['success' => false, 'error' => 'Błąd pobierania treści']);
    }
}

/**
 * Zapisz treść "O mnie"
 */
function save_about_content() {
    $db = get_db_connection();
    
    try {
        $id = isset($_POST['id']) ? (int)$_POST['id'] : 0;
        $title = isset($_POST['title']) ? trim($_POST['title']) : '';
        $content = isset($_POST['content']) ? $_POST['content'] : '';
        
        // Obsługa uploadu obrazu
        $imagePath = null;
        if (isset($_FILES['image']) && $_FILES['image']['error'] === UPLOAD_ERR_OK) {
            $imagePath = handle_image_upload($_FILES['image'], '');
        }
        
        if ($id > 0) {
            // Aktualizuj
            $query = "UPDATE about SET title = ?, content = ?, image_path = ?, updated_at = NOW() WHERE id = ?";
            $stmt = $db->prepare($query);
            $stmt->execute([$title, $content, $imagePath, $id]);
        } else {
            // Utwórz nowy
            $stmt = $db->prepare("INSERT INTO about (title, content, image_path, created_at, updated_at) 
                                  VALUES (?, ?, ?, NOW(), NOW())");
            $stmt->execute([$title, $content, $imagePath]);
            $id = $db->lastInsertId();
        }
        
        header('Content-Type: application/json');
        echo json_encode(['success' => true, 'message' => 'Treść zaktualizowana pomyślnie']);
    } catch (PDOException $e) {
        header('Content-Type: application/json');
        echo json_encode(['success' => false, 'error' => 'Błąd zapisu treści']);
    }
}

/**
 * Pobiera wiadomości
 */
function get_messages() {
    $db = get_db_connection();
    
    try {
        $page = isset($_GET['page']) ? (int)$_GET['page'] : 1;
        $perPage = isset($_GET['per_page']) ? (int)$_GET['per_page'] : 10;
        $offset = ($page - 1) * $perPage;
        
        $query = "SELECT * FROM messages ORDER BY created_at DESC LIMIT ? OFFSET ?";
        $stmt = $db->prepare($query);
        $stmt->execute([$perPage, $offset]);
        $messages = $stmt->fetchAll(PDO::FETCH_ASSOC);
        
        // Pobierz łączną liczbę wiadomości
        $stmt = $db->prepare("SELECT COUNT(*) as count FROM messages");
        $stmt->execute();
        $total = $stmt->fetch(PDO::FETCH_ASSOC)['count'];
        
        header('Content-Type: application/json');
        echo json_encode([
            'success' => true,
            'messages' => $messages,
            'total' => $total,
            'page' => $page,
            'per_page' => $perPage
        ]);
    } catch (PDOException $e) {
        header('Content-Type: application/json');
        echo json_encode(['success' => false, 'error' => 'Błąd pobierania wiadomości']);
    }
}

/**
 * Oznacz wiadomość jako przeczytaną
 */
function mark_message_read() {
    $db = get_db_connection();
    
    try {
        $id = isset($_POST['id']) ? (int)$_POST['id'] : 0;
        
        if ($id <= 0) {
            header('Content-Type: application/json');
            echo json_encode(['success' => false, 'error' => 'Nieprawidłowe ID wiadomości']);
            exit;
        }
        
        $stmt = $db->prepare("UPDATE messages SET is_read = 1 WHERE id = ?");
        $stmt->execute([$id]);
        
        header('Content-Type: application/json');
        echo json_encode(['success' => true, 'message' => 'Wiadomość oznaczona jako przeczytana']);
    } catch (PDOException $e) {
        header('Content-Type: application/json');
        echo json_encode(['success' => false, 'error' => 'Błąd aktualizacji wiadomości']);
    }
}

/**
 * Usuwa wiadomość
 */
function delete_message() {
    $db = get_db_connection();
    
    try {
        $id = isset($_POST['id']) ? (int)$_POST['id'] : 0;
        
        if ($id <= 0) {
            header('Content-Type: application/json');
            echo json_encode(['success' => false, 'error' => 'Nieprawidłowe ID wiadomości']);
            exit;
        }
        
        $stmt = $db->prepare("DELETE FROM messages WHERE id = ?");
        $stmt->execute([$id]);
        
        header('Content-Type: application/json');
        echo json_encode(['success' => true, 'message' => 'Wiadomość usunięta pomyślnie']);
    } catch (PDOException $e) {
        header('Content-Type: application/json');
        echo json_encode(['success' => false, 'error' => 'Błąd usuwania wiadomości']);
    }
}

/**
 * Pobiera ustawienia
 */
function get_settings_admin() {
    $db = get_db_connection();
    
    try {
        $stmt = $db->prepare("SELECT * FROM settings LIMIT 1");
        $stmt->execute();
        $settings = $stmt->fetch(PDO::FETCH_ASSOC);
        
        header('Content-Type: application/json');
        echo json_encode(['success' => true, 'settings' => $settings ?: []]);
    } catch (PDOException $e) {
        header('Content-Type: application/json');
        echo json_encode(['success' => false, 'error' => 'Błąd pobierania ustawień']);
    }
}

/**
 * Zapisz ustawienia
 */
function save_settings() {
    $db = get_db_connection();
    
    try {
        $id = isset($_POST['id']) ? (int)$_POST['id'] : 0;
        $siteTitle = isset($_POST['site_title']) ? trim($_POST['site_title']) : '';
        $siteDescription = isset($_POST['site_description']) ? trim($_POST['site_description']) : '';
        $theme = isset($_POST['theme']) ? trim($_POST['theme']) : 'light';
        $githubUrl = isset($_POST['github_url']) ? trim($_POST['github_url']) : '';
        $linkedinUrl = isset($_POST['linkedin_url']) ? trim($_POST['linkedin_url']) : '';
        $instagramUrl = isset($_POST['instagram_url']) ? trim($_POST['instagram_url']) : '';
        $email = isset($_POST['email']) ? trim($_POST['email']) : '';
        
        if ($id > 0) {
            $stmt = $db->prepare("UPDATE settings SET 
                                  site_title = ?, site_description = ?, theme = ?, 
                                  github_url = ?, linkedin_url = ?, instagram_url = ?, 
                                  email = ?, updated_at = NOW() 
                                  WHERE id = ?");
            $stmt->execute([
                $siteTitle, $siteDescription, $theme,
                $githubUrl, $linkedinUrl, $instagramUrl,
                $email, $id
            ]);
        } else {
            $stmt = $db->prepare("INSERT INTO settings 
                                  (site_title, site_description, theme, github_url, linkedin_url, instagram_url, email, created_at, updated_at) 
                                  VALUES (?, ?, ?, ?, ?, ?, ?, NOW(), NOW())");
            $stmt->execute([
                $siteTitle, $siteDescription, $theme,
                $githubUrl, $linkedinUrl, $instagramUrl,
                $email
            ]);
        }
        
        header('Content-Type: application/json');
        echo json_encode(['success' => true, 'message' => 'Ustawienia zaktualizowane pomyślnie']);
    } catch (PDOException $e) {
        header('Content-Type: application/json');
        echo json_encode(['success' => false, 'error' => 'Błąd zapisu ustawień']);
    }
}

/**
 * Pobiera użytkowników
 */
function get_users() {
    try {
        $users = get_admin_users();
        
        header('Content-Type: application/json');
        echo json_encode(['success' => true, 'users' => $users]);
    } catch (Exception $e) {
        header('Content-Type: application/json');
        echo json_encode(['success' => false, 'error' => 'Błąd pobierania użytkowników']);
    }
}

/**
 * Zapisz użytkownika
 */
function save_user() {
    try {
        $id = isset($_POST['id']) ? (int)$_POST['id'] : 0;
        $username = isset($_POST['username']) ? trim($_POST['username']) : '';
        $email = isset($_POST['email']) ? trim($_POST['email']) : '';
        $password = isset($_POST['password']) ? $_POST['password'] : null;
        
        if ($id > 0) {
            // Aktualizuj
            $result = update_admin_user($id, [
                'username' => $username,
                'email' => $email,
                'password' => $password
            ]);
            
            if ($result) {
                header('Content-Type: application/json');
                echo json_encode(['success' => true, 'message' => 'Użytkownik zaktualizowany pomyślnie']);
            } else {
                header('Content-Type: application/json');
                echo json_encode(['success' => false, 'error' => 'Błąd aktualizacji użytkownika']);
            }
        } else {
            // Utwórz nowego
            $result = create_admin_user([
                'username' => $username,
                'email' => $email,
                'password' => $password
            ]);
            
            if ($result['success']) {
                header('Content-Type: application/json');
                echo json_encode(['success' => true, 'message' => 'Użytkownik utworzony pomyślnie']);
            } else {
                header('Content-Type: application/json');
                echo json_encode(['success' => false, 'error' => $result['message']]);
            }
        }
    } catch (Exception $e) {
        header('Content-Type: application/json');
        echo json_encode(['success' => false, 'error' => 'Błąd zapisu użytkownika']);
    }
}

/**
 * Usuwa użytkownika
 */
function delete_user_admin() {
    try {
        $id = isset($_POST['id']) ? (int)$_POST['id'] : 0;
        
        if ($id <= 0) {
            header('Content-Type: application/json');
            echo json_encode(['success' => false, 'error' => 'Nieprawidłowe ID użytkownika']);
            exit;
        }
        
        $result = delete_admin_user($id);
        
        if ($result) {
            header('Content-Type: application/json');
            echo json_encode(['success' => true, 'message' => 'Użytkownik usunięty pomyślnie']);
        } else {
            header('Content-Type: application/json');
            echo json_encode(['success' => false, 'error' => 'Błąd usuwania użytkownika']);
        }
    } catch (Exception $e) {
        header('Content-Type: application/json');
        echo json_encode(['success' => false, 'error' => 'Błąd usuwania użytkownika']);
    }
}

/**
 * Obsługuje upload obrazu
 * @param array $file - Plik z $_FILES
 * @param string $subfolder - Podfolder w uploads
 * @return string|null - Nazwa pliku lub null
 */
function handle_image_upload($file, $subfolder) {
    // Sprawdź typ pliku
    $allowedTypes = ALLOWED_IMAGE_TYPES;
    $fileType = $file['type'];
    
    if (!in_array($fileType, $allowedTypes)) {
        return null;
    }
    
    // Sprawdź rozszerzenie
    $fileExt = strtolower(pathinfo($file['name'], PATHINFO_EXTENSION));
    $allowedExtensions = ALLOWED_EXTENSIONS;
    
    if (!in_array($fileExt, $allowedExtensions)) {
        return null;
    }
    
    // Sprawdź rozmiar
    if ($file['size'] > MAX_UPLOAD_SIZE) {
        return null;
    }
    
    // Wygeneruj losową nazwę pliku
    $newFilename = generate_random_filename($fileExt);
    
    // Utwórz ścieżkę docelową
    $targetPath = __DIR__ . "/../uploads/" . ($subfolder ? $subfolder . '/' : '') . $newFilename;
    
    // Przenieś plik
    if (move_uploaded_file($file['tmp_name'], $targetPath)) {
        return $newFilename;
    }
    
    return null;
}
