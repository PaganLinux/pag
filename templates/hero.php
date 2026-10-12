<?php
/**
 * Szablon sekcji Hero
 * Stół warsztatowy z narzędziami
 */

// Pobierz ustawienia
$settings = get_settings();
$about = get_about_content();
?>

<!-- ============================================ -->
<!-- HERO SECTION -->
<!-- ============================================ -->
<section class="hero-section" id="hero" aria-labelledby="hero-title">
    <div class="hero-background">
        <!-- Tekstura drewniana -->
        <div class="wood-texture"></div>
        
        <!-- Narzędzia w tle (animowane) -->
        <div class="background-tools">
            <div class="tool floating" style="--delay: 0s; --x: 10%; --y: 20%;">🔨</div>
            <div class="tool floating" style="--delay: 0.5s; --x: 80%; --y: 15%;">📏</div>
            <div class="tool floating" style="--delay: 1s; --x: 20%; --y: 70%;">🔧</div>
            <div class="tool floating" style="--delay: 1.5s; --x: 70%; --y: 60%;">🗜️</div>
            <div class="tool floating" style="--delay: 2s; --x: 40%; --y: 80%;">📸</div>
            <div class="tool floating" style="--delay: 2.5s; --x: 90%; --y: 40%;">🪛</div>
        </div>
    </div>
    
    <div class="hero-container">
        <div class="hero-content">
            <!-- Tytuł -->
            <div class="hero-title-container">
                <h1 id="hero-title" class="hero-title">
                    <span class="title-line">PAWEŁ</span>
                    <span class="title-line">GAJDA</span>
                </h1>
            </div>
            
            <!-- Podtytuł -->
            <p class="hero-subtitle">
                <span class="typing-text" data-text="Web Developer + Photographer"></span>
                <span class="cursor-blink">|</span>
            </p>
            
            <!-- Opis -->
            <p class="hero-description">
                <?php echo nl2br(escape_output($about['content'] ?? 'Tworzę unikalne strony internetowe i dokumentuję świat przez obiektyw.')); ?>
            </p>
            
            <!-- CTA Buttons -->
            <div class="hero-cta">
                <a href="<?php echo SITE_URL; ?>/projekty" 
                   class="btn btn-primary btn-large cursor-hand"
                   aria-label="Zobacz moje projekty">
                    <span class="btn-icon">💻</span>
                    <span class="btn-text">PROJEKTY WEB</span>
                </a>
                
                <a href="<?php echo SITE_URL; ?>/galeria" 
                   class="btn btn-secondary btn-large cursor-hand"
                   aria-label="Zobacz galerię zdjęć">
                    <span class="btn-icon">📷</span>
                    <span class="btn-text">GALERIA</span>
                </a>
            </div>
            
            <!-- Scroll indicator -->
            <div class="scroll-indicator" aria-hidden="true">
                <span class="scroll-text">Przeciągnij w dół</span>
                <span class="scroll-arrow">↓</span>
            </div>
        </div>
        
        <!-- Dekoracyjne elementy -->
        <div class="hero-decoration">
            <div class="corner top-left"></div>
            <div class="corner top-right"></div>
            <div class="corner bottom-left"></div>
            <div class="corner bottom-right"></div>
        </div>
    </div>
</section>

<!-- ============================================ -->
<!-- SEPARATOR -->
<!-- ============================================ -->
<div class="section-separator">
    <div class="separator-line"></div>
    <div class="separator-icon">🔨</div>
    <div class="separator-line"></div>
</div>
