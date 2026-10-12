<?php
/**
 * Nagłówek strony
 * Zawiera meta tagi, CSS, początek HTML
 */

// Ustaw Content-Type
header('Content-Type: text/html; charset=UTF-8');

// Pobierz ustawienia strony
$settings = get_settings();
$theme = $settings['theme'] ?? 'light';

// Ustaw język
$currentLang = 'pl';
?>
<!DOCTYPE html>
<html lang="<?php echo escape_output($currentLang); ?>">
<head>
    <!-- ============================================ -->
    <!-- META TAGI -->
    <!-- ============================================ -->
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="description" content="<?php echo escape_output($currentPageDescription); ?>">
    <meta name="keywords" content="portfolio, web developer, fotograf, Paweł Gajda, projekty webowe, galeria zdjęć">
    <meta name="author" content="Paweł Gajda">
    
    <!-- Open Graph / Social Media -->
    <meta property="og:title" content="<?php echo escape_output($currentPageTitle); ?>">
    <meta property="og:description" content="<?php echo escape_output($currentPageDescription); ?>">
    <meta property="og:type" content="website">
    <meta property="og:url" content="<?php echo SITE_URL; ?>">
    <meta property="og:image" content="<?php echo UPLOADS_URL; ?>/og-image.jpg">
    
    <!-- Twitter Card -->
    <meta name="twitter:card" content="summary_large_image">
    <meta name="twitter:title" content="<?php echo escape_output($currentPageTitle); ?>">
    <meta name="twitter:description" content="<?php echo escape_output($currentPageDescription); ?>">
    <meta name="twitter:image" content="<?php echo UPLOADS_URL; ?>/og-image.jpg">
    
    <!-- Canonical URL -->
    <link rel="canonical" href="<?php echo SITE_URL . $_SERVER['REQUEST_URI']; ?>">
    
    <!-- ============================================ -->
    <!-- TYTUŁ -->
    <!-- ============================================ -->
    <title><?php echo escape_output($currentPageTitle); ?></title>
    
    <!-- ============================================ -->
    <!-- FAVICON -->
    <!-- ============================================ -->
    <link rel="icon" type="image/x-icon" href="<?php echo ASSETS_URL; ?>/img/favicon.ico">
    <link rel="apple-touch-icon" sizes="180x180" href="<?php echo ASSETS_URL; ?>/img/apple-touch-icon.png">
    
    <!-- ============================================ -->
    <!-- CSS -->
    <!-- ============================================ -->
    
    <!-- Google Fonts -->
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Bebas+Neue&family=Roboto+Mono:wght@300;400;700&display=swap" rel="stylesheet">
    
    <!-- Main CSS -->
    <link rel="stylesheet" href="<?php echo ASSETS_URL; ?>/css/style.css">
    <link rel="stylesheet" href="<?php echo ASSETS_URL; ?>/css/animations.css">
    
    <!-- Theme CSS -->
    <link rel="stylesheet" href="<?php echo ASSETS_URL; ?>/css/<?php echo $theme; ?>-mode.css" id="theme-css">
    
    <!-- Lightbox CSS -->
    <link rel="stylesheet" href="<?php echo ASSETS_URL; ?>/css/lightbox.css">
    
    <!-- ============================================ -->
    <!-- CSRF Token (dla AJAX) -->
    <!-- ============================================ -->
    <?php echo csrf_meta_tag(); ?>
    
    <!-- ============================================ -->
    <!-- JS (ładowane asynchronicznie) -->
    <!-- ============================================ -->
    
    <!-- GSAP (dla animacji) -->
    <script src="https://cdnjs.cloudflare.com/ajax/libs/gsap/3.12.2/gsap.min.js" defer></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/gsap/3.12.2/ScrollTrigger.min.js" defer></script>
    
    <!-- Lenis (smooth scroll) -->
    <script src="https://unpkg.com/lenis@1.0.27/dist/lenis.min.js" defer></script>
    
    <!-- Interact.js (drag & drop) -->
    <script src="https://cdn.jsdelivr.net/npm/interactjs/dist/interact.min.js" defer></script>
    
    <!-- Main JS -->
    <script src="<?php echo ASSETS_URL; ?>/js/theme.js" defer></script>
    <script src="<?php echo ASSETS_URL; ?>/js/main.js" defer></script>
    <script src="<?php echo ASSETS_URL; ?>/js/drag-drop.js" defer></script>
    <script src="<?php echo ASSETS_URL; ?>/js/form.js" defer></script>
    
    <!-- ============================================ -->
    <!-- INLINE STYLES (dla preloadera) -->
    <!-- ============================================ -->
    <style>
        /* Preloader */
        #preloader {
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            background: #5C4033;
            display: flex;
            justify-content: center;
            align-items: center;
            z-index: 9999;
            transition: opacity 0.5s ease, visibility 0.5s ease;
        }
        
        #preloader.hidden {
            opacity: 0;
            visibility: hidden;
        }
        
        .preloader-content {
            text-align: center;
            color: #F8F4E6;
        }
        
        .preloader-icon {
            font-size: 3rem;
            margin-bottom: 1rem;
            animation: rotate 2s linear infinite;
        }
        
        .preloader-text {
            font-family: 'Bebas Neue', sans-serif;
            font-size: 1.5rem;
            letter-spacing: 0.2em;
        }
        
        .preloader-bar {
            width: 200px;
            height: 4px;
            background: #A8A8A8;
            margin: 2rem auto 0;
            border-radius: 2px;
            overflow: hidden;
        }
        
        .preloader-bar-fill {
            height: 100%;
            background: #4A90E2;
            width: 0%;
            animation: loadBar 2s ease-in-out forwards;
        }
        
        @keyframes rotate {
            from { transform: rotate(0deg); }
            to { transform: rotate(360deg); }
        }
        
        @keyframes loadBar {
            0% { width: 0%; }
            50% { width: 70%; }
            100% { width: 100%; }
        }
        
        /* Custom Cursor */
        .custom-cursor {
            position: fixed;
            width: 30px;
            height: 30px;
            border-radius: 50%;
            background: #4A90E2;
            pointer-events: none;
            z-index: 9998;
            mix-blend-mode: difference;
            transition: transform 0.1s ease-out, opacity 0.1s ease-out;
            transform: translate(-50%, -50%);
        }
        
        .custom-cursor::before {
            content: '';
            position: absolute;
            width: 100%;
            height: 100%;
            border: 2px solid #4A90E2;
            border-radius: 50%;
            opacity: 0.5;
        }
        
        .custom-cursor:hover {
            transform: translate(-50%, -50%) scale(1.5);
        }
        
        /* Cursor states */
        .cursor-hand .custom-cursor {
            background: url('data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="%234A90E2"><path d="M12 2c1.1 0 2 .9 2 2s-.9 2-2 2-2-.9-2-2 .9-2 2-2zm9 7h-6v13h-2v-6h-2v6H9V9H3V7h1v2h1V7h1v2h1V7h1v2h2V7h1v2h1V7h2v2h1V7h1v2h2V7h1v2h1V7h2v2zm-3 4h-2v2h2v-2z"/></svg>') center/cover;
        }
        
        .cursor-pincer .custom-cursor {
            background: url('data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="%234A90E2"><path d="M11.5 2C11.5 2 10 8.5 10 12s1.5 10 1.5 10 1.5-10 1.5-10zm-2.5 10c0-1.93 1.57-3.5 3.5-3.5s3.5 1.57 3.5 3.5-1.57 3.5-3.5 3.5-3.5-1.57-3.5-3.5z"/></svg>') center/cover;
        }
        
        .cursor-hammer .custom-cursor {
            background: url('data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="%234A90E2"><path d="M12 2l3.09 8.26L22 9.27l-5.73 4.36L18.18 22H5.82l1.82-7.37L1 9.27l6.91-1.01L12 2z"/></svg>') center/cover;
        }
    </style>
