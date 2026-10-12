<?php
/**
 * Panel administracyjny - Formularz projektu
 */

// Pobierz kategorie
$categories = get_project_categories();

// Inicjalizacja zmiennych formularza
$formErrors = [];
$formData = [
    'id' => $project ? $project['id'] : 0,
    'title' => $project ? $project['title'] : '',
    'slug' => $project ? $project['slug'] : '',
    'short_description' => $project ? $project['short_description'] : '',
    'description' => $project ? $project['description'] : '',
    'category_id' => $project ? $project['category_id'] : '',
    'technologies' => $project ? implode(', ', $project['technologies'] ?? []) : '',
    'live_url' => $project ? $project['live_url'] : '',
    'github_url' => $project ? $project['github_url'] : '',
    'is_published' => $project ? $project['is_published'] : true,
    'order_index' => $project ? $project['order_index'] : 0,
    'cover_image' => $project ? $project['cover_image'] : '',
    'gallery_images' => $project ? $project['gallery_images'] : []
];

// Tytuł strony
$pageTitle = $project ? 'Edytuj projekt' : 'Dodaj nowy projekt';
?>

<!-- ============================================
     PROJEKTY - FORMULARZ
     ============================================ -->
<div class="admin-page-header">
    <div class="admin-breadcrumb">
        <a href="<?php echo SITE_URL; ?>/admin" class="admin-breadcrumb-link">Dashboard</a>
        <span class="admin-breadcrumb-separator">/</span>
        <a href="<?php echo SITE_URL; ?>/admin/projekty" class="admin-breadcrumb-link">Projekty</a>
        <span class="admin-breadcrumb-separator">/</span>
        <span class="admin-breadcrumb-current"><?php echo $pageTitle; ?></span>
    </div>
    
    <h1 class="admin-page-title">💻 <?php echo $pageTitle; ?></h1>
    <p class="admin-page-subtitle">Wypełnij formularz, aby <?php echo $project ? 'zaktualizować' : 'dodać'; ?> projekt</p>
</div>

