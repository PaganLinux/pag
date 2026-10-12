<?php
/**
 * Funkcje pomocnicze
 * Walidacja, sanityzacja, formatowanie, itp.
 */

/**
 * Sanityzacja danych wejściowych
 * @param mixed $data
 * @return string
 */
function sanitize_input($data) {
    if (is_array($data)) {
        return array_map('sanitize_input', $data);
    }
    
    $data = trim($data);
    $data = stripslashes($data);
    $data = htmlspecialchars($data, ENT_QUOTES | ENT_HTML5, 'UTF-8');
    return $data;
}

/**
 * Sanityzacja dla URL (slug)
 * @param string $string
 * @return string
 */
function sanitize_slug($string) {
    // Zamień polskie znaki
    $polishChars = [
        'ą' => 'a', 'ć' => 'c', 'ę' => 'e', 'ł' => 'l', 'ń' => 'n',
        'ó' => 'o', 'ś' => 's', 'ź' => 'z', 'ż' => 'z',
        'Ą' => 'A', 'Ć' => 'C', 'Ę' => 'E', 'Ł' => 'L', 'Ń' => 'N',
        'Ó' => 'O', 'Ś' => 'S', 'Ź' => 'Z', 'Ż' => 'Z'
    ];
    
    $string = strtr($string, $polishChars);
    
    // Usuń wszystkie znaki, które nie są alfanumeryczne, myślnikiem lub podkreślnikiem
    $string = preg_replace('/[^a-zA-Z0-9\-_]/', '-', $string);
    
    // Zamień wiele myślników na jeden
    $string = preg_replace('/-+/', '-', $string);
    
    // Usuń myślniki na początku i końcu
    $string = trim($string, '-');
    
    // Zamień na małe litery
    $string = strtolower($string);
    
    return $string;
}

/**
 * Escapowanie wyjścia (zapobieganie XSS)
 * @param string $string
 * @return string
 */
function escape_output($string) {
    return htmlspecialchars($string, ENT_QUOTES | ENT_HTML5, 'UTF-8');
}

/**
 * Generowanie losowej nazwy pliku
 * @param string $extension
 * @return string
 */
function generate_random_filename($extension) {
    $random = bin2hex(random_bytes(16));
    $timestamp = time();
    return $random . '_' . $timestamp . '.' . $extension;
}

/**
 * Sprawdza czy plik jest obrazem
 * @param string $filePath
 * @return bool
 */
function is_valid_image($filePath) {
    $allowedMimeTypes = ['image/jpeg', 'image/png', 'image/webp', 'image/gif'];
    $mimeType = mime_content_type($filePath);
    return in_array($mimeType, $allowedMimeTypes);
}

/**
 * Generowanie miniaturki
 * @param string $sourcePath
 * @param string $targetPath
 * @param int $width
 * @param int $height
 * @return bool
 */
function generate_thumbnail($sourcePath, $targetPath, $width = THUMB_WIDTH, $height = THUMB_HEIGHT) {
    if (!file_exists($sourcePath)) {
        return false;
    }
    
    $mimeType = mime_content_type($sourcePath);
    
    switch ($mimeType) {
        case 'image/jpeg':
            $source = imagecreatefromjpeg($sourcePath);
            break;
        case 'image/png':
            $source = imagecreatefrompng($sourcePath);
            imagealphablending($source, false);
            imagesavealpha($source, true);
            break;
        case 'image/webp':
            $source = imagecreatefromwebp($sourcePath);
            imagealphablending($source, false);
            imagesavealpha($source, true);
            break;
        case 'image/gif':
            $source = imagecreatefromgif($sourcePath);
            break;
        default:
            return false;
    }
    
    if (!$source) {
        return false;
    }
    
    $sourceWidth = imagesx($source);
    $sourceHeight = imagesy($source);
    
    // Oblicz proporcje
    $ratio = min($width / $sourceWidth, $height / $sourceHeight);
    $newWidth = (int)($sourceWidth * $ratio);
    $newHeight = (int)($sourceHeight * $ratio);
    
    // Utwórz nowy obraz
    $thumbnail = imagecreatetruecolor($width, $height);
    imagealphablending($thumbnail, false);
    imagesavealpha($thumbnail, true);
    
    // Wypełnij przezroczystym kolorem
    $transparent = imagecolorallocatealpha($thumbnail, 0, 0, 0, 127);
    imagefill($thumbnail, 0, 0, $transparent);
    
    // Skaluj obraz
    imagecopyresampled(
        $thumbnail, $source,
        (int)(($width - $newWidth) / 2), (int)(($height - $newHeight) / 2),
        0, 0,
        $newWidth, $newHeight,
        $sourceWidth, $sourceHeight
    );
    
    // Zapisz miniaturkę
    $mimeParts = explode('/', $mimeType);
    $ext = end($mimeParts);
    
    switch ($ext) {
        case 'jpeg':
            imagejpeg($thumbnail, $targetPath, 90);
            break;
        case 'png':
            imagepng($thumbnail, $targetPath, 9);
            break;
        case 'webp':
            imagewebp($thumbnail, $targetPath, 90);
            break;
        case 'gif':
            imagegif($thumbnail, $targetPath);
            break;
    }
    
    // Zwolnij pamięć
    imagedestroy($source);
    imagedestroy($thumbnail);
    
    return file_exists($targetPath);
}

