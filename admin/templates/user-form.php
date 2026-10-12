<?php
/**
 * Panel administracyjny - Formularz użytkownika
 */

// Inicjalizacja zmiennych formularza
$formErrors = [];
$formData = [
    'id' => $user ? $user['id'] : 0,
    'username' => $user ? $user['username'] : '',
    'email' => $user ? $user['email'] : '',
    'password' => '',
    'password_confirm' => ''
];

// Tytuł strony
$pageTitle = $user ? 'Edytuj użytkownika' : 'Dodaj nowego użytkownika';
?>

<!-- ============================================
     UŻYTKOWNIK - FORMULARZ
     ============================================ -->
<div class="admin-page-header">
    <div class="admin-breadcrumb">
        <a href="<?php echo SITE_URL; ?>/admin" class="admin-breadcrumb-link">Dashboard</a>
        <span class="admin-breadcrumb-separator">/</span>
        <a href="<?php echo SITE_URL; ?>/admin/uzytkownicy" class="admin-breadcrumb-link">Użytkownicy</a>
        <span class="admin-breadcrumb-separator">/</span>
        <span class="admin-breadcrumb-current"><?php echo $pageTitle; ?></span>
    </div>
    
    <h1 class="admin-page-title">👥 <?php echo $pageTitle; ?></h1>
    <p class="admin-page-subtitle">Wypełnij formularz, aby <?php echo $user ? 'zaktualizować' : 'dodać'; ?> użytkownika</p>
</div>

<!-- Formularz -->
<div class="admin-form-container">
    <form class="admin-form" id="user-form" method="POST">
        <?php echo csrf_input(); ?>
        
        <input type="hidden" name="id" value="<?php echo $formData['id']; ?>">
        
        <!-- Podstawowe informacje -->
        <div class="admin-form-section">
            <h2 class="admin-form-title">Dane logowania</h2>
            
            <div class="admin-form-row">
                <div class="admin-form-group">
                    <label for="username" class="admin-form-label">Nazwa użytkownika *</label>
                    <input type="text" 
                           id="username" 
                           name="username"
                           class="admin-form-input"
                           value="<?php echo escape_output($formData['username']); ?>"
                           placeholder="Nazwa użytkownika"
                           required>
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
            
            <?php if (!$user): ?>
            <div class="admin-form-row">
                <div class="admin-form-group">
                    <label for="password" class="admin-form-label">Hasło *</label>
                    <input type="password" 
                           id="password" 
                           name="password"
                           class="admin-form-input"
                           placeholder="Hasło"
                           required
                           minlength="8">
                    <small style="color: var(--color-ink-light); font-size: 0.75rem;">
                        Minimum 8 znaków
                    </small>
                </div>
                
                <div class="admin-form-group">
                    <label for="password_confirm" class="admin-form-label">Powtórz hasło *</label>
                    <input type="password" 
                           id="password_confirm" 
                           name="password_confirm"
                           class="admin-form-input"
                           placeholder="Powtórz hasło"
                           required
                           minlength="8">
                </div>
            </div>
            <?php else: ?>
            <div class="admin-form-group">
                <label for="password" class="admin-form-label">Nowe hasło (opcjonalne)</label>
                <input type="password" 
                       id="password" 
                       name="password"
                       class="admin-form-input"
                       placeholder="Nowe hasło (zostaw puste, aby nie zmieniać)"
                       minlength="8">
                <small style="color: var(--color-ink-light); font-size: 0.75rem;">
                    Zostaw puste, aby zachować obecne hasło
                </small>
            </div>
            <?php endif; ?>
        </div>
        
        <!-- Akcje -->
        <div class="admin-form-actions">
            <a href="<?php echo SITE_URL; ?>/admin/uzytkownicy" class="admin-form-btn admin-form-btn-secondary">
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
// Walidacja hasła
const passwordInput = document.getElementById('password');
const passwordConfirmInput = document.getElementById('password_confirm');
const form = document.getElementById('user-form');

if (passwordInput && passwordConfirmInput && form) {
    function validatePasswords() {
        if (passwordInput.value !== passwordConfirmInput.value) {
            passwordConfirmInput.setCustomValidity('Hasła nie są identyczne');
        } else {
            passwordConfirmInput.setCustomValidity('');
        }
    }
    
    passwordInput.addEventListener('change', validatePasswords);
    passwordConfirmInput.addEventListener('change', validatePasswords);
    
    // Walidacja na submit
    form.addEventListener('submit', (e) => {
        if (passwordInput.value !== passwordConfirmInput.value) {
            e.preventDefault();
            showNotification('Hasła nie są identyczne', 'error');
        }
    });
}

// Obsługa formularza
const submitBtn = document.getElementById('form-submit');
const submitText = document.getElementById('form-submit-text');
const submitLoading = document.getElementById('form-submit-loading');

if (form && submitBtn) {
    form.addEventListener('submit', (e) => {
        e.preventDefault();
        
        // Walidacja hasła
        if (passwordInput && passwordConfirmInput && passwordInput.value !== passwordConfirmInput.value) {
            showNotification('Hasła nie są identyczne', 'error');
            return;
        }
        
        // Pokazuj loading
        submitBtn.disabled = true;
        submitText.style.display = 'none';
        submitLoading.style.display = 'inline-block';
        
        // Wyślij formularz
        const formData = new FormData(form);
        
        fetch('<?php echo SITE_URL; ?>/admin?admin_page=save-user', {
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
                showNotification(data.message || 'Użytkownik zapisan pomyślnie', 'success');
                
                // Przekieruj po 1 sekundzie
                setTimeout(() => {
                    window.location.href = '<?php echo SITE_URL; ?>/admin/uzytkownicy';
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
