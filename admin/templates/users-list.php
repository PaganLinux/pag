<?php
/**
 * Panel administracyjny - Lista użytkowników
 */
?>

<!-- ============================================
     UŻYTKOWNICY - LISTA
     ============================================ -->
<div class="admin-page-header">
    <div class="admin-breadcrumb">
        <a href="<?php echo SITE_URL; ?>/admin" class="admin-breadcrumb-link">Dashboard</a>
        <span class="admin-breadcrumb-separator">/</span>
        <span class="admin-breadcrumb-current">Użytkownicy</span>
    </div>
    
    <h1 class="admin-page-title">👥 ZARZĄDZANIE UŻYTKOWNIKAMI</h1>
    <p class="admin-page-subtitle">Dodawaj, edytuj i usuwaj konta administratorów</p>
</div>

<!-- Lista użytkowników -->
<div class="admin-table-container">
    <div class="admin-table-header">
        <h2 class="admin-table-title">Lista użytkowników (<?php echo count($users); ?>)</h2>
        <div class="admin-table-actions">
            <a href="<?php echo SITE_URL; ?>/admin/uzytkownicy?action=new" class="admin-card-btn">
                <span>➕ NOWY UŻYTKOWNIK</span>
            </a>
        </div>
    </div>
    
    <?php if (!empty($users)): ?>
    <table class="admin-table">
        <thead>
            <tr>
                <th>ID</th>
                <th>Nazwa użytkownika</th>
                <th>Email</th>
                <th>Ostatnie logowanie</th>
                <th>Akcje</th>
            </tr>
        </thead>
        <tbody>
            <?php foreach ($users as $user): 
                // Pomijamy własne konto w akcji usuwania
                $isCurrentUser = $adminUser && $adminUser['id'] == $user['id'];
            ?>
            <tr>
                <td><?php echo $user['id']; ?></td>
                <td><?php echo escape_output($user['username']); ?></td>
                <td><?php echo escape_output($user['email']); ?></td>
                <td><?php echo $user['last_login'] ? format_polish_date($user['last_login']) : 'Nigdy'; ?></td>
                <td class="admin-table-actions">
                    <a href="<?php echo SITE_URL; ?>/admin/uzytkownicy?action=edit&id=<?php echo $user['id']; ?>" 
                       class="admin-action-btn" 
                       title="Edytuj">
                        <span>✏️</span>
                    </a>
                    
                    <?php if (!$isCurrentUser): ?>
                    <button class="admin-action-btn delete" 
                            onclick="confirmDelete('<?php echo SITE_URL; ?>/admin?admin_page=delete-user&id=<?php echo $user['id']; ?>', 'Czy na pewno chcesz usunąć użytkownika \"' + '<?php echo addslashes($user["username"]); ?>' + '\"?')"
                            title="Usuń">
                        <span>🗑️</span>
                    </button>
                    <?php else: ?>
                    <button class="admin-action-btn delete" 
                            title="Nie można usunąć własnego konta"
                            disabled
                            style="opacity: 0.5; cursor: not-allowed;">
                        <span>🗑️</span>
                    </button>
                    <?php endif; ?>
                </td>
            </tr>
            <?php endforeach; ?>
        </tbody>
    </table>
    
    <?php else: ?>
    <p style="padding: var(--spacing-xl); text-align: center; color: var(--color-ink-light);">
        Brak użytkowników. <a href="<?php echo SITE_URL; ?>/admin/uzytkownicy?action=new" style="color: var(--color-highlight);">Dodaj pierwszego użytkownika</a>.
    </p>
    <?php endif; ?>
</div>