<!-- Formularz -->
<div class="admin-form-container">
    <form class="admin-form" id="project-form" method="POST" enctype="multipart/form-data">
        <?php echo csrf_input(); ?>
        
        <input type="hidden" name="id" value="<?php echo $formData['id']; ?>">
        
        <!-- Podstawowe informacje -->
        <div class="admin-form-section">
            <h2 class="admin-form-title">Podstawowe informacje</h2>
            
            <div class="admin-form-row">
                <div class="admin-form-group">
                    <label for="title" class="admin-form-label">Tytuł *</label>
                    <input type="text" 
                           id="title" 
                           name="title"
                           class="admin-form-input"
                           value="<?php echo escape_output($formData['title']); ?>"
                           placeholder="Nazwa projektu"
                           required>
                </div>
                
                <div class="admin-form-group">
                    <label for="slug" class="admin-form-label">Slug (URL)</label>
                    <input type="text" 
                           id="slug" 
                           name="slug"
                           class="admin-form-input"
                           value="<?php echo escape_output($formData['slug']); ?>"
                           placeholder="nazwa-projektu">
                    <small style="color: var(--color-ink-light); font-size: 0.75rem;">
                        Jeśli puste, zostanie wygenerowane automatycznie
                    </small>
                </div>
            </div>
            
            <div class="admin-form-row">
                <div class="admin-form-group">
                    <label for="category_id" class="admin-form-label">Kategoria</label>
                    <select id="category_id" name="category_id" class="admin-form-select">
                        <option value="">-- Brak kategorii --</option>
                        <?php foreach ($categories as $category): ?>
                        <option value="<?php echo $category['id']; ?>" 
                                <?php echo $formData['category_id'] == $category['id'] ? 'selected' : ''; ?>>
                            <?php echo escape_output($category['name']); ?>
                        </option>
                        <?php endforeach; ?>
                    </select>
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
            </div>
            
            <div class="admin-form-group">
                <label for="short_description" class="admin-form-label">Krótki opis</label>
                <textarea id="short_description" 
                          name="short_description"
                          class="admin-form-textarea"
                          rows="3"
                          placeholder="Krótki opis projektu (wyświetlany na karcie)"><?php echo escape_output($formData['short_description']); ?></textarea>
            </div>
            
            <div class="admin-form-group">
                <label for="description" class="admin-form-label">Pełny opis</label>
                <textarea id="description" 
                          name="description"
                          class="admin-form-textarea"
                          rows="6"
                          placeholder="Pełny opis projektu (obsługa HTML)"><?php echo $formData['description']; ?></textarea>
            </div>
        </div>
        
        <!-- Technologie -->
        <div class="admin-form-section">
            <h2 class="admin-form-title">Technologie</h2>
            
            <div class="admin-form-group">
                <label for="technologies" class="admin-form-label">Technologie (oddzielone przecinkiem)</label>
                <input type="text" 
                       id="technologies" 
                       name="technologies"
                       class="admin-form-input"
                       value="<?php echo escape_output($formData['technologies']); ?>"
                       placeholder="HTML, CSS, PHP, JavaScript">
                <small style="color: var(--color-ink-light); font-size: 0.75rem;">
                    Wprowadź technologie oddzielone przecinkiem
                </small>
            </div>
        </div>
        
        <!-- Linki -->
        <div class="admin-form-section">
            <h2 class="admin-form-title">Linki</h2>
            
            <div class="admin-form-row">
                <div class="admin-form-group">
                    <label for="live_url" class="admin-form-label">URL Demo</label>
                    <input type="url" 
                           id="live_url" 
                           name="live_url"
                           class="admin-form-input"
                           value="<?php echo escape_output($formData['live_url']); ?>"
                           placeholder="https://demo.com">
                </div>
                
                <div class="admin-form-group">
                    <label for="github_url" class="admin-form-label">URL GitHub</label>
                    <input type="url" 
                           id="github_url" 
                           name="github_url"
                           class="admin-form-input"
                           value="<?php echo escape_output($formData['github_url']); ?>"
                           placeholder="https://github.com/user/repo">
                </div>
            </div>
        </div>
        
        <!-- Obrazy -->
        <div class="admin-form-section">
            <h2 class="admin-form-title">Obrazy</h2>
            
            <div class="admin-form-row">
                <div class="admin-form-group">
                    <label for="cover_image" class="admin-form-label">Okładka projektu</label>
                    <input type="file" 
                           id="cover_image" 
                           name="cover_image"
                           class="admin-form-input"
                           accept="image/*">
                    
                    <?php if (!empty($formData['cover_image'])): ?>
                    <div style="margin-top: var(--spacing-sm);">
                        <img src="<?php echo UPLOADS_URL . '/projects/' . $formData['cover_image']; ?>" 
                             alt="Aktualna okładka" 
                             style="max-width: 200px; max-height: 150px; border-radius: var(--radius-sm);">
                        <div style="margin-top: var(--spacing-xs);">
                            <label class="admin-form-checkbox">
                                <input type="checkbox" name="remove_cover" value="1">
                                <span class="checkbox-label">Usuń obecną okładkę</span>
                            </label>
                        </div>
                    </div>
                    <?php endif; ?>
                </div>
                
                <div class="admin-form-group">
                    <label for="gallery_images" class="admin-form-label">Galeria zdjęć</label>
                    <input type="file" 
                           id="gallery_images" 
                           name="gallery_images[]"
                           class="admin-form-input"
                           accept="image/*"
                           multiple>
                    
                    <small style="color: var(--color-ink-light); font-size: 0.75rem;">
                        Wybierz wiele plików (CTRL+Klik lub CMD+Klik)
                    </small>
                    
                    <?php if (!empty($formData['gallery_images'])): ?>
                    <div style="margin-top: var(--spacing-sm); display: flex; flex-wrap: wrap; gap: var(--spacing-sm);">
                        <?php foreach ($formData['gallery_images'] as $image): ?>
                        <div style="position: relative; width: 80px; height: 60px;">
                            <img src="<?php echo UPLOADS_URL . '/projects/' . $image; ?>" 
                                 alt="Galeria" 
                                 style="width: 100%; height: 100%; object-fit: cover; border-radius: var(--radius-sm);">
                            <label class="admin-form-checkbox" style="position: absolute; top: 2px; right: 2px; background: rgba(0,0,0,0.7); padding: 2px; border-radius: var(--radius-sm);">
                                <input type="checkbox" name="remove_gallery[]" value="<?php echo $image; ?>">
                                <span class="checkbox-label" style="color: white; font-size: 0.75rem;">✕</span>
                            </label>
                        </div>
                        <?php endforeach; ?>
                    </div>
                    <input type="hidden" name="existing_gallery_images" value="<?php echo implode(',', $formData['gallery_images']); ?>">
                    <?php endif; ?>
                </div>
            </div>
        </div>
        
        <!-- Opublikowanie -->
        <div class="admin-form-section">
            <h2 class="admin-form-title">Status</h2>
            
            <div class="admin-form-group">
                <label class="admin-form-checkbox">
                    <input type="checkbox" 
                           name="is_published"
                           value="1"
                           <?php echo $formData['is_published'] ? 'checked' : ''; ?>>
                    <span class="checkbox-label">Opublikuj projekt</span>
                </label>
                <small style="color: var(--color-ink-light); font-size: 0.75rem;">
                    Odznacz, aby zapisać jako szkic
                </small>
            </div>
        </div>
        
        <!-- Akcje -->
        <div class="admin-form-actions">
            <a href="<?php echo SITE_URL; ?>/admin/projekty" class="admin-form-btn admin-form-btn-secondary">
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
const titleInput = document.getElementById('title');
const slugInput = document.getElementById('slug');

if (titleInput && slugInput) {
    titleInput.addEventListener('input', () => {
        if (!slugInput.value.trim()) {
            // Generuj slug z tytułu
            const slug = titleInput.value.trim()
                .toLowerCase()
                .replace(/[^\w\s-]/g, '')
                .replace(/[\s_]+/g, '-')
                .replace(/^-+|-+$/g, '');
            
            slugInput.value = slug;
        }
    });
}

// Obsługa formularza
const form = document.getElementById('project-form');
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
        
        fetch('<?php echo SITE_URL; ?>/admin?admin_page=save-project', {
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
                showNotification(data.message || 'Projekt zapisan pomyślnie', 'success');
                
                // Przekieruj po 1 sekundzie
                setTimeout(() => {
                    window.location.href = '<?php echo SITE_URL; ?>/admin/projekty';
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
