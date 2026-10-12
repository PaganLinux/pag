<?php
/**
 * Panel administracyjny - Nagłówek
 */

// Pobierz ustawienia
$settings = get_settings();
$adminUser = get_admin_user();
?>
<!DOCTYPE html>
<html lang="pl">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="robots" content="noindex, nofollow">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    
    <title>Panel Administracyjny - <?php echo escape_output($settings['site_title'] ?? 'Portfolio'); ?></title>
    
    <!-- Google Fonts -->
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Bebas+Neue&family=Roboto+Mono:wght@300;400;700&display=swap" rel="stylesheet">
    
    <!-- Font Awesome (opcjonalnie) -->
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css">
    
    <!-- GSAP -->
    <script src="https://cdnjs.cloudflare.com/ajax/libs/gsap/3.12.2/gsap.min.js" defer></script>
    
    <!-- Main CSS -->
    <link rel="stylesheet" href="<?php echo ASSETS_URL; ?>/css/style.css">
    
    <!-- Admin CSS -->
    <link rel="stylesheet" href="<?php echo ASSETS_URL; ?>/css/animations.css">
    <style>
        /* ============================================
           ADMIN - STYL
           ============================================ */
        
        body {
            font-family: var(--font-secondary);
            background: var(--color-paper-dark);
            color: var(--color-ink);
            min-height: 100vh;
            display: flex;
            flex-direction: column;
        }
        
        /* Admin Navbar */
        .admin-navbar {
            background: var(--color-wood-dark);
            color: var(--color-paper);
            padding: 0;
            position: sticky;
            top: 0;
            z-index: var(--z-navbar);
            box-shadow: var(--shadow-md);
        }
        
        .admin-navbar-container {
            display: flex;
            align-items: center;
            justify-content: space-between;
            max-width: 100%;
            margin: 0 auto;
            padding: 0 var(--spacing-lg);
            height: var(--navbar-height);
        }
        
        .admin-navbar-logo {
            display: flex;
            align-items: center;
            gap: var(--spacing-sm);
            text-decoration: none;
            color: var(--color-paper);
            font-family: var(--font-primary);
            font-size: 1.25rem;
            letter-spacing: 0.1em;
        }
        
        .admin-navbar-logo .logo-icon {
            font-size: 1.5rem;
        }
        
        .admin-navbar-menu {
            display: flex;
            align-items: center;
            gap: var(--spacing-md);
            list-style: none;
            margin: 0;
            padding: 0;
        }
        
        .admin-navbar-link {
            display: flex;
            align-items: center;
            gap: var(--spacing-xs);
            padding: var(--spacing-sm) var(--spacing-md);
            color: var(--color-paper);
            text-decoration: none;
            border-radius: var(--radius-sm);
            transition: all var(--transition-fast);
            font-size: 0.875rem;
            font-family: var(--font-secondary);
        }
        
        .admin-navbar-link:hover {
            background: var(--color-wood);
        }
        
        .admin-navbar-link.active {
            background: var(--color-highlight);
        }
        
        .admin-navbar-link .nav-icon {
            font-size: 1rem;
        }
        
        .admin-navbar-actions {
            display: flex;
            align-items: center;
            gap: var(--spacing-md);
        }
        
        .admin-user-menu {
            position: relative;
        }
        
        .admin-user-btn {
            display: flex;
            align-items: center;
            gap: var(--spacing-sm);
            padding: var(--spacing-sm) var(--spacing-md);
            background: none;
            border: none;
            color: var(--color-paper);
            cursor: pointer;
            border-radius: var(--radius-sm);
            transition: all var(--transition-fast);
        }
        
        .admin-user-btn:hover {
            background: var(--color-wood);
        }
        
        .admin-user-avatar {
            width: 35px;
            height: 35px;
            background: var(--color-highlight);
            color: white;
            border-radius: 50%;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 1rem;
        }
        
        .admin-user-name {
            font-size: 0.875rem;
        }
        
        .admin-user-dropdown {
            position: absolute;
            top: 100%;
            right: 0;
            background: var(--color-wood-dark);
            min-width: 200px;
            border-radius: var(--radius-sm);
            box-shadow: var(--shadow-lg);
            opacity: 0;
            visibility: hidden;
            transform: translateY(-10px);
            transition: all var(--transition-normal);
            z-index: 1000;
        }
        
        .admin-user-dropdown.active {
            opacity: 1;
            visibility: visible;
            transform: translateY(0);
        }
        
        .admin-user-dropdown ul {
            list-style: none;
            margin: 0;
            padding: var(--spacing-xs) 0;
        }
        
        .admin-user-dropdown li {
            margin: 0;
        }
        
        .admin-user-dropdown a {
            display: block;
            padding: var(--spacing-sm) var(--spacing-md);
            color: var(--color-paper);
            text-decoration: none;
            transition: all var(--transition-fast);
        }
        
        .admin-user-dropdown a:hover {
            background: var(--color-wood);
        }
        
        .admin-user-dropdown .dropdown-icon {
            margin-right: var(--spacing-sm);
        }
        
        /* Admin Sidebar */
        .admin-sidebar {
            background: var(--color-wood);
            color: var(--color-paper);
            width: var(--sidebar-width);
            min-height: 100vh;
            position: fixed;
            top: var(--navbar-height);
            left: 0;
            z-index: calc(var(--z-navbar) - 1);
            padding: var(--spacing-lg) 0;
            display: flex;
            flex-direction: column;
        }
        
        .admin-sidebar-header {
            padding: 0 var(--spacing-lg) var(--spacing-md);
            border-bottom: 1px solid rgba(255, 255, 255, 0.1);
            margin-bottom: var(--spacing-md);
        }
        
        .admin-sidebar-title {
            font-family: var(--font-primary);
            font-size: 0.75rem;
            text-transform: uppercase;
            letter-spacing: 0.2em;
            color: rgba(255, 255, 255, 0.7);
        }
        
        .admin-sidebar-nav {
            flex: 1;
            padding: 0 var(--spacing-sm);
        }
        
        .admin-sidebar-nav ul {
            list-style: none;
            margin: 0;
            padding: 0;
        }
        
        .admin-sidebar-nav li {
            margin: 0;
        }
        
        .admin-sidebar-link {
            display: flex;
            align-items: center;
            gap: var(--spacing-sm);
            padding: var(--spacing-sm) var(--spacing-md);
            color: var(--color-paper);
            text-decoration: none;
            border-radius: var(--radius-sm);
            transition: all var(--transition-fast);
            margin: var(--spacing-xs) var(--spacing-sm);
        }
        
        .admin-sidebar-link:hover {
            background: var(--color-wood-dark);
        }
        
        .admin-sidebar-link.active {
            background: var(--color-highlight);
        }
        
        .admin-sidebar-link .nav-icon {
            font-size: 1.125rem;
            width: 24px;
            text-align: center;
        }
        
        .admin-sidebar-link .nav-text {
            font-size: 0.875rem;
        }
        
        .admin-sidebar-footer {
            padding: var(--spacing-md) var(--spacing-lg);
            border-top: 1px solid rgba(255, 255, 255, 0.1);
            margin-top: auto;
        }
        
        /* Admin Main Content */
        .admin-main {
            flex: 1;
            margin-left: var(--sidebar-width);
            padding: var(--spacing-xl);
            min-height: calc(100vh - var(--navbar-height));
        }
        
        .admin-page-header {
            margin-bottom: var(--spacing-xl);
        }
        
        .admin-page-title {
            font-family: var(--font-primary);
            font-size: 2rem;
            color: var(--color-ink);
            margin-bottom: var(--spacing-sm);
        }
        
        .admin-page-subtitle {
            color: var(--color-ink-light);
            font-size: 1rem;
        }
        
        .admin-breadcrumb {
            display: flex;
            align-items: center;
            gap: var(--spacing-sm);
            margin-bottom: var(--spacing-md);
            font-size: 0.875rem;
        }
        
        .admin-breadcrumb a {
            color: var(--color-highlight);
            text-decoration: none;
        }
        
        .admin-breadcrumb a:hover {
            text-decoration: underline;
        }
        
        .admin-breadcrumb-separator {
            color: var(--color-metal);
        }
        
        .admin-breadcrumb-current {
            color: var(--color-ink-light);
        }
        
        /* Admin Cards */
        .admin-cards {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: var(--spacing-lg);
            margin-bottom: var(--spacing-xl);
        }
        
        .admin-card {
            background: var(--color-paper);
            border-radius: var(--radius-lg);
            padding: var(--spacing-lg);
            box-shadow: var(--shadow-sm);
            border: 1px solid var(--color-metal-light);
        }
        
        .admin-card-header {
            display: flex;
            align-items: center;
            justify-content: space-between;
            margin-bottom: var(--spacing-md);
        }
        
        .admin-card-title {
            font-family: var(--font-primary);
            font-size: 1rem;
            color: var(--color-ink);
        }
        
        .admin-card-icon {
            font-size: 1.5rem;
            color: var(--color-wood-dark);
        }
        
        .admin-card-content {
            margin-bottom: var(--spacing-md);
        }
        
        .admin-card-value {
            font-family: var(--font-primary);
            font-size: 2rem;
            color: var(--color-wood-dark);
        }
        
        .admin-card-label {
            font-size: 0.75rem;
            color: var(--color-ink-light);
            text-transform: uppercase;
            letter-spacing: 0.1em;
        }
        
        .admin-card-actions {
            display: flex;
            gap: var(--spacing-sm);
        }
        
        .admin-card-btn {
            padding: var(--spacing-xs) var(--spacing-sm);
            background: var(--color-wood-dark);
            color: var(--color-paper);
            border: none;
            border-radius: var(--radius-sm);
            font-size: 0.75rem;
            cursor: pointer;
            transition: all var(--transition-fast);
        }
        
        .admin-card-btn:hover {
            background: var(--color-wood);
        }
        
        /* Admin Tables */
        .admin-table-container {
            background: var(--color-paper);
            border-radius: var(--radius-lg);
            border: 1px solid var(--color-metal-light);
            overflow: hidden;
        }
        
        .admin-table-header {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: var(--spacing-lg);
            border-bottom: 1px solid var(--color-metal-light);
        }
        
        .admin-table-title {
            font-family: var(--font-primary);
            font-size: 1.25rem;
            color: var(--color-ink);
        }
        
        .admin-table-actions {
            display: flex;
            gap: var(--spacing-sm);
        }
        
        .admin-table {
            width: 100%;
            border-collapse: collapse;
        }
        
        .admin-table th,
        .admin-table td {
            padding: var(--spacing-md) var(--spacing-lg);
            text-align: left;
            border-bottom: 1px solid var(--color-metal-light);
        }
        
        .admin-table th {
            background: var(--color-paper-dark);
            font-family: var(--font-primary);
            font-size: 0.75rem;
            text-transform: uppercase;
            letter-spacing: 0.1em;
            color: var(--color-ink-light);
        }
        
        .admin-table tr:hover {
            background: var(--color-paper-dark);
        }
        
        .admin-table-actions {
            display: flex;
            gap: var(--spacing-xs);
        }
        
        .admin-action-btn {
            padding: var(--spacing-xs) var(--spacing-sm);
            border: 1px solid var(--color-metal);
            background: var(--color-paper);
            color: var(--color-ink);
            border-radius: var(--radius-sm);
            font-size: 0.75rem;
            cursor: pointer;
            transition: all var(--transition-fast);
        }
        
        .admin-action-btn:hover {
            background: var(--color-wood-dark);
            color: var(--color-paper);
            border-color: var(--color-wood-dark);
        }
        
        .admin-action-btn.delete:hover {
            background: var(--color-warning);
            color: white;
            border-color: var(--color-warning);
        }
        
        /* Admin Forms */
        .admin-form-container {
            background: var(--color-paper);
            border-radius: var(--radius-lg);
            border: 1px solid var(--color-metal-light);
            padding: var(--spacing-xl);
        }
        
        .admin-form {
            display: flex;
            flex-direction: column;
            gap: var(--spacing-lg);
        }
        
        .admin-form-section {
            padding-bottom: var(--spacing-lg);
            border-bottom: 1px solid var(--color-metal-light);
        }
        
        .admin-form-section:last-child {
            border-bottom: none;
            padding-bottom: 0;
        }
        
        .admin-form-title {
            font-family: var(--font-primary);
            font-size: 1.25rem;
            color: var(--color-ink);
            margin-bottom: var(--spacing-md);
        }
        
        .admin-form-group {
            margin-bottom: var(--spacing-md);
        }
        
        .admin-form-label {
            display: block;
            font-family: var(--font-primary);
            font-size: 0.875rem;
            color: var(--color-ink);
            margin-bottom: var(--spacing-xs);
        }
        
        .admin-form-input,
        .admin-form-textarea,
        .admin-form-select {
            width: 100%;
            padding: var(--spacing-sm) var(--spacing-md);
            border: 2px solid var(--color-metal);
            border-radius: var(--radius-sm);
            font-family: var(--font-secondary);
            font-size: 1rem;
            color: var(--color-ink);
            background: var(--color-paper-dark);
            transition: all var(--transition-fast);
        }
        
        .admin-form-input:focus,
        .admin-form-textarea:focus,
        .admin-form-select:focus {
            outline: none;
            border-color: var(--color-highlight);
            box-shadow: 0 0 0 3px rgba(74, 144, 226, 0.1);
        }
        
        .admin-form-input::placeholder,
        .admin-form-textarea::placeholder {
            color: var(--color-ink-light);
        }
        
        .admin-form-textarea {
            resize: vertical;
            min-height: 100px;
        }
        
        .admin-form-row {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: var(--spacing-lg);
        }
        
        .admin-form-checkbox {
            display: flex;
            align-items: center;
            gap: var(--spacing-sm);
        }
        
        .admin-form-checkbox input[type="checkbox"] {
            width: 18px;
            height: 18px;
            accent-color: var(--color-highlight);
        }
        
        .admin-form-checkbox .checkbox-label {
            font-size: 0.875rem;
            color: var(--color-ink);
        }
        
        .admin-form-actions {
            display: flex;
            gap: var(--spacing-md);
            justify-content: flex-end;
            padding-top: var(--spacing-lg);
            border-top: 1px solid var(--color-metal-light);
        }
        
        .admin-form-btn {
            padding: var(--spacing-md) var(--spacing-lg);
            border: none;
            border-radius: var(--radius-sm);
            font-family: var(--font-primary);
            font-size: 0.875rem;
            letter-spacing: 0.1em;
            text-transform: uppercase;
            cursor: pointer;
            transition: all var(--transition-fast);
        }
        
        .admin-form-btn-primary {
            background: var(--color-wood-dark);
            color: var(--color-paper);
        }
        
        .admin-form-btn-primary:hover {
            background: var(--color-wood);
        }
        
        .admin-form-btn-secondary {
            background: var(--color-paper-dark);
            color: var(--color-ink);
            border: 1px solid var(--color-metal);
        }
        
        .admin-form-btn-secondary:hover {
            background: var(--color-metal-light);
        }
        
        .admin-form-btn-danger {
            background: var(--color-warning);
            color: white;
        }
        
        .admin-form-btn-danger:hover {
            background: #b00020;
        }
        
        .admin-form-error {
            display: block;
            font-size: 0.75rem;
            color: var(--color-warning);
            margin-top: var(--spacing-xs);
        }
        
        .admin-form-success {
            padding: var(--spacing-md);
            background: rgba(56, 142, 60, 0.1);
            border: 1px solid var(--color-success);
            border-radius: var(--radius-sm);
            margin-bottom: var(--spacing-md);
            color: var(--color-success);
            font-size: 0.875rem;
        }
        
        /* Pagination */
        .admin-pagination {
            display: flex;
            align-items: center;
            justify-content: center;
            gap: var(--spacing-sm);
            margin-top: var(--spacing-xl);
        }
        
        .admin-pagination-btn {
            padding: var(--spacing-xs) var(--spacing-sm);
            border: 1px solid var(--color-metal);
            background: var(--color-paper);
            color: var(--color-ink);
            border-radius: var(--radius-sm);
            cursor: pointer;
            transition: all var(--transition-fast);
        }
        
        .admin-pagination-btn:hover:not(:disabled) {
            background: var(--color-wood-dark);
            color: var(--color-paper);
            border-color: var(--color-wood-dark);
        }
        
        .admin-pagination-btn:disabled {
            opacity: 0.5;
            cursor: not-allowed;
        }
        
        .admin-pagination-info {
            font-size: 0.875rem;
            color: var(--color-ink-light);
        }
        
        /* Modal */
        .admin-modal {
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            background: rgba(0, 0, 0, 0.7);
            z-index: var(--z-modal);
            display: flex;
            align-items: center;
            justify-content: center;
            opacity: 0;
            visibility: hidden;
            transition: all var(--transition-normal);
        }
        
        .admin-modal.active {
            opacity: 1;
            visibility: visible;
        }
        
        .admin-modal-content {
            background: var(--color-paper);
            border-radius: var(--radius-lg);
            max-width: 500px;
            width: 90%;
            max-height: 90vh;
            overflow-y: auto;
            position: relative;
        }
        
        .admin-modal-header {
            padding: var(--spacing-lg);
            border-bottom: 1px solid var(--color-metal-light);
            display: flex;
            align-items: center;
            justify-content: space-between;
        }
        
        .admin-modal-title {
            font-family: var(--font-primary);
            font-size: 1.25rem;
            color: var(--color-ink);
        }
        
        .admin-modal-close {
            background: none;
            border: none;
            font-size: 1.5rem;
            color: var(--color-ink-light);
            cursor: pointer;
            padding: var(--spacing-xs);
            line-height: 1;
        }
        
        .admin-modal-close:hover {
            color: var(--color-ink);
        }
        
        .admin-modal-body {
            padding: var(--spacing-lg);
        }
        
        .admin-modal-footer {
            padding: var(--spacing-lg);
            border-top: 1px solid var(--color-metal-light);
            display: flex;
            gap: var(--spacing-md);
            justify-content: flex-end;
        }
        
        /* Loading State */
        .loading {
            position: relative;
            pointer-events: none;
        }
        
        .loading::after {
            content: '';
            position: absolute;
            top: 50%;
            left: 50%;
            width: 20px;
            height: 20px;
            margin: -10px 0 0 -10px;
            border: 2px solid var(--color-highlight);
            border-top-color: transparent;
            border-radius: 50%;
            animation: spin 0.8s linear infinite;
        }
        
        @keyframes spin {
            to { transform: rotate(360deg); }
        }
        
        /* Responsive */
        @media (max-width: 1024px) {
            .admin-sidebar {
                width: 250px;
            }
            
            .admin-main {
                margin-left: 250px;
            }
        }
        
        @media (max-width: 768px) {
            .admin-sidebar {
                position: fixed;
                top: 0;
                left: -100%;
                width: var(--sidebar-width);
                z-index: calc(var(--z-navbar) + 1);
                transition: all var(--transition-normal);
            }
            
            .admin-sidebar.active {
                left: 0;
            }
            
            .admin-main {
                margin-left: 0;
                padding: var(--spacing-lg);
            }
            
            .admin-navbar-container {
                padding: 0 var(--spacing-md);
            }
            
            .admin-navbar-menu {
                display: none;
            }
            
            .admin-navbar-toggle {
                display: flex;
                flex-direction: column;
                gap: 5px;
                padding: var(--spacing-xs);
                background: none;
                border: none;
                cursor: pointer;
            }
            
            .admin-navbar-toggle .toggle-line {
                width: 25px;
                height: 2px;
                background: var(--color-paper);
                transition: all var(--transition-fast);
            }
            
            .admin-navbar-toggle.active .toggle-line:nth-child(1) {
                transform: translateY(7px) rotate(45deg);
            }
            
            .admin-navbar-toggle.active .toggle-line:nth-child(2) {
                opacity: 0;
            }
            
            .admin-navbar-toggle.active .toggle-line:nth-child(3) {
                transform: translateY(-7px) rotate(-45deg);
            }
            
            .admin-cards {
                grid-template-columns: 1fr;
            }
            
            .admin-form-row {
                grid-template-columns: 1fr;
            }
            
            .admin-table-header {
                flex-direction: column;
                align-items: flex-start;
                gap: var(--spacing-md);
            }
            
            .admin-table-actions {
                width: 100%;
                flex-wrap: wrap;
            }
            
            .admin-form-actions {
                flex-wrap: wrap;
            }
            
            .admin-form-actions .admin-form-btn {
                flex: 1;
                min-width: 120px;
            }
        }
        
        @media (max-width: 480px) {
            .admin-navbar-logo .logo-text {
                display: none;
            }
            
            .admin-user-name {
                display: none;
            }
            
            .admin-page-title {
                font-size: 1.5rem;
            }
            
            .admin-form-btn {
                padding: var(--spacing-sm) var(--spacing-md);
                font-size: 0.75rem;
            }
        }
    </style>
    
    <!-- GSAP ScrollTrigger -->
    <script src="https://cdnjs.cloudflare.com/ajax/libs/gsap/3.12.2/ScrollTrigger.min.js" defer></script>
    
    <!-- CSRF Token -->
    <?php echo csrf_meta_tag(); ?>
