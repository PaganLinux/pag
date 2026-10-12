<?php
/**
 * Panel administracyjny - Formularz uploadu zdjęć
 */

// Pobierz albumy
$albums = get_gallery_albums();
?>

<!-- ============================================
     UPLOAD - FORMULARZ
     ============================================ -->
<div class="admin-page-header">
    <div class="admin-breadcrumb">
        <a href="<?php echo SITE_URL; ?>/admin" class="admin-breadcrumb-link">Dashboard</a>
        <span class="admin-breadcrumb-separator">/</span>
        <a href="<?php echo SITE_URL; ?>/admin/galeria" class="admin-breadcrumb-link">Galeria</a>
        <span class="admin-breadcrumb-separator">/</span>
        <span class="admin-breadcrumb-current">Upload zdjęć</span>
    </div>
    
    <h1 class="admin-page-title">📤 UPLOAD ZDJĘĆ</h1>
    <p class="admin-page-subtitle">Dodaj nowe zdjęcia do galerii</p>
</div>

<!-- Formularz -->
<div class="admin-form-container">
    <form class="admin-form" id="upload-form" method="POST" enctype="multipart/form-data">
        <?php echo csrf_input(); ?>
        
        <!-- Album -->
        <div class="admin-form-section">
            <h2 class="admin-form-title">Album</h2>
            
            <div class="admin-form-group">
                <label for="album_id" class="admin-form-label">Wybierz album</label>
                <select id="album_id" name="album_id" class="admin-form-select">
                    <option value="">-- Brak albumu (zdjęcia będą w "Wszystkie") --</option>
                    <?php foreach ($albums as $album): ?>
                    <option value="<?php echo $album['id']; ?>">
                        <?php echo escape_output($album['name']); ?>
                    </option>
                    <?php endforeach; ?>
                </select>
            </div>
        </div>
        
        <!-- Obrazy -->
        <div class="admin-form-section">
            <h2 class="admin-form-title">Zdjęcia</h2>
            
            <div class="admin-form-group">
                <label for="images" class="admin-form-label">Wybierz zdjęcia *</label>
                <input type="file" 
                       id="images" 
                       name="gallery_images[]"
                       class="admin-form-input"
                       accept="image/*"
                       multiple
                       required>
                <small style="color: var(--color-ink-light); font-size: 0.75rem;">
                    Wybierz jeden lub więcej plików (maks. <?php echo MAX_UPLOAD_SIZE / (1024 * 1024); ?>MB na plik)
                </small>
            </div>
        </div>
        
        <!-- Informacje o zdjęciach (opcjonalne) -->
        <div class="admin-form-section">
            <h2 class="admin-form-title">Informacje (opcjonalne)</h2>
            
            <div class="admin-form-row">
                <div class="admin-form-group">
                    <label for="title" class="admin-form-label">Tytuł (dla wszystkich)</label>
                    <input type="text" 
                           id="title" 
                           name="title"
                           class="admin-form-input"
                           placeholder="Tytuł zdjęcia">
                </div>
                
                <div class="admin-form-group">
                    <label for="alt_text" class="admin-form-label">Tekst alternatywny (dla wszystkich)</label>
                    <input type="text" 
                           id="alt_text" 
                           name="alt_text"
                           class="admin-form-input"
                           placeholder="Opis dla SEO">
                </div>
            </div>
            
            <div class="admin-form-group">
                <label for="description" class="admin-form-label">Opis (dla wszystkich)</label>
                <textarea id="description" 
                          name="description"
                          class="admin-form-textarea"
                          rows="3"
                          placeholder="Opis zdjęcia"></textarea>
            </div>
        </div>
        
        <!-- Akcje -->
        <div class="admin-form-actions">
            <a href="<?php echo SITE_URL; ?>/admin/galeria" class="admin-form-btn admin-form-btn-secondary">
                ANULUJ
            </a>
            <button type="submit" class="admin-form-btn admin-form-btn-primary" id="form-submit">
                <span id="form-submit-text">UPLOAD</span>
                <span id="form-submit-loading" style="display: none;">
                    <span class="loading-spinner" style="display: inline-block; width: 1em; height: 1em; border: 2px solid rgba(255, 255, 255, 0.3); border-top-color: white; border-radius: 50%; animation: spin 0.8s linear infinite;"></span>
                </span>
            </button>
        </div>
    </form>
    
    <!-- Postęp uploadu -->
    <div id="upload-progress" style="display: none; margin-top: var(--spacing-lg);">
        <div style="background: var(--color-paper-dark); border-radius: var(--radius-sm); padding: var(--spacing-md);">
            <div style="display: flex; align-items: center; gap: var(--spacing-md);">
                <div style="font-size: 1.5rem;">📤</div>
                <div style="flex: 1;">
                    <div style="font-family: var(--font-primary); font-size: 1rem; margin-bottom: var(--spacing-xs);">
                        Upload w toku...
                    </div>
                    <div class="progress-bar" style="height: 4px; background: var(--color-metal-light); border-radius: 2px; overflow: hidden;">
                        <div class="progress-bar-fill" style="height: 100%; width: 0%; background: var(--color-highlight); transition: width 0.3s ease;"></div>
                    </div>
                    <div id="progress-text" style="font-size: 0.75rem; color: var(--color-ink-light); margin-top: var(--spacing-xs);">
                        0%
                    </div>
                </div>
            </div>
        </div>
    </div>
    
    <!-- Wynik -->
    <div id="upload-result" style="display: none; margin-top: var(--spacing-lg);"></div>
