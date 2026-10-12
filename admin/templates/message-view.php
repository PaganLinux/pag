<?php
/**
 * Panel administracyjny - Podgląd wiadomości
 */
?>

<!-- ============================================
     WIADOMOŚĆ - PODGLĄD
     ============================================ -->
<div class="admin-page-header">
    <div class="admin-breadcrumb">
        <a href="<?php echo SITE_URL; ?>/admin" class="admin-breadcrumb-link">Dashboard</a>
        <span class="admin-breadcrumb-separator">/</span>
        <a href="<?php echo SITE_URL; ?>/admin/wiadomosci" class="admin-breadcrumb-link">Wiadomości</a>
        <span class="admin-breadcrumb-separator">/</span>
        <span class="admin-breadcrumb-current">Wiadomość #<?php echo $message['id']; ?></span>
    </div>
    
    <h1 class="admin-page-title">✉️ WIADOMOŚĆ OD <?php echo escape_output(strtoupper($message['name'])); ?></h1>
    <p class="admin-page-subtitle">
        Otrzymano: <?php echo format_polish_date($message['created_at']); ?>
    </p>
</div>

<!-- Treść wiadomości -->
<div class="admin-form-container">
    <div class="admin-form">
        <!-- Nagłówek -->
        <div class="admin-form-section">
            <h2 class="admin-form-title">Nadawca</h2>
            
            <div class="admin-form-row">
                <div class="admin-form-group">
                    <label class="admin-form-label">Imię i nazwisko</label>
                    <div style="padding: var(--spacing-sm) var(--spacing-md); background: var(--color-paper-dark); border-radius: var(--radius-sm);">
                        <?php echo escape_output($message['name']); ?>
                    </div>
                </div>
                
                <div class="admin-form-group">
                    <label class="admin-form-label">Email</label>
                    <div style="padding: var(--spacing-sm) var(--spacing-md); background: var(--color-paper-dark); border-radius: var(--radius-sm);">
                        <a href="mailto:<?php echo escape_output($message['email']); ?>" 
                           style="color: var(--color-highlight); text-decoration: none;">
                            <?php echo escape_output($message['email']); ?>
                        </a>
                    </div>
                </div>
            </div>
            
            <?php if (!empty($message['subject'])): ?>
            <div class="admin-form-group">
                <label class="admin-form-label">Temat</label>
                <div style="padding: var(--spacing-sm) var(--spacing-md); background: var(--color-paper-dark); border-radius: var(--radius-sm);">
                    <?php echo escape_output($message['subject']); ?>
                </div>
            </div>
            <?php endif; ?>
        </div>
        
        <!-- Treść -->
        <div class="admin-form-section">
            <h2 class="admin-form-title">Treść wiadomości</h2>
            
            <div class="admin-form-group">
                <div style="padding: var(--spacing-lg); background: var(--color-paper-dark); border-radius: var(--radius-sm); line-height: 1.8; white-space: pre-wrap;">
                    <?php echo nl2br(escape_output($message['message'])); ?>
                </div>
            </div>
        </div>
        
        <!-- Informacje dodatkowe -->
        <div class="admin-form-section">
            <h2 class="admin-form-title">Informacje dodatkowe</h2>
            
            <div class="admin-form-row">
                <div class="admin-form-group">
                    <label class="admin-form-label">IP</label>
                    <div style="padding: var(--spacing-sm) var(--spacing-md); background: var(--color-paper-dark); border-radius: var(--radius-sm); font-family: monospace; font-size: 0.875rem;">
                        <?php echo escape_output($message['ip_address'] ?? 'Nieznane'); ?>
                    </div>
                </div>
                
                <div class="admin-form-group">
                    <label class="admin-form-label">User Agent</label>
                    <div style="padding: var(--spacing-sm) var(--spacing-md); background: var(--color-paper-dark); border-radius: var(--radius-sm); font-family: monospace; font-size: 0.75rem; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">
                        <?php echo escape_output($message['user_agent'] ?? 'Nieznane'); ?>
                    </div>
                </div>
            </div>
        </div>
        
        <!-- Akcje -->
        <div class="admin-form-actions">
            <button class="admin-form-btn admin-form-btn-danger" 
                    onclick="confirmDelete('<?php echo SITE_URL; ?>/admin?admin_page=delete-message&id=<?php echo $message['id']; ?>', 'Czy na pewno chcesz usunąć tą wiadomość?')">
                USUŃ WIADOMOŚĆ
            </button>
            
            <a href="<?php echo SITE_URL; ?>/admin/wiadomosci" 
               class="admin-form-btn admin-form-btn-secondary" 
               style="margin-left: auto;">
                WRÓĆ DO LISTY
            </a>
        </div>
    </div>
</div>
