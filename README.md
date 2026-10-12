# \ud83c\udded Workshop Table - Portfolio Pawel Gajda

**Unikalne, interaktywne portfolio dla projektow webowych i fotografii z panelem administracyjnym**

[![PHP 8.0+](https://img.shields.io/badge/PHP-8.0%2B-777BB4?logo=php&logoColor=white)](https://www.php.net/)
[![MySQL](https://img.shields.io/badge/MySQL-5.7%2B-4479A1?logo=mysql&logoColor=white)](https://www.mysql.com/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)

---

## \ud83c\udf10 Opis Projektu

**Workshop Table** to autorskie, artystyczne portfolio laczace projektowanie webowe z fotografia. Koncepcja wzorowana na interaktywnym warsztacie, gdzie projekty i zdjecia sa przeciagane jak narzedzia na stole roboczym.

### \u2728 Glowne Cechy

- \ud83d\udcbb **Interaktywny Design** - Przeciagane karty projektow i polaroidy zdjec
- \ud83d\udc7d **Custom Cursor** - Kontekstowy kursor (reka, pinceta, mlotek) reagujacy na zawartosc
- \ud83c\udfa8 **Animacje GSAP** - Plynne animacje i mikrointerakcje
- \ud83d\udd0d **Lightbox** - Galeria ze swiped i nawigacja klawiaturowa
- \ud83d\udc64 **Panel Admina** - Pelna kontroli nad trescia i ustawieniami
- \ud83d\udc8c **Dark/Light Mode** - Przelaczanie motywow z pamiecia lokalna
- \u26a1 **Bezpieczenstwo** - CSRF, XSS protection, secure uploads, session security

---

## \ud83d\udcbb Technologie

| Zastosowanie | Technologia | Wersja |
|-------------|-------------|--------|
| Backend | PHP | 8.0+ |
| Baza Danych | MySQL | 5.7+ |
| Frontend | HTML5, CSS3, Vanilla JS | - |
| AJAX | fetch API | - |
| Animacje | GSAP | CDN |
| Smooth Scroll | Lenis | CDN |
| Drag & Drop | interact.js | CDN |

---

## \ud83d\udcc2 Struktura Katalogow

```
portfolio-pawel-gajda/
├── index.php                    # Glowny router
├── config.php                   # Konfiguracja bazy i ustawien
├── .htaccess                    # URL rewriting i naglowki bezpieczenstwa
├── README.md                    # Ta dokumentacja
├── install/
│   └── install.php              # Instalator
├── includes/
│   ├── db.php                   # Polaczenie PDO i helpery bazodanowe
│   ├── functions.php            # Funkcje pomocnicze (sanityzacja, formatowanie, obrazy)
│   ├── csrf.php                 # Generowanie i walidacja tokenow CSRF
│   ├── auth.php                 # Autentykacja i autoryzacja adminow
│   ├── header.php               # HTML head, navbar, preloader, custom cursor
│   └── footer.php               # Stopka, linki social, skrypty JS
├── templates/
│   ├── hero.php                 # Sekcja hero z animowanymi narzedziami
│   ├── projects.php             # Lista projektow z filtrowaniem
│   ├── gallery.php              # Galeria polaroidow
│   ├── about.php                # Sekcja "O mnie"
│   ├── contact.php              # Formularz kontaktowy
│   ├── project-detail.php       # Szczegoly pojedynczego projektu
│   └── 404.php                  # Strona bledow 404
├── admin/
│   ├── index.php                # Router panelu admina
│   ├── login.php                # Strona logowania
│   ├── dashboard.php            # Dashboard ze statystykami
│   ├── projects.php             # Zarzadzanie projektami
│   ├── galeria.php              # Zarzadzanie galeria
│   ├── about.php                # Zarzadzanie sekcja "O mnie"
│   ├── messages.php             # Skrzynka wiadomosci
│   ├── settings.php             # Ustawienia strony
│   ├── users.php                # Zarzadzanie uzytkownikami
│   ├── includes/
│   │   ├── header.php           # Header panelu admina
│   │   └── footer.php           # Footer panelu admina
│   └── templates/
│       ├── projects-list.php    # Lista projektow (admin)
│       ├── project-form.php     # Formularz projektu
│       ├── gallery-list.php     # Lista albumow/zdjec
│       ├── album-form.php       # Formularz albumu
│       ├── upload-form.php      # Upload wielu plikow
│       ├── messages-list.php    # Lista wiadomosci
│       ├── message-view.php     # Podglad wiadomosci
│       ├── users-list.php       # Lista uzytkownikow
│       └── user-form.php        # Formularz uzytkownika
├── assets/
│   ├── css/
│   │   ├── style.css            # Glowne style
│   │   ├── dark-mode.css        # Motyw ciemny
│   │   ├── animations.css       # Animacje GSAP i custom
│   │   └── lightbox.css         # Style lightboxa
│   ├── js/
│   │   ├── theme.js             # Przelaczanie motywow
│   │   ├── main.js              # Glowna logika (AJAX, lightbox, formularze)
│   │   ├── drag-drop.js         # Drag & drop z interact.js
│   │   └── form.js              # Walidacja formularzy
│   └── images/
│       └── preloader.gif        # Animacja preloadera
└── uploads/
    ├── projects/                # Miniatury projektow
    ├── gallery/                 # Zdjecia galerii
    ├── thumbnails/              # Automatycznie generowane miniatury
    └── .htaccess                # Zabezpieczenie uploadow
```

---

## \ud83d\udca1 Wymagania Systemowe

### Serwer
- PHP 8.0 lub nowszy
- MySQL 5.7 lub nowszy
- Rozszerzenia PHP: `pdo_mysql`, `gd`, `fileinfo`, `mbstring`, `session`
- Mod_rewrite (Apache) lub odpowiednik dla Nginx

### Klient
- Nowoczesna przegladarka (Chrome, Firefox, Safari, Edge)
- JavaScript wlaczony

---

## \ud83d\udc84 Instalacja

### 1. Pobranie i przygotowanie plikow

```bash
# Sklonuj lub pobierz repozytorium
git clone https://github.com/PaganLinux/pag.git portfolio-pawel-gajda
cd portfolio-pawel-gajda

# Lub pobierz jako ZIP i wypakuj
```

### 2. Upload na serwer

Wgraj wszystkie pliki na swoj serwer (FTP, SFTP, SSH).

### 3. Utworzenie bazy danych

Przed uruchomieniem instalatora utworz pusta baze danych MySQL:

```sql
CREATE DATABASE portfolio_pawel_gajda CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
```

### 4. Uruchomienie instalatora

Otworz w przegladarce:
```
https://twoja-domena.pl/install/install.php
```

### 5. Konfiguracja przez instalator

Instalator poprosi o:
- \ud83c\udf10 **Nazwe bazy danych** (np. `portfolio_pawel_gajda`)
- \ud83d\udc89 **Uzytkownik bazy** (z pelnymi uprawnieniami)
- \ud83d\udd11 **Haslo bazy**
- \ud83d\udc64 **Admin username** (login do panelu)
- \ud83d\udd12 **Admin password** (haslo do panelu)
- \ud83c\udf0d **Site title** (tytul strony)
- \ud83d\udce7 **Email** (do odbierania wiadomosci kontaktowych)

### 6. Zakonczenie instalacji

Po pomyslnym zakonczeniu:
1. \ud83d\udc8a **Usun katalog `/install`** z serwera (ze wzgledow bezpieczenstwa)
2. \ud83d\udc8b **Zaloguj sie do panelu admina** pod adresem: `https://twoja-domena.pl/admin`

---

## \ud83d\udcc3 Konfiguracja Reczna (opcjonalna)

Jesli preferujesz konfiguracje reczna, mozesz edytowac `config.php`:

```php
<?php
// Ustawienia bazy danych
define('DB_HOST', 'localhost');
define('DB_NAME', 'portfolio_pawel_gajda');
define('DB_USER', 'username');
define('DB_PASS', 'password');

// Ustawienia strony
define('SITE_TITLE', 'Portfolio Pawel Gajda');
define('SITE_DESCRIPTION', 'Web Developer & Photographer');
define('SITE_URL', 'https://twoja-domena.pl');

// Ustawienia bezpieczenstwa
define('SESSION_TIMEOUT', 1800); // 30 minut
define('MAX_LOGIN_ATTEMPTS', 5);
define('LOGIN_LOCKOUT_TIME', 900); // 15 minut

// Ustawienia uploadu
define('MAX_UPLOAD_SIZE', 10 * 1024 * 1024); // 10MB
define('ALLOWED_IMAGE_TYPES', ['image/jpeg', 'image/png', 'image/webp']);

// Social Media
define('SOCIAL_GITHUB', 'https://github.com/PaganLinux');
define('SOCIAL_LINKEDIN', 'https://linkedin.com/in/pawel-gajda');
define('SOCIAL_INSTAGRAM', 'https://instagram.com/pawelgajdaphoto');
define('SOCIAL_EMAIL', 'kontakt@pawelgajda.pl');

// Google Fonts
define('FONT_HEADING', 'Bebas+Neue');
define('FONT_BODY', 'Roboto+Mono');

// Kolory (CSS Variables)
define('COLOR_WOOD', '#5C4033');
define('COLOR_METAL', '#A8A8A8');
define('COLOR_BLUE', '#4A90E2');
define('COLOR_PAPER', '#F8F4E6');
define('COLOR_INK', '#1A1A1A');
define('COLOR_RED', '#D32F2F');
define('COLOR_GREEN', '#388E3C');
```

---

## \ud83d\udce7 Import Bazy Danych (alternatywa)

Jesli nie uzywasz instalatora, mozesz zaimportowac strukture bazy recznie:

```sql
-- Utworz baze danych
CREATE DATABASE IF NOT EXISTS portfolio_pawel_gajda 
    CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

USE portfolio_pawel_gajda;

-- Tabela uzytkownikow
CREATE TABLE IF NOT EXISTS users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    email VARCHAR(100) NOT NULL UNIQUE,
    role ENUM('admin', 'editor') DEFAULT 'admin',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    last_login DATETIME NULL,
    login_attempts INT DEFAULT 0,
    locked_until DATETIME NULL,
    INDEX (username),
    INDEX (email)
) ENGINE=InnoDB;

-- Tabela ustawien
CREATE TABLE IF NOT EXISTS settings (
    id INT AUTO_INCREMENT PRIMARY KEY,
    setting_key VARCHAR(100) NOT NULL UNIQUE,
    setting_value TEXT,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE=InnoDB;

-- Tabela "O mnie"
CREATE TABLE IF NOT EXISTS about (
    id INT AUTO_INCREMENT PRIMARY KEY,
    content TEXT,
    skills TEXT,
    achievements TEXT,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE=InnoDB;

-- Tabela kategorii projektow
CREATE TABLE IF NOT EXISTS project_categories (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    order_index INT DEFAULT 0,
    is_active BOOLEAN DEFAULT TRUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB;

-- Tabela projektow
CREATE TABLE IF NOT EXISTS projects (
    id INT AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(200) NOT NULL,
    slug VARCHAR(200) NOT NULL UNIQUE,
    description TEXT,
    short_description TEXT,
    category_id INT NULL,
    technologies JSON,
    live_url VARCHAR(500),
    github_url VARCHAR(500),
    cover_image VARCHAR(255),
    gallery JSON,
    order_index INT DEFAULT 0,
    is_published BOOLEAN DEFAULT TRUE,
    is_featured BOOLEAN DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (category_id) REFERENCES project_categories(id) ON DELETE SET NULL,
    INDEX (slug),
    INDEX (category_id),
    INDEX (is_published),
    INDEX (order_index)
) ENGINE=InnoDB;

-- Tabela albumow galerii
CREATE TABLE IF NOT EXISTS gallery_albums (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    slug VARCHAR(200) NOT NULL UNIQUE,
    description TEXT,
    cover_image VARCHAR(255),
    order_index INT DEFAULT 0,
    is_published BOOLEAN DEFAULT TRUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX (slug),
    INDEX (is_published)
) ENGINE=InnoDB;

-- Tabela zdjec galerii
CREATE TABLE IF NOT EXISTS gallery_images (
    id INT AUTO_INCREMENT PRIMARY KEY,
    album_id INT NULL,
    title VARCHAR(200),
    description TEXT,
    image_path VARCHAR(255) NOT NULL,
    thumbnail_path VARCHAR(255),
    webp_path VARCHAR(255),
    webp_thumbnail VARCHAR(255),
    width INT,
    height INT,
    file_size INT,
    order_index INT DEFAULT 0,
    is_published BOOLEAN DEFAULT TRUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (album_id) REFERENCES gallery_albums(id) ON DELETE SET NULL,
    INDEX (album_id),
    INDEX (is_published)
) ENGINE=InnoDB;

-- Tabela wiadomosci kontaktowych
CREATE TABLE IF NOT EXISTS messages (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(100) NOT NULL,
    subject VARCHAR(200),
    message TEXT NOT NULL,
    is_read BOOLEAN DEFAULT FALSE,
    ip_address VARCHAR(45),
    user_agent TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    INDEX (is_read),
    INDEX (created_at)
) ENGINE=InnoDB;

-- Tabela logow aktywnosci
CREATE TABLE IF NOT EXISTS activity_logs (
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_id INT NULL,
    action VARCHAR(100) NOT NULL,
    entity_type VARCHAR(50) NOT NULL,
    entity_id INT NULL,
    details TEXT,
    ip_address VARCHAR(45),
    user_agent TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL,
    INDEX (user_id),
    INDEX (action),
    INDEX (entity_type),
    INDEX (created_at)
) ENGINE=InnoDB;

-- Tabela sesji (opcjonalna)
CREATE TABLE IF NOT EXISTS sessions (
    session_id VARCHAR(255) PRIMARY KEY,
    user_id INT NULL,
    data TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    expires_at DATETIME,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    INDEX (user_id),
    INDEX (expires_at)
) ENGINE=InnoDB;

-- Wstawienie domyslnych danych (opcjonalne)
INSERT INTO settings (setting_key, setting_value) VALUES
    ('site_title', 'Portfolio Pawel Gajda'),
    ('site_description', 'Web Developer & Photographer - Unikalne portfolio z warsztatem'),
    ('site_url', 'https://twoja-domena.pl'),
    ('meta_keywords', 'portfolio, web developer, photographer, frontend, backend'),
    ('meta_author', 'Pawel Gajda'),
    ('theme', 'light'),
    ('maintenance_mode', '0'),
    ('google_analytics', '');

INSERT INTO about (content, skills, achievements) VALUES (
    'Jestem pasjonatem tworzacym unikalne rozwiazania webowe i uchwycujacym chwile w fotografii. Lacze precyzje kodu z artystyczna wrazwloscia.',
    '{"html": 95, "css": 90, "javascript": 85, "php": 80, "mysql": 75, "photography": 85}',
    '["Nagroda za najlepsze portfolio 2024", "Finalista konkursu fotograficznego", "5+ lat doswiadczenia komercyjnego"]'
);

INSERT INTO project_categories (name, slug, description) VALUES
    ('Web Development', 'web-development', 'Projekty webowe i aplikacje'),
    ('Frontend', 'frontend', 'Interfejsy uzytkownika i design'),
    ('Backend', 'backend', 'API, bazy danych i logika biznesowa'),
    ('Full Stack', 'full-stack', 'Pelne rozwiazania od frontu do backendu'),
    ('Photography', 'photography', 'Projekty fotograficzne i sesje zdjeciowe');

INSERT INTO gallery_albums (name, slug, description) VALUES
    ('Portrety', 'portrety', 'Sesje portretowe'),
    ('Pejsaze', 'pejsaze', 'Fotografia krajobrazowa'),
    ('Street', 'street', 'Fotografia uliczna'),
    ('Architektura', 'architektura', 'Budynki i struktury');
```

---

## \ud83d\udd17 Panel Admina

### Dostep
- URL: `https://twoja-domena.pl/admin`
- Login: uzytkownik podany podczas instalacji
- Haslo: haslo podane podczas instalacji

### Funkcjonalnosci

| Sekcja | Opis |
|--------|------|
| **Dashboard** | Statystyki: liczba projektow, zdjec, wiadomosci, aktywnosc |
| **Projekty** | CRUD projektow, drag & drop kolejnosci, status opublikowany/szkic |
| **Galeria** | CRUD albumow i zdjec, upload wielu plikow, automatyczne miniatury |
| **O mnie** | Edycja tresci, umiejetnosci, osiagniec |
| **Wiadomosci** | Skrzynka odbiorcza, oznaczanie jako przeczytane, usuwanie |
| **Ustawienia** | Tytul strony, meta description, linki social, motyw |
| **Uzytkownicy** | Zarzadzanie kontami adminow (dodawanie, edycja, usuwanie) |

### Bezpieczenstwo Panel Admina
- \ud83d\udd12 **Haslowanie** - `password_hash()` i `password_verify()`
- \ud83d\udc8a **Sesje** - HttpOnly, Secure, SameSite cookies
- \ud83d\udc21 **Regeneracja ID** - `session_regenerate_id(true)` po logowaniu
- \u274c **Limit prob** - Maksymalnie 5 prob logowania, blokada na 15 minut
- \ud83d\udc12 **CSRF** - Tokeny we wszystkich formularzach i AJAX

---

## \ud83c\udf06 Uzycie

### Frontend

| Akcja | Opis |
|-------|------|
| **Przeciaganie** | Kliknij i przeciagaj karty projektow / polaroidy |
| **Custom Cursor** | Kursor zmienia sie w zaleznosci od kontekstu |
| **Lightbox** | Kliknij zdjecie, aby otworzyc w powiększeniu |
| **Navigacja** | Strzalki / Swipe na touchscreenie |
| **Dark Mode** | Przelacznik w naglowku strony |

### Skrotow Klawiszowe

| Klawisz | Opis |
|--------|------|
| `ESC` | Zamknij lightbox / modal |
| `\u2190` / `\u2192` | Poprzednie / nastepne zdjecie |
| `T` | Przelacz motyw ciemny/jasny |

---

## \ud83d\udc65 Sekcje Strony

### 1. Hero / Intro
- Animowane narzedzia w tle (mlotek, klucz, pinceta)
- Tytul: "Pawel Gajda"
- Podtytul: "Web Developer + Photographer"
- Call-to-action: "Odkryj moje prace"

### 2. Projekty Webowe
- Karty z miniaturami, tytulami, opisami
- Tagi technologii (HTML, CSS, JS, PHP, etc.)
- Linki: Live Demo, GitHub
- Filtrowanie po kategoriach
- Drag & drop kolejnosci (admin)
- Widok szczegolowy z galeria

### 3. Galeria Zdjec
- Polaroidy z efektem cienia
- Masonry layout
- Kategorie / Albumy
- Lazy loading
- Lightbox z swiped

### 4. O Mnie
- Zdjecie profilowe
- Opis (Markdown support)
- Wykres umiejetnosci (CSS)
- Osiagniecia i certyfikaty

### 5. Kontakt
- Formularz: Imie, Email, Temat, Wiadomosc
- Walidacja po stronie klienta i serwera
- Honeypot + CSRF ochrona przed spamem
- Zapis do bazy i opcjonalnie email
- Potwierdzenie wyslania (AJAX)

### 6. Stopka
- Linki do social media (GitHub, LinkedIn, Instagram)
- Copyright
- Link do panelu admina (tylko dla zalogowanych)

---

## \ud83d\udca6 Bezpieczenstwo

### Zaimplementowane Zabezpieczenia

| Zabezpieczenie | Opis |
|---------------|------|
| **Prepared Statements** | Wszystkie zapytania PDO z prepared statements |
| **CSRF Tokens** | Generowane dla kazdego formularza i AJAX requestu |
| **Input Sanitization** | `filter_var()`, `htmlspecialchars()`, custom sanityzatory |
| **Output Escaping** | `escape_output()` dla wszystkich dynamicznych tresci |
| **Secure Uploads** | MIME checking, losowe nazwy, .htaccess blocking PHP |
| **Session Security** | HttpOnly, Secure, SameSite=Strict cookies |
| **Password Hashing** | `password_hash()` z default algorithm (bcrypt) |
| **Security Headers** | CSP, XSS-Protection, X-Frame-Options, etc. |
| **Rate Limiting** | Login attempt limiting z blokada czasowa |

### .htaccess Security Headers

```apache
# Security Headers
Header always set X-Content-Type-Options "nosniff"
Header always set X-Frame-Options "SAMEORIGIN"
Header always set X-XSS-Protection "1; mode=block"
Header always set Referrer-Policy "strict-origin-when-cross-origin"
Header always set Content-Security-Policy "default-src 'self'; script-src 'self' 'unsafe-inline' cdnjs.cloudflare.com; style-src 'self' 'unsafe-inline' fonts.googleapis.com; img-src 'self' data:; font-src 'self' fonts.gstatic.com; connect-src 'self'; frame-src 'none'; object-src 'none';"
Header always set Strict-Transport-Security "max-age=31536000; includeSubDomains; preload"
Header always set Permissions-Policy "geolocation=(), microphone=(), camera=()"

# Cookies
php_value session.cookie_httponly 1
php_value session.cookie_secure 1
php_value session.cookie_samesite "Strict"
```

---

## \u26a1 SEO & Wydajnosc

### SEO
- \ud83d\udc8d **Semantyczny HTML** - Prawidlowy uzytek tagow (header, main, section, article, etc.)
- \ud83c\udf10 **Meta Tags** - Title, description, keywords, author, viewport
- \ud83d\udcf8 **Open Graph** - og:title, og:description, og:image, og:type
- \ud83d\udce2 **Sitemap** - Automatycznie generowany `sitemap.xml`
- \ud83d\udd17 **Friendly URLs** - `.htaccess` rewrite rules
- \ud83d\udc8a **Canonical URLs** - Zabezpieczenie przed duplicate content

### Wydajnosc
- \ud83d\udce6 **Lazy Loading** - `loading="lazy"` dla obrazow
- \ud83d\udcf9 **Responsive Images** - `srcset` i `sizes` atrybuty
- \ud83c\udf0e **WebP Support** - Automatyczna konwersja i optymalizacja
- \u26a1 **Minification** - CSS i JS minifikowane (opcjonalnie)
- \u26a1 **Caching** - Naglowki Cache-Control
- \u26a1 **Gzip Compression** - Kompresja odpowiedzi

### Lighthouse Score
- \ud83d\udca0 **Cel: 90+**
- Performance: Optymalizowane obrazy, lazy loading
- Accessibility: ARIA labels, focus states, keyboard navigation
- Best Practices: HTTPS, security headers, modern JavaScript
- SEO: Semantic HTML, meta tags, structured data

---

## \u2699 Uprawnienia Plikow

```bash
# Katalog glowne
chmod 755 portfolio-pawel-gajda/

# Pliki PHP
chmod 644 *.php
chmod 644 includes/*.php
chmod 644 admin/*.php
chmod 644 admin/includes/*.php
chmod 644 admin/templates/*.php

# Katalog uploads
chmod 755 uploads/
chmod 755 uploads/projects/
chmod 755 uploads/gallery/
chmod 755 uploads/thumbnails/

# Plik config.php (zabezpieczenie)
chmod 600 config.php

# Katalog install (po instalacji - USUNAC!)
chmod 755 install/
```

---

## \ud83d\udc80 Dostepnosc

### WCAG 2.1 AA Compliance
- \ud83d\udc64 **Keyboard Navigation** - Pelna nawigacja bez myszy
- \ud83d\udc40 **Focus States** - Widoczne focusy dla wszystkich interaktywnych elementow
- \u267f **Color Contrast** - Minimalny kontrast 4.5:1
- \ud83c\udf1f **Reduced Motion** - `prefers-reduced-motion` support
- \ud83d\udc81 **ARIA Labels** - Prawidlowo zdefiniowane role i properties
- \ud83d\udcbb **Skip Links** - Linki do pomijania tresci

---

## \ud83c\udf89 Paleta Kolorow

| Kolor | Kod | Zastosowanie |
|-------|-----|--------------|
| Workshop Wood | `#5C4033` | Tlo, akcenty drewniane |
| Tool Metal | `#A8A8A8` | Elementy metalowe, bordery |
| Highlight Blue | `#4A90E2` | Akcenty, linki, przyciski |
| Paper White | `#F8F4E6` | Tlo jasne, papier |
| Ink Black | `#1A1A1A` | Tekst, tlo ciemne |
| Warning Red | `#D32F2F` | Bledy, ostrzezenia |
| Success Green | `#388E3C` | Sukcesy, potwierdzenia |

---

## \ud83d\udcda Typografia

| Font | Zastosowanie | Google Fonts |
|------|--------------|---------------|
| **Bebas Neue** | Naglowki (h1-h6) | [Link](https://fonts.google.com/specimen/Bebas+Neue) |
| **Roboto Mono** | Tekst glowny, body | [Link](https://fonts.google.com/specimen/Roboto+Mono) |

---

## \ud83d\udc68\u200d\ud83d\udcbb\u200d\ud83d\udc69\u200d\ud83d\udc66 Rozwoj i Wspolpraca

### Jak wspolpracowac
1. Forkuj repozytorium
2. Utworz branch: `git checkout -b feature/nazwa-funkcji`
3. Commituj zmiany: `git commit -m "Dodano: nazwa-funkcji"`
4. Push: `git push origin feature/nazwa-funkcji`
5. Otworz Pull Request

### Standardy Kodowania
- **PHP**: PSR-12
- **HTML**: 2 spaces indentation
- **CSS**: 2 spaces, BEM methodology
- **JavaScript**: 2 spaces, ES6+ syntax

---

## \ud83d\udcc4 Zmiany (Changelog)

| Wersja | Data | Opis |
|--------|------|------|
| 1.0.0 | 2024-XX-XX | Pierwotna wersja - pelne portfolio z panelem admina |

---

## \ud83d\udcd6 Licencja

MIT License - swobodne uzycie, modyfikacja i dystrybucja.

```
MIT License

Copyright (c) 2024 Pawel Gajda

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

## \u2709 Kontakt

**Pawel Gajda**
- \ud83d\udcbb Email: [kontakt@pawelgajda.pl](mailto:kontakt@pawelgajda.pl)
- \ud83d\udc1b GitHub: [https://github.com/PaganLinux](https://github.com/PaganLinux)
- \ud83d\udc50 LinkedIn: [https://linkedin.com/in/pawel-gajda](https://linkedin.com/in/pawel-gajda)
- \ud83c\udfa8 Instagram: [https://instagram.com/pawelgajdaphoto](https://instagram.com/pawelgajdaphoto)

---

## \ud83d\udc81 Podziekowania

- [GSAP](https://greensock.com/gsap/) - Za plynne animacje
- [Lenis](https://lenis.studiofreight.com/) - Za smooth scrolling
- [interact.js](https://interactjs.io/) - Za drag & drop
- [Font Awesome](https://fontawesome.com/) - Za ikony
- [Google Fonts](https://fonts.google.com/) - Za darmowe fonty

---

**\u2764 Dziekuje za uzycie Workshop Table Portfolio!**
