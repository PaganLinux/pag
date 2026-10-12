<?php
/**
 * Szablon sekcji "O mnie"
 * Narzędzia i informacje o autorze
 */

// Pobierz treść sekcji
$about = get_about_content();
$settings = get_settings();
?>

<!-- ============================================ -->
<!-- ABOUT SECTION -->
<!-- ============================================ -->
<section class="about-section" id="about" aria-labelledby="about-title">
    <div class="section-container">
        <!-- Nagłówek sekcji -->
        <div class="section-header">
            <div class="section-title-container">
                <h2 id="about-title" class="section-title">
                    <span class="title-icon">👤</span>
                    <span>O MNIE</span>
                </h2>
            </div>
            
            <p class="section-subtitle">
                Poznaj moją historię i pasję do tworzenia
            </p>
        </div>
        
        <div class="about-content">
            <!-- Lewa kolumna - Zdjęcie -->
            <div class="about-column about-image-column">
                <div class="about-image-container">
                    <?php if (!empty($about['image_path'])): ?>
                    <img src="<?php echo UPLOADS_URL . '/' . $about['image_path']; ?>" 
                         alt="<?php echo escape_output($about['title'] ?? 'Zdjęcie autora'); ?>"
                         class="about-image"
                         loading="lazy"
                         width="400"
                         height="500">
                    <?php else: ?>
                    <div class="about-image-placeholder">
                        <span class="placeholder-icon">📷</span>
                        <span class="placeholder-text">Zdjęcie autora</span>
                    </div>
                    <?php endif; ?>
                    
                    <!-- Dekoracyjne narzędzia -->
                    <div class="floating-tools">
                        <div class="tool floating" style="--delay: 0s; --x: -20%; --y: 10%;">🔨</div>
                        <div class="tool floating" style="--delay: 0.5s; --x: 110%; --y: 20%;">💻</div>
                        <div class="tool floating" style="--delay: 1s; --x: -10%; --y: 80%;">📏</div>
                        <div class="tool floating" style="--delay: 1.5s; --x: 105%; --y: 70%;">🗜️</div>
                    </div>
                </div>
            </div>
            
            <!-- Prawa kolumna - Tekst -->
            <div class="about-column about-text-column">
                <div class="about-text-container">
                    <!-- Tytuł -->
                    <?php if (!empty($about['title'])): ?>
                    <h3 class="about-section-title">
                        <?php echo escape_output($about['title']); ?>
                    </h3>
                    <?php endif; ?>
                    
                    <!-- Treść -->
                    <div class="about-description">
                        <?php echo nl2br(escape_output($about['content'] ?? '')); ?>
                    </div>
                    
                    <!-- Umiejętności -->
                    <div class="about-skills">
                        <h4 class="skills-title">
                            <span class="skills-icon">🛠️</span>
                            <span>Umiejętności</span>
                        </h4>
                        
                        <div class="skills-grid">
                            <div class="skill-item">
                                <span class="skill-icon">💻</span>
                                <span class="skill-name">Web Development</span>
                            </div>
                            <div class="skill-item">
                                <span class="skill-icon">📱</span>
                                <span class="skill-name">Responsive Design</span>
                            </div>
                            <div class="skill-item">
                                <span class="skill-icon">🎨</span>
                                <span class="skill-name">UI/UX Design</span>
                            </div>
                            <div class="skill-item">
                                <span class="skill-icon">📷</span>
                                <span class="skill-name">Fotografia</span>
                            </div>
                            <div class="skill-item">
                                <span class="skill-icon">🔧</span>
                                <span class="skill-name">Backend (PHP)</span>
                            </div>
                            <div class="skill-item">
                                <span class="skill-icon">⚡</span>
                                <span class="skill-name">Frontend (JS)</span>
                            </div>
                        </div>
                    </div>
                    
                    <!-- CTA Buttons -->
                    <div class="about-cta">
                        <?php if (!empty($settings['github_url'])): ?>
                        <a href="<?php echo escape_output($settings['github_url']); ?>" 
                           class="btn btn-outline cursor-hand"
                           target="_blank"
                           rel="noopener noreferrer"
                           aria-label="GitHub">
                            <span class="btn-icon">💻</span>
                            <span class="btn-text">GITHUB</span>
                        </a>
                        <?php endif; ?>
                        
                        <?php if (!empty($settings['linkedin_url'])): ?>
                        <a href="<?php echo escape_output($settings['linkedin_url']); ?>" 
                           class="btn btn-outline cursor-hand"
                           target="_blank"
                           rel="noopener noreferrer"
                           aria-label="LinkedIn">
                            <span class="btn-icon">💼</span>
                            <span class="btn-text">LINKEDIN</span>
                        </a>
                        <?php endif; ?>
                        
                        <?php if (!empty($settings['instagram_url'])): ?>
                        <a href="<?php echo escape_output($settings['instagram_url']); ?>" 
                           class="btn btn-outline cursor-hand"
                           target="_blank"
                           rel="noopener noreferrer"
                           aria-label="Instagram">
                            <span class="btn-icon">📷</span>
                            <span class="btn-text">INSTAGRAM</span>
                        </a>
                        <?php endif; ?>
                        
                        <a href="<?php echo SITE_URL; ?>/kontakt" 
                           class="btn btn-primary cursor-hand"
                           aria-label="Skontaktuj się">
                            <span class="btn-icon">✉️</span>
                            <span class="btn-text">KONTAKT</span>
                        </a>
                    </div>
                </div>
            </div>
        </div>
        
        <!-- Osiągnięcia / Statystyki -->
        <div class="about-achievements">
            <div class="achievement-item">
                <span class="achievement-icon">📅</span>
                <div class="achievement-content">
                    <span class="achievement-number">10+</span>
                    <span class="achievement-label">Lat doświadczenia</span>
                </div>
            </div>
            
            <div class="achievement-item">
                <span class="achievement-icon">💼</span>
                <div class="achievement-content">
                    <span class="achievement-number">50+</span>
                    <span class="achievement-label">Zrealizowanych projektów</span>
                </div>
            </div>
            
            <div class="achievement-item">
                <span class="achievement-icon">📸</span>
                <div class="achievement-content">
                    <span class="achievement-number">1000+</span>
                    <span class="achievement-label">Zdjęć w galerii</span>
                </div>
            </div>
            
            <div class="achievement-item">
                <span class="achievement-icon">🌟</span>
                <div class="achievement-content">
                    <span class="achievement-number">100%</span>
                    <span class="achievement-label">Zaangażowania</span>
                </div>
            </div>
        </div>
    </div>
</section>

<!-- ============================================ -->
<!-- SEPARATOR -->
<!-- ============================================ -->
<div class="section-separator">
    <div class="separator-line"></div>
    <div class="separator-icon">👤</div>
    <div class="separator-line"></div>
</div>
