<?php
/**
 * Panel administracyjny - Lista wiadomości
 */
?>

<!-- ============================================
     WIADOMOŚCI - LISTA
     ============================================ -->
<div class="admin-page-header">
    <div class="admin-breadcrumb">
        <a href="<?php echo SITE_URL; ?>/admin" class="admin-breadcrumb-link">Dashboard</a>
        <span class="admin-breadcrumb-separator">/</span>
        <span class="admin-breadcrumb-current">Wiadomości</span>
    </div>
    
    <h1 class="admin-page-title">✉️ SKRZYNKA WIADOMOŚCI</h1>
    <p class="admin-page-subtitle">
        <?php echo $totalMessages; ?> wiadomości,
        <?php echo $unreadCount; ?> nieprzeczytane
    </p>
</div>

<!-- Filtry -->
<div class="admin-table-container" style="margin-bottom: var(--spacing-lg);">
    <div class="admin-table-header">
        <h2 class="admin-table-title">Filtry</h2>
    </div>
    
    <div style="display: flex; gap: var(--spacing-sm); flex-wrap: wrap; padding: var(--spacing-md) var(--spacing-lg);">
        <a href="<?php echo SITE_URL; ?>/admin/wiadomosci" 
           class="admin-card-btn <?php echo !isset($_GET['filter']) ? 'active' : ''; ?>"
           style="padding: var(--spacing-sm) var(--spacing-md);">
            Wszystkie (<?php echo $totalMessages; ?>)
        </a>
        
        <a href="<?php echo SITE_URL; ?>/admin/wiadomosci?filter=unread" 
           class="admin-card-btn <?php echo isset($_GET['filter']) && $_GET['filter'] === 'unread' ? 'active' : ''; ?>"
           style="padding: var(--spacing-sm) var(--spacing-md);">
            Nieprzeczytane (<?php echo $unreadCount; ?>)
        </a>
        
        <a href="<?php echo SITE_URL; ?>/admin/wiadomosci?filter=read" 
           class="admin-card-btn <?php echo isset($_GET['filter']) && $_GET['filter'] === 'read' ? 'active' : ''; ?>"
           style="padding: var(--spacing-sm) var(--spacing-md);">
            Przeczytane (<?php echo $totalMessages - $unreadCount; ?>)
        </a>
    </div>
</div>

<!-- Lista wiadomości -->
<div class="admin-table-container">
    <div class="admin-table-header">
        <h2 class="admin-table-title">Lista wiadomości</h2>
        <div class="admin-table-actions">
            <!-- Można dodać przycisk do masowego usuwania -->
        </div>
    </div>
    
    <?php if (!empty($messages)): ?>
    <table class="admin-table">
        <thead>
            <tr>
                <th>Od</th>
                <th>Email</th>
                <th>Temat</th>
                <th>Data</th>
                <th>Status</th>
                <th>Akcje</th>
            </tr>
        </thead>
        <tbody>
            <?php foreach ($messages as $message): ?>
            <tr class="<?php echo !$message['is_read'] ? 'unread' : ''; ?>">
                <td><?php echo escape_output($message['name']); ?></td>
                <td><?php echo escape_output($message['email']); ?></td>
                <td><?php echo escape_output(truncate_text($message['subject'] ?? 'Brak tematu', 40)); ?></td>
                <td><?php echo format_polish_date($message['created_at']); ?></td>
                <td>
                    <span style="font-size: 0.75rem; padding: var(--spacing-xs) var(--spacing-sm); border-radius: var(--radius-sm); background: <?php echo $message['is_read'] ? 'var(--color-success)' : 'var(--color-warning)'; ?>; color: white;">
                        <?php echo $message['is_read'] ? 'Przeczytana' : 'Nowa'; ?>
                    </span>
                </td>
                <td class="admin-table-actions">
                    <a href="<?php echo SITE_URL; ?>/admin/wiadomosci?action=view&id=<?php echo $message['id']; ?>" 
                       class="admin-action-btn" 
                       title="Zobacz">
                        <span>👁️</span>
                    </a>
                    <button class="admin-action-btn delete" 
                            onclick="confirmDelete('<?php echo SITE_URL; ?>/admin?admin_page=delete-message&id=<?php echo $message['id']; ?>', 'Czy na pewno chcesz usunąć tą wiadomość?')"
                            title="Usuń">
                        <span>🗑️</span>
                    </button>
                </td>
            </tr>
            <?php endforeach; ?>
        </tbody>
    </table>
    
    <!-- Paginacja -->
    <?php if ($totalPages > 1): ?>
    <div class="admin-pagination">
        <button class="admin-pagination-btn" 
                onclick="window.location.href='<?php echo SITE_URL; ?>/admin/wiadomosci?page=<?php echo max(1, $page - 1); ?>'"
                <?php echo $page <= 1 ? 'disabled' : ''; ?>>
            ← Poprzednia
        </button>
        
        <span class="admin-pagination-info">
            Strona <?php echo $page; ?> z <?php echo $totalPages; ?>
        </span>
        
        <button class="admin-pagination-btn" 
                onclick="window.location.href='<?php echo SITE_URL; ?>/admin/wiadomosci?page=<?php echo min($totalPages, $page + 1); ?>'"
                <?php echo $page >= $totalPages ? 'disabled' : ''; ?>>
            Następna →
        </button>
    </div>
    <?php endif; ?>
    
    <?php else: ?>
    <p style="padding: var(--spacing-xl); text-align: center; color: var(--color-ink-light);">
        Brak wiadomości.
    </p>
    <?php endif; ?>
</div>

<style>
    .admin-table tr.unread {
        background: rgba(255, 255, 255, 0.02);
    }
    
    .admin-table tr.unread:hover {
        background: rgba(255, 255, 255, 0.05);
    }
    
    .admin-card-btn.active {
        background: var(--color-highlight);
        color: white;
    }
</style>
