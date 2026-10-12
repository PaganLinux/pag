<?php
/**
 * Panel administracyjny - Lista galerii
 */
?>

<!-- ============================================
     GALERIA - LISTA
     ============================================ -->
<div class="admin-page-header">
    <div class="admin-breadcrumb">
        <a href="<?php echo SITE_URL; ?>/admin" class="admin-breadcrumb-link">Dashboard</a>
        <span class="admin-breadcrumb-separator">/</span>
        <span class="admin-breadcrumb-current">Galeria</span>
    </div>
    
    <h1 class="admin-page-title">📷 ZARZĄDZANIE GALERIĄ</h1>
    <p class="admin-page-subtitle">Dodawaj, edytuj i usuwaj albumy oraz zdjęcia</p>
</div>

<!-- Albumy -->
<div class="admin-table-container" style="margin-bottom: var(--spacing-xl);">
    <div class="admin-table-header">
        <h2 class="admin-table-title">Albumy (<?php echo count($albums); ?>)</h2>
        <div class="admin-table-actions">
            <a href="<?php echo SITE_URL; ?>/admin/galeria?action=new-album" class="admin-card-btn">
                <span>➕ NOWY ALBUM</span>
            </a>
        </div>
    </div>
    
    <?php if (!empty($albums)): ?>
    <table class="admin-table">
        <thead>
            <tr>
                <th>Nazwa</th>
                <th>Slug</th>
                <th>Zdjęć</th>
                <th>Data utworzenia</th>
                <th>Akcje</th>
            </tr>
        </thead>
        <tbody>
            <?php foreach ($albums as $album): 
                // Pobierz liczbę zdjęć w albumie
                $stmt = $db->prepare("SELECT COUNT(*) as count FROM gallery_images WHERE album_id = ?");
                $stmt->execute([$album['id']]);
                $imageCount = $stmt->fetch(PDO::FETCH_ASSOC)['count'];
            ?>
            <tr>
                <td><?php echo escape_output($album['name']); ?></td>
                <td><code><?php echo escape_output($album['slug']); ?></code></td>
                <td><?php echo $imageCount; ?></td>
                <td><?php echo format_polish_date($album['created_at']); ?></td>
                <td class="admin-table-actions">
                    <a href="<?php echo SITE_URL; ?>/admin/galeria?action=edit-album&id=<?php echo $album['id']; ?>" 
                       class="admin-action-btn" 
                       title="Edytuj">
                        <span>✏️</span>
                    </a>
                    <button class="admin-action-btn delete" 
                            onclick="confirmDelete('<?php echo SITE_URL; ?>/admin?admin_page=delete-album&id=<?php echo $album['id']; ?>', 'Czy na pewno chcesz usunąć album \"' + '<?php echo addslashes($album["name"]); ?>' + '\"?')"
                            title="Usuń">
                        <span>🗑️</span>
                    </button>
                </td>
            </tr>
            <?php endforeach; ?>
        </tbody>
    </table>
    <?php else: ?>
    <p style="padding: var(--spacing-lg); text-align: center; color: var(--color-ink-light);">
        Brak albumów. <a href="<?php echo SITE_URL; ?>/admin/galeria?action=new-album" style="color: var(--color-highlight);">Dodaj pierwszy album</a>.
    </p>
    <?php endif; ?>
</div>

