<?php
/**
 * Panel administracyjny - Stopka
 */
?>
        </main>
    </div>

    <!-- ============================================
         MODAL (dla potwierdzeń, formularzy)
         ============================================ -->
    <div class="admin-modal" id="admin-modal" aria-hidden="true" role="dialog" aria-modal="true">
        <div class="admin-modal-content">
            <div class="admin-modal-header">
                <h2 class="admin-modal-title" id="modal-title">Potwierdź działanie</h2>
                <button class="admin-modal-close" id="modal-close" aria-label="Zamknij">
                    ✕
                </button>
            </div>
            <div class="admin-modal-body" id="modal-body">
                <p id="modal-message">Czy na pewno chcesz wykonać tę czynność?</p>
            </div>
            <div class="admin-modal-footer">
                <button class="admin-form-btn admin-form-btn-secondary" id="modal-cancel" aria-label="Anuluj">
                    ANULUJ
                </button>
                <button class="admin-form-btn admin-form-btn-danger" id="modal-confirm" aria-label="Potwierdź">
                    POTWIERDŹ
                </button>
            </div>
        </div>
    </div>

    <!-- ============================================
         NOTYFIKACJE
         ============================================ -->
    <div class="admin-notifications" id="admin-notifications"></div>

    <!-- ============================================
         SCRIPTY
         ============================================ -->
    
    <!-- Interact.js (dla drag & drop) -->
    <script src="https://cdn.jsdelivr.net/npm/interactjs/dist/interact.min.js" defer></script>
    
    <!-- Admin JS -->
    <script>
        // ============================================
        // USTAWIENIA
        // ============================================
        const SITE_URL = '<?php echo SITE_URL; ?>';
        const CSRF_TOKEN = '<?php echo get_csrf_token(); ?>';
        
        // ============================================
        // FUNKCJE POMOCNICZE
        // ============================================
        
        /**
         * Wysyła żądanie AJAX
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
                    showNotification('Wystąpił błąd. Spróbuj ponownie.', 'error');
                    throw error;
                });
        }
        
        /**
         * Pokazuje notyfikację
         */
        function showNotification(message, type = 'info') {
            const container = document.getElementById('admin-notifications');
            if (!container) return;
            
            const notification = document.createElement('div');
            notification.className = `admin-notification admin-notification-${type}`;
            notification.innerHTML = `
                <span class="notification-icon">${getNotificationIcon(type)}</span>
                <span class="notification-message">${message}</span>
                <button class="notification-close" aria-label="Zamknij">✕</button>
            `;
            
            container.appendChild(notification);
            
            // Usuń po 5 sekundach
            setTimeout(() => {
                notification.classList.add('hide');
                setTimeout(() => notification.remove(), 300);
            }, 5000);
            
            // Obsługa zamknięcia
            notification.querySelector('.notification-close').addEventListener('click', () => {
                notification.classList.add('hide');
                setTimeout(() => notification.remove(), 300);
            });
        }
        
        /**
         * Zwraca ikonę notyfikacji
         */
        function getNotificationIcon(type) {
            const icons = {
                'success': '✅',
                'error': '❌',
                'warning': '⚠️',
                'info': 'ℹ️'
            };
            return icons[type] || 'ℹ️';
        }
        
        // ============================================
        // NAWIGACJA
        // ============================================
        
        // Przełącz sidebar na mobile
        const sidebarToggle = document.querySelector('.admin-navbar-toggle');
        const sidebar = document.querySelector('.admin-sidebar');
        
        if (sidebarToggle && sidebar) {
            sidebarToggle.addEventListener('click', () => {
                const isExpanded = sidebarToggle.getAttribute('aria-expanded') === 'true';
                sidebarToggle.setAttribute('aria-expanded', !isExpanded);
                sidebar.classList.toggle('active');
            });
        }
        
        // ============================================
        // MENU UŻYTKOWNIKA
        // ============================================
        
        const userMenuBtn = document.getElementById('user-menu-btn');
        const userMenuDropdown = document.getElementById('user-menu-dropdown');
        
        if (userMenuBtn && userMenuDropdown) {
            userMenuBtn.addEventListener('click', (e) => {
                e.stopPropagation();
                userMenuDropdown.classList.toggle('active');
            });
            
            // Zamknij menu po kliknięciu poza
            document.addEventListener('click', () => {
                userMenuDropdown.classList.remove('active');
            });
            
            // Powstrzymaj propagację kliknięcia w menu
            userMenuDropdown.addEventListener('click', (e) => {
                e.stopPropagation();
            });
        }
        
        // ============================================
        // MODAL
        // ============================================
        
        const modal = document.getElementById('admin-modal');
        const modalTitle = document.getElementById('modal-title');
        const modalMessage = document.getElementById('modal-message');
        const modalClose = document.getElementById('modal-close');
        const modalCancel = document.getElementById('modal-cancel');
        const modalConfirm = document.getElementById('modal-confirm');
        
        let modalCallback = null;
        
        function showModal(title, message, callback = null) {
            if (!modal) return;
            
            modalTitle.textContent = title;
            modalMessage.textContent = message;
            modal.classList.add('active');
            modal.setAttribute('aria-hidden', 'false');
            modalCallback = callback;
        }
        
        function hideModal() {
            if (!modal) return;
            
            modal.classList.remove('active');
            modal.setAttribute('aria-hidden', 'true');
            modalCallback = null;
        }
        
        if (modalClose) {
            modalClose.addEventListener('click', hideModal);
        }
        
        if (modalCancel) {
            modalCancel.addEventListener('click', hideModal);
        }
        
        if (modalConfirm) {
            modalConfirm.addEventListener('click', () => {
                if (modalCallback) {
                    modalCallback();
                }
                hideModal();
            });
        }
        
        // Zamknij modal na kliknięcie w overlay
        if (modal) {
            modal.addEventListener('click', (e) => {
                if (e.target === modal) {
                    hideModal();
                }
            });
        }
        
        // ============================================
        // FUNKCJE GLOBALNE
        // ============================================
        
        /**
         * Potwierdza i usuwa element
         */
        function confirmDelete(url, message = 'Czy na pewno chcesz usunąć ten element?') {
            showModal('Potwierdź usunięcie', message, () => {
                sendAjaxRequest(url, 'POST', {})
                    .then(data => {
                        if (data.success) {
                            showNotification(data.message || 'Element usunięty pomyślnie', 'success');
                            location.reload();
                        } else {
                            showNotification(data.error || 'Wystąpił błąd', 'error');
                        }
                    });
            });
        }
        
        /**
         * Formatuje datę
         */
        function formatDate(dateString) {
            const months = [
                'stycznia', 'lutego', 'marca', 'kwietnia', 'maja', 'czerwca',
                'lipca', 'sierpnia', 'września', 'października', 'listopada', 'grudzia'
            ];
            
            const date = new Date(dateString);
            const day = date.getDate();
            const month = date.getMonth();
            const year = date.getFullYear();
            const hours = date.getHours().toString().padStart(2, '0');
            const minutes = date.getMinutes().toString().padStart(2, '0');
            
            return `${day} ${months[month]} ${year} ${hours}:${minutes}`;
        }
        
        /**
         * Escapuje HTML
         */
        function escapeHtml(text) {
            const div = document.createElement('div');
            div.textContent = text;
            return div.innerHTML;
        }
        
        // ============================================
        // WYLOGOWANIE
        // ============================================
        
        // Obsługa wylogowania
        const logoutLink = document.querySelector('a[href*="action=logout"]');
        if (logoutLink) {
            logoutLink.addEventListener('click', (e) => {
                e.preventDefault();
                
                sendAjaxRequest('<?php echo SITE_URL; ?>/admin?admin_page=logout', 'POST', {})
                    .then(data => {
                        if (data.success) {
                            window.location.href = '<?php echo SITE_URL; ?>/admin/login';
                        } else {
                            showNotification(data.error || 'Wystąpił błąd', 'error');
                        }
                    });
            });
        }
        
        // ============================================
        // STYLE DLA NOTYFIKACJI
        // ============================================
        
        const notificationStyles = document.createElement('style');
        notificationStyles.textContent = `
            .admin-notifications {
                position: fixed;
                top: var(--spacing-lg);
                right: var(--spacing-lg);
                z-index: calc(var(--z-modal) + 1);
                display: flex;
                flex-direction: column;
                gap: var(--spacing-sm);
                max-width: 350px;
                width: 100%;
            }
            
            .admin-notification {
                display: flex;
                align-items: center;
                gap: var(--spacing-sm);
                padding: var(--spacing-md);
                background: var(--color-paper);
                border-radius: var(--radius-sm);
                box-shadow: var(--shadow-lg);
                animation: slideInRight 0.3s ease-out;
                border-left: 4px solid var(--color-highlight);
            }
            
            .admin-notification-success {
                border-left-color: var(--color-success);
            }
            
            .admin-notification-error {
                border-left-color: var(--color-warning);
            }
            
            .admin-notification-warning {
                border-left-color: #FF9800;
            }
            
            .admin-notification-info {
                border-left-color: var(--color-highlight);
            }
            
            .notification-icon {
                font-size: 1.25rem;
            }
            
            .notification-message {
                flex: 1;
                font-size: 0.875rem;
            }
            
            .notification-close {
                background: none;
                border: none;
                font-size: 1rem;
                color: var(--color-ink-light);
                cursor: pointer;
                padding: 0;
                line-height: 1;
            }
            
            .notification-close:hover {
                color: var(--color-ink);
            }
            
            .admin-notification.hide {
                opacity: 0;
                transform: translateX(100%);
                transition: all 0.3s ease-out;
            }
            
            @keyframes slideInRight {
                from {
                    opacity: 0;
                    transform: translateX(100%);
                }
                to {
                    opacity: 1;
                    transform: translateX(0);
                }
            }
        `;
        document.head.appendChild(notificationStyles);
        
        // ============================================
        // INICJALIZACJA DODATKOWYCH FUNKCJI
        // ============================================
        
        // Dodaj klasę 'loaded' do body po załadowaniu
        document.addEventListener('DOMContentLoaded', () => {
            document.body.classList.add('loaded');
        });
        
        // Obsługa wylogowania przez GET (jeśli ktoś użyje linka bezpośrednio)
        if (window.location.search.includes('action=logout')) {
            sendAjaxRequest('<?php echo SITE_URL; ?>/admin?admin_page=logout', 'POST', {})
                .then(data => {
                    if (data.success) {
                        window.location.href = '<?php echo SITE_URL; ?>/admin/login';
                    }
                });
        }
    </script>
</body>
</html>
