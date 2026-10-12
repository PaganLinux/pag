<?php
/**
 * Szablon sekcji Projektów Webowych
 * Karty projektów na stole warsztatowym (przeciągalne)
 */

// Pobierz projekty i kategorie
$db = get_db_connection();
$categories = get_project_categories();

// Pobierz filtr kategori
$categorySlug = isset($_GET['kategoria']) ? trim($_GET['kategoria']) : null;
$categoryId = null;

if ($categorySlug) {
    $stmt = $db->prepare("SELECT id FROM project_categories WHERE slug = ?");
    $stmt->execute([$categorySlug]);
    $category = $stmt->fetch(PDO::FETCH_ASSOC);
    if ($category) {
        $categoryId = $category['id'];
    }
}

// Pobierz projekty
$query = "SELECT p.*, pc.name as category_name, pc.slug as category_slug, pc.icon as category_icon 
          FROM projects p 
          LEFT JOIN project_categories pc ON p.category_id = pc.id 
          WHERE p.is_published = 1";
$params = [];

if ($categoryId) {
    $query .= " AND p.category_id = ?";
    $params[] = $categoryId;
}

$query .= " ORDER BY p.order_index ASC";

$stmt = $db->prepare($query);
$stmt->execute($params);
$projects = $stmt->fetchAll(PDO::FETCH_ASSOC);

// Pobierz wszystkie projekty (dla statystyk)
$allProjectsStmt = $db->prepare("SELECT COUNT(*) as total FROM projects WHERE is_published = 1");
$allProjectsStmt->execute();
$allProjectsCount = $allProjectsStmt->fetch(PDO::FETCH_ASSOC)['total'];
?>

