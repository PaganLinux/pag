/**
 * Portfolio - Walidacja i obsługa formularzy
 * Obsługuje walidację formularzy i ochronę przed spamem
 */

(function() {
    'use strict';

    // ============================================
    // USTAWIENIA
    // ============================================
    const CSRF_TOKEN = document.querySelector('meta[name="csrf-token"]')?.content || '';

    // ============================================
    // FUNKCJE POMOCNICZE
    // ============================================

    /**
     * Sprawdza czy pole jest puste
     * @param {string} value - Wartość pola
     * @returns {boolean}
     */
    function isEmpty(value) {
        if (value === null || value === undefined) return true;
        if (typeof value === 'string') return value.trim() === '';
        return false;
    }

    /**
     * Sprawdza czy email jest poprawny
     * @param {string} email - Adres email
     * @returns {boolean}
     */
    function isValidEmail(email) {
        const regex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        return regex.test(email);
    }

    /**
     * Sprawdza minimalną długość
     * @param {string} value - Wartość
     * @param {number} min - Minimalna długość
     * @returns {boolean}
     */
    function hasMinLength(value, min) {
        if (isEmpty(value)) return false;
        return value.trim().length >= min;
    }

    /**
     * Sprawdza maksymalną długość
     * @param {string} value - Wartość
     * @param {number} max - Maksymalna długość
     * @returns {boolean}
     */
    function hasMaxLength(value, max) {
        if (isEmpty(value)) return true;
        return value.trim().length <= max;
    }

    /**
     * Waliduje pole formularza
     * @param {HTMLElement} field - Pole formularza
     * @returns {Object} { isValid: boolean, error: string|null }
     */
    function validateField(field) {
        const value = field.value.trim();
        const type = field.type;
        const name = field.name || field.id;
        let isValid = true;
        let error = null;
        
        // Sprawdź wymagane pole
        if (field.required && isEmpty(value)) {
            isValid = false;
            error = 'To pole jest wymagane';
        }
        
        // Sprawdź typ email
        if (type === 'email' && !isEmpty(value) && !isValidEmail(value)) {
            isValid = false;
            error = 'Wprowadź poprawny adres email';
        }
        
        // Sprawdź minimalną długość
        if (field.dataset.minLength && !hasMinLength(value, parseInt(field.dataset.minLength))) {
            isValid = false;
            error = `To pole musi mieć co najmniej ${field.dataset.minLength} znaków`;
        }
        
        // Sprawdź maksymalną długość
        if (field.dataset.maxLength && !hasMaxLength(value, parseInt(field.dataset.maxLength))) {
            isValid = false;
            error = `To pole może mieć maksymalnie ${field.dataset.maxLength} znaków`;
        }
        
        // Sprawdź pattern
        if (field.pattern && !isEmpty(value)) {
            const regex = new RegExp(field.pattern);
            if (!regex.test(value)) {
                isValid = false;
                error = field.dataset.patternError || 'Nieprawidłowy format';
            }
        }
        
        // Sprawdź typ number
        if (type === 'number' && !isEmpty(value)) {
            if (isNaN(parseFloat(value))) {
                isValid = false;
                error = 'Wprowadź poprawną liczbę';
            }
            
            if (field.min && parseFloat(value) < parseFloat(field.min)) {
                isValid = false;
                error = `Wartość musi być większa lub równa ${field.min}`;
            }
            
            if (field.max && parseFloat(value) > parseFloat(field.max)) {
                isValid = false;
                error = `Wartość musi być mniejsza lub równa ${field.max}`;
            }
        }
        
        return { isValid, error };
    }

    /**
     * Wyświetla błąd dla pola
     * @param {HTMLElement} field - Pole formularza
     * @param {string} error - Komunikat błędu
     */
    function showFieldError(field, error) {
        const formGroup = field.closest('.form-group');
        
        if (!formGroup) {
            console.error('Nie znaleziono form-group dla pola:', field);
            return;
        }
        
        formGroup.classList.add('error');
        
        let errorElement = formGroup.querySelector('.form-error');
        
        if (!errorElement) {
            errorElement = document.createElement('span');
            errorElement.className = 'form-error';
            formGroup.appendChild(errorElement);
        }
        
        errorElement.textContent = error;
    }

    /**
     * Usuwa błąd dla pola
     * @param {HTMLElement} field - Pole formularza
     */
    function clearFieldError(field) {
        const formGroup = field.closest('.form-group');
        
        if (!formGroup) return;
        
        formGroup.classList.remove('error');
        
        const errorElement = formGroup.querySelector('.form-error');
        if (errorElement) {
            errorElement.remove();
        }
    }

    /**
     * Waliduje cały formularz
     * @param {HTMLFormElement} form - Formularz
     * @returns {boolean}
     */
    function validateForm(form) {
        let isValid = true;
        const inputs = form.querySelectorAll('.form-input, .form-textarea, input, textarea, select');
        
        inputs.forEach(field => {
            // Pomiń ukryte pola i przyciski
            if (field.type === 'hidden' || field.type === 'submit' || field.type === 'button') {
                return;
            }
            
            const result = validateField(field);
            
            if (!result.isValid) {
                isValid = false;
                showFieldError(field, result.error);
            } else {
                clearFieldError(field);
            }
        });
        
        return isValid;
    }

    /**
     * Sprawdza honeypot
     * @param {HTMLFormElement} form - Formularz
     * @returns {boolean}
     */
    function checkHoneypot(form) {
        const honeypot = form.querySelector('[name="honeypot"]');
        
        if (!honeypot) return true;
        
        return isEmpty(honeypot.value);
    }

    /**
     * Wysyła formularz przez AJAX
     * @param {HTMLFormElement} form - Formularz
     * @param {string} url - Adres URL
     * @param {Function} callback - Funkcja zwrotna
     */
    function submitFormAjax(form, url, callback) {
        if (!validateForm(form)) {
            if (callback) callback({ success: false, error: 'Formularz zawiera błędy' });
            return;
        }
        
        if (!checkHoneypot(form)) {
            if (callback) callback({ success: false, error: 'Spam detected' });
            return;
        }
        
        const formData = new FormData(form);
        const data = {};
        
        // Dodaj token CSRF
        data[CSRF_TOKEN] = CSRF_TOKEN;
        
        // Dodaj dane z formularza
        for (let [key, value] of formData.entries()) {
            // Pomiń honeypot
            if (key === 'honeypot') continue;
            data[key] = value;
        }
        
        const options = {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'X-Requested-With': 'XMLHttpRequest',
                'X-CSRF-Token': CSRF_TOKEN
            },
            body: JSON.stringify(data)
        };
        
        fetch(url, options)
            .then(response => {
                if (!response.ok) {
                    throw new Error(`HTTP error! status: ${response.status}`);
                }
                return response.json();
            })
            .then(data => {
                if (callback) callback(data);
            })
            .catch(error => {
                console.error('Błąd:', error);
                if (callback) callback({ success: false, error: 'Wystąpił błąd. Spróbuj ponownie.' });
            });
    }

    // ============================================
    // INICJALIZACJA FORMULARZY
    // ============================================

    /**
     * Inicjalizuje walidację w czasie rzeczywistym
     * @param {HTMLFormElement} form - Formularz
     */
    function initRealtimeValidation(form) {
        const inputs = form.querySelectorAll('.form-input, .form-textarea, input, textarea, select');
        
        inputs.forEach(field => {
            // Pomiń ukryte pola i przyciski
            if (field.type === 'hidden' || field.type === 'submit' || field.type === 'button') {
                return;
            }
            
            // Walidacja na blur
            field.addEventListener('blur', () => {
                const result = validateField(field);
                
                if (!result.isValid) {
                    showFieldError(field, result.error);
                } else {
                    clearFieldError(field);
                }
            });
            
            // Usuń błąd na input
            field.addEventListener('input', () => {
                clearFieldError(field);
            });
        });
    }

    /**
     * Inicjalizuje formularz kontaktowy
     */
    function initContactForm() {
        const form = document.getElementById('contact-form');
        
        if (!form) return;
        
        // Inicjalizuj walidację w czasie rzeczywistym
        initRealtimeValidation(form);
        
        // Obsługa przesłania
        form.addEventListener('submit', (e) => {
            e.preventDefault();
            
            const submitBtn = form.querySelector('[type="submit"]');
            
            // Pokazuj loading
            if (submitBtn) {
                submitBtn.classList.add('btn-loading');
                submitBtn.disabled = true;
            }
            
            // Wyślij formularz
            submitFormAjax(form, form.action || window.location.href, (response) => {
                // Ukryj loading
                if (submitBtn) {
                    submitBtn.classList.remove('btn-loading');
                    submitBtn.disabled = false;
                }
                
                if (response && response.success) {
                    // Pokazuj komunikat sukcesu
                    showSuccessMessage(form, response.message || 'Wiadomość została wysłana pomyślnie!');
                } else if (response && response.error) {
                    // Pokazuj błąd
                    showErrorMessage(form, response.error);
                }
            });
        });
    }

    /**
     * Pokazuje komunikat sukcesu
     * @param {HTMLFormElement} form - Formularz
     * @param {string} message - Komunikat
     */
    function showSuccessMessage(form, message) {
        form.innerHTML = `
            <div class="form-success">
                <div class="success-icon">✅</div>
                <h3 class="success-title">Dziękuję!</h3>
                <p class="success-text">${escapeHtml(message)}</p>
            </div>
        `;
    }

    /**
     * Pokazuje komunikat błędu
     * @param {HTMLFormElement} form - Formularz
     * @param {string|Array} error - Błąd lub tablica błędów
     */
    function showErrorMessage(form, error) {
        const errors = Array.isArray(error) ? error : [error];
        
        const errorContainer = document.createElement('div');
        errorContainer.className = 'form-general-errors';
        
        errors.forEach(err => {
            const errorEl = document.createElement('span');
            errorEl.className = 'form-error';
            errorEl.textContent = err;
            errorContainer.appendChild(errorEl);
        });
        
        // Usuń istniejące błędy
        const existingError = form.querySelector('.form-general-errors');
        if (existingError) {
            existingError.remove();
        }
        
        form.insertBefore(errorContainer, form.firstChild);
    }

    /**
     * Escapuje HTML
     * @param {string} text - Tekst do escapowania
     * @returns {string}
     */
    function escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }

    // ============================================
    // INICJALIZACJA
    // ============================================
    
    // Poczekaj na załadowanie DOM
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', () => {
            initContactForm();
        });
    } else {
        initContactForm();
    }

    // Eksportuj do window (dla globalnego dostępu)
    window.PortfolioForm = {
        validateField,
        validateForm,
        checkHoneypot,
        submitFormAjax,
        initRealtimeValidation,
        initContactForm,
        showSuccessMessage,
        showErrorMessage,
        isEmpty,
        isValidEmail,
        hasMinLength,
        hasMaxLength
    };

})();
