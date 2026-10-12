<?php
/**
 * Panel administracyjny - Dashboard
 * Strona główna panelu z statystykami
 */

// Pobierz statystyki
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
    
    // Ostatnie wiadomości
    $stmt = $db->prepare("SELECT * FROM messages ORDER BY created_at DESC LIMIT 5");
    $stmt->execute();
    $recentMessages = $stmt->fetchAll(PDO::FETCH_ASSOC);
    
    // Ostatnie aktywności
    $stmt = $db->prepare("SELECT * FROM activity_logs ORDER BY created_at DESC LIMIT 5");
    $stmt->execute();
    $recentActivities = $stmt->fetchAll(PDO::FETCH_ASSOC);
    
} catch (PDOException $e) {
    $error = 'Wystąpił błąd podczas pobierania statystyk';
}

// Pobierz ustawienia
$settings = get_settings();
?>

<!-- ============================================
     DASHBOARD
     ============================================ -->
<div class="admin-page-header">
    <div class="admin-breadcrumb">
        <span class="admin-breadcrumb-current">Dashboard</span>
    </div>
    
    <h1 class="admin-page-title">📊 DASHBOARD</h1>
    <p class="admin-page-subtitle">Podsumowanie aktywności i statystyki</p>
</div>

<!-- Statystyki -->
<div class="admin-cards">
    <article class="admin-card">
        <div class="admin-card-header">
            <h3 class="admin-card-title">Projekty</h3>
            <span class="admin-card-icon">💻</span>
        </div>
        <div class="admin-card-content">
            <div class="admin-card-value"><?php echo $projectsCount; ?></div>
            <div class="admin-card-label">Wszystkie / <?php echo $publishedProjectsCount; ?> Opublikowane</div>
        </div>
        <div class="admin-card-actions">
            <a href="<?php echo SITE_URL; ?>/admin/projekty" class="admin-card-btn">
                ZARZĄDZAJ
            </a>
        </div>
    </article>
    
    <article class="admin-card">
        <div class="admin-card-header">
            <h3 class="admin-card-title">Galeria</h3>
            <span class="admin-card-icon">📷</span>
        </div>
        <div class="admin-card-content">
            <div class="admin-card-value"><?php echo $imagesCount; ?></div>
            <div class="admin-card-label">Zdjęć w <?php echo $albumsCount; ?> albumach</div>
        </div>
        <div class="admin-card-actions">
            <a href="<?php echo SITE_URL; ?>/admin/galeria" class="admin-card-btn">
                ZARZĄDZAJ
            </a>
        </div>
    </article>
    
    <article class="admin-card">
        <div class="admin-card-header">
            <h3 class="admin-card-title">Wiadomości</h3>
            <span class="admin-card-icon">✉️</span>
        </div>
        <div class="admin-card-content">
            <div class="admin-card-value"><?php echo $messagesCount; ?></div>
            <div class="admin-card-label">
                <?php echo $unreadMessagesCount; ?> nieprzeczytane
            </div>
        </div>
        <div class="admin-card-actions">
            <a href="<?php echo SITE_URL; ?>/admin/wiadomosci" class="admin-card-btn">
                ZOBACZ
            </a>
        </div>
    </article>
    
    <article class="admin-card">
        <div class="admin-card-header">
            <h3 class="admin-card-title">Ustawienia</h3>
            <span class="admin-card-icon">⚙️</span>
        </div>
        <div class="admin-card-content">
            <div class="admin-card-value">-</div>
            <div class="admin-card-label">Motyw: <?php echo escape_output($settings['theme'] ?? 'light'); ?></div>
        </div>
        <div class="admin-card-actions">
            <a href="<?php echo SITE_URL; ?>/admin/ustawienia" class="admin-card-btn">
                EDYTUJ
            </a>
        </div>
    </article>
</div>