<!-- ============================================ -->
<!-- PROJECTS SECTION -->
<!-- ============================================ -->
<section class="projects-section" id="projects" aria-labelledby="projects-title">
    <div class="section-container">
        <!-- Nagłówek sekcji -->
        <div class="section-header">
            <div class="section-title-container">
                <h2 id="projects-title" class="section-title">
                    <span class="title-icon">💻</span>
                    <span>PROJEKTY WEB</span>
                </h2>
            </div>
            
            <p class="section-subtitle">
                <?php echo $categorySlug ? 
                    'Projekty z kategorii: <strong>' . escape_output($categories[array_search($categoryId, array_column($categories, 'id'))]['name'] ?? '') . '</strong>' : 
                    'Wszystkie moje projekty webowe'; ?>
            </p>
        </div>
        
        <!-- Filtry -->
        <div class="projects-filters">
            <div class="filter-group">
                <button class="filter-btn <?php echo !$categorySlug ? 'active' : ''; ?>" 
                        data-filter="all"
                        aria-label="Pokaż wszystkie projekty">
                    <span class="filter-icon">🌐</span>
                    <span>WSZYSTKIE</span>
                    <span class="filter-count">(<?php echo $allProjectsCount; ?>)</span>
                </button>
                
                <?php foreach ($categories as $category): ?>
                <button class="filter-btn <?php echo $categorySlug === $category['slug'] ? 'active' : ''; ?>" 
                        data-filter="<?php echo escape_output($category['slug']); ?>"
                        aria-label="Pokaż projekty z kategorii <?php echo escape_output($category['name']); ?>">
                    <span class="filter-icon"><?php echo escape_output($category['icon'] ?? '📁'); ?></span>
                    <span><?php echo escape_output($category['name']); ?></span>
                    <?php 
                    $countStmt = $db->prepare("SELECT COUNT(*) as count FROM projects WHERE category_id = ? AND is_published = 1");
                    $countStmt->execute([$category['id']]);
                    $count = $countStmt->fetch(PDO::FETCH_ASSOC)['count'];
                    if ($count > 0): 
                    ?>
                    <span class="filter-count">(<?php echo $count; ?>)</span>
                    <?php endif; ?>
                </button>
                <?php endforeach; ?>
            </div>
        </div>
        
        <!-- Kontener na projekty (stół warsztatowy) -->
        <div class="workshop-table" id="projects-container">
            <?php if (empty($projects)): ?>
            <div class="empty-state">
                <div class="empty-icon">🔨</div>
                <p class="empty-text">Brak projektów w tej kategorii.</p>
                <p class="empty-subtext">Spróbuj innego filtra.</p>
            </div>
            <?php else: ?>
            <?php foreach ($projects as $index => $project): 
                $technologies = json_decode($project['technologies'] ?? '[]', true);
                $galleryImages = json_decode($project['gallery_images'] ?? '[]', true);
                $coverImage = !empty($project['cover_image']) ? 
                    UPLOADS_URL . '/projects/' . $project['cover_image'] : 
                    ASSETS_URL . '/img/placeholder-project.jpg';
            ?>
            
            <article class="project-card draggable-item" 
                     data-project-id="<?php echo $project['id']; ?>"
                     data-category="<?php echo escape_output($project['category_slug'] ?? ''); ?>"
                     style="--delay: <?php echo $index * 0.1; ?>s;">
                
                <div class="project-card-inner">
                    <!-- Przyczepka (pin) -->
                    <div class="project-pin" aria-hidden="true">📌</div>
                    
                    <!-- Miniatura projektu -->
                    <div class="project-image-container cursor-pincer">
                        <?php if (!empty($project['live_url'])): ?>
                        <a href="<?php echo escape_output($project['live_url']); ?>" 
                           class="project-image-link"
                           target="_blank"
                           rel="noopener noreferrer"
                           aria-label="Zobacz projekt na żywo">
                        <?php endif; ?>
                            <img src="<?php echo $coverImage; ?>" 
                                 alt="<?php echo escape_output($project['title'] ?? 'Projekt'); ?>"
                                 class="project-image"
                                 loading="lazy"
                                 width="400"
                                 height="300">
                        <?php if (!empty($project['live_url'])): ?>
                        </a>
                        <?php endif; ?>
                        
                        <!-- Overlay z ikonami -->
                        <div class="project-overlay">
                            <div class="overlay-actions">
                                <?php if (!empty($project['live_url'])): ?>
                                <a href="<?php echo escape_output($project['live_url']); ?>" 
                                   class="action-btn action-live"
                                   target="_blank"
                                   rel="noopener noreferrer"
                                   aria-label="Zobacz demo"
                                   title="Zobacz demo">
                                    <span>👁️</span>
                                </a>
                                <?php endif; ?>
                                
                                <?php if (!empty($project['github_url'])): ?>
                                <a href="<?php echo escape_output($project['github_url']); ?>" 
                                   class="action-btn action-github"
                                   target="_blank"
                                   rel="noopener noreferrer"
                                   aria-label="Zobacz kod na GitHub"
                                   title="GitHub">
                                    <span>💻</span>
                                </a>
                                <?php endif; ?>
                            </div>
                        </div>
                    </div>
                    
                    <!-- Treść karty -->
                    <div class="project-content">
                        <!-- Kategoria -->
                        <?php if (!empty($project['category_name'])): ?>
                        <div class="project-category">
                            <span class="category-icon"><?php echo escape_output($project['category_icon'] ?? '📁'); ?></span>
                            <span class="category-name"><?php echo escape_output($project['category_name']); ?></span>
                        </div>
                        <?php endif; ?>
                        
                        <!-- Tytuł -->
                        <h3 class="project-title">
                            <a href="<?php echo get_project_url($project); ?>" 
                               class="project-title-link cursor-hand">
                                <?php echo escape_output($project['title']); ?>
                            </a>
                        </h3>
                        
                        <!-- Krótki opis -->
                        <p class="project-description">
                            <?php echo truncate_text(strip_tags($project['short_description'] ?? $project['description'] ?? ''), 120); ?>
                        </p>
                        
                        <!-- Technologie (tagi) -->
                        <?php if (!empty($technologies)): ?>
                        <div class="project-tech">
                            <?php foreach ($technologies as $tech): ?>
                            <span class="tech-tag" title="<?php echo escape_output($tech); ?>">
                                <?php echo escape_output($tech); ?>
                            </span>
                            <?php endforeach; ?>
                        </div>
                        <?php endif; ?>
                        
                        <!-- Linki -->
                        <div class="project-links">
                            <?php if (!empty($project['live_url'])): ?>
                            <a href="<?php echo escape_output($project['live_url']); ?>" 
                               class="project-link link-live cursor-hand"
                               target="_blank"
                               rel="noopener noreferrer">
                                <span>Live Demo</span>
                                <span class="link-icon">↗</span>
                            </a>
                            <?php endif; ?>
                            
                            <?php if (!empty($project['github_url'])): ?>
                            <a href="<?php echo escape_output($project['github_url']); ?>" 
                               class="project-link link-github cursor-hand"
                               target="_blank"
                               rel="noopener noreferrer">
                                <span>GitHub</span>
                                <span class="link-icon">↗</span>
                            </a>
                            <?php endif; ?>
                        </div>
                    </div>
                </div>
                
                <!-- Cień karty (efekt 3D) -->
                <div class="project-shadow" aria-hidden="true"></div>
            </article>
            
            <?php endforeach; ?>
            <?php endif; ?>
        </div>
        
        <!-- Statystyki -->
        <div class="projects-stats">
            <div class="stat-item">
                <span class="stat-icon">📊</span>
                <span class="stat-number"><?php echo count($projects); ?></span>
                <span class="stat-label">Projektów</span>
            </div>
            
            <div class="stat-item">
                <span class="stat-icon">📁</span>
                <span class="stat-number"><?php echo count($categories); ?></span>
                <span class="stat-label">Kategorii</span>
            </div>
            
            <div class="stat-item">
                <span class="stat-icon">💾</span>
                <span class="stat-number">100%</span>
                <span class="stat-label">Custom</span>
            </div>
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
