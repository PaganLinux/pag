<?php
/**
 * Panel administracyjny - Edycja sekcji "O mnie"
 */

// Pobierz treść sekcji
$about = get_about_content();

// Akcja
$action = isset($_GET['action']) ? trim($_GET['action']) : 'edit';

// Pobierz treść do edycji
$formData = [
    'id' => $about ? $about['id'] : 0,
    'title' => $about ? $about['title'] : '',
    'content' => $about ? $about['content'] : '',
    'image_path' => $about ? $about['image_path'] : ''
];
?>

<!-- ============================================
     O MNIE - EDYCJA
     ============================================ -->
<div class="admin-page-header">
    <div class="admin-breadcrumb">
        <a href="<?php echo SITE_URL; ?>/admin" class="admin-breadcrumb-link">Dashboard</a>
        <span class="admin-breadcrumb-separator">/</span>
        <span class="admin-breadcrumb-current">O mnie</span>
    </div>
    
    <h1 class="admin-page-title">👤 EDYCJA SEKCJI "O MNIE"</h1>
    <p class="admin-page-subtitle">Zaktualizuj informacje o sobie</p>
</div>

<!-- Formularz -->
<div class="admin-form-container">
    <form class="admin-form" id="about-form" method="POST" enctype="multipart/form-data">
        <?php echo csrf_input(); ?>
        
        <input type="hidden" name="id" value="<?php echo $formData['id']; ?>">
        
        <!-- Podstawowe informacje -->
        <div class="admin-form-section">
            <h2 class="admin-form-title">Podstawowe informacje</h2>
            
            <div class="admin-form-group">
                <label for="title" class="admin-form-label">Tytuł</label>
                <input type="text" 
                       id="title" 
                       name="title"
                       class="admin-form-input"
                       value="<?php echo escape_output($formData['title']); ?>"
                       placeholder="Tytuł sekcji">
            </div>
        </div>
        
        <!-- Treść -->
        <div class="admin-form-section">
            <h2 class="admin-form-title">Treść</h2>
            
            <div class="admin-form-group">
                <label for="content" class="admin-form-label">Treść *</label>
                <textarea id="content" 
                          name="content"
                          class="admin-form-textarea"
                          rows="10"
                          placeholder="Wprowadź treść sekcji "O mnie"..." required><?php echo $formData['content']; ?></textarea>
                <small style="color: var(--color-ink-light); font-size: 0.75rem;">
                    Obsługa HTML: &lt;p&gt;, &lt;br&gt;, &lt;strong&gt;, &lt;em&gt;, &lt;a&gt;, itp.
                </small>
            </div>
        </div>
        
        <!-- Obraz -->
        <div class="admin-form-section">
            <h2 class="admin-form-title">Obraz</h2>
            
            <div class="admin-form-group">
                <label for="image" class="admin-form-label">Zdjęcie profilowe</label>
                <input type="file" 
                       id="image" 
                       name="image"
                       class="admin-form-input"
                       accept="image/*">
                
                <?php if (!empty($formData['image_path'])): ?>
                <div style="margin-top: var(--spacing-sm);">
                    <img src="<?php echo UPLOADS_URL . '/' . $formData['image_path']; ?>" 
                         alt="Aktualne zdjęcie" 
                         style="max-width: 200px; max-height: 200px; border-radius: var(--radius-sm);">
                    <div style="margin-top: var(--spacing-xs);">
                        <label class="admin-form-checkbox">
                            <input type="checkbox" name="remove_image" value="1">
                            <span class="checkbox-label">Usuń obecne zdjęcie</span>
                        </label>
                    </div>
                </div>
                <?php endif; ?>
            </div>
        </div>
        
        <!-- Akcje -->
        <div class="admin-form-actions">
            <a href="<?php echo SITE_URL; ?>/admin" class="admin-form-btn admin-form-btn-secondary">
                ANULUJ
            </a>
            <button type="submit" class="admin-form-btn admin-form-btn-primary" id="form-submit">
                <span id="form-submit-text">ZAPISZ</span>
                <span id="form-submit-loading" style="display: none;">
                    <span class="loading-spinner" style="display: inline-block; width: 1em; height: 1em; border: 2px solid rgba(255, 255, 255, 0.3); border-top-color: white; border-radius: 50%; animation: spin 0.8s linear infinite;"></span>
                </span>
            </button>
        </div>
    </form>
</div>

<script>
// Obsługa formularza
const form = document.getElementById('about-form');
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
        
        fetch('<?php echo SITE_URL; ?>/admin?admin_page=save-about', {
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
                showNotification(data.message || 'Treść zaktualizowana pomyślnie', 'success');
                
                // Przekieruj po 1 sekundzie
                setTimeout(() => {
                    window.location.href = '<?php echo SITE_URL; ?>/admin';
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
