<?php
/**
 * Szablon sekcji Kontaktu
 * Formularz na kartce papieru
 */

// Pobierz ustawienia
$settings = get_settings();

// Inicjalizacja zmiennych formularza
$formErrors = [];
$formSuccess = false;
$formData = [
    'name' => '',
    'email' => '',
    'message' => ''
];

// Obsługa formularza (jeśli nie jest AJAX)
if ($_SERVER['REQUEST_METHOD'] === 'POST' && 
    !isset($_SERVER['HTTP_X_REQUESTED_WITH'])) {
    
    // Sprawdź token CSRF
    if (!isset($_POST[CSRF_TOKEN_NAME]) || !validate_csrf_token($_POST[CSRF_TOKEN_NAME])) {
        $formErrors[] = 'Nieważny token bezpieczeństwa. Odśwież stronę i spróbuj ponownie.';
    } else {
        // Walidacja
        $name = isset($_POST['name']) ? trim($_POST['name']) : '';
        $email = isset($_POST['email']) ? trim($_POST['email']) : '';
        $message = isset($_POST['message']) ? trim($_POST['message']) : '';
        $honeypot = isset($_POST['honeypot']) ? trim($_POST['honeypot']) : '';
        
        // Sprawdź honeypot
        if (!empty($honeypot)) {
            $formErrors[] = 'Spam detected.';
        }
        
        // Walidacja pól
        if (empty($name)) {
            $formErrors[] = 'Imię jest wymagane';
        }
        
        if (empty($email) || !filter_var($email, FILTER_VALIDATE_EMAIL)) {
            $formErrors[] = 'Poprawny adres email jest wymagany';
        }
        
        if (empty($message) || strlen($message) < 10) {
            $formErrors[] = 'Wiadomość musi mieć co najmniej 10 znaków';
        }
        
        if (empty($formErrors)) {
            // Sanityzacja
            $name = sanitize_input($name);
            $email = sanitize_input($email);
            $message = sanitize_input($message);
            
            // Zapisz do bazy
            $db = get_db_connection();
            $stmt = $db->prepare("INSERT INTO messages (name, email, message, ip_address, user_agent) 
                                  VALUES (?, ?, ?, ?, ?)");
            $stmt->execute([
                $name,
                $email,
                $message,
                $_SERVER['REMOTE_ADDR'] ?? null,
                $_SERVER['HTTP_USER_AGENT'] ?? null
            ]);
            
            // Wyślij email (jeśli skonfigurowano)
            if (defined('MAIL_TO') && MAIL_TO) {
                $subject = MAIL_SUBJECT_PREFIX . 'Nowa wiadomość od ' . $name;
                $headers = "From: " . MAIL_FROM . "\r\n";
                $headers .= "Reply-To: " . $email . "\r\n";
                $headers .= "Content-Type: text/plain; charset=UTF-8\r\n";
                
                $mailBody = "Imię: " . $name . "\n";
                $mailBody .= "Email: " . $email . "\n";
                $mailBody .= "Wiadomość:\n" . $message . "\n";
                $mailBody .= "\n---\n";
                $mailBody .= "IP: " . ($_SERVER['REMOTE_ADDR'] ?? 'unknown') . "\n";
                $mailBody .= "Data: " . date('Y-m-d H:i:s') . "\n";
                
                @mail(MAIL_TO, $subject, $mailBody, $headers);
            }
            
            $formSuccess = true;
            $formData = ['name' => '', 'email' => '', 'message' => ''];
        } else {
            $formData = [
                'name' => $name,
                'email' => $email,
                'message' => $message
            ];
        }
    }
}
?>

<!-- ============================================ -->
<!-- CONTACT SECTION -->
<!-- ============================================ -->
<section class="contact-section" id="contact" aria-labelledby="contact-title">
    <div class="section-container">
        <!-- Nagłówek sekcji -->
        <div class="section-header">
            <div class="section-title-container">
                <h2 id="contact-title" class="section-title">
                    <span class="title-icon">✉️</span>
                    <span>KONTAKT</span>
                </h2>
            </div>
            
            <p class="section-subtitle">
                Masz pytania lub chcesz współpracować? Napisz do mnie!
            </p>
        </div>
        
        <div class="contact-content">
            <!-- Lewa kolumna - Informacje kontaktowe -->
            <div class="contact-column contact-info-column">
                <div class="contact-info">
                    <!-- Email -->
                    <div class="contact-item">
                        <div class="contact-icon">✉️</div>
                        <div class="contact-details">
                            <span class="contact-label">Email</span>
                            <a href="mailto:<?php echo escape_output($settings['email'] ?? 'twoj@email.com'); ?>" 
                               class="contact-value cursor-hand">
                                <?php echo escape_output($settings['email'] ?? 'twoj@email.com'); ?>
                            </a>
                        </div>
                    </div>
                    
                    <!-- Social Media -->
                    <div class="contact-social">
                        <div class="social-grid">
                            <?php if (!empty($settings['github_url'])): ?>
                            <a href="<?php echo escape_output($settings['github_url']); ?>" 
                               class="social-item cursor-hand"
                               target="_blank"
                               rel="noopener noreferrer"
                               aria-label="GitHub">
                                <span class="social-icon">💻</span>
                                <span class="social-name">GitHub</span>
                            </a>
                            <?php endif; ?>
                            
                            <?php if (!empty($settings['linkedin_url'])): ?>
                            <a href="<?php echo escape_output($settings['linkedin_url']); ?>" 
                               class="social-item cursor-hand"
                               target="_blank"
                               rel="noopener noreferrer"
                               aria-label="LinkedIn">
                                <span class="social-icon">💼</span>
                                <span class="social-name">LinkedIn</span>
                            </a>
                            <?php endif; ?>
                            
                            <?php if (!empty($settings['instagram_url'])): ?>
                            <a href="<?php echo escape_output($settings['instagram_url']); ?>" 
                               class="social-item cursor-hand"
                               target="_blank"
                               rel="noopener noreferrer"
                               aria-label="Instagram">
                                <span class="social-icon">📷</span>
                                <span class="social-name">Instagram</span>
                            </a>
                            <?php endif; ?>
                        </div>
                    </div>
                    
                    <!-- Godziny pracy -->
                    <div class="contact-item">
                        <div class="contact-icon">⏰</div>
                        <div class="contact-details">
                            <span class="contact-label">Dostępność</span>
                            <span class="contact-value">Pn-Pt: 9:00 - 17:00</span>
                        </div>
                    </div>
                    
                    <!-- Lokalizacja -->
                    <div class="contact-item">
                        <div class="contact-icon">📍</div>
                        <div class="contact-details">
                            <span class="contact-label">Lokalizacja</span>
                            <span class="contact-value">Polska (Zdalnie)</span>
                        </div>
                    </div>
                </div>
            </div>
            
            <!-- Prawa kolumna - Formularz -->
            <div class="contact-column contact-form-column">
                <div class="form-container">
                    <!-- Kartka papieru -->
                    <div class="paper-sheet">
                        <div class="paper-corner top-left"></div>
                        <div class="paper-corner top-right"></div>
                        
                        <?php if ($formSuccess): ?>
                        <!-- Sukces -->
                        <div class="form-success">
                            <div class="success-icon">✅</div>
                            <h3 class="success-title">Dziękuję za wiadomość!</h3>
                            <p class="success-text">
                                Twoja wiadomość została wysłana pomyślnie. 
                                Odpowiem jak najszybciej.
                            </p>
                            <a href="<?php echo SITE_URL; ?>" class="btn btn-primary cursor-hand">
                                WRÓĆ DO STRONY GŁÓWNEJ
                            </a>
                        </div>
                        
                        <?php else: ?>
                        <!-- Formularz -->
                        <form class="contact-form" 
                              id="contact-form"
                              method="POST"
                              action="<?php echo SITE_URL; ?>/kontakt"
                              novalidate>
                            
                            <?php echo csrf_input(); ?>
                            
                            <!-- Honeypot (ochrona przed spamem) -->
                            <div class="honeypot-field" aria-hidden="true">
                                <label for="honeypot">Leave this empty</label>
                                <input type="text" id="honeypot" name="honeypot" tabindex="-1" autocomplete="off">
                            </div>
                            
                            <!-- Imię -->
                            <div class="form-group <?php echo isset($formErrors['name']) ? 'error' : ''; ?>">
                                <label for="name" class="form-label">Imię *</label>
                                <input type="text" 
                                       id="name" 
                                       name="name"
                                       class="form-input"
                                       value="<?php echo escape_output($formData['name']); ?>"
                                       placeholder="Twoje imię"
                                       required
                                       aria-required="true"
                                       autocomplete="name">
                                <?php if (isset($formErrors['name'])): ?>
                                <span class="form-error"><?php echo escape_output($formErrors['name']); ?></span>
                                <?php endif; ?>
                            </div>
                            
                            <!-- Email -->
                            <div class="form-group <?php echo isset($formErrors['email']) ? 'error' : ''; ?>">
                                <label for="email" class="form-label">Email *</label>
                                <input type="email" 
                                       id="email" 
                                       name="email"
                                       class="form-input"
                                       value="<?php echo escape_output($formData['email']); ?>"
                                       placeholder="twój@email.com"
                                       required
                                       aria-required="true"
                                       autocomplete="email">
                                <?php if (isset($formErrors['email'])): ?>
                                <span class="form-error"><?php echo escape_output($formErrors['email']); ?></span>
                                <?php endif; ?>
                            </div>
                            
                            <!-- Wiadomość -->
                            <div class="form-group <?php echo isset($formErrors['message']) ? 'error' : ''; ?>">
                                <label for="message" class="form-label">Wiadomość *</label>
                                <textarea id="message" 
                                          name="message"
                                          class="form-textarea"
                                          rows="5"
                                          placeholder="Napisz swoją wiadomość..."
                                          required
                                          aria-required="true"><?php echo escape_output($formData['message']); ?></textarea>
                                <?php if (isset($formErrors['message'])): ?>
                                <span class="form-error"><?php echo escape_output($formErrors['message']); ?></span>
                                <?php endif; ?>
                            </div>
                            
                            <!-- General errors -->
                            <?php if (!empty($formErrors) && !isset($formErrors['name']) && !isset($formErrors['email']) && !isset($formErrors['message'])): ?>
                            <div class="form-general-errors">
                                <?php foreach ($formErrors as $error): ?>
                                <span class="form-error"><?php echo escape_output($error); ?></span>
                                <?php endforeach; ?>
                            </div>
                            <?php endif; ?>
                            
                            <!-- Przycisk -->
                            <button type="submit" 
                                    class="btn btn-primary btn-large cursor-hammer"
                                    id="form-submit">
                                <span class="btn-icon">📤</span>
                                <span class="btn-text">WYŚLIJ WIADOMOŚĆ</span>
                                <span class="btn-loading">
                                    <span class="loading-spinner"></span>
                                </span>
                            </button>
                        </form>
                        <?php endif; ?>
                        
                        <div class="paper-corner bottom-left"></div>
                        <div class="paper-corner bottom-right"></div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</section>

<!-- ============================================ -->
<!-- SEPARATOR -->
<!-- ============================================ -->
<div class="section-separator">
    <div class="separator-line"></div>
    <div class="separator-icon">✉️</div>
    <div class="separator-line"></div>
</div>