<!-- Obrazy -->
<div class="admin-table-container">
    <div class="admin-table-header">
        <h2 class="admin-table-title">Zdjęcia (<?php echo count($images); ?>)</h2>
        <div class="admin-table-actions">
            <a href="<?php echo SITE_URL; ?>/admin/galeria?action=upload" class="admin-card-btn">
                <span>➕ UPLOAD ZDJĘĆ</span>
            </a>
        </div>
    </div>
    
    <?php if (!empty($images)): ?>
    <div id="gallery-list" class="gallery-drag-container">
        <?php foreach ($images as $index => $image): ?>
        <div class="gallery-item" data-id="<?php echo $image['id']; ?>">
            <div class="gallery-item-image">
                <img src="<?php echo UPLOADS_URL . '/gallery/' . $image['image_path']; ?>" 
                     alt="<?php echo escape_output($image['alt_text'] ?? 'Zdjęcie'); ?>" 
                     style="width: 100%; height: 150px; object-fit: cover; border-radius: var(--radius-sm);">
            </div>
            <div class="gallery-item-info">
                <h4 class="gallery-item-title">
                    <?php echo escape_output(truncate_text($image['title'], 30)); ?>
                </h4>
                <p class="gallery-item-album">
                    Album: <?php echo escape_output($image['album_name'] ?? 'Brak'); ?>
                </p>
                <div class="gallery-item-meta">
                    <span><?php echo format_file_size(filesize(UPLOADS_PATH . '/gallery/' . $image['image_path'])); ?></span>
                    <span><?php echo format_polish_date($image['created_at']); ?></span>
                </div>
            </div>
            <div class="gallery-item-actions">
                <a href="<?php echo UPLOADS_URL . '/gallery/' . $image['image_path']; ?>" 
                   class="admin-action-btn" 
                   target="_blank"
                   title="Zobacz">
                    <span>👁️</span>
                </a>
                <button class="admin-action-btn delete" 
                        onclick="confirmDelete('<?php echo SITE_URL; ?>/admin?admin_page=delete-image&id=<?php echo $image['id']; ?>', 'Czy na pewno chcesz usunąć to zdjęcie?')"
                        title="Usuń">
                    <span>🗑️</span>
                </button>
            </div>
            <div class="gallery-drag-handle" title="Przeciągnij, aby zmienić kolejność">
                <span>⠿⠿⠿</span>
            </div>
        </div>
        <?php endforeach; ?>
    </div>
    
    <script>
    // Inicjalizacja drag & drop dla galerii
    function initGalleryDragDrop() {
        const container = document.getElementById('gallery-list');
        if (!container) return;
        
        if (typeof interact !== 'undefined') {
            interact('.gallery-item')
                .draggable({
                    ignoreFrom: '.admin-action-btn, a, button',
                    allowFrom: '.gallery-drag-handle',
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
                    const items = Array.from(container.querySelectorAll('.gallery-item'));
                    const order = items.map((item, index) => ({
                        id: parseInt(item.dataset.id),
                        order_index: index
                    }));
                    
                    sendAjaxRequest('<?php echo SITE_URL; ?>/admin?admin_page=reorder-images', 'POST', { items: order })
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
    document.addEventListener('DOMContentLoaded', initGalleryDragDrop);
    </script>
    
    <style>
        .gallery-item {
            background: var(--color-paper);
            border: 1px solid var(--color-metal-light);
            border-radius: var(--radius-lg);
            padding: var(--spacing-md);
            margin-bottom: var(--spacing-md);
            display: grid;
            grid-template-columns: 150px 1fr auto auto;
            gap: var(--spacing-md);
            align-items: center;
            transition: all var(--transition-normal);
        }
        
        .gallery-item:hover {
            box-shadow: var(--shadow-md);
        }
        
        .gallery-item.dragging {
            opacity: 0.8;
            box-shadow: var(--shadow-lg);
            z-index: 1000;
        }
        
        .gallery-item-image {
            width: 150px;
            height: 150px;
            border-radius: var(--radius-sm);
            overflow: hidden;
        }
        
        .gallery-item-info {
            flex: 1;
        }
        
        .gallery-item-title {
            font-family: var(--font-primary);
            font-size: 1rem;
            color: var(--color-ink);
            margin-bottom: var(--spacing-xs);
        }
        
        .gallery-item-album {
            font-size: 0.75rem;
            color: var(--color-ink-light);
            margin-bottom: var(--spacing-xs);
        }
        
        .gallery-item-meta {
            display: flex;
            gap: var(--spacing-md);
            font-size: 0.75rem;
            color: var(--color-ink-light);
        }
        
        .gallery-item-actions {
            display: flex;
            flex-direction: column;
            gap: var(--spacing-xs);
        }
        
        .gallery-drag-handle {
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
        
        .gallery-drag-handle:hover {
            background: var(--color-metal-light);
        }
        
        .gallery-drag-handle:active {
            cursor: grabbing;
        }
        
        .gallery-drag-container {
            display: flex;
            flex-direction: column;
            gap: var(--spacing-md);
        }
        
        @media (max-width: 768px) {
            .gallery-item {
                grid-template-columns: 1fr;
                gap: var(--spacing-sm);
            }
            
            .gallery-item-image {
                width: 100%;
                height: 200px;
            }
            
            .gallery-item-actions {
                flex-direction: row;
                justify-content: flex-end;
            }
        }
    </style>
    
    <?php else: ?>
    <p style="padding: var(--spacing-lg); text-align: center; color: var(--color-ink-light);">
        Brak zdjęć. <a href="<?php echo SITE_URL; ?>/admin/galeria?action=upload" style="color: var(--color-highlight);">Dodaj pierwsze zdjęcia</a>.
    </p>
    <?php endif; ?>
</div>