</div>

<script>
// Obsługa formularza uploadu
const form = document.getElementById('upload-form');
const submitBtn = document.getElementById('form-submit');
const submitText = document.getElementById('form-submit-text');
const submitLoading = document.getElementById('form-submit-loading');
const progressContainer = document.getElementById('upload-progress');
const progressBar = progressContainer.querySelector('.progress-bar-fill');
const progressText = document.getElementById('progress-text');
const resultContainer = document.getElementById('upload-result');

if (form && submitBtn) {
    form.addEventListener('submit', (e) => {
        e.preventDefault();
        
        // Pokazuj loading
        submitBtn.disabled = true;
        submitText.style.display = 'none';
        submitLoading.style.display = 'inline-block';
        
        // Pokazuj postęp
        progressContainer.style.display = 'block';
        
        // Wyślij formularz
        const formData = new FormData(form);
        
        fetch('<?php echo SITE_URL; ?>/admin?admin_page=upload-image', {
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
            
            // Ukryj postęp
            progressContainer.style.display = 'none';
            
            if (data.success) {
                // Pokazuj wynik
                resultContainer.style.display = 'block';
                resultContainer.innerHTML = `
                    <div style="padding: var(--spacing-lg); background: rgba(56, 142, 60, 0.1); border: 1px solid var(--color-success); border-radius: var(--radius-sm);">
                        <div style="color: var(--color-success); font-family: var(--font-primary); font-size: 1.25rem; margin-bottom: var(--spacing-sm);">
                            ✅ Sukces!
                        </div>
                        <div style="color: var(--color-ink); margin-bottom: var(--spacing-sm);">
                            ${data.message}
                        </div>
                        <div style="color: var(--color-ink-light); font-size: 0.875rem;">
                            ${data.image ? `Dodano: ${data.image.image_path}` : ''}
                        </div>
                    </div>
                `;
                
                // Przekieruj po 2 sekundach
                setTimeout(() => {
                    window.location.href = '<?php echo SITE_URL; ?>/admin/galeria';
                }, 2000);
            } else {
                showNotification(data.error || 'Wystąpił błąd', 'error');
            }
        })
        .catch(error => {
            // Ukryj loading
            submitBtn.disabled = false;
            submitText.style.display = 'inline';
            submitLoading.style.display = 'none';
            
            // Ukryj postęp
            progressContainer.style.display = 'none';
            
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
