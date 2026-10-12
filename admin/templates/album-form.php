<?php
/**
 * Panel administracyjny - Formularz albumu
 */

// Inicjalizacja zmiennych formularza
$formErrors = [];
$formData = [
    'id' => $album ? $album['id'] : 0,
    'name' => $album ? $album['name'] : '',
    'slug' => $album ? $album['slug'] : '',
    'description' => $album ? $album['description'] : '',
    'order_index' => $album ? $album['order_index'] : 0
];

// Tytuł strony
$pageTitle = $album ? 'Edytuj album' : 'Dodaj nowy album';
?>

<!-- ============================================
     ALBUM - FORMULARZ
     ============================================ -->
<div class="admin-page-header">
    <div class="admin-breadcrumb">
        <a href="<?php echo SITE_URL; ?>/admin" class="admin-breadcrumb-link">Dashboard</a>
        <span class="admin-breadcrumb-separator">/</span>
        <a href="<?php echo SITE_URL; ?>/admin/galeria" class="admin-breadcrumb-link">Galeria</a>
        <span class="admin-breadcrumb-separator">/</span>
        <span class="admin-breadcrumb-current"><?php echo $pageTitle; ?></span>
    </div>
    
    <h1 class="admin-page-title">📂 <?php echo $pageTitle; ?></h1>
    <p class="admin-page-subtitle">Wypełnij formularz, aby <?php echo $album ? 'zaktualizować' : 'dodać'; ?> album</p>
</div>

<!-- Formularz -->
<div class="admin-form-container">
    <form class="admin-form" id="album-form" method="POST">
        <?php echo csrf_input(); ?>
        
        <input type="hidden" name="id" value="<?php echo $formData['id']; ?>">
        
        <!-- Podstawowe informacje -->
        <div class="admin-form-section">
            <h2 class="admin-form-title">Podstawowe informacje</h2>
            
            <div class="admin-form-row">
                <div class="admin-form-group">
                    <label for="name" class="admin-form-label">Nazwa *</label>
                    <input type="text" 
                           id="name" 
                           name="name"
                           class="admin-form-input"
                           value="<?php echo escape_output($formData['name']); ?>"
                           placeholder="Nazwa albumu"
                           required>
                </div>
                
                <div class="admin-form-group">
                    <label for="slug" class="admin-form-label">Slug (URL)</label>
                    <input type="text" 
                           id="slug" 
                           name="slug"
                           class="admin-form-input"
                           value="<?php echo escape_output($formData['slug']); ?>"
                           placeholder="nazwa-albumu">
                    <small style="color: var(--color-ink-light); font-size: 0.75rem;">
                        Jeśli puste, zostanie wygenerowane automatycznie
                    </small>
                </div>
            </div>
            
            <div class="admin-form-group">
                <label for="order_index" class="admin-form-label">Kolejność</label>
                <input type="number" 
                       id="order_index" 
                       name="order_index"
                       class="admin-form-input"
                       value="<?php echo $formData['order_index']; ?>"
                       min="0">
            </div>
            
            <div class="admin-form-group">
                <label for="description" class="admin-form-label">Opis</label>
                <textarea id="description" 
                          name="description"
                          class="admin-form-textarea"
                          rows="3"
                          placeholder="Opis albumu (opcjonalny)"><?php echo escape_output($formData['description']); ?></textarea>
            </div>
        </div>
        
        <!-- Akcje -->
        <div class="admin-form-actions">
            <a href="<?php echo SITE_URL; ?>/admin/galeria" class="admin-form-btn admin-form-btn-secondary">
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
// Generuj slug automatycznie
const nameInput = document.getElementById('name');
const slugInput = document.getElementById('slug');

if (nameInput && slugInput) {
    nameInput.addEventListener('input', () => {
        if (!slugInput.value.trim()) {
            // Generuj slug z nazwy
            const slug = nameInput.value.trim()
                .toLowerCase()
                .replace(/[^\w\s-]/g, '')
                .replace(/[\s_]+/g, '-')
                .replace(/^-+|-+$/g, '');
            
            slugInput.value = slug;
        }
    });
}

// Obsługa formularza
const form = document.getElementById('album-form');
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
        
        // Dodaj admin_page do URL
        formData.append('admin_page', 'save-album');
        
        fetch('<?php echo SITE_URL; ?>/admin', {
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
                showNotification(data.message || 'Album zapisan pomyślnie', 'success');
                
                // Przekieruj po 1 sekundzie
                setTimeout(() => {
                    window.location.href = '<?php echo SITE_URL; ?>/admin/galeria';
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
