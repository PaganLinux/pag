<?php
/**
 * Stopka strony
 */

// Pobierz ustawienia
$settings = get_settings();
?>
        </main>
        
        <!-- ============================================ -->
        <!-- STOPKA -->
        <!-- ============================================ -->
        <footer class="footer" role="contentinfo">
            <div class="footer-container">
                <!-- Narzędzia (ikony social media) -->
                <div class="footer-tools">
                    <div class="tool-icon">🔨</div>
                    <div class="tool-icon">📏</div>
                    <div class="tool-icon">🔧</div>
                    <div class="tool-icon">🗜️</div>
                    <div class="tool-icon">📸</div>
                </div>
                
                <!-- Linki social media -->
                <div class="footer-social">
                    <?php if (!empty($settings['github_url'])): ?>
                    <a href="<?php echo escape_output($settings['github_url']); ?>" 
                       class="social-link" 
                       aria-label="GitHub"
                       target="_blank" 
                       rel="noopener noreferrer">
                        <span class="social-icon">💻</span>
                    </a>
                    <?php endif; ?>
                    
                    <?php if (!empty($settings['linkedin_url'])): ?>
                    <a href="<?php echo escape_output($settings['linkedin_url']); ?>" 
                       class="social-link" 
                       aria-label="LinkedIn"
                       target="_blank" 
                       rel="noopener noreferrer">
                        <span class="social-icon">💼</span>
                    </a>
                    <?php endif; ?>
                    
                    <?php if (!empty($settings['instagram_url'])): ?>
                    <a href="<?php echo escape_output($settings['instagram_url']); ?>" 
                       class="social-link" 
                       aria-label="Instagram"
                       target="_blank" 
                       rel="noopener noreferrer">
                        <span class="social-icon">📷</span>
                    </a>
                    <?php endif; ?>
                    
                    <a href="mailto:<?php echo escape_output($settings['email'] ?? 'twoj@email.com'); ?>" 
                       class="social-link" 
                       aria-label="Email">
                        <span class="social-icon">✉️</span>
                    </a>
                </div>
                
                <!-- Copyright -->
                <div class="footer-copyright">
                    <p>&copy; <?php echo date('Y'); ?> Paweł Gajda. Wszelkie prawa zastrzeżone.</p>
                </div>
            </div>
        </footer>
    </body>
</html>

<!-- ============================================ -->
<!-- SCRIPTS (ładowane na końcu) -->
<!-- ============================================ -->

<script>
// Inicjalizacja preloadera
document.addEventListener('DOMContentLoaded', function() {
    const preloader = document.getElementById('preloader');
    
    // Ukryj preloader po załadowaniu strony
    window.addEventListener('load', function() {
        setTimeout(() => {
            preloader.classList.add('hidden');
            document.body.style.overflow = 'auto';
        }, 500);
    });
    
    // Blokuj scroll podczas ładowania
    document.body.style.overflow = 'hidden';
});

// Inicjalizacja custom cursora
document.addEventListener('DOMContentLoaded', function() {
    const cursor = document.querySelector('.custom-cursor');
    const body = document.body;
    
    // Poruszanie cursorem
    document.addEventListener('mousemove', (e) => {
        const x = e.clientX;
        const y = e.clientY;
        cursor.style.left = x + 'px';
        cursor.style.top = y + 'px';
    });
    
    // Zmiana cursora na podstawie klasy body
    document.addEventListener('mouseover', (e) => {
        const target = e.target;
        
        // Sprawdź czy element ma klasę cursor-*
        if (target.classList.contains('cursor-hand')) {
            body.classList.add('cursor-hand');
        } else if (target.classList.contains('cursor-pincer')) {
            body.classList.add('cursor-pincer');
        } else if (target.classList.contains('cursor-hammer')) {
            body.classList.add('cursor-hammer');
        }
        
        // Sprawdź rodziców
        let parent = target.parentElement;
        while (parent) {
            if (parent.classList.contains('cursor-hand')) {
                body.classList.add('cursor-hand');
                break;
            } else if (parent.classList.contains('cursor-pincer')) {
                body.classList.add('cursor-pincer');
                break;
            } else if (parent.classList.contains('cursor-hammer')) {
                body.classList.add('cursor-hammer');
                break;
            }
            parent = parent.parentElement;
        }
    });
    
    document.addEventListener('mouseout', (e) => {
        const target = e.target;
        
        if (target.classList.contains('cursor-hand')) {
            body.classList.remove('cursor-hand');
        } else if (target.classList.contains('cursor-pincer')) {
            body.classList.remove('cursor-pincer');
        } else if (target.classList.contains('cursor-hammer')) {
            body.classList.remove('cursor-hammer');
        }
    });
});

// Inicjalizacja nawigacji mobilnej
document.addEventListener('DOMContentLoaded', function() {
    const toggle = document.querySelector('.navbar-toggle');
    const menu = document.querySelector('.navbar-menu');
    
    if (toggle && menu) {
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
    }
});
</script>
