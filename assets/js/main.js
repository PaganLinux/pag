/**
 * Portfolio - Główny skrypt
 * Obsługuje podstawowe funkcjonalności strony
 */

(function() {
    'use strict';

    // ============================================
    // USTAWIENIA
    // ============================================
    const SITE_URL = window.location.origin;
    const CSRF_TOKEN = document.querySelector('meta[name="csrf-token"]')?.content || '';

    // ============================================
    // FUNKCJE POMOCNICZE
    // ============================================

    /**
     * Wysyła żądanie AJAX
     * @param {string} url - Adres URL
     * @param {string} method - Metoda (GET, POST, itd.)
     * @param {Object} data - Dane do wysłania
     * @param {Function} callback - Funkcja zwrotna
     */
    function sendAjaxRequest(url, method = 'GET', data = null, callback = null) {
        const options = {
            method: method,
            headers: {
                'Content-Type': 'application/json',
                'X-Requested-With': 'XMLHttpRequest',
                'X-CSRF-Token': CSRF_TOKEN
            }
        };
        
        if (data) {
            options.body = JSON.stringify(data);
        }
        
        return fetch(url, options)
            .then(response => {
                if (!response.ok) {
                    throw new Error(`HTTP error! status: ${response.status}`);
                }
                return response.json();
            })
            .then(data => {
                if (callback) callback(null, data);
                return data;
            })
            .catch(error => {
                if (callback) callback(error, null);
                console.error('Błąd AJAX:', error);
                throw error;
            });
    }

    /**
     * Formatuje datę na polski format
     * @param {string} dateString - Data w formacie ISO
     * @returns {string} Sformatowana data
     */
    function formatPolishDate(dateString) {
        const months = [
            'stycznia', 'lutego', 'marca', 'kwietnia', 'maja', 'czerwca',
            'lipca', 'sierpnia', 'września', 'października', 'listopada', 'grudnia'
        ];
        
        const date = new Date(dateString);
        const day = date.getDate();
        const month = date.getMonth();
        const year = date.getFullYear();
        
        return `${day} ${months[month]} ${year}`;
    }

    /**
     * Skraca tekst
     * @param {string} text - Tekst do skrócenia
     * @param {number} length - Maksymalna długość
     * @returns {string} Skrócony tekst
     */
    function truncateText(text, length = 100) {
        if (text.length <= length) return text;
        
        let truncated = text.substr(0, length);
        const lastSpace = truncated.lastIndexOf(' ');
        
        if (lastSpace > 0) {
            truncated = truncated.substr(0, lastSpace);
        }
        
        return truncated + '...';
    }

    /**
     * Escapuje HTML
     * @param {string} text - Tekst do escapowania
     * @returns {string} Escapowany tekst
     */
    function escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }

    // ============================================
    // INICJALIZACJA STRONY
    // ============================================

    /**
     * Inicjalizuje preloader
     */
    function initPreloader() {
        const preloader = document.getElementById('preloader');
        
        if (!preloader) return;
        
        // Ukryj preloader po załadowaniu strony
        window.addEventListener('load', () => {
            setTimeout(() => {
                preloader.classList.add('hidden');
                document.body.style.overflow = 'auto';
            }, 500);
        });
        
        // Blokuj scroll podczas ładowania
        document.body.style.overflow = 'hidden';
    }

    /**
     * Inicjalizuje smooth scroll
     */
    function initSmoothScroll() {
        // Używamy lenis do smooth scroll
        if (typeof Lenis !== 'undefined') {
            const lenis = new Lenis({
                lerp: 0.1,
                smoothWheel: true,
                smoothTouch: true
            });
            
            // Synchronizuj z GSAP ScrollTrigger
            if (typeof gsap !== 'undefined') {
                lenis.on('scroll', ScrollTrigger.update);
                gsap.ticker.add((time) => {
                    lenis.raf(time * 1000);
                });
                gsap.ticker.lagSmoothing(0);
            }
            
            return lenis;
        }
        
        return null;
    }

    /**
     * Inicjalizuje animacje GSAP
     */
    function initGSAPAnimations() {
        if (typeof gsap === 'undefined') return;
        
        // Rejestruj ScrollTrigger plugin
        if (typeof ScrollTrigger !== 'undefined') {
            gsap.registerPlugin(ScrollTrigger);
        }
        
        // Animacja dla sekcji
        gsap.utils.toArray('.section-header').forEach((section, index) => {
            gsap.from(section, {
                scrollTrigger: {
                    trigger: section,
                    start: 'top 80%',
                    toggleActions: 'play none none none'
                },
                opacity: 0,
                y: 50,
                duration: 0.8,
                delay: index * 0.1
            });
        });
        
        // Animacja dla kart projektów
        gsap.utils.toArray('.project-card').forEach((card, index) => {
            gsap.from(card, {
                scrollTrigger: {
                    trigger: card,
                    start: 'top 85%',
                    toggleActions: 'play none none none'
                },
                opacity: 0,
                y: 30,
                duration: 0.6,
                delay: index * 0.05
            });
        });
        
        // Animacja dla polaroidów
        gsap.utils.toArray('.polaroid').forEach((polaroid, index) => {
            gsap.from(polaroid, {
                scrollTrigger: {
                    trigger: polaroid,
                    start: 'top 85%',
                    toggleActions: 'play none none none'
                },
                opacity: 0,
                y: 30,
                rotation: gsap.utils.random(-10, 10),
                duration: 0.6,
                delay: index * 0.03
            });
        });
        
        // Animacja dla statystyk
        gsap.utils.toArray('.stat-item').forEach((stat, index) => {
            gsap.from(stat, {
                scrollTrigger: {
                    trigger: stat,
                    start: 'top 80%',
                    toggleActions: 'play none none none'
                },
                opacity: 0,
                y: 20,
                duration: 0.6,
                delay: index * 0.1
            });
        });
        
        // Paralaxa dla hero
        if (document.querySelector('.hero-background')) {
            gsap.to('.hero-background', {
                scrollTrigger: {
                    trigger: '.hero-section',
                    start: 'top top',
                    end: 'bottom top',
                    scrub: 1
                },
                y: 100,
                ease: 'none'
            });
        }
    }

    /**
     * Inicjalizuje nawigację
     */
    function initNavigation() {
        const navbar = document.querySelector('.navbar');
        const toggle = document.querySelector('.navbar-toggle');
        const menu = document.querySelector('.navbar-menu');
        
        if (!navbar || !toggle || !menu) return;
        
        // Przełącz menu mobilne
        toggle.addEventListener('click', () => {
            const isExpanded = toggle.getAttribute('aria-expanded') === 'true';
            toggle.setAttribute('aria-expanded', !isExpanded);
            menu.classList.toggle('active');
        });
        
        // Zamknij menu po kliknięciu w link
        menu.querySelectorAll('.navbar-link').forEach(link => {
            link.addEventListener('click', () => {
                toggle.setAttribute('aria-expanded', 'false');
                menu.classList.remove('active');
            });
        });
        
        // Dodaj klasę 'scrolled' do navbar po przewinięciu
        window.addEventListener('scroll', () => {
            if (window.scrollY > 50) {
                navbar.classList.add('scrolled');
            } else {
                navbar.classList.remove('scrolled');
            }
        });
    }

    /**
     * Inicjalizuje typowanie tekstu w hero
     */
    function initTypingEffect() {
        const typingText = document.querySelector('.typing-text');
        const cursorBlink = document.querySelector('.cursor-blink');
        
        if (!typingText) return;
        
        const text = typingText.dataset.text || typingText.textContent;
        const speed = 100; // ms na znak
        let i = 0;
        
        typingText.textContent = '';
        
        function typeWriter() {
            if (i < text.length) {
                typingText.textContent += text.charAt(i);
                i++;
                setTimeout(typeWriter, speed);
            } else {
                // Uruchom miganie kursora
                if (cursorBlink) {
                    cursorBlink.style.display = 'inline-block';
                }
            }
        }
        
        // Rozpocznij typowanie po załadowaniu strony
        window.addEventListener('load', () => {
            setTimeout(typeWriter, 1000);
        });
    }

    /**
     * Inicjalizuje filtry
     */
    function initFilters() {
        // Filtry projektów
        const projectFilters = document.querySelectorAll('.filter-btn[data-type="category"]');
        
        projectFilters.forEach(btn => {
            btn.addEventListener('click', () => {
                const filter = btn.dataset.filter;
                
                // Usuń aktywną klasę z innych przycisków
                projectFilters.forEach(b => b.classList.remove('active'));
                btn.classList.add('active');
                
                // Przekieruj lub załaduj projekty
                if (filter === 'all') {
                    window.location.href = `${SITE_URL}/projekty`;
                } else {
                    window.location.href = `${SITE_URL}/projekty?kategoria=${filter}`;
                }
            });
        });
        
        // Filtry galerii
        const galleryFilters = document.querySelectorAll('.filter-btn[data-type="album"]');
        
        galleryFilters.forEach(btn => {
            btn.addEventListener('click', () => {
                const filter = btn.dataset.filter;
                
                // Usuń aktywną klasę z innych przycisków
                galleryFilters.forEach(b => b.classList.remove('active'));
                btn.classList.add('active');
                
                // Przekieruj lub załaduj galerię
                if (filter === 'all') {
                    window.location.href = `${SITE_URL}/galeria`;
                } else {
                    window.location.href = `${SITE_URL}/galeria?album=${filter}`;
                }
            });
        });
    }

    /**
     * Inicjalizuje lightbox
     */
    function initLightbox() {
        const lightbox = document.getElementById('lightbox');
        const lightboxImage = document.getElementById('lightbox-image');
        const lightboxTitle = document.getElementById('lightbox-title');
        const lightboxDescription = document.getElementById('lightbox-description');
        const lightboxCounter = document.getElementById('lightbox-counter');
        const lightboxClose = document.querySelector('.lightbox-close');
        const lightboxPrev = document.querySelector('.lightbox-prev');
        const lightboxNext = document.querySelector('.lightbox-next');
        const triggers = document.querySelectorAll('.lightbox-trigger');
        
        if (!lightbox || !lightboxImage) return;
        
        let currentIndex = 0;
        let images = [];
        
        // Zbierz wszystkie obrazy
        function collectImages() {
            const galleryImages = document.querySelectorAll('.polaroid-image');
            images = [];
            
            galleryImages.forEach((img, index) => {
                images.push({
                    src: img.dataset.fullSrc || img.src,
                    alt: img.alt,
                    title: img.closest('.polaroid')?.querySelector('.polaroid-title')?.textContent || '',
                    description: img.closest('.polaroid')?.querySelector('.polaroid-description')?.textContent || ''
                });
            });
        }
        
        // Otwórz lightbox
        function openLightbox(index) {
            if (index >= images.length) return;
            
            currentIndex = index;
            updateLightboxContent();
            lightbox.classList.add('active');
            lightbox.setAttribute('aria-hidden', 'false');
            document.body.style.overflow = 'hidden';
        }
        
        // Zamknij lightbox
        function closeLightbox() {
            lightbox.classList.remove('active');
            lightbox.setAttribute('aria-hidden', 'true');
            document.body.style.overflow = '';
        }
        
        // Aktualizuj treść lightbox
        function updateLightboxContent() {
            const image = images[currentIndex];
            
            lightboxImage.src = image.src;
            lightboxImage.alt = image.alt;
            
            if (lightboxTitle) {
                lightboxTitle.textContent = image.title;
            }
            
            if (lightboxDescription) {
                lightboxDescription.textContent = image.description;
            }
            
            if (lightboxCounter) {
                lightboxCounter.textContent = `${currentIndex + 1} / ${images.length}`;
            }
        }
        
        // Następne zdjęcie
        function nextImage() {
            currentIndex = (currentIndex + 1) % images.length;
            updateLightboxContent();
        }
        
        // Poprzednie zdjęcie
        function prevImage() {
            currentIndex = (currentIndex - 1 + images.length) % images.length;
            updateLightboxContent();
        }
        
        // Inicjalizuj
        collectImages();
        
        // Dodaj event listeners
        triggers.forEach((trigger, index) => {
            trigger.addEventListener('click', (e) => {
                e.preventDefault();
                const imageIndex = parseInt(trigger.dataset.imageId) || index;
                
                // Znajdź indeks obrazu w tablicy
                const polaroid = trigger.closest('.polaroid');
                if (polaroid) {
                    const polaroidIndex = Array.from(document.querySelectorAll('.polaroid')).indexOf(polaroid);
                    openLightbox(polaroidIndex);
                } else {
                    openLightbox(imageIndex);
                }
            });
        });
        
        if (lightboxClose) {
            lightboxClose.addEventListener('click', closeLightbox);
        }
        
        if (lightboxPrev) {
            lightboxPrev.addEventListener('click', prevImage);
        }
        
        if (lightboxNext) {
            lightboxNext.addEventListener('click', nextImage);
        }
        
        // Zamknij na klawisz Escape
        document.addEventListener('keydown', (e) => {
            if (e.key === 'Escape' && lightbox.classList.contains('active')) {
                closeLightbox();
            }
            
            if (e.key === 'ArrowRight' && lightbox.classList.contains('active')) {
                nextImage();
            }
            
            if (e.key === 'ArrowLeft' && lightbox.classList.contains('active')) {
                prevImage();
            }
        });
        
        // Zamknij na kliknięcie w overlay
        lightbox.addEventListener('click', (e) => {
            if (e.target === lightbox) {
                closeLightbox();
            }
        });
        
        // Obsługa gestów dotykowych
        let touchStartX = 0;
        let touchEndX = 0;
        
        lightbox.addEventListener('touchstart', (e) => {
            touchStartX = e.changedTouches[0].screenX;
        }, { passive: true });
        
        lightbox.addEventListener('touchend', (e) => {
            touchEndX = e.changedTouches[0].screenX;
            handleSwipe();
        }, { passive: true });
        
        function handleSwipe() {
            if (Math.abs(touchEndX - touchStartX) < 50) return;
            
            if (touchEndX < touchStartX) {
                nextImage();
            } else {
                prevImage();
            }
        }
    }

    /**
     * Inicjalizuje formularz kontaktowy
     */
    function initContactForm() {
        const form = document.getElementById('contact-form');
        const submitBtn = document.getElementById('form-submit');
        
        if (!form) return;
        
        // Walidacja w czasie rzeczywistym
        const inputs = form.querySelectorAll('.form-input, .form-textarea');
        
        inputs.forEach(input => {
            input.addEventListener('blur', () => {
                validateField(input);
            });
            
            input.addEventListener('input', () => {
                const formGroup = input.closest('.form-group');
                if (formGroup) {
                    formGroup.classList.remove('error');
                }
            });
        });
        
        // Walidacja pojedynczego pola
        function validateField(field) {
            const formGroup = field.closest('.form-group');
            const value = field.value.trim();
            let isValid = true;
            let errorMessage = '';
            
            if (field.required && !value) {
                isValid = false;
                errorMessage = 'To pole jest wymagane';
            }
            
            if (field.type === 'email' && value && !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value)) {
                isValid = false;
                errorMessage = 'Wprowadź poprawny adres email';
            }
            
            if (field.id === 'message' && value && value.length < 10) {
                isValid = false;
                errorMessage = 'Wiadomość musi mieć co najmniej 10 znaków';
            }
            
            // Usuń lub dodaj klasę error
            if (formGroup) {
                if (isValid) {
                    formGroup.classList.remove('error');
                    const error = formGroup.querySelector('.form-error');
                    if (error) error.remove();
                } else {
                    formGroup.classList.add('error');
                    let error = formGroup.querySelector('.form-error');
                    if (!error) {
                        error = document.createElement('span');
                        error.className = 'form-error';
                        formGroup.appendChild(error);
                    }
                    error.textContent = errorMessage;
                }
            }
            
            return isValid;
        }
        
        // Walidacja całego formularza
        function validateForm() {
            let isValid = true;
            
            inputs.forEach(input => {
                if (!validateField(input)) {
                    isValid = false;
                }
            });
            
            return isValid;
        }
        
        // Obsługa przesłania formularza
        form.addEventListener('submit', async (e) => {
            e.preventDefault();
            
            if (!validateForm()) {
                return;
            }
            
            // Pokazuj loading
            if (submitBtn) {
                submitBtn.classList.add('btn-loading');
                submitBtn.disabled = true;
            }
            
            // Zbierz dane
            const formData = new FormData(form);
            const data = {};
            
            for (let [key, value] of formData.entries()) {
                data[key] = value;
            }
            
            try {
                // Wyślij przez AJAX
                const response = await sendAjaxRequest(
                    `${SITE_URL}/send-message`,
                    'POST',
                    data
                );
                
                if (response.success) {
                    // Pokazuj komunikat sukcesu
                    form.innerHTML = `
                        <div class="form-success">
                            <div class="success-icon">✅</div>
                            <h3 class="success-title">Dziękuję za wiadomość!</h3>
                            <p class="success-text">
                                Twoja wiadomość została wysłana pomyślnie. 
                                Odpowiem jak najszybciej.
                            </p>
                            <a href="${SITE_URL}" class="btn btn-primary cursor-hand">
                                WRÓĆ DO STRONY GŁÓWNEJ
                            </a>
                        </div>
                    `;
                } else {
                    // Pokazuj błędy
                    const errorContainer = document.createElement('div');
                    errorContainer.className = 'form-general-errors';
                    
                    if (response.errors) {
                        response.errors.forEach(error => {
                            const errorEl = document.createElement('span');
                            errorEl.className = 'form-error';
                            errorEl.textContent = error;
                            errorContainer.appendChild(errorEl);
                        });
                    } else if (response.error) {
                        const errorEl = document.createElement('span');
                        errorEl.className = 'form-error';
                        errorEl.textContent = response.error;
                        errorContainer.appendChild(errorEl);
                    }
                    
                    form.insertBefore(errorContainer, form.firstChild);
                }
            } catch (error) {
                console.error('Błąd:', error);
                
                const errorContainer = document.createElement('div');
                errorContainer.className = 'form-general-errors';
                
                const errorEl = document.createElement('span');
                errorEl.className = 'form-error';
                errorEl.textContent = 'Wystąpił błąd podczas wysyłania wiadomości. Spróbuj ponownie.';
                errorContainer.appendChild(errorEl);
                
                form.insertBefore(errorContainer, form.firstChild);
            } finally {
                // Ukryj loading
                if (submitBtn) {
                    submitBtn.classList.remove('btn-loading');
                    submitBtn.disabled = false;
                }
            }
        });
    }

    // ============================================
    // INICJALIZACJA
    // ============================================
    
    // Poczekaj na załadowanie DOM
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', () => {
            initPreloader();
            initSmoothScroll();
            initGSAPAnimations();
            initNavigation();
            initTypingEffect();
            initFilters();
            initLightbox();
            initContactForm();
        });
    } else {
        initPreloader();
        initSmoothScroll();
        initGSAPAnimations();
        initNavigation();
        initTypingEffect();
        initFilters();
        initLightbox();
        initContactForm();
    }

    // Eksportuj do window (dla globalnego dostępu)
    window.PortfolioMain = {
        sendAjaxRequest,
        formatPolishDate,
        truncateText,
        escapeHtml
    };

})();
