<?php
/**
 * Panel administracyjny - Ustawienia strony
 */

// Pobierz ustawienia
$settings = get_settings();

// Pobierz kategorie projektów
$categories = get_project_categories();

// Pobierz albumy
$albums = get_gallery_albums();

// Akcja
$action = isset($_GET['action']) ? trim($_GET['action']) : 'edit';

// Inicjalizacja zmiennych formularza
$formData = [
    'id' => $settings ? $settings['id'] : 0,
    'site_title' => $settings ? $settings['site_title'] : '',
    'site_description' => $settings ? $settings['site_description'] : '',
    'theme' => $settings ? $settings['theme'] : 'light',
    'github_url' => $settings ? $settings['github_url'] : '',
    'linkedin_url' => $settings ? $settings['linkedin_url'] : '',
    'instagram_url' => $settings ? $settings['instagram_url'] : '',
    'email' => $settings ? $settings['email'] : ''
];
?>

<!-- ============================================
     USTAWIENIA
     ============================================ -->
<div class="admin-page-header">
    <div class="admin-breadcrumb">
        <a href="<?php echo SITE_URL; ?>/admin" class="admin-breadcrumb-link">Dashboard</a>
        <span class="admin-breadcrumb-separator">/</span>
        <span class="admin-breadcrumb-current">Ustawienia</span>
    </div>
    
    <h1 class="admin-page-title">⚙️ USTAWIENIA STRONY</h1>
    <p class="admin-page-subtitle">Konfiguruj podstawowe ustawienia portfolio</p>
</div>

<!-- Statystyki -->
<div class="admin-cards" style="margin-bottom: var(--spacing-xl);">
    <article class="admin-card">
        <div class="admin-card-header">
            <h3 class="admin-card-title">Projekty</h3>
            <span class="admin-card-icon">💻</span>
        </div>
        <div class="admin-card-content">
            <div class="admin-card-value"><?php echo count($projects); ?></div>
            <div class="admin-card-label">Aktywnych</div>
        </div>
        <div class="admin-card-actions">
            <a href="<?php echo SITE_URL; ?>/admin/projekty" class="admin-card-btn">
                ZARZĄDZAJ
            </a>
        </div>
    </article>
    
    <article class="admin-card">
        <div class="admin-card-header">
            <h3 class="admin-card-title">Kategorie</h3>
            <span class="admin-card-icon">📁</span>
        </div>
        <div class="admin-card-content">
            <div class="admin-card-value"><?php echo count($categories); ?></div>
            <div class="admin-card-label">Kategorii projektów</div>
        </div>
        <div class="admin-card-actions">
            <button class="admin-card-btn" onclick="showModal('Nowa kategoria', 'Formularz zostanie dodany w następnej wersji')">
                DODAJ
            </button>
        </div>
    </article>
    
    <article class="admin-card">
        <div class="admin-card-header">
            <h3 class="admin-card-title">Albumy</h3>
            <span class="admin-card-icon">📂</span>
        </div>
        <div class="admin-card-content">
            <div class="admin-card-value"><?php echo count($albums); ?></div>
            <div class="admin-card-label">Albumów galerii</div>
        </div>
        <div class="admin-card-actions">
            <a href="<?php echo SITE_URL; ?>/admin/galeria?action=new-album" class="admin-card-btn">
                DODAJ
            </a>
        </div>
    </article>
</div>

