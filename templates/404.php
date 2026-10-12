<?php
/**
 * Szablon strony 404
 */
?>

<!-- ============================================ -->
<!-- 404 SECTION -->
<!-- ============================================ -->
<section class="error-section" id="error-404" aria-labelledby="error-title">
    <div class="error-container">
        <div class="error-content">
            <div class="error-icon">⚠️</div>
            <h1 id="error-title" class="error-title">404</h1>
            <p class="error-message">Strona nie została znaleziona</p>
            <p class="error-submessage">
                Przepraszamy, ale strona której szukasz nie istnieje lub została usunięta.
            </p>
            
            <div class="error-actions">
                <a href="<?php echo SITE_URL; ?>" class="btn btn-primary cursor-hand">
                    <span class="btn-icon">🏠</span>
                    <span class="btn-text">WRÓĆ DO STRONY GŁÓWNEJ</span>
                </a>
                
                <a href="<?php echo SITE_URL; ?>/kontakt" class="btn btn-outline cursor-hand">
                    <span class="btn-icon">✉️</span>
                    <span class="btn-text">KONTAKT</span>
                </a>
            </div>
        </div>
        
        <!-- Dekoracyjne narzędzia -->
        <div class="error-tools">
            <div class="tool floating" style="--delay: 0s; --x: 10%; --y: 10%;">🔨</div>
            <div class="tool floating" style="--delay: 0.5s; --x: 90%; --y: 20%;">📏</div>
            <div class="tool floating" style="--delay: 1s; --x: 20%; --y: 80%;">🔧</div>
            <div class="tool floating" style="--delay: 1.5s; --x: 80%; --y: 70%;">🗜️</div>
        </div>
    </div>
</section>
