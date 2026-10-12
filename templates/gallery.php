<?php
/**
 * Szablon sekcji Galerii
 * Polaroidy na stole warsztatowym (przeciągalne, obracalne)
 */

// Pobierz albumy i obrazy
$db = get_db_connection();
$albums = get_gallery_albums();

// Pobierz filtr albumu
$albumSlug = isset($_GET['album']) ? trim($_GET['album']) : null;
$albumId = null;

if ($albumSlug) {
    $stmt = $db->prepare("SELECT id FROM gallery_albums WHERE slug = ?");
    $stmt->execute([$albumSlug]);
    $album = $stmt->fetch(PDO::FETCH_ASSOC);
    if ($album) {
        $albumId = $album['id'];
    }
}

// Pobierz obrazy
$query = "SELECT * FROM gallery_images WHERE is_published = 1";
$params = [];

if ($albumId) {
    $query .= " AND album_id = ?";
    $params[] = $albumId;
}

$query .= " ORDER BY order_index ASC";

$stmt = $db->prepare($query);
$stmt->execute($params);
$images = $stmt->fetchAll(PDO::FETCH_ASSOC);

// Pobierz wszystkie obrazy (dla statystyk)
$allImagesStmt = $db->prepare("SELECT COUNT(*) as total FROM gallery_images WHERE is_published = 1");
$allImagesStmt->execute();
$allImagesCount = $allImagesStmt->fetch(PDO::FETCH_ASSOC)['total'];

// Pobierz obrazy z albumu (jeśli wybrany)
$albumImagesCount = 0;
if ($albumId) {
    $albumImagesStmt = $db->prepare("SELECT COUNT(*) as count FROM gallery_images WHERE album_id = ? AND is_published = 1");
    $albumImagesStmt->execute([$albumId]);
    $albumImagesCount = $albumImagesStmt->fetch(PDO::FETCH_ASSOC)['count'];
}
?>