<!-- Formularz ustawień -->
<div class="admin-form-container">
    <form class="admin-form" id="settings-form" method="POST">
        <?php echo csrf_input(); ?>
        
        <input type="hidden" name="id" value="<?php echo $formData['id']; ?>">
        
        <!-- Ogólne -->
        <div class="admin-form-section">
            <h2 class="admin-form-title">Ustawienia ogólne</h2>
            
            <div class="admin-form-group">
                <label for="site_title" class="admin-form-label">Tytuł strony *</label>
                <input type="text" 
                       id="site_title" 
                       name="site_title"
                       class="admin-form-input"
                       value="<?php echo escape_output($formData['site_title']); ?>"
                       placeholder="Tytuł strony (wyświetlany w nagłówku)"
                       required>
            </div>
            
            <div class="admin-form-group">
                <label for="site_description" class="admin-form-label">Opis strony</label>
                <textarea id="site_description" 
                          name="site_description"
                          class="admin-form-textarea"
                          rows="3"
                          placeholder="Opis strony (meta description)"><?php echo escape_output($formData['site_description']); ?></textarea>
                <small style="color: var(--color-ink-light); font-size: 0.75rem;">
                    Wykorzystywany w wynikach wyszukiwania
                </small>
            </div>
            
            <div class="admin-form-group">
                <label for="theme" class="admin-form-label">Motyw kolorystyczny</label>
                <select id="theme" name="theme" class="admin-form-select">
                    <option value="light" <?php echo $formData['theme'] === 'light' ? 'selected' : ''; ?>>Jasny</option>
                    <option value="dark" <?php echo $formData['theme'] === 'dark' ? 'selected' : ''; ?>>Ciemny</option>
                </select>
            </div>
        </div>
        
        <!-- Linki social media -->
        <div class="admin-form-section">
            <h2 class="admin-form-title">Linki social media</h2>
            
            <div class="admin-form-row">
                <div class="admin-form-group">
                    <label for="github_url" class="admin-form-label">GitHub</label>
                    <input type="url" 
                           id="github_url" 
                           name="github_url"
                           class="admin-form-input"
                           value="<?php echo escape_output($formData['github_url']); ?>"
                           placeholder="https://github.com/user">
                </div>
                
                <div class="admin-form-group">
                    <label for="linkedin_url" class="admin-form-label">LinkedIn</label>
                    <input type="url" 
                           id="linkedin_url" 
                           name="linkedin_url"
                           class="admin-form-input"
                           value="<?php echo escape_output($formData['linkedin_url']); ?>"
                           placeholder="https://linkedin.com/in/user">
                </div>
            </div>
            
            <div class="admin-form-row">
                <div class="admin-form-group">
                    <label for="instagram_url" class="admin-form-label">Instagram</label>
                    <input type="url" 
                           id="instagram_url" 
                           name="instagram_url"
                           class="admin-form-input"
                           value="<?php echo escape_output($formData['instagram_url']); ?>"
                           placeholder="https://instagram.com/user">
                </div>
                
                <div class="admin-form-group">
                    <label for="email" class="admin-form-label">Email *</label>
                    <input type="email" 
                           id="email" 
                           name="email"
                           class="admin-form-input"
                           value="<?php echo escape_output($formData['email']); ?>"
                           placeholder="twój@email.com"
                           required>
                </div>
            </div>
        </div>
        
        <!-- Akcje -->
        <div class="admin-form-actions">
            <button type="button" class="admin-form-btn admin-form-btn-secondary" onclick="window.location.reload()">
                PRZYWRÓĆ
            </button>
            <button type="submit" class="admin-form-btn admin-form-btn-primary" id="form-submit">
                <span id="form-submit-text">ZAPISZ USTAWIENIA</span>
                <span id="form-submit-loading" style="display: none;">
                    <span class="loading-spinner" style="display: inline-block; width: 1em; height: 1em; border: 2px solid rgba(255, 255, 255, 0.3); border-top-color: white; border-radius: 50%; animation: spin 0.8s linear infinite;"></span>
                </span>
            </button>
        </div>
    </form>
</div>

<script>
// Obsługa formularza
const form = document.getElementById('settings-form');
const submitBtn = document.getElementById('form-submit');
const submitText = document.getElementById('form-submit-text');
const submitLoading = document.getElementById('form-submit-loading');

if (form && submitBtn) {
    form.addEventListener('submit', (e) => {
        e.preventDefault();
        
        // Pokazuj loading
        submitBtn.disabled = true;
        submitText.style.display = 'none';
        submitLoading.style.display = 'inline-block';
        
        // Wyślij formularz
        const formData = new FormData(form);
        
        fetch('<?php echo SITE_URL; ?>/admin?admin_page=save-settings', {
            method: 'POST',
            body: formData,
            headers: {
                'X-Requested-With': 'XMLHttpRequest',
                'X-CSRF-Token': '<?php echo get_csrf_token(); ?>'
            }
        })
        .then(response => response.json())
        .then(data => {
            // Ukryj loading
            submitBtn.disabled = false;
            submitText.style.display = 'inline';
            submitLoading.style.display = 'none';
            
            if (data.success) {
                showNotification(data.message || 'Ustawienia zaktualizowane pomyślnie', 'success');
                
                // Odśwież stronę po 1 sekundzie, aby zaktualizować motyw
                setTimeout(() => {
                    window.location.reload();
                }, 1000);
            } else {
                showNotification(data.error || 'Wystąpił błąd', 'error');
            }
        })
        .catch(error => {
            // Ukryj loading
            submitBtn.disabled = false;
            submitText.style.display = 'inline';
            submitLoading.style.display = 'none';
            
            showNotification('Wystąpił błąd. Spróbuj ponownie.', 'error');
            console.error('Błąd:', error);
        });
    });
}

// Dodaj style dla loading spinner
const style = document.createElement('style');
style.textContent = `
    @keyframes spin {
        to { transform: rotate(360deg); }
    }
`;
document.head.appendChild(style);
</script>
