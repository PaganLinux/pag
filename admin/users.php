<?php
/**
 * Panel administracyjny - Zarządzanie użytkownikami
 */

// Pobierz użytkowników
$users = get_admin_users();
$adminUser = get_admin_user();

// Akcja
$action = isset($_GET['action']) ? trim($_GET['action']) : 'list';

// Obsługa akcji
switch ($action) {
    case 'new':
    case 'edit':
        // Pobierz użytkownika do edycji
        $user = null;
        if ($action === 'edit' && isset($_GET['id'])) {
            $id = (int)$_GET['id'];
            $user = get_admin_user_by_id($id);
            
            if (!$user) {
                header("HTTP/1.1 404 Not Found");
                echo "Użytkownik nie został znaleziony";
                exit;
            }
        }
        
        // Formularz użytkownika
        include 'templates/user-form.php';
        break;
        
    default:
        // Lista użytkowników
        include 'templates/users-list.php';
        break;
}
