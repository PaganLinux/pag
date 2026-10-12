<?php
/**
 * Panel administracyjny - Zarządzanie projektami
 */

// Pobierz kategorie
$categories = get_project_categories();

// Pobierz projekty
$db = get_db_connection();
$stmt = $db->prepare("SELECT p.*, pc.name as category_name 
                      FROM projects p 
                      LEFT JOIN project_categories pc ON p.category_id = pc.id 
                      ORDER BY p.order_index ASC");
$stmt->execute();
$projects = $stmt->fetchAll(PDO::FETCH_ASSOC);

// Pobierz ustawienia
$settings = get_settings();

// Akcja
$action = isset($_GET['action']) ? trim($_GET['action']) : 'list';

// Obsługa akcji
switch ($action) {
    case 'new':
    case 'edit':
        // Pobierz projekt do edycji
        $project = null;
        if ($action === 'edit' && isset($_GET['id'])) {
            $id = (int)$_GET['id'];
            $stmt = $db->prepare("SELECT * FROM projects WHERE id = ?");
            $stmt->execute([$id]);
            $project = $stmt->fetch(PDO::FETCH_ASSOC);
            
            if (!$project) {
                header("HTTP/1.1 404 Not Found");
                echo "Projekt nie został znaleziony";
                exit;
            }
            
            // Dekoduj dane
            $project['technologies'] = json_decode($project['technologies'] ?? '[]', true);
            $project['gallery_images'] = json_decode($project['gallery_images'] ?? '[]', true);
        }
        
        // Formularz edycji/nowego projektu
        include 'templates/project-form.php';
        break;
        
    default:
        // Lista projektów
        include 'templates/projects-list.php';
        break;
}
