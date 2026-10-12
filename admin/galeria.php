<?php
/**
 * Panel administracyjny - Zarządzanie galerią
 */

// Pobierz albumy
$albums = get_gallery_albums();

// Pobierz obrazy
$db = get_db_connection();
$stmt = $db->prepare("SELECT gi.*, ga.name as album_name 
                      FROM gallery_images gi 
                      LEFT JOIN gallery_albums ga ON gi.album_id = ga.id 
                      ORDER BY gi.order_index ASC");
$stmt->execute();
$images = $stmt->fetchAll(PDO::FETCH_ASSOC);

// Akcja
$action = isset($_GET['action']) ? trim($_GET['action']) : 'list';

// Obsługa akcji
switch ($action) {
    case 'new-album':
    case 'edit-album':
        // Pobierz album do edycji
        $album = null;
        if ($action === 'edit-album' && isset($_GET['id'])) {
            $id = (int)$_GET['id'];
            $stmt = $db->prepare("SELECT * FROM gallery_albums WHERE id = ?");
            $stmt->execute([$id]);
            $album = $stmt->fetch(PDO::FETCH_ASSOC);
            
            if (!$album) {
                header("HTTP/1.1 404 Not Found");
                echo "Album nie został znaleziony";
                exit;
            }
        }
        
        // Formularz albumu
        include 'templates/album-form.php';
        break;
        
    case 'upload':
        // Formularz uploadu obrazów
        include 'templates/upload-form.php';
        break;
        
    default:
        // Lista albumów i obrazów
        include 'templates/gallery-list.php';
        break;
}