<!-- Ostatnie wiadomości i aktywności -->
<div style="display: grid; grid-template-columns: 1fr 1fr; gap: var(--spacing-xl); margin-bottom: var(--spacing-xl);">
    
    <!-- Ostatnie wiadomości -->
    <div class="admin-table-container">
        <div class="admin-table-header">
            <h2 class="admin-table-title">Ostatnie wiadomości</h2>
            <a href="<?php echo SITE_URL; ?>/admin/wiadomosci" class="admin-card-btn" style="margin-left: auto;">
                WSZYSTKIE WIADOMOŚCI
            </a>
        </div>
        
        <?php if (!empty($recentMessages)): ?>
        <table class="admin-table">
            <thead>
                <tr>
                    <th>Od</th>
                    <th>Temat</th>
                    <th>Data</th>
                    <th>Status</th>
                </tr>
            </thead>
            <tbody>
                <?php foreach ($recentMessages as $message): ?>
                <tr>
                    <td><?php echo escape_output(truncate_text($message['name'], 20)); ?></td>
                    <td><?php echo escape_output(truncate_text($message['subject'] ?? 'Brak tematu', 30)); ?></td>
                    <td><?php echo format_polish_date($message['created_at']); ?></td>
                    <td>
                        <span style="font-size: 0.75rem; padding: var(--spacing-xs) var(--spacing-sm); border-radius: var(--radius-sm); background: <?php echo $message['is_read'] ? 'var(--color-success)' : 'var(--color-warning)'; ?>; color: white;">
                            <?php echo $message['is_read'] ? 'Przeczytana' : 'Nowa'; ?>
                        </span>
                    </td>
                </tr>
                <?php endforeach; ?>
            </tbody>
        </table>
        <?php else: ?>
        <p style="padding: var(--spacing-lg); text-align: center; color: var(--color-ink-light);">
            Brak wiadomości
        </p>
        <?php endif; ?>
    </div>
    
    <!-- Ostatnie aktywności -->
    <div class="admin-table-container">
        <div class="admin-table-header">
            <h2 class="admin-table-title">Ostatnie aktywności</h2>
            <a href="#" class="admin-card-btn" style="margin-left: auto;">
                ZOBACZ WIĘCEJ
            </a>
        </div>
        
        <?php if (!empty($recentActivities)): ?>
        <table class="admin-table">
            <thead>
                <tr>
                    <th>Użytkownik</th>
                    <th>Akcja</th>
                    <th>Data</th>
                </tr>
            </thead>
            <tbody>
                <?php foreach ($recentActivities as $activity): 
                    $userName = '';
                    if ($activity['user_id']) {
                        $user = get_admin_user_by_id($activity['user_id']);
                        $userName = $user ? $user['username'] : 'Niezidentyfikowany';
                    }
                ?>
                <tr>
                    <td><?php echo escape_output($userName); ?></td>
                    <td><?php echo escape_output($activity['action']); ?></td>
                    <td><?php echo format_polish_date($activity['created_at']); ?></td>
                </tr>
                <?php endforeach; ?>
            </tbody>
        </table>
        <?php else: ?>
        <p style="padding: var(--spacing-lg); text-align: center; color: var(--color-ink-light);">
            Brak aktywności
        </p>
        <?php endif; ?>
    </div>
</div>

<!-- Szybkie akcje -->
<div class="admin-table-container">
    <div class="admin-table-header">
        <h2 class="admin-table-title">Szybkie akcje</h2>
    </div>
    
    <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: var(--spacing-md); padding: var(--spacing-lg);">
        <a href="<?php echo SITE_URL; ?>/admin/projekty?action=new" class="admin-card-btn" style="text-align: center; padding: var(--spacing-lg);">
            <span style="display: block; font-size: 2rem; margin-bottom: var(--spacing-xs);">➕</span>
            <span>Dodaj nowy projekt</span>
        </a>
        
        <a href="<?php echo SITE_URL; ?>/admin/galeria?action=upload" class="admin-card-btn" style="text-align: center; padding: var(--spacing-lg);">
            <span style="display: block; font-size: 2rem; margin-bottom: var(--spacing-xs);">📤</span>
            <span>Upload zdjęć</span>
        </a>
        
        <a href="<?php echo SITE_URL; ?>/admin/o-mnie?action=edit" class="admin-card-btn" style="text-align: center; padding: var(--spacing-lg);">
            <span style="display: block; font-size: 2rem; margin-bottom: var(--spacing-xs);">📝</span>
            <span>Edytuj "O mnie"</span>
        </a>
        
        <a href="<?php echo SITE_URL; ?>" class="admin-card-btn" style="text-align: center; padding: var(--spacing-lg);" target="_blank">
            <span style="display: block; font-size: 2rem; margin-bottom: var(--spacing-xs);">👁️</span>
            <span>Zobacz stronę</span>
        </a>
    </div>
</div>

<script>
// Ładowanie statystyk przez AJAX
function loadStats() {
    sendAjaxRequest('<?php echo SITE_URL; ?>/admin?admin_page=get-stats', 'GET')
        .then(data => {
            if (data.success) {
                // Aktualizuj statystyki
                const stats = data.stats;
                
                // Projekty
                document.querySelector('.admin-card-value').textContent = stats.projects;
                document.querySelector('.admin-card-label').textContent = `Wszystkie / ${stats.published_projects} Opublikowane`;
                
                // Galeria
                const galleryCard = document.querySelectorAll('.admin-card')[1];
                galleryCard.querySelector('.admin-card-value').textContent = stats.images;
                galleryCard.querySelector('.admin-card-label').textContent = `Zdjęć w ${stats.albums} albumach`;
                
                // Wiadomości
                const messagesCard = document.querySelectorAll('.admin-card')[2];
                messagesCard.querySelector('.admin-card-value').textContent = stats.messages;
                messagesCard.querySelector('.admin-card-label').textContent = `${stats.unread_messages} nieprzeczytane`;
            }
        });
}

// Ładuj statystyki co 30 sekund
setInterval(loadStats, 30000);
</script>
