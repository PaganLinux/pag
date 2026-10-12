/**
 * Portfolio - Przełączanie trybu ciemny/jasny
 * Obsługuje lokalne przechowywanie preferencji użytkownika
 */

(function() {
    'use strict';

    // ============================================
    // USTAWIENIA
    // ============================================
    const STORAGE_KEY = 'pag-portfolio-theme';
    const DARK_CLASS = 'theme-dark';
    const LIGHT_CLASS = 'theme-light';
    const THEME_CSS_ID = 'theme-css';

    // ============================================
    // FUNKCJE POMOCNICZE
    // ============================================

    /**
     * Pobiera aktualny tryb z localStorage lub domyślny
     * @returns {string} 'dark' lub 'light'
     */
    function getCurrentTheme() {
        const saved = localStorage.getItem(STORAGE_KEY);
        if (saved) {
            return saved;
        }
        
        // Sprawdź preferencje systemowe
        if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
            return 'dark';
        }
        
        return 'light';
    }

    /**
     * Zapisuje tryb do localStorage
     * @param {string} theme - 'dark' lub 'light'
     */
    function saveTheme(theme) {
        localStorage.setItem(STORAGE_KEY, theme);
    }

    /**
     * Zmienia arkusz CSS
     * @param {string} theme - 'dark' lub 'light'
     */
    function changeThemeCSS(theme) {
        const themeLink = document.getElementById(THEME_CSS_ID);
        if (!themeLink) return;
        
        // Zmień href na odpowiedni arkusz
        themeLink.href = `/assets/css/${theme}-mode.css`;
    }

    /**
     * Zmienia klasę body
     * @param {string} theme - 'dark' lub 'light'
     */
    function changeBodyClass(theme) {
        const body = document.body;
        
        if (theme === 'dark') {
            body.classList.remove(LIGHT_CLASS);
            body.classList.add(DARK_CLASS);
        } else {
            body.classList.remove(DARK_CLASS);
            body.classList.add(LIGHT_CLASS);
        }
    }

    /**
     * Aktualizuje ikonę przycisku
     * @param {string} theme - 'dark' lub 'light'
     */
    function updateButtonIcon(theme) {
        const themeToggle = document.querySelector('.theme-toggle');
        if (!themeToggle) return;
        
        const lightIcon = themeToggle.querySelector('.theme-icon.light');
        const darkIcon = themeToggle.querySelector('.theme-icon.dark');
        
        if (!lightIcon || !darkIcon) return;
        
        if (theme === 'dark') {
            lightIcon.classList.remove('active');
            darkIcon.classList.add('active');
        } else {
            lightIcon.classList.add('active');
            darkIcon.classList.remove('active');
        }
    }

    /**
     * Zmienia tryb
     * @param {string} theme - 'dark' lub 'light'
     */
    function setTheme(theme) {
        const currentTheme = getCurrentTheme();
        
        if (currentTheme === theme) return;
        
        // Zmień CSS
        changeThemeCSS(theme);
        
        // Zmień klasę body
        changeBodyClass(theme);
        
        // Zmień ikonę
        updateButtonIcon(theme);
        
        // Zapisz do localStorage
        saveTheme(theme);
        
        // Wyzwól zdarzenie
        window.dispatchEvent(new CustomEvent('themeChange', { detail: { theme } }));
    }

    /**
     * Przełącza tryb
     */
    function toggleTheme() {
        const currentTheme = getCurrentTheme();
        const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
        setTheme(newTheme);
    }

    /**
     * Inicjalizuje tryb na podstawie zapisanego
     */
    function initTheme() {
        const theme = getCurrentTheme();
        
        // Ustaw klasę body od razu (zanim CSS się załaduje)
        changeBodyClass(theme);
        
        // Zmień CSS
        changeThemeCSS(theme);
        
        // Ustaw ikonę
        updateButtonIcon(theme);
        
        // Dodaj listener do przycisku
        const themeToggle = document.querySelector('.theme-toggle');
        if (themeToggle) {
            themeToggle.addEventListener('click', toggleTheme);
        }
        
        // Dodaj listener do systemowych preferencji
        if (window.matchMedia) {
            window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
                if (!localStorage.getItem(STORAGE_KEY)) {
                    setTheme(e.matches ? 'dark' : 'light');
                }
            });
        }
    }

    // ============================================
    // INICJALIZACJA
    // ============================================
    
    // Poczekaj na załadowanie DOM
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', initTheme);
    } else {
        initTheme();
    }

    // Eksportuj funkcje (jeśli używamy modułów)
    if (typeof module !== 'undefined' && module.exports) {
        module.exports = { setTheme, toggleTheme, getCurrentTheme };
    }

    // Eksportuj do window (dla globalnego dostępu)
    window.PortfolioTheme = {
        setTheme,
        toggleTheme,
        getCurrentTheme
    };

})();
