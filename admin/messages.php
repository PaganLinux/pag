<?php
/**
 * Panel administracyjny - Wiadomości kontaktowe
 */

// Pobierz wiadomości
$db = get_db_connection();
$page = isset($_GET['page']) ? max(1, (int)$_GET['page']) : 1;
$perPage = 10;
$offset = ($page - 1) * $perPage;

$stmt = $db->prepare("SELECT * FROM messages ORDER BY created_at DESC LIMIT ? OFFSET ?");
$stmt->execute([$perPage, $offset]);
$messages = $stmt->fetchAll(PDO::FETCH_ASSOC);

// Pobierz łączną liczbę wiadomości
$stmt = $db->prepare("SELECT COUNT(*) as count FROM messages");
$stmt->execute();
$totalMessages = $stmt->fetch(PDO::FETCH_ASSOC)['count'];
$totalPages = ceil($totalMessages / $perPage);

// Pobierz liczbę nieprzeczytanych wiadomości
$stmt = $db->prepare("SELECT COUNT(*) as count FROM messages WHERE is_read = 0");
$stmt->execute();
$unreadCount = $stmt->fetch(PDO::FETCH_ASSOC)['count'];

// Akcja
$action = isset($_GET['action']) ? trim($_GET['action']) : 'list';

// Obsługa akcji
switch ($action) {
    case 'view':
        // Pobierz wiadomość
        $id = isset($_GET['id']) ? (int)$_GET['id'] : 0;
        $stmt = $db->prepare("SELECT * FROM messages WHERE id = ?");
        $stmt->execute([$id]);
        $message = $stmt->fetch(PDO::FETCH_ASSOC);
        
        if (!$message) {
            header("HTTP/1.1 404 Not Found");
            echo "Wiadomość nie została znaleziona";
            exit;
        }
        
        // Oznacz jako przeczytaną
        if (!$message['is_read']) {
            $stmt = $db->prepare("UPDATE messages SET is_read = 1 WHERE id = ?");
            $stmt->execute([$id]);
        }
        
        // Wyświetl wiadomość
        include 'templates/message-view.php';
        break;
        
    default:
        // Lista wiadomości
        include 'templates/messages-list.php';
        break;
}
