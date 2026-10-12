<?php
/**
 * Panel administracyjny - Lista projektów
 */
?>

<!-- ============================================
     PROJEKTY - LISTA
     ============================================ -->
<div class="admin-page-header">
    <div class="admin-breadcrumb">
        <a href="<?php echo SITE_URL; ?>/admin" class="admin-breadcrumb-link">Dashboard</a>
        <span class="admin-breadcrumb-separator">/</span>
        <span class="admin-breadcrumb-current">Projekty</span>
    </div>
    
    <h1 class="admin-page-title">💻 ZARZĄDZANIE PROJEKTAMI</h1>
    <p class="admin-page-subtitle">Dodawaj, edytuj i usuwaj projekty</p>
</div>

<!-- Akcje -->
<div class="admin-table-container">
    <div class="admin-table-header">
        <h2 class="admin-table-title">Lista projektów (<?php echo count($projects); ?>)</h2>
        <div class="admin-table-actions">
            <a href="<?php echo SITE_URL; ?>/admin/projekty?action=new" class="admin-card-btn">
                <span>➕ DODAJ NOWY</span>
            </a>
        </div>
    </div>
    
    <?php if (!empty($projects)): ?>
    <div id="projects-list" class="projects-drag-container">
        <?php foreach ($projects as $index => $project): 
            $technologies = json_decode($project['technologies'] ?? '[]', true);
        ?>
        <div class="project-item" data-id="<?php echo $project['id']; ?>">
            <div class="project-item-header">
                <div class="project-item-info">
                    <h3 class="project-item-title">
                        <?php echo escape_output($project['title']); ?>
                        <?php if (!$project['is_published']): ?>
                        <span class="project-status draft">Szkic</span>
                        <?php endif; ?>
                    </h3>
                    <p class="project-item-description">
                        <?php echo truncate_text(strip_tags($project['short_description'] ?? $project['description'] ?? ''), 100); ?>
                    </p>
                </div>
                <div class="project-item-actions">
                    <a href="<?php echo SITE_URL; ?>/projekt/<?php echo $project['slug']; ?>" 
                       class="admin-action-btn" 
                       target="_blank"
                       title="Zobacz na stronie">
                        <span>👁️</span>
                    </a>
                    <a href="<?php echo SITE_URL; ?>/admin/projekty?action=edit&id=<?php echo $project['id']; ?>" 
                       class="admin-action-btn" 
                       title="Edytuj">
                        <span>✏️</span>
                    </a>
                    <button class="admin-action-btn delete" 
                            onclick="confirmDelete('<?php echo SITE_URL; ?>/admin?admin_page=delete-project', 'Czy na pewno chcesz usunąć projekt \"' + '<?php echo addslashes($project["title"]); ?>' + '\"?')"
                            title="Usuń">
                        <span>🗑️</span>
                    </button>
                </div>
            </div>
            <div class="project-item-meta">
                <div class="project-meta-group">
                    <span class="project-meta-item">
                        <span class="meta-icon">📁</span>
                        <span><?php echo escape_output($project['category_name'] ?? 'Brak kategorii'); ?></span>
                    </span>
                    <span class="project-meta-item">
                        <span class="meta-icon">🔧</span>
                        <span><?php echo escape_output(implode(', ', $technologies)); ?></span>
                    </span>
                </div>
                <div class="project-meta-group">
                    <span class="project-meta-item">
                        <span class="meta-icon">📅</span>
                        <span><?php echo format_polish_date($project['created_at']); ?></span>
                    </span>
                    <span class="project-meta-item">
                        <span class="meta-icon">👁️</span>
                        <span><?php echo $project['views'] ?? 0; ?></span>
                    </span>
                </div>
            </div>
            <div class="project-drag-handle" title="Przeciągnij, aby zmienić kolejność">
                <span>⠿⠿⠿</span>
            </div>
        </div>
        <?php endforeach; ?>
    </div>
    
    <script>
    // Inicjalizacja drag & drop dla projektów
    function initProjectsDragDrop() {
        const container = document.getElementById('projects-list');
        if (!container) return;
        
        if (typeof interact !== 'undefined') {
            interact('.project-item')
                .draggable({
                    ignoreFrom: '.admin-action-btn, a, button',
                    allowFrom: '.project-drag-handle',
                    listeners: {
                        move: dragMoveListener
                    },
                    modifiers: [
                        interact.modifiers.restrictRect({
                            restriction: container,
                            endOnly: true
                        })
                    ],
                    inertia: true
                })
                .on('down', (event) => {
                    event.target.classList.add('dragging');
                })
                .on('up', (event) => {
                    event.target.classList.remove('dragging');
                    
                    // Zapisz nową kolejność
                    const items = Array.from(container.querySelectorAll('.project-item'));
                    const order = items.map((item, index) => ({
                        id: parseInt(item.dataset.id),
                        order_index: index
                    }));
                    
                    sendAjaxRequest('<?php echo SITE_URL; ?>/admin?admin_page=reorder-projects', 'POST', { items: order })
                        .then(data => {
                            if (data.success) {
                                showNotification('Kolejność zaktualizowana', 'success');
                            }
                        });
                });
        }
    }
    
    function dragMoveListener(event) {
        const target = event.target;
        const x = (parseFloat(target.dataset.x) || 0) + event.dx;
        const y = (parseFloat(target.dataset.y) || 0) + event.dy;
        
        target.dataset.x = x;
        target.dataset.y = y;
        
        target.style.transform = `translate(${x}px, ${y}px)`;
    }
    
    // Inicjalizuj po załadowaniu
    document.addEventListener('DOMContentLoaded', initProjectsDragDrop);
    </script>
    
    <style>
        .project-item {
            background: var(--color-paper);
            border: 1px solid var(--color-metal-light);
            border-radius: var(--radius-lg);
            padding: var(--spacing-lg);
            margin-bottom: var(--spacing-md);
            transition: all var(--transition-normal);
        }
        
        .project-item:hover {
            box-shadow: var(--shadow-md);
        }
        
        .project-item.dragging {
            opacity: 0.8;
            box-shadow: var(--shadow-lg);
            z-index: 1000;
        }
        
        .project-item-header {
            display: flex;
            align-items: flex-start;
            justify-content: space-between;
            gap: var(--spacing-md);
            margin-bottom: var(--spacing-md);
        }
        
        .project-item-info {
            flex: 1;
        }
        
        .project-item-title {
            font-family: var(--font-primary);
            font-size: 1.25rem;
            color: var(--color-ink);
            margin-bottom: var(--spacing-xs);
            display: flex;
            align-items: center;
            gap: var(--spacing-sm);
        }
        
        .project-status {
            font-size: 0.75rem;
            padding: var(--spacing-xs) var(--spacing-sm);
            border-radius: var(--radius-sm);
            background: var(--color-wood-dark);
            color: var(--color-paper);
        }
        
        .project-status.draft {
            background: var(--color-warning);
            color: white;
        }
        
        .project-item-description {
            font-size: 0.875rem;
            color: var(--color-ink-light);
            line-height: 1.5;
        }
        
        .project-item-actions {
            display: flex;
            gap: var(--spacing-xs);
            flex-shrink: 0;
        }
        
        .project-item-meta {
            display: flex;
            flex-wrap: wrap;
            gap: var(--spacing-lg);
            margin-bottom: var(--spacing-sm);
        }
        
        .project-meta-group {
            display: flex;
            gap: var(--spacing-md);
        }
        
        .project-meta-item {
            display: flex;
            align-items: center;
            gap: var(--spacing-xs);
            font-size: 0.75rem;
            color: var(--color-ink-light);
        }
        
        .meta-icon {
            font-size: 0.875rem;
        }
        
        .project-drag-handle {
            display: flex;
            align-items: center;
            justify-content: center;
            padding: var(--spacing-sm);
            background: var(--color-paper-dark);
            border: 1px solid var(--color-metal-light);
            border-radius: var(--radius-sm);
            cursor: grab;
            transition: all var(--transition-fast);
        }
        
        .project-drag-handle:hover {
            background: var(--color-metal-light);
        }
        
        .project-drag-handle:active {
            cursor: grabbing;
        }
        
        .projects-drag-container {
            display: flex;
            flex-direction: column;
            gap: var(--spacing-md);
        }
    </style>
    
    <?php else: ?>
    <p style="padding: var(--spacing-xl); text-align: center; color: var(--color-ink-light);">
        Brak projektów. <a href="<?php echo SITE_URL; ?>/admin/projekty?action=new" style="color: var(--color-highlight);">Dodaj pierwszy projekt</a>.
    </p>
    <?php endif; ?>
</div>