</head>
<body>
    <!-- Admin Navbar -->
    <nav class="admin-navbar" role="navigation" aria-label="Nawigacja admina">
        <div class="admin-navbar-container">
            <button class="admin-navbar-toggle" aria-label="Przełącz menu" aria-expanded="false">
                <span class="toggle-line"></span>
                <span class="toggle-line"></span>
                <span class="toggle-line"></span>
            </button>
            
            <a href="<?php echo SITE_URL; ?>/admin" class="admin-navbar-logo">
                <span class="logo-icon">🔨</span>
                <span class="logo-text">PANEL ADMINA</span>
            </a>
            
            <ul class="admin-navbar-menu">
                <li>
                    <a href="<?php echo SITE_URL; ?>/admin" class="admin-navbar-link <?php echo $adminPage === 'dashboard' ? 'active' : ''; ?>">
                        <span class="nav-icon">📊</span>
                        <span>Dashboard</span>
                    </a>
                </li>
                <li>
                    <a href="<?php echo SITE_URL; ?>/admin/projekty" class="admin-navbar-link <?php echo $adminPage === 'projekty' ? 'active' : ''; ?>">
                        <span class="nav-icon">💻</span>
                        <span>Projekty</span>
                    </a>
                </li>
                <li>
                    <a href="<?php echo SITE_URL; ?>/admin/galeria" class="admin-navbar-link <?php echo $adminPage === 'galeria' ? 'active' : ''; ?>">
                        <span class="nav-icon">📷</span>
                        <span>Galeria</span>
                    </a>
                </li>
                <li>
                    <a href="<?php echo SITE_URL; ?>/admin/o-mnie" class="admin-navbar-link <?php echo $adminPage === 'o-mnie' ? 'active' : ''; ?>">
                        <span class="nav-icon">👤</span>
                        <span>O mnie</span>
                    </a>
                </li>
                <li>
                    <a href="<?php echo SITE_URL; ?>/admin/wiadomosci" class="admin-navbar-link <?php echo $adminPage === 'wiadomosci' ? 'active' : ''; ?>">
                        <span class="nav-icon">✉️</span>
                        <span>Wiadomości</span>
                    </a>
                </li>
                <li>
                    <a href="<?php echo SITE_URL; ?>/admin/ustawienia" class="admin-navbar-link <?php echo $adminPage === 'ustawienia' ? 'active' : ''; ?>">
                        <span class="nav-icon">⚙️</span>
                        <span>Ustawienia</span>
                    </a>
                </li>
            </ul>
            
            <div class="admin-navbar-actions">
                <div class="admin-user-menu">
                    <button class="admin-user-btn" id="user-menu-btn" aria-label="Menu użytkownika">
                        <div class="admin-user-avatar">
                            <?php echo substr($adminUser['username'], 0, 1); ?>
                        </div>
                        <span class="admin-user-name">
                            <?php echo escape_output($adminUser['username']); ?>
                        </span>
                    </button>
                    
                    <div class="admin-user-dropdown" id="user-menu-dropdown">
                        <ul>
                            <li>
                                <a href="<?php echo SITE_URL; ?>/admin/uzytkownicy">
                                    <span class="dropdown-icon">👥</span>
                                    <span>Użytkownicy</span>
                                </a>
                            </li>
                            <li>
                                <a href="<?php echo SITE_URL; ?>">
                                    <span class="dropdown-icon">🏠</span>
                                    <span>Strona główna</span>
                                </a>
                            </li>
                            <li>
                                <a href="<?php echo SITE_URL; ?>/admin?action=logout">
                                    <span class="dropdown-icon">🚪</span>
                                    <span>Wyloguj się</span>
                                </a>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    </nav>
    
    <div class="admin-layout">
        <!-- Admin Sidebar -->
        <aside class="admin-sidebar">
            <div class="admin-sidebar-header">
                <div class="admin-sidebar-title">MENU</div>
            </div>
            
            <nav class="admin-sidebar-nav">
                <ul>
                    <li>
                        <a href="<?php echo SITE_URL; ?>/admin" class="admin-sidebar-link <?php echo $adminPage === 'dashboard' ? 'active' : ''; ?>">
                            <span class="nav-icon">📊</span>
                            <span class="nav-text">Dashboard</span>
                        </a>
                    </li>
                    <li>
                        <a href="<?php echo SITE_URL; ?>/admin/projekty" class="admin-sidebar-link <?php echo $adminPage === 'projekty' ? 'active' : ''; ?>">
                            <span class="nav-icon">💻</span>
                            <span class="nav-text">Projekty</span>
                        </a>
                    </li>
                    <li>
                        <a href="<?php echo SITE_URL; ?>/admin/galeria" class="admin-sidebar-link <?php echo $adminPage === 'galeria' ? 'active' : ''; ?>">
                            <span class="nav-icon">📷</span>
                            <span class="nav-text">Galeria</span>
                        </a>
                    </li>
                    <li>
                        <a href="<?php echo SITE_URL; ?>/admin/o-mnie" class="admin-sidebar-link <?php echo $adminPage === 'o-mnie' ? 'active' : ''; ?>">
                            <span class="nav-icon">👤</span>
                            <span class="nav-text">O mnie</span>
                        </a>
                    </li>
                    <li>
                        <a href="<?php echo SITE_URL; ?>/admin/wiadomosci" class="admin-sidebar-link <?php echo $adminPage === 'wiadomosci' ? 'active' : ''; ?>">
                            <span class="nav-icon">✉️</span>
                            <span class="nav-text">Wiadomości</span>
                        </a>
                    </li>
                    <li>
                        <a href="<?php echo SITE_URL; ?>/admin/ustawienia" class="admin-sidebar-link <?php echo $adminPage === 'ustawienia' ? 'active' : ''; ?>">
                            <span class="nav-icon">⚙️</span>
                            <span class="nav-text">Ustawienia</span>
                        </a>
                    </li>
                </ul>
            </nav>
            
            <div class="admin-sidebar-footer">
                <div style="font-size: 0.75rem; color: rgba(255, 255, 255, 0.5);">
                    v1.0.0
                </div>
            </div>
        </aside>
        
        <!-- Admin Main Content -->
        <main class="admin-main">