/**
 * Konwersja obrazu do formatu WebP
 * @param string $sourcePath
 * @param string $targetPath
 * @param int $quality
 * @return bool
 */
function convert_to_webp($sourcePath, $targetPath, $quality = 80) {
    if (!file_exists($sourcePath)) {
        return false;
    }
    
    $mimeType = mime_content_type($sourcePath);
    
    switch ($mimeType) {
        case 'image/jpeg':
            $source = imagecreatefromjpeg($sourcePath);
            break;
        case 'image/png':
            $source = imagecreatefrompng($sourcePath);
            imagealphablending($source, false);
            imagesavealpha($source, true);
            break;
        default:
            return false;
    }
    
    if (!$source) {
        return false;
    }
    
    $result = imagewebp($source, $targetPath, $quality);
    imagedestroy($source);
    
    return $result;
}

/**
 * Pobiera ustawienia strony z bazy
 * @return array
 */
function get_settings() {
    $db = get_db_connection();
    
    try {
        $stmt = $db->prepare("SELECT * FROM settings LIMIT 1");
        $stmt->execute();
        $settings = $stmt->fetch(PDO::FETCH_ASSOC);
        
        if (!$settings) {
            // Jeśli nie ma ustawień w bazie, użyj domyślnych
            return [
                'site_title' => SITE_TITLE,
                'site_description' => SITE_DESCRIPTION,
                'theme' => 'light',
                'github_url' => null,
                'linkedin_url' => null,
                'instagram_url' => null,
                'email' => null
            ];
        }
        
        return $settings;
    } catch (PDOException $e) {
        if (DEBUG) {
            error_log('Błąd pobierania ustawień: ' . $e->getMessage());
        }
        return [
            'site_title' => SITE_TITLE,
            'site_description' => SITE_DESCRIPTION,
            'theme' => 'light'
        ];
    }
}

/**
 * Pobiera treść sekcji "O mnie"
 * @return array
 */
function get_about_content() {
    $db = get_db_connection();
    
    try {
        $stmt = $db->prepare("SELECT * FROM about LIMIT 1");
        $stmt->execute();
        return $stmt->fetch(PDO::FETCH_ASSOC) ?: [];
    } catch (PDOException $e) {
        if (DEBUG) {
            error_log('Błąd pobierania treści "O mnie": ' . $e->getMessage());
        }
        return [];
    }
}

/**
 * Pobiera kategorie projektów
 * @return array
 */
function get_project_categories() {
    $db = get_db_connection();
    
    try {
        $stmt = $db->prepare("SELECT * FROM project_categories ORDER BY order_index ASC");
        $stmt->execute();
        return $stmt->fetchAll(PDO::FETCH_ASSOC);
    } catch (PDOException $e) {
        if (DEBUG) {
            error_log('Błąd pobierania kategorii projektów: ' . $e->getMessage());
        }
        return [];
    }
}

/**
 * Pobiera albumy galerii
 * @return array
 */
function get_gallery_albums() {
    $db = get_db_connection();
    
    try {
        $stmt = $db->prepare("SELECT * FROM gallery_albums ORDER BY order_index ASC");
        $stmt->execute();
        return $stmt->fetchAll(PDO::FETCH_ASSOC);
    } catch (PDOException $e) {
        if (DEBUG) {
            error_log('Błąd pobierania albumów galerii: ' . $e->getMessage());
        }
        return [];
    }
}

/**
 * Pobiera pojedynczy projekt po slug
 * @param string $slug
 * @return array|null
 */
