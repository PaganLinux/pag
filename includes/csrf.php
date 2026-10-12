<?php
/**
 * Obsługa tokenów CSRF
 * Generowanie, walidacja, przechowywanie w sesji
 */

/**
 * Generuje nowy token CSRF
 * @return string
 */
function generate_csrf_token() {
    if (empty($_SESSION[CSRF_TOKEN_NAME])) {
        $_SESSION[CSRF_TOKEN_NAME] = bin2hex(random_bytes(32));
    }
    return $_SESSION[CSRF_TOKEN_NAME];
}

/**
 * Zwraca aktualny token CSRF (lub generuje nowy)
 * @return string
 */
function get_csrf_token() {
    return generate_csrf_token();
}

/**
 * Waliduje token CSRF
 * @param string $token
 * @return bool
 */
function validate_csrf_token($token) {
    if (empty($token) || empty($_SESSION[CSRF_TOKEN_NAME])) {
        return false;
    }
    
    // Porównuj w sposób bezpieczny (stały czas)
    return hash_equals($_SESSION[CSRF_TOKEN_NAME], $token);
}

/**
 * Odświeża token CSRF (generuje nowy)
 */
function refresh_csrf_token() {
    unset($_SESSION[CSRF_TOKEN_NAME]);
    generate_csrf_token();
}

/**
 * Zwraca ukryte pole z tokenem CSRF
 * @return string
 */
function csrf_input() {
    return '<input type="hidden" name="' . CSRF_TOKEN_NAME . '" value="' . get_csrf_token() . '">';
}

/**
 * Zwraca meta tag z tokenem CSRF (dla AJAX)
 * @return string
 */
function csrf_meta_tag() {
    return '<meta name="csrf-token" content="' . get_csrf_token() . '">';
}

/**
 * Inicjalizuje CSRF dla formularzy
 * Powinno być wywoływane na początku strony
 */
function init_csrf() {
    if (!isset($_SESSION[CSRF_TOKEN_NAME])) {
        generate_csrf_token();
    }
}

// Inicjalizuj CSRF przy każdym załadowaniu strony
init_csrf();