</head>
<body class="theme-<?php echo $theme; ?>">
    <!-- ============================================ -->
    <!-- PRELOADER -->
    <!-- ============================================ -->
    <div id="preloader">
        <div class="preloader-content">
            <div class="preloader-icon">🔨</div>
            <div class="preloader-text">PAWEŁ GAJDA</div>
            <div class="preloader-bar">
                <div class="preloader-bar-fill"></div>
            </div>
        </div>
    </div>
    
    <!-- ============================================ -->
    <!-- CUSTOM CURSOR -->
    <!-- ============================================ -->
    <div class="custom-cursor"></div>
    
    <!-- ============================================ -->
    <!-- NAVBAR -->
    <!-- ============================================ -->
    <nav class="navbar" role="navigation" aria-label="Główna nawigacja">
        <div class="navbar-container">
            <a href="<?php echo SITE_URL; ?>" class="navbar-logo" aria-label="Strona główna">
                <span class="logo-icon">🔨</span>
                <span class="logo-text">PAWEŁ GAJDA</span>
            </a>
            
            <div class="navbar-toggle" aria-label="Przełącz menu" aria-expanded="false" aria-controls="navbar-menu">
                <span class="toggle-line"></span>
                <span class="toggle-line"></span>
                <span class="toggle-line"></span>
            </div>
            
            <ul class="navbar-menu" id="navbar-menu" role="menubar">
                <li role="none">
                    <a href="<?php echo SITE_URL; ?>" class="navbar-link <?php echo $page === 'home' ? 'active' : ''; ?>" role="menuitem">
                        <span class="nav-icon">🏠</span>
                        <span>START</span>
                    </a>
                </li>
                <li role="none">
                    <a href="<?php echo SITE_URL; ?>/projekty" class="navbar-link <?php echo $page === 'projekty' ? 'active' : ''; ?>" role="menuitem">
                        <span class="nav-icon">💻</span>
                        <span>PROJEKTY</span>
                    </a>
                </li>
                <li role="none">
                    <a href="<?php echo SITE_URL; ?>/galeria" class="navbar-link <?php echo $page === 'galeria' ? 'active' : ''; ?>" role="menuitem">
                        <span class="nav-icon">📷</span>
                        <span>GALERIA</span>
                    </a>
                </li>
                <li role="none">
                    <a href="<?php echo SITE_URL; ?>/o-mnie" class="navbar-link <?php echo $page === 'o-mnie' ? 'active' : ''; ?>" role="menuitem">
                        <span class="nav-icon">👤</span>
                        <span>O MNIE</span>
                    </a>
                </li>
                <li role="none">
                    <a href="<?php echo SITE_URL; ?>/kontakt" class="navbar-link <?php echo $page === 'kontakt' ? 'active' : ''; ?>" role="menuitem">
                        <span class="nav-icon">✉️</span>
                        <span>KONTAKT</span>
                    </a>
                </li>
                
                <?php if (is_admin_logged_in()): ?>
                <li role="none">
                    <a href="<?php echo SITE_URL; ?>/admin" class="navbar-link admin-link" role="menuitem">
                        <span class="nav-icon">⚙️</span>
                        <span>ADMIN</span>
                    </a>
                </li>
                <?php endif; ?>
            </ul>
            
            <div class="navbar-actions">
                <button class="theme-toggle" aria-label="Przełącz tryb kolorów" title="Przełącz tryb">
                    <span class="theme-icon light">☀️</span>
                    <span class="theme-icon dark">🌙</span>
                </button>
            </div>
        </div>
    </nav>
    
    <!-- ============================================ -->
    <!-- GŁÓWNY KONTAINER -->
    <!-- ============================================ -->
    <main class="main-content" role="main">