function get_project_by_slug($slug) {
    $db = get_db_connection();
    
    try {
        $stmt = $db->prepare("SELECT p.*, pc.name as category_name, pc.slug as category_slug 
                              FROM projects p 
                              LEFT JOIN project_categories pc ON p.category_id = pc.id 
                              WHERE p.slug = ? AND p.is_published = 1");
        $stmt->execute([$slug]);
        return $stmt->fetch(PDO::FETCH_ASSOC);
    } catch (PDOException $e) {
        if (DEBUG) {
            error_log('Błąd pobierania projektu: ' . $e->getMessage());
        }
        return null;
    }
}

/**
 * Pobiera pojedynczy album po slug
 * @param string $slug
 * @return array|null
 */
function get_album_by_slug($slug) {
    $db = get_db_connection();
    
    try {
        $stmt = $db->prepare("SELECT * FROM gallery_albums WHERE slug = ?");
        $stmt->execute([$slug]);
        return $stmt->fetch(PDO::FETCH_ASSOC);
    } catch (PDOException $e) {
        if (DEBUG) {
            error_log('Błąd pobierania albumu: ' . $e->getMessage());
        }
        return null;
    }
}

/**
 * Pobiera obrazy z albumu
 * @param int $albumId
 * @return array
 */
function get_images_by_album($albumId) {
    $db = get_db_connection();
    
    try {
        $stmt = $db->prepare("SELECT * FROM gallery_images 
                              WHERE album_id = ? AND is_published = 1 
                              ORDER BY order_index ASC");
        $stmt->execute([$albumId]);
        return $stmt->fetchAll(PDO::FETCH_ASSOC);
    } catch (PDOException $e) {
        if (DEBUG) {
            error_log('Błąd pobierania obrazów z albumu: ' . $e->getMessage());
        }
        return [];
    }
}

/**
 * Formatuje datę na polski format
 * @param string $date
 * @return string
 */
function format_polish_date($date) {
    if (empty($date)) {
        return '';
    }
    
    $timestamp = strtotime($date);
    $monthNames = [
        1 => 'stycznia', 'lutego', 'marca', 'kwietnia', 'maja', 'czerwca',
        'lipca', 'sierpnia', 'września', 'października', 'listopada', 'grudnia'
    ];
    
    $day = date('j', $timestamp);
    $month = date('n', $timestamp);
    $year = date('Y', $timestamp);
    
    return "$day {$monthNames[$month]} $year";
}

/**
 * Skraca tekst i dodaje wielokropek
 * @param string $text
 * @param int $length
 * @return string
 */
function truncate_text($text, $length = 100) {
    if (strlen($text) <= $length) {
        return $text;
    }
    
    $truncated = substr($text, 0, $length);
    $lastSpace = strrpos($truncated, ' ');
    
    if ($lastSpace !== false) {
        $truncated = substr($truncated, 0, $lastSpace);
    }
    
    return $truncated . '...';
}

/**
 * Generuje link do projektu
 * @param array $project
 * @return string
 */
function get_project_url($project) {
    return SITE_URL . '/projekt/' . ($project['slug'] ?? '');
}

/**
 * Generuje link do albumu
 * @param array $album
 * @return string
 */
function get_album_url($album) {
    return SITE_URL . '/album/' . ($album['slug'] ?? '');
}

/**
 * Sprawdza czy użytkownik jest zalogowany (admin)
 * @return bool
 */
function is_admin_logged_in() {
    return isset($_SESSION['admin_logged_in']) && $_SESSION['admin_logged_in'] === true;
}

/**
 * Pobiera dane zalogowanego admina
 * @return array|null
 */
function get_admin_user() {
    if (!is_admin_logged_in()) {
        return null;
    }
    
    return $_SESSION['admin_user'] ?? null;
}

/**
 * Generuje unikalny identyfikator
 * @return string
 */
function generate_uuid() {
    return sprintf(
        '%04x%04x-%04x-%04x-%04x-%04x%04x%04x',
        mt_rand(0, 0xffff), mt_rand(0, 0xffff),
        mt_rand(0, 0xffff),
        mt_rand(0, 0x0fff) | 0x4000,
        mt_rand(0, 0x3fff) | 0x8000,
        mt_rand(0, 0xffff), mt_rand(0, 0xffff), mt_rand(0, 0xffff)
    );
}

/**
 * Pobiera rozmiar pliku w czytelnej formie
 * @param int $bytes
 * @return string
 */
function format_file_size($bytes) {
    $units = ['B', 'KB', 'MB', 'GB'];
    $i = 0;
    
    while ($bytes >= 1024 && $i < count($units) - 1) {
        $bytes /= 1024;
        $i++;
    }
    
    return round($bytes, 2) . ' ' . $units[$i];
}