<!-- ============================================ -->
<!-- GALLERY SECTION -->
<!-- ============================================ -->
<section class="gallery-section" id="gallery" aria-labelledby="gallery-title">
    <div class="section-container">
        <!-- Nagłówek sekcji -->
        <div class="section-header">
            <div class="section-title-container">
                <h2 id="gallery-title" class="section-title">
                    <span class="title-icon">📷</span>
                    <span>GALERIA</span>
                </h2>
            </div>
            
            <p class="section-subtitle">
                <?php echo $albumSlug ? 
                    'Zdjęcia z albumu: <strong>' . escape_output($albums[array_search($albumId, array_column($albums, 'id'))]['name'] ?? '') . '</strong>' : 
                    'Wszystkie moje zdjęcia'; ?>
            </p>
        </div>
        
        <!-- Filtry albumów -->
        <div class="gallery-filters">
            <div class="filter-group">
                <button class="filter-btn <?php echo !$albumSlug ? 'active' : ''; ?>" 
                        data-filter="all"
                        data-type="album"
                        aria-label="Pokaż wszystkie zdjęcia">
                    <span class="filter-icon">🖼️</span>
                    <span>WSZYSTKIE</span>
                    <span class="filter-count">(<?php echo $allImagesCount; ?>)</span>
                </button>
                
                <?php foreach ($albums as $album): ?>
                <button class="filter-btn <?php echo $albumSlug === $album['slug'] ? 'active' : ''; ?>" 
                        data-filter="<?php echo escape_output($album['slug']); ?>"
                        data-type="album"
                        aria-label="Pokaż zdjęcia z albumu <?php echo escape_output($album['name']); ?>">
                    <span class="filter-icon">📂</span>
                    <span><?php echo escape_output($album['name']); ?></span>
                    <?php 
                    $countStmt = $db->prepare("SELECT COUNT(*) as count FROM gallery_images WHERE album_id = ? AND is_published = 1");
                    $countStmt->execute([$album['id']]);
                    $count = $countStmt->fetch(PDO::FETCH_ASSOC)['count'];
                    if ($count > 0): 
                    ?>
                    <span class="filter-count">(<?php echo $count; ?>)</span>
                    <?php endif; ?>
                </button>
                <?php endforeach; ?>
            </div>
        </div>
        
        <!-- Kontener na obrazy (stół z polaroidami) -->
        <div class="polaroid-container" id="gallery-container">
            <?php if (empty($images)): ?>
            <div class="empty-state">
                <div class="empty-icon">📷</div>
                <p class="empty-text">Brak zdjęć w tym albumie.</p>
                <p class="empty-subtext">Spróbuj innego filtra.</p>
            </div>
            <?php else: ?>
            <?php foreach ($images as $index => $image): 
                $albumInfo = null;
                if ($image['album_id']) {
                    foreach ($albums as $album) {
                        if ($album['id'] == $image['album_id']) {
                            $albumInfo = $album;
                            break;
                        }
                    }
                }
                
                // Losowy kąt obrotu (-15 do +15 stopni)
                $rotation = rand(-15, 15);
                // Losowe przesunięcie na osi Z (efekt 3D)
                $translateZ = rand(0, 20);
            ?>
            
            <article class="polaroid draggable-item" 
                     data-image-id="<?php echo $image['id']; ?>"
                     data-album="<?php echo escape_output($albumInfo['slug'] ?? ''); ?>"
                     style="
                         --delay: <?php echo $index * 0.05; ?>s;
                         --rotation: <?php echo $rotation; ?>deg;
                         --translate-z: <?php echo $translateZ; ?>px;
                     ">
                
                <div class="polaroid-inner">
                    <!-- Obrazek -->
                    <div class="polaroid-image-container cursor-pincer">
                        <img src="<?php echo UPLOADS_URL . '/gallery/' . $image['image_path']; ?>" 
                             alt="<?php echo escape_output($image['alt_text'] ?? $image['title'] ?? 'Zdjęcie'); ?>"
                             class="polaroid-image"
                             loading="lazy"
                             width="300"
                             height="400"
                             data-src="<?php echo UPLOADS_URL . '/gallery/' . $image['image_path']; ?>"
                             data-full-src="<?php echo UPLOADS_URL . '/gallery/' . $image['image_path']; ?>">
                        
                        <!-- Overlay z ikonami -->
                        <div class="polaroid-overlay">
                            <div class="overlay-actions">
                                <button class="action-btn action-zoom lightbox-trigger"
                                        data-image-id="<?php echo $image['id']; ?>"
                                        aria-label="Powiększ zdjęcie"
                                        title="Powiększ">
                                    <span>🔍</span>
                                </button>
                                
                                <?php if ($albumInfo): ?>
                                <span class="album-badge" title="<?php echo escape_output($albumInfo['name']); ?>">
                                    <?php echo escape_output(substr($albumInfo['name'], 0, 3)); ?>
                                </span>
                                <?php endif; ?>
                            </div>
                        </div>
                    </div>
                    
                    <!-- Treść polaroida -->
                    <div class="polaroid-content">
                        <?php if (!empty($image['title'])): ?>
                        <h3 class="polaroid-title">
                            <?php echo escape_output($image['title']); ?>
                        </h3>
                        <?php endif; ?>
                        
                        <?php if (!empty($image['description'])): ?>
                        <p class="polaroid-description">
                            <?php echo truncate_text($image['description'], 50); ?>
                        </p>
                        <?php endif; ?>
                    </div>
                    
                    <!-- Pasek polaroida (dolny) -->
                    <div class="polaroid-footer" aria-hidden="true">
                        <div class="footer-stamp"></div>
                    </div>
                </div>
                
                <!-- Cień (efekt 3D) -->
                <div class="polaroid-shadow" aria-hidden="true"></div>
            </article>
            
            <?php endforeach; ?>
            <?php endif; ?>
        </div>
        
        <!-- Statystyki -->
        <div class="gallery-stats">
            <div class="stat-item">
                <span class="stat-icon">📸</span>
                <span class="stat-number"><?php echo count($images); ?></span>
                <span class="stat-label">Zdjęć</span>
            </div>
            
            <div class="stat-item">
                <span class="stat-icon">📂</span>
                <span class="stat-number"><?php echo count($albums); ?></span>
                <span class="stat-label">Albumów</span>
            </div>
            
            <div class="stat-item">
                <span class="stat-icon">🎨</span>
                <span class="stat-number">100%</span>
                <span class="stat-label">Artystyczne</span>
            </div>
        </div>
    </div>
</section>

<!-- ============================================ -->
<!-- LIGHTBOX -->
<!-- ============================================ -->
<div class="lightbox" id="lightbox" aria-hidden="true" role="dialog" aria-modal="true">
    <button class="lightbox-close" aria-label="Zamknij lightbox">
        <span>✕</span>
    </button>
    
    <button class="lightbox-nav lightbox-prev" aria-label="Poprzednie zdjęcie">
        <span>❮</span>
    </button>
    
    <button class="lightbox-nav lightbox-next" aria-label="Następne zdjęcie">
        <span>❯</span>
    </button>
    
    <div class="lightbox-content">
        <div class="lightbox-image-container">
            <img src="" alt="" class="lightbox-image" id="lightbox-image">
        </div>
        
        <div class="lightbox-info">
            <h3 class="lightbox-title" id="lightbox-title"></h3>
            <p class="lightbox-description" id="lightbox-description"></p>
            <p class="lightbox-counter" id="lightbox-counter"></p>
        </div>
    </div>
</div>

<!-- ============================================ -->
<!-- SEPARATOR -->
<!-- ============================================ -->
<div class="section-separator">
    <div class="separator-line"></div>
    <div class="separator-icon">📷</div>
    <div class="separator-line"></div>
</div>
