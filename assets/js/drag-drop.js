/**
 * Portfolio - Drag & Drop
 * Obsługuje przeciąganie kart projektów i polaroidów
 */

(function() {
    'use strict';

    // ============================================
    // USTAWIENIA
    // ============================================
    const DRAGGABLE_SELECTOR = '.draggable-item';
    const CONTAINER_SELECTOR = '.workshop-table, .polaroid-container';

    // ============================================
    // FUNKCJE POMOCNICZE
    // ============================================

    /**
     * Sprawdza czy element jest przeciągalny
     * @param {HTMLElement} element
     * @returns {boolean}
     */
    function isDraggable(element) {
        return element.matches(DRAGGABLE_SELECTOR);
    }

    /**
     * Znajduje kontener dla elementu
     * @param {HTMLElement} element
     * @returns {HTMLElement|null}
     */
    function findContainer(element) {
        let container = element.parentElement;
        
        while (container) {
            if (container.matches(CONTAINER_SELECTOR)) {
                return container;
            }
            container = container.parentElement;
        }
        
        return null;
    }

    /**
     * Dodaje klasę 'dragging' do elementu i jego kopii
     * @param {HTMLElement} element
     */
    function addDraggingClass(element) {
        element.classList.add('dragging');
        
        // Dodaj efekt do cienia
        const shadow = element.querySelector('.project-shadow, .polaroid-shadow');
        if (shadow) {
            shadow.style.opacity = '0.8';
            shadow.style.transform = 'scaleX(1.2)';
        }
    }

    /**
     * Usuwa klasę 'dragging' z elementu
     * @param {HTMLElement} element
     */
    function removeDraggingClass(element) {
        element.classList.remove('dragging');
        
        // Przywróć cień
        const shadow = element.querySelector('.project-shadow, .polaroid-shadow');
        if (shadow) {
            shadow.style.opacity = '';
            shadow.style.transform = '';
        }
    }

    /**
     * Tworzy klon elementu do przeciągania
     * @param {HTMLElement} element
     * @returns {HTMLElement}
     */
    function createDragClone(element) {
        const clone = element.cloneNode(true);
        clone.style.position = 'absolute';
        clone.style.top = '0';
        clone.style.left = '0';
        clone.style.width = `${element.offsetWidth}px`;
        clone.style.height = `${element.offsetHeight}px`;
        clone.style.pointerEvents = 'none';
        clone.style.zIndex = '9999';
        clone.style.opacity = '0.8';
        clone.style.transform = 'rotate(0deg) scale(1)';
        
        // Usuń animacje
        clone.style.animation = 'none';
        
        // Dodaj do body
        document.body.appendChild(clone);
        
        return clone;
    }

    /**
     * Usuwa klon elementu
     * @param {HTMLElement} clone
     */
    function removeDragClone(clone) {
        if (clone && clone.parentNode) {
            clone.parentNode.removeChild(clone);
        }
    }

    // ============================================
    // INTERACT.JS - DRAG & DROP
    // ============================================

    /**
     * Inicjalizuje Interact.js
     */
    function initInteractDrag() {
        if (typeof interact === 'undefined') {
            console.warn('Interact.js nie jest załadowany. Używam fallback.');
            initFallbackDrag();
            return;
        }
        
        // Konfiguracja dla kart projektów
        interact(DRAGGABLE_SELECTOR)
            .draggable({
                ignoreFrom: '.action-btn, .project-link, a',
                allowFrom: '.project-card-inner, .polaroid-inner',
                listeners: {
                    move: dragMoveListener
                },
                modifiers: [
                    interact.modifiers.restrictRect({
                        restriction: 'parent',
                        endOnly: true
                    })
                ],
                inertia: true
            })
            .on('down', (event) => {
                const target = event.target;
                
                if (!isDraggable(target)) return;
                
                // Dodaj klasę dragging
                addDraggingClass(target);
                
                // Twórz klon
                const clone = createDragClone(target);
                target.dataset.dragClone = clone.id = `drag-clone-${Date.now()}`;
            })
            .on('move', (event) => {
                const target = event.target;
                const cloneId = target.dataset.dragClone;
                const clone = cloneId ? document.getElementById(cloneId) : null;
                
                if (clone) {
                    const x = (parseFloat(clone.style.left) || 0) + event.dx;
                    const y = (parseFloat(clone.style.top) || 0) + event.dy;
                    
                    clone.style.left = `${x}px`;
                    clone.style.top = `${y}px`;
                    
                    // Dodaj lekki obrót
                    clone.style.transform = `rotate(${x * 0.05}deg) scale(1)`;
                }
            })
            .on('up', (event) => {
                const target = event.target;
                const cloneId = target.dataset.dragClone;
                const clone = cloneId ? document.getElementById(cloneId) : null;
                
                // Usuń klasę dragging
                removeDraggingClass(target);
                
                // Usuń klon
                if (clone) {
                    removeDragClone(clone);
                }
                
                delete target.dataset.dragClone;
            });
    }

    /**
     * Listener dla ruchu przeciągania
     */
    function dragMoveListener(event) {
        const target = event.target;
        
        // Oblicz nową pozycję
        const x = (parseFloat(target.dataset.x) || 0) + event.dx;
        const y = (parseFloat(target.dataset.y) || 0) + event.dy;
        
        // Zapisz pozycję
        target.dataset.x = x;
        target.dataset.y = y;
        
        // Ustaw transform
        target.style.transform = `translate(${x}px, ${y}px) rotate(${x * 0.05}deg)`;
    }

    // ============================================
    // FALLBACK - CZYSTY JS (jeśli Interact.js nie działa)
    // ============================================

    /**
     * Inicjalizuje fallback drag & drop
     */
    function initFallbackDrag() {
        let draggedElement = null;
        let clone = null;
        let startX = 0;
        let startY = 0;
        let offsetX = 0;
        let offsetY = 0;
        
        // Dodaj event listeners do wszystkich przeciągalnych elementów
        document.querySelectorAll(DRAGGABLE_SELECTOR).forEach(element => {
            // Mouse events
            element.addEventListener('mousedown', (e) => {
                if (!isDraggable(e.target) && !isDraggable(e.target.closest(DRAGGABLE_SELECTOR))) {
                    return;
                }
                
                draggedElement = element;
                addDraggingClass(draggedElement);
                
                // Twórz klon
                clone = createDragClone(draggedElement);
                
                startX = e.clientX;
                startY = e.clientY;
                
                const rect = element.getBoundingClientRect();
                offsetX = startX - rect.left;
                offsetY = startY - rect.top;
                
                e.preventDefault();
            });
            
            // Touch events
            element.addEventListener('touchstart', (e) => {
                if (!isDraggable(e.target) && !isDraggable(e.target.closest(DRAGGABLE_SELECTOR))) {
                    return;
                }
                
                draggedElement = element;
                addDraggingClass(draggedElement);
                
                clone = createDragClone(draggedElement);
                
                const touch = e.touches[0];
                startX = touch.clientX;
                startY = touch.clientY;
                
                const rect = element.getBoundingClientRect();
                offsetX = startX - rect.left;
                offsetY = startY - rect.top;
                
                e.preventDefault();
            });
        });
        
        // Mouse move
        document.addEventListener('mousemove', (e) => {
            if (!draggedElement || !clone) return;
            
            const x = e.clientX - offsetX;
            const y = e.clientY - offsetY;
            
            clone.style.left = `${x}px`;
            clone.style.top = `${y}px`;
            clone.style.transform = `rotate(${x * 0.05}deg) scale(1)`;
            
            e.preventDefault();
        });
        
        // Touch move
        document.addEventListener('touchmove', (e) => {
            if (!draggedElement || !clone) return;
            
            const touch = e.touches[0];
            const x = touch.clientX - offsetX;
            const y = touch.clientY - offsetY;
            
            clone.style.left = `${x}px`;
            clone.style.top = `${y}px`;
            clone.style.transform = `rotate(${x * 0.05}deg) scale(1)`;
            
            e.preventDefault();
        });
        
        // Mouse up / Touch end
        function endDrag() {
            if (draggedElement) {
                removeDraggingClass(draggedElement);
                draggedElement = null;
            }
            
            if (clone) {
                removeDragClone(clone);
                clone = null;
            }
        }
        
        document.addEventListener('mouseup', endDrag);
        document.addEventListener('touchend', endDrag);
        document.addEventListener('touchcancel', endDrag);
    }

    // ============================================
    // OBRÓT POLAROIDÓW
    // ============================================

    /**
     * Inicjalizuje obrót polaroidów
     */
    function initPolaroidRotation() {
        const polaroids = document.querySelectorAll('.polaroid');
        
        polaroids.forEach(polaroid => {
            // Dodaj event listener do obrotu na hover
            polaroid.addEventListener('mouseenter', () => {
                // Losowy obrót
                const rotation = parseInt(polaroid.style.getPropertyValue('--rotation')) || 0;
                polaroid.style.transform = `rotate(${rotation}deg) translateY(-15px) translateZ(20px)`;
            });
            
            polaroid.addEventListener('mouseleave', () => {
                const rotation = parseInt(polaroid.style.getPropertyValue('--rotation')) || 0;
                const translateZ = polaroid.style.getPropertyValue('--translate-z') || '0px';
                polaroid.style.transform = `rotate(${rotation}deg) translateZ(${translateZ})`;
            });
        });
    }

    // ============================================
    // INICJALIZACJA
    // ============================================
    
    // Poczekaj na załadowanie DOM i Interact.js
    function initDragDrop() {
        // Inicjalizuj Interact.js
        initInteractDrag();
        
        // Inicjalizuj obrót polaroidów
        initPolaroidRotation();
    }

    // Sprawdź czy Interact.js jest załadowany
    if (typeof interact !== 'undefined') {
        initDragDrop();
    } else {
        // Poczekaj na załadowanie Interact.js
        const script = document.createElement('script');
        script.src = 'https://cdn.jsdelivr.net/npm/interactjs/dist/interact.min.js';
        script.onload = initDragDrop;
        document.head.appendChild(script);
    }

    // Eksportuj do window (dla globalnego dostępu)
    window.PortfolioDragDrop = {
        initDragDrop,
        isDraggable,
        findContainer
    };

})();
