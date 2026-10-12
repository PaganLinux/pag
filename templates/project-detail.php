<?php
/**
 * Szablon strony szczegółów projektu
 */

// Pobierz projekt po slug
$project = get_project_by_slug($slug);

if (!$project) {
    header("HTTP/1.1 404 Not Found");
    require_once __DIR__ . '/../templates/ '404.php';
    exit;
}

// Dekoduj dane
$technologies = json_decode($project['technologies'] ?? '[]', true);
$galleryImages = json_decode($project['gallery_images'] ?? '[]', true);
$coverImage = !empty($project['cover_image']) ? 
    UPLOADS_URL . '/projects/' . $project['cover_image'] : 
    ASSETS_URL . '/img/placeholder-project.jpg';

// Pobierz kategorie
$categories = get_project_categories();
$category = null;
foreach ($categories as $cat) {
    if ($cat['id'] == $project['category_id']) {
        $category = $cat;
        break;
    }
}

// Inkrementuj licznik wyświetleń (opcjonalnie)
$db = get_db_connection();
$stmt = $db->prepare("UPDATE projects SET views = views + 1 WHERE id = ?");
$stmt->execute([$project['id']]);
?>

<!-- ============================================ -->
<!-- PROJECT DETAIL SECTION -->
<!-- ============================================ -->
<section class="project-detail-section" id="project-detail" aria-labelledby="project-detail-title">
    <div class="section-container">
        <!-- Nagłówek -->
        <div class="detail-header">
            <div class="breadcrumb">
                <a href="<?php echo SITE_URL; ?>/projekty" class="breadcrumb-link cursor-hand">
                    <span>💻 PROJEKTY</span>
                </a>
                <span class="breadcrumb-separator">/</span>
                <span class="breadcrumb-current"><?php echo escape_output($project['title']); ?></span>
            </div>
            
            <div class="detail-title-container">
                <h1 id="project-detail-title" class="detail-title">
                    <?php echo escape_output($project['title']); ?>
                </h1>
                
                <?php if ($category): ?>
                <div class="detail-category">
                    <span class="category-icon"><?php echo escape_output($category['icon'] ?? '📁'); ?></span>
                    <span class="category-name"><?php echo escape_output($category['name']); ?></span>
                </div>
                <?php endif; ?>
            </div>
            
            <!-- Linki -->
            <div class="detail-links">
                <?php if (!empty($project['live_url'])): ?>
                <a href="<?php echo escape_output($project['live_url']); ?>" 
                   class="detail-link link-live cursor-hand"
                   target="_blank"
                   rel="noopener noreferrer">
                    <span class="link-icon">👁️</span>
                    <span>ZOBACZ DEMO</span>
                </a>
                <?php endif; ?>
                
                <?php if (!empty($project['github_url'])): ?>
                <a href="<?php echo escape_output($project['github_url']); ?>" 
                   class="detail-link link-github cursor-hand"
                   target="_blank"
                   rel="noopener noreferrer">
                    <span class="link-icon">💻</span>
                    <span>KOREK NA GITHUB</span>
                </a>
                <?php endif; ?>
            </div>
        </div>
        
        <!-- Główna treść -->
        <div class="detail-content">
            <!-- Lewa kolumna - Miniatura -->
            <div class="detail-column detail-image-column">
                <div class="detail-image-container">
                    <img src="<?php echo $coverImage; ?>" 
                         alt="<?php echo escape_output($project['title']); ?>"
                         class="detail-image"
                         loading="lazy"
                         width="600"
                         height="400">
                    
                    <!-- Galeria miniatur -->
                    <?php if (!empty($galleryImages) && count($galleryImages) > 0): ?>
                    <div class="detail-thumbnails">
                        <?php foreach ($galleryImages as $index => $image): ?>
                        <div class="thumbnail-item cursor-pincer">
                            <img src="<?php echo UPLOADS_URL . '/projects/' . $image; ?>" 
                                 alt="Miniatura <?php echo $index + 1; ?>"
                                 class="thumbnail-image"
                                 loading="lazy"
                                 width="100"
                                 height="75">
                        </div>
                        <?php endforeach; ?>
                    </div>
                    <?php endif; ?>
                </div>
            </div>
            
            <!-- Prawa kolumna - Opis -->
            <div class="detail-column detail-info-column">
                <div class="detail-info">
                    <!-- Krótki opis -->
                    <?php if (!empty($project['short_description'])): ?>
                    <p class="detail-short-description">
                        <?php echo nl2br(escape_output($project['short_description'])); ?>
                    </p>
                    <?php endif; ?>
                    
                    <!-- Pełny opis -->
                    <div class="detail-full-description">
                        <?php echo nl2br(escape_output($project['description'])); ?>
                    </div>
                    
                    <!-- Technologie -->
                    <div class="detail-tech">
                        <h3 class="tech-title">
                            <span class="tech-icon">🔧</span>
                            <span>Technologie</span>
                        </h3>
                        <div class="tech-tags">
                            <?php if (!empty($technologies)): ?>
                                <?php foreach ($technologies as $tech): ?>
                                <span class="tech-tag" title="<?php echo escape_output($tech); ?>">
                                    <?php echo escape_output($tech); ?>
                                </span>
                                <?php endforeach; ?>
                            <?php else: ?>
                            <span class="tech-tag">Brak informacji</span>
                            <?php endif; ?>
                        </div>
                    </div>
                    
                    <!-- Informacje dodatkowe -->
                    <div class="detail-meta">
                        <div class="meta-item">
                            <span class="meta-icon">📅</span>
                            <span class="meta-label">Data utworzenia:</span>
                            <span class="meta-value">
                                <?php echo format_polish_date($project['created_at']); ?>
                            </span>
                        </div>
                        
                        <div class="meta-item">
                            <span class="meta-icon">🔄</span>
                            <span class="meta-label">Ostatnia aktualizacja:</span>
                            <span class="meta-value">
                                <?php echo format_polish_date($project['updated_at']); ?>
                            </span>
                        </div>
                        
                        <div class="meta-item">
                            <span class="meta-icon">👁️</span>
                            <span class="meta-label">Wyświetlenia:</span>
                            <span class="meta-value"><?php echo $project['views'] ?? 0; ?></span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        
        <!-- Powiązane projekty -->
        <?php 
        $relatedProjects = [];
        if ($category) {
            $stmt = $db->prepare("SELECT * FROM projects 
                                  WHERE category_id = ? AND id != ? AND is_published = 1 
                                  ORDER BY RAND() LIMIT 3");
            $stmt->execute([$category['id'], $project['id']]);
            $relatedProjects = $stmt->fetchAll(PDO::FETCH_ASSOC);
        }
        
        if (!empty($relatedProjects)):
        ?>
        <div class="related-projects">
            <h3 class="related-title">
                <span class="related-icon">🔗</span>
                <span>POWIĄZANE PROJEKTY</span>
            </h3>
            
            <div class="related-grid">
                <?php foreach ($relatedProjects as $related): 
                    $relatedTech = json_decode($related['technologies'] ?? '[]', true);
                    $relatedCover = !empty($related['cover_image']) ? 
                        UPLOADS_URL . '/projects/' . $related['cover_image'] : 
                        ASSETS_URL . '/img/placeholder-project.jpg';
                ?>
                
                <article class="related-project">
                    <a href="<?php echo get_project_url($related); ?>" class="related-link cursor-hand">
                        <div class="related-image">
                            <img src="<?php echo $relatedCover; ?>" 
                                 alt="<?php echo escape_output($related['title']); ?>"
                                 loading="lazy"
                                 width="200"
                                 height="150">
                        </div>
                        <div class="related-info">
                            <h4 class="related-title"><?php echo escape_output($related['title']); ?></h4>
                            <div class="related-tech">
                                <?php foreach ($relatedTech as $tech): ?>
                                <span class="tech-tag small"><?php echo escape_output($tech); ?></span>
                                <?php endforeach; ?>
                            </div>
                        </div>
                    </a>
                </article>
                
                <?php endforeach; ?>
            </div>
        </div>
        <?php endif; ?>
        
        <!-- Nawigacja -->
        <div class="detail-navigation">
            <a href="<?php echo SITE_URL; ?>/projekty" class="nav-link cursor-hand">
                <span class="nav-icon">←</span>
                <span>WRÓĆ DO PROJEKTÓW</span>
            </a>
        </div>
    </div>
</section>

<!-- ============================================ -->
<!-- SEPARATOR -->
<!-- ============================================ -->
<div class="section-separator">
    <div class="separator-line"></div>
    <div class="separator-icon">💻</div>
    <div class="separator-line"></div>
</div>
