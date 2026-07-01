# 🚀 PaganOS — Kompletny Tutorial: Od Zera do Produkcji na Contabo VPS

> **Ostatnia aktualizacja:** 2026-07-01  
> **Wersja systemu:** Ubuntu 24.04 LTS  
> **Czas instalacji:** ~30-45 minut  

---

## 📋 Spis treści

1. [Wymagania i przygotowanie](#1-wymagania-i-przygotowanie)
2. [Zakup i konfiguracja Contabo VPS](#2-zakup-i-konfiguracja-contabo-vps)
3. [DNS — konfiguracja domen](#3-dns--konfiguracja-domen)
4. [Logowanie i aktualizacja systemu](#4-logowanie-i-aktualizacja-systemu)
5. [Instalacja narzędzi: Rust, Node.js, PM2](#5-instalacja-narzędzi)
6. [Klonowanie repozytorium i budowa](#6-klonowanie-repozytorium-i-budowa)
7. [Uruchomienie serwisu repo-server](#7-uruchomienie-serwisu-repo-server)
8. [Uruchomienie stron WWW (Astro)](#8-uruchomienie-stron-www)
9. [Uruchomienie CMS (Panel Admina)](#9-uruchomienie-cms)
10. [Nginx Reverse Proxy + SSL](#10-nginx-reverse-proxy--ssl)
11. [Firewall i bezpieczeństwo](#11-firewall-i-bezpieczeństwo)
12. [Forgejo/Gitea — integracja z CMS](#12-forgejogitea--integracja-z-cms)
13. [Pierwszy pakiet .pag](#13-pierwszy-pakiet-pag)
14. [Aktualizacja systemu](#14-aktualizacja-systemu)
15. [Rozwiązywanie problemów](#15-rozwiązywanie-problemów)
16. [Podsumowanie portów](#16-podsumowanie-portów)

---

## 1. Wymagania i przygotowanie

| Składnik | Minimum | Zalecane |
|----------|---------|----------|
| Serwer | Contabo VPS S (4 vCPU, 8 GB RAM) | VPS M (6 vCPU, 16 GB) |
| Dysk | 50 GB SSD | 100+ GB SSD |
| System | Ubuntu 24.04 LTS | Ubuntu 24.04 LTS |
| Domeny | 5 subdomen | Wskazane na VPS |

### Lista wymaganych domen:

```
paganlinux.eu         → Strona główna + download
repos.paganlinux.eu   → Repozytorium pakietów .pag
pagports.paganlinux.eu → Drzewo portów online
cms.paganlinux.eu     → Panel administracyjny CMS
git.paganlinux.eu     → Forgejo/Gitea (instalowane osobno)
```

---

## 2. Zakup i konfiguracja Contabo VPS

1. Wejdź na [contabo.com](https://contabo.com) → **VPS**
2. Wybierz **VPS S** (lub większy) z **Ubuntu 24.04**
3. Po zakupie otrzymasz mail z:
   - IP serwera (np. `123.45.67.89`)
   - Hasło root
4. Zapisz dane w bezpiecznym miejscu

> 💡 **Tip:** Contabo wysyła dane logowania w ciągu kilku godzin od zakupu (nie natychmiast).

---

## 3. DNS — konfiguracja domen

### Opcja A: Cloudflare (zalecana — darmowa)

1. Dodaj domenę `paganlinux.eu` do Cloudflare
2. Ustaw nameservery Cloudflare u swojego rejestratora
3. W panelu DNS dodaj rekordy:

| Typ | Nazwa | Wartość | TTL |
|-----|-------|---------|-----|
| A | `@` | `<IP_VPS>` | Auto |
| A | `repos` | `<IP_VPS>` | Auto |
| A | `pagports` | `<IP_VPS>` | Auto |
| A | `cms` | `<IP_VPS>` | Auto |
| A | `git` | `<IP_VPS>` | Auto |

4. SSL/TLS → ustaw **Full (strict)**
5. Poczekaj na propagację DNS (5-60 minut)

### Opcja B: Panel Contabo

W panelu Contabo → DNS → dodaj te same rekordy A.

---

## 4. Logowanie i aktualizacja systemu

```bash
# Zaloguj się przez SSH
ssh root@<IP_TWOJEGO_SERWERA>

# Ubuntu poprosi o zmianę hasła przy pierwszym logowaniu — zmień na silne!

# Sprawdź, czy wszystko jest OK
lsb_release -a        # Ubuntu 24.04 LTS
df -h                  # miejsce na dysku
free -h                # RAM
uname -a               # kernel

# Aktualizacja systemu
apt update && apt upgrade -y

# Ustaw strefę czasową
timedatectl set-timezone Europe/Warsaw

# Utwórz strukturę katalogów
mkdir -p /opt/pagan-os
mkdir -p /var/www
mkdir -p /var/pagan-os/{stage3-base,build-space,iso-builder}
mkdir -p /var/lib/pag/repo/{core,extra,community}
mkdir -p /etc/pag
```

---

## 5. Instalacja narzędzi

```bash
# ─── Podstawowe narzędzia ─────────────────────────────
apt install -y \
  curl wget git build-essential \
  pkg-config libssl-dev \
  gpg gnupg2 nginx certbot python3-certbot-nginx \
  ufw apache2-utils bash make cmake

# ─── Rust (dla pag, pagbuild, repo-server, cms-server) ─
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# ─── Node.js 22 LTS (dla stron Astro) ─────────────────
curl -fsSL https://deb.nodesource.com/setup_22.x | bash -
apt install -y nodejs

# ─── PM2 (process manager dla Node) ───────────────────
npm install -g pm2

# ─── Weryfikacja ──────────────────────────────────────
rustc --version    # ≥ 1.80
cargo --version
node --version     # ≥ 22.x
npm --version
pm2 --version
```

---

## 6. Klonowanie repozytorium i budowa

```bash
cd /opt
git clone https://github.com/PaganLinux/pag.git
cd pag

# ─── 6a. Budowa CLI pag ──────────────────────────────
cd cli
cargo build --release
cp target/release/pag /usr/local/bin/
cd /opt/pag

# ─── 6b. Budowa pagbuild ─────────────────────────────
cd pagbuild
cargo build --release
cp target/release/pagbuild /usr/local/bin/
cd /opt/pag

# ─── 6c. Budowa repo-server ──────────────────────────
cd repo-server
cargo build --release
cp target/release/pag-repo /usr/local/bin/
cd /opt/pag

# ─── 6d. Budowa CMS backend (NOWE!) ──────────────────
cd cms-server
cargo build --release
cp target/release/pag-cms /usr/local/bin/
cd /opt/pag

# Weryfikacja
pag --version       # (jeśli zaimplementowane)
pagbuild --version
pag-repo --version
pag-cms --version
```

---

## 7. Uruchomienie serwisu repo-server

```bash
# Skopiuj konfigurację
cp /opt/pag/cms-server/cms.example.toml /etc/pag/cms.toml
# (repo-server nie ma własnego config.toml — używa ENV)

# Utwórz plik z tokenami API
cat > /etc/pag/api-tokens.conf << 'EOF'
# PaganLinux API Tokens
# Jeden token na linię. Linie zaczynające się od # to komentarze.
2137papiezpolak
EOF
chmod 600 /etc/pag/api-tokens.conf

# ─── Systemd service dla repo-server ─────────────────
cat > /etc/systemd/system/pag-repo.service << 'EOF'
[Unit]
Description=PaganLinux Repository Server
After=network.target

[Service]
Type=simple
User=root
ExecStart=/usr/local/bin/pag-repo
Restart=always
RestartSec=5
Environment=PAG_REPO_DIR=/var/lib/pag/repo
Environment=PAG_REPO_BIND=0.0.0.0:3001
Environment=PAG_API_TOKENS=/etc/pag/api-tokens.conf
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
EOF

systemctl daemon-reload
systemctl enable pag-repo --now
systemctl status pag-repo

# Test
curl http://localhost:3001/api/v1/index.json
```

---

## 8. Uruchomienie stron WWW

```bash
cd /opt/pag/web

# ─── 8a. Strona główna (paganlinux.eu) ───────────────
cd main
npm install
npm run build
cd /opt/pag/web

# ─── 8b. Repozytorium (repos.paganlinux.eu) ───────────
cd repos
npm install
npm run build
cd /opt/pag/web

# ─── 8c. Porty (pagports.paganlinux.eu) ───────────────
cd ports
npm install
npm run build
cd /opt/pag/web

# ─── Uruchom przez PM2 ────────────────────────────────
pm2 start main/dist/server/entry.mjs    --name pagan-main   -- --port 3004
pm2 start repos/dist/server/entry.mjs   --name pagan-repos  -- --port 3002
pm2 start ports/dist/server/entry.mjs   --name pagan-ports  -- --port 3003

pm2 save
pm2 startup systemd
# (wykonaj komendę wyświetloną przez PM2)
```

---

## 9. Uruchomienie CMS (Panel Admina)

To jest **NOWY** komponent — panel administracyjny do zarządzania pakietami, buildami i zgłoszeniami społeczności.

### 9a. Backend CMS (Rust)

```bash
# Konfiguracja CMS
cp /opt/pag/cms-server/cms.example.toml /etc/pag/cms.toml

# EDYTUJ /etc/pag/cms.toml — dostosuj do swojego środowiska:
nano /etc/pag/cms.toml
```

**Minimalna konfiguracja w `/etc/pag/cms.toml`:**

```toml
[server]
bind = "0.0.0.0"
port = 3005
cors_origins = ["http://localhost:3006", "https://cms.paganlinux.eu"]

[database]
path = "/opt/pagan-cms/cms.db"

[build]
stage3_path = "/var/pagan-os/stage3-base"
build_space = "/var/pagan-os/build-space"
max_concurrent_builds = 2
build_timeout_seconds = 7200

[forgejo]
base_url = "https://git.paganlinux.eu"
api_token = ""           # ← wypełnij później
community_repo = "pagan-community"
webhook_secret = ""      # ← wypełnij później

[auth]
session_secret = "wygeneruj-losowy-ciag-znakow-min-32-bajty"
admin_username = "admin"
token_expiry_days = 7
```

```bash
# Utwórz katalogi na dane CMS
mkdir -p /opt/pagan-cms
mkdir -p /var/pagan-os/build-space

# ─── Systemd service dla CMS backend ──────────────────
cat > /etc/systemd/system/pag-cms.service << 'EOF'
[Unit]
Description=PaganOS CMS Backend
After=network.target

[Service]
Type=simple
User=root
ExecStart=/usr/local/bin/pag-cms
Restart=always
RestartSec=5
Environment=RUST_LOG=info,pag_cms=debug
Environment=PAG_CMS_ADMIN_PASSWORD=Patryk1991!/
Environment=PAG_CMS_CONFIG=/etc/pag/cms.toml

[Install]
WantedBy=multi-user.target
EOF

systemctl daemon-reload
systemctl enable pag-cms --now
systemctl status pag-cms

# Test — powinien zwrócić {"status":"ok"}
curl http://localhost:3005/api/v1/health
```

### 9b. Frontend CMS (Astro)

```bash
cd /opt/pag/web/cms
npm install
npm run build

pm2 start dist/server/entry.mjs --name pagan-cms -- --port 3006
pm2 save
```

### 9c. Pierwsze logowanie do CMS

1. Otwórz `https://cms.paganlinux.eu` (po skonfigurowaniu Nginx — krok 10)
2. Lub lokalnie: `http://<IP>:3006`
3. Login: `admin`
4. Hasło: to ustawione w `PAG_CMS_ADMIN_PASSWORD`

> ⚠️ **Natychmiast zmień hasło!** Domyślne hasło jest słabe.

---

## 10. Nginx Reverse Proxy + SSL

```bash
# ─── 10a. Strona główna ──────────────────────────────
cat > /etc/nginx/sites-available/paganlinux.eu << 'EOF'
server {
    listen 80;
    server_name paganlinux.eu www.paganlinux.eu;

    location / {
        proxy_pass http://127.0.0.1:3004;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
EOF

ln -s /etc/nginx/sites-available/paganlinux.eu /etc/nginx/sites-enabled/

# ─── 10b. Repozytorium ────────────────────────────────
cat > /etc/nginx/sites-available/repos.paganlinux.eu << 'EOF'
server {
    listen 80;
    server_name repos.paganlinux.eu;

    location / {
        proxy_pass http://127.0.0.1:3002;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }

    # API repo-servera
    location /api/v1/ {
        proxy_pass http://127.0.0.1:3001;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }

    # Statyczne pliki .pag
    location /core/ {
        alias /var/lib/pag/repo/core/;
        expires 7d;
    }
    location /extra/ {
        alias /var/lib/pag/repo/extra/;
        expires 7d;
    }
    location /community/ {
        alias /var/lib/pag/repo/community/;
        expires 7d;
    }
}
EOF

ln -s /etc/nginx/sites-available/repos.paganlinux.eu /etc/nginx/sites-enabled/

# ─── 10c. Porty online ────────────────────────────────
cat > /etc/nginx/sites-available/pagports.paganlinux.eu << 'EOF'
server {
    listen 80;
    server_name pagports.paganlinux.eu;

    location / {
        proxy_pass http://127.0.0.1:3003;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}
EOF

ln -s /etc/nginx/sites-available/pagports.paganlinux.eu /etc/nginx/sites-enabled/

# ─── 10d. CMS Panel ───────────────────────────────────
cat > /etc/nginx/sites-available/cms.paganlinux.eu << 'EOF'
server {
    listen 80;
    server_name cms.paganlinux.eu;

    # Dodatkowa warstwa HTTP Basic Auth
    auth_basic "PaganOS CMS";
    auth_basic_user_file /etc/nginx/.htpasswd-cms;

    # Frontend Astro CMS
    location / {
        proxy_pass http://127.0.0.1:3006;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # Backend API CMS
    location /api/v1/ {
        proxy_pass http://127.0.0.1:3005;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }

    # WebSocket — live logi buildów
    location /api/v1/builds/ {
        proxy_pass http://127.0.0.1:3005;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_read_timeout 86400s;  # 24h — buildy mogą trwać długo
    }
}
EOF

ln -s /etc/nginx/sites-available/cms.paganlinux.eu /etc/nginx/sites-enabled/

# ─── Utwórz hasło HTTP Basic Auth dla CMS ─────────────
htpasswd -c /etc/nginx/.htpasswd-cms admin
# (podaj silne hasło — to dodatkowa warstwa przed loginem CMS)

# ─── Aktywuj Nginx ────────────────────────────────────
nginx -t                    # sprawdź składnię
systemctl reload nginx      # załaduj konfigurację

# ─── SSL przez Let's Encrypt ──────────────────────────
certbot --nginx -d paganlinux.eu -d www.paganlinux.eu
certbot --nginx -d repos.paganlinux.eu
certbot --nginx -d pagports.paganlinux.eu
certbot --nginx -d cms.paganlinux.eu

# Automatyczne odnowienie certyfikatów (sprawdź cron)
systemctl status certbot.timer
```

---

## 11. Firewall i bezpieczeństwo

```bash
# ─── Podstawowa konfiguracja UFW ──────────────────────
ufw default deny incoming
ufw default allow outgoing
ufw allow 22/tcp        # SSH
ufw allow 80/tcp        # HTTP
ufw allow 443/tcp       # HTTPS
ufw --force enable
ufw status verbose

# ─── Fail2ban (ochrona przed brute-force) ─────────────
apt install -y fail2ban

cat > /etc/fail2ban/jail.local << 'EOF'
[sshd]
enabled = true
port = 22
maxretry = 5
bantime = 3600

[nginx-http-auth]
enabled = true
maxretry = 5
bantime = 3600
EOF

systemctl enable fail2ban --now

# ─── Wyłącz logowanie root przez hasło ────────────────
# (najpierw skonfiguruj klucze SSH!)
# sed -i 's/PermitRootLogin yes/PermitRootLogin prohibit-password/' /etc/ssh/sshd_config
# sed -i 's/#PasswordAuthentication yes/PasswordAuthentication no/' /etc/ssh/sshd_config
# systemctl reload sshd
```

---

## 12. Forgejo/Gitea — integracja z CMS

### Instalacja Forgejo

```bash
# Pobierz i zainstaluj Forgejo (lekka alternatywa dla Gitea)
cd /opt
wget https://codeberg.org/forgejo/forgejo/releases/download/v10.0.0/forgejo-10.0.0-linux-amd64
chmod +x forgejo-10.0.0-linux-amd64

# Utwórz użytkownika
adduser --system --group --disabled-password --home /opt/forgejo forgejo

# Systemd service
cat > /etc/systemd/system/forgejo.service << 'EOF'
[Unit]
Description=Forgejo (Git)
After=network.target

[Service]
Type=simple
User=forgejo
Group=forgejo
WorkingDirectory=/opt/forgejo
ExecStart=/opt/forgejo-10.0.0-linux-amd64 web --config /opt/forgejo/custom/conf/app.ini
Restart=always
Environment=USER=forgejo
Environment=HOME=/opt/forgejo
Environment=GITEA_WORK_DIR=/opt/forgejo

[Install]
WantedBy=multi-user.target
EOF

# Konfiguracja Nginx dla Forgejo
cat > /etc/nginx/sites-available/git.paganlinux.eu << 'EOF'
server {
    listen 80;
    server_name git.paganlinux.eu;

    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}
EOF

ln -s /etc/nginx/sites-available/git.paganlinux.eu /etc/nginx/sites-enabled/
nginx -t && systemctl reload nginx
certbot --nginx -d git.paganlinux.eu

systemctl daemon-reload
systemctl enable forgejo --now
```

### Konfiguracja webhooka Forgejo → CMS

1. Otwórz `https://git.paganlinux.eu` i dokończ instalację Forgejo
2. Utwórz repozytorium `pagan-community`
3. Wygeneruj API token: Ustawienia → Aplikacje → Generate Token
4. Wklej token do `/etc/pag/cms.toml` w sekcji `[forgejo]`:
   ```toml
   api_token = "wygenerowany-token-forgejo"
   webhook_secret = "twoj-losowy-webhook-secret"
   ```
5. W repozytorium `pagan-community`:
   - Ustawienia → Webhooks → Add Webhook (Forgejo)
   - URL: `https://cms.paganlinux.eu/api/v1/hooks/forgejo`
   - Content Type: `application/json`
   - Secret: `twoj-losowy-webhook-secret`
   - Events: **Pull Request — Opened**
6. Zrestartuj CMS backend:
   ```bash
   systemctl restart pag-cms
   ```

### Test integracji — kompletny scenariusz

Aby przetestować cały przepływ: zgłoszenie PR → CMS → zatwierdzenie → build:

#### Krok 1: Sklonuj repozytorium społeczności i przygotuj testowe zgłoszenie

```bash
# Sklonuj puste repozytorium
git clone https://git.paganlinux.eu/MijagiKutasamoto/pagan-community.git
cd pagan-community

# Utwórz przykładowy skrypt PaganBuild
cat > nginx.pagbuild << 'EOF'
name="nginx"
version="1.27.0"
release=1
description="High-performance HTTP server and reverse proxy"
license="BSD-2-Clause"
arch="x86_64"
homepage="https://nginx.org"

source="https://nginx.org/download/nginx-1.27.0.tar.gz"

depends=("pcre2" "openssl" "zlib")

build() {
    cd "$srcdir/nginx-${version}"
    ./configure \
        --prefix=/usr \
        --sbin-path=/usr/bin/nginx \
        --conf-path=/etc/nginx/nginx.conf \
        --with-http_ssl_module \
        --with-http_v2_module
    make -j$(nproc)
}

package() {
    cd "$srcdir/nginx-${version}"
    make DESTDIR="$pkgdir" install
}
EOF

# Dodaj, commit i push
git checkout -b add-nginx
git add nginx.pagbuild
git commit -m "Add nginx: 1.27.0 — High-performance HTTP server"
git push -u origin add-nginx
```

#### Krok 2: Utwórz Pull Request

1. Otwórz `https://git.paganlinux.eu/MijagiKutasamoto/pagan-community`
2. Kliknij **"New Pull Request"**
3. Wybierz gałąź `add-nginx` → `main`
4. Tytuł: `nginx: 1.27.0`
5. Opis:
   ```
   Nginx 1.27.0 — high-performance HTTP server and reverse proxy.

   ```pagbuild
   name="nginx"
   version="1.27.0"
   release=1
   description="High-performance HTTP server and reverse proxy"
   license="BSD-2-Clause"
   arch="x86_64"
   homepage="https://nginx.org"

   source="https://nginx.org/download/nginx-1.27.0.tar.gz"
   depends=("pcre2" "openssl" "zlib")

   build() {
       cd "$srcdir/nginx-${version}"
       ./configure --prefix=/usr --with-http_ssl_module --with-http_v2_module
       make -j$(nproc)
   }

   package() {
       cd "$srcdir/nginx-${version}"
       make DESTDIR="$pkgdir" install
   }
   ```
   ```
6. Kliknij **"Create Pull Request"**

#### Krok 3: Sprawdź CMS

1. Otwórz `https://cms.paganlinux.eu`
2. Przejdź do zakładki **Submissions**
3. Powinieneś zobaczyć nowe zgłoszenie `nginx` ze statusem **pending**
4. Kliknij **"Szczegóły"** aby zobaczyć pełny skrypt budujący
5. Jeśli wszystko wygląda dobrze, kliknij **"✅ Zatwierdź i kompiluj"**

#### Krok 4: Monitoruj build

1. Przejdź do zakładki **Builds**
2. Kliknij na nowy build — zobaczysz **live terminal** z logami kompilacji
3. Po zakończeniu build powinien zmienić status na **completed**
4. Status zgłoszenia automatycznie zmieni się na **published**

#### Krok 5: Weryfikacja artefaktu

```bash
# Sprawdź, czy paczka .pag została wygenerowana
ls -la /var/lib/pag/repo/core/nginx-*.pag

# Sprawdź w API repo
curl https://repos.paganlinux.eu/api/v1/packages/nginx | jq
```

#### Szybki test — nowe repo od zera

Jeśli chcesz przetestować integrację z zupełnie nowym kontem:

```bash
# Na swoim komputerze:
mkdir test-pagan-pkg && cd test-pagan-pkg
touch README.md
git init
git checkout -b main
git add README.md
git commit -m "first commit"
git remote add origin https://git.paganlinux.eu/MijagiKutasamoto/pagan-community.git
git push -u origin main
```

Następnie wykonaj Kroki 1-5 powyżej.

---

## 13. Pierwszy pakiet .pag

### 13a. Struktura katalogów z przykładami

```bash
# Skopiuj przykłady portów
cp -r /opt/pag/examples/pagports/* /var/pagan-os/ports/
```

### 13b. Przykładowy PaganBuild skrypt

```bash
cat > /tmp/hello.pagbuild << 'EOF'
name="hello-world"
version="1.0.0"
release=1
description="Test package for PaganOS"
license="MIT"
arch="x86_64"

source="https://example.com/hello-1.0.0.tar.gz"
sha512="SKIP"  # pomiń weryfikację dla testów

build() {
    echo "Building hello-world..."
    mkdir -p "$pkgdir/usr/bin"
    echo '#!/bin/bash' > "$pkgdir/usr/bin/hello"
    echo 'echo "Hello from PaganOS!"' >> "$pkgdir/usr/bin/hello"
    chmod +x "$pkgdir/usr/bin/hello"
}
EOF

# Zbuduj pakiet
pagbuild build /tmp/hello.pagbuild

# Podpisz (jeśli masz klucz GPG)
# pagbuild sign packages/hello-world-1.0.0-1-x86_64.pag

# Wyślij do repo
PAGBUILD_API_TOKEN="twoj-super-tajny-token-do-uploadu-pakietow" \
  pagbuild upload packages/hello-world-*.pag --repo extra --server https://repos.paganlinux.eu
```

---

## 14. Aktualizacja systemu

### 14a. Aktualizacja wszystkich komponentów (jedno polecenie)

```bash
#!/bin/bash
# Zapisz jako /opt/pag/update-all.sh i nadaj chmod +x

set -e
cd /opt/pag
echo "⬇️  Pobieranie najnowszych zmian..."
git pull origin main

echo "🦀 Budowanie komponentów Rust..."
cd cli && cargo build --release && cp target/release/pag /usr/local/bin/ && cd ..
cd pagbuild && cargo build --release && cp target/release/pagbuild /usr/local/bin/ && cd ..
cd repo-server && cargo build --release && cp target/release/pag-repo /usr/local/bin/ && cd ..
cd cms-server && cargo build --release && cp target/release/pag-cms /usr/local/bin/ && cd ..

echo "📦 Budowanie frontendów Astro..."
cd web/main && npm install && npm run build && cd ../..
cd web/repos && npm install && npm run build && cd ../..
cd web/ports && npm install && npm run build && cd ../..
cd web/cms && npm install && npm run build && cd ../..

echo "🔄 Restart serwisów..."
systemctl restart pag-repo
systemctl restart pag-cms
pm2 restart all

echo "✅ Aktualizacja zakończona!"
```

```bash
chmod +x /opt/pag/update-all.sh
```

### 14b. Aktualizacja pojedynczego komponentu

```bash
# Tylko CMS
cd /opt/pag
git pull
cd cms-server && cargo build --release && cp target/release/pag-cms /usr/local/bin/
cd ../web/cms && npm install && npm run build
systemctl restart pag-cms
pm2 restart pagan-cms

# Tylko CLI
cd /opt/pag/cli && cargo build --release && cp target/release/pag /usr/local/bin/

# Tylko strony WWW
cd /opt/pag/web/main && npm install && npm run build
pm2 restart pagan-main
```

---

## 15. Rozwiązywanie problemów

### Logi i diagnostyka

```bash
# ─── Logi systemowe ───────────────────────────────────
journalctl -u pag-repo -f        # repo-server (Rust)
journalctl -u pag-cms -f         # CMS backend (Rust)
journalctl -u forgejo -f         # Forgejo (jeśli używane)

# ─── Logi PM2 (strony WWW) ────────────────────────────
pm2 logs                          # wszystkie
pm2 logs pagan-main               # strona główna
pm2 logs pagan-cms                # frontend CMS

# ─── Logi Nginx ───────────────────────────────────────
tail -f /var/log/nginx/access.log
tail -f /var/log/nginx/error.log

# ─── Testy endpointów ─────────────────────────────────
curl http://localhost:3001/api/v1/health    # repo-server
curl http://localhost:3005/api/v1/health    # cms-server
curl http://localhost:3004/                 # strona główna
curl http://localhost:3006/                 # CMS
```

### Częste problemy

| Problem | Rozwiązanie |
|---------|-------------|
| **502 Bad Gateway** | Sprawdź, czy serwis działa: `systemctl status pag-cms` |
| **CMS nie ładuje się** | Sprawdź port: `ss -tlnp \| grep 3005` |
| **Brak SSL** | Uruchom ponownie certbot: `certbot --nginx -d cms.paganlinux.eu` |
| **Buildy nie startują** | Sprawdź `journalctl -u pag-cms -f` |
| **WebSocket nie działa** | Sprawdź konfigurację Nginx dla `/api/v1/builds/` |
| **Port zajęty** | `lsof -i :3005` i `kill -9 <PID>` |
| **Brak miejsca** | `df -h` — wyczyść `/var/pagan-os/build-space/` |

### Restart wszystkich serwisów

```bash
systemctl restart pag-repo pag-cms
pm2 restart all
systemctl reload nginx
```

---

## 16. Podsumowanie portów

| Serwis | Port | URL | Technologia |
|--------|------|-----|-------------|
| Strona główna | 3004 | `https://paganlinux.eu` | Astro SSR |
| Repozytorium | 3002 | `https://repos.paganlinux.eu` | Astro SSR |
| Porty online | 3003 | `https://pagports.paganlinux.eu` | Astro SSR |
| **CMS Frontend** | **3006** | **`https://cms.paganlinux.eu`** | **Astro SSR** |
| Repo API | 3001 | wewnętrzny (przez Nginx) | Rust/Axum |
| **CMS API** | **3005** | wewnętrzny (przez Nginx) | **Rust/Axum** |
| Forgejo | 3000 | `https://git.paganlinux.eu` | Go |
| Nginx | 80, 443 | reverse proxy | — |

### Architektura sieciowa

```
Internet
  │
  ▼
[Nginx :80/:443] ─── SSL (Let's Encrypt)
  │
  ├─ paganlinux.eu ───────► :3004 (Astro SSR — strona główna)
  ├─ repos.paganlinux.eu ─► :3002 (Astro SSR) + :3001 (Rust API)
  ├─ pagports.pag... ─────► :3003 (Astro SSR)
  ├─ cms.paganlinux.eu ───► :3006 (Astro SSR) + :3005 (Rust CMS API)
  └─ git.paganlinux.eu ───► :3000 (Forgejo)
```

---

## ✅ Checklista po deployu

- [ ] Wszystkie 5 domen działa z HTTPS (zielona kłódka)
- [ ] `curl https://paganlinux.eu` zwraca 200
- [ ] `curl https://repos.paganlinux.eu/api/v1/index.json` zwraca JSON
- [ ] `curl https://cms.paganlinux.eu/api/v1/health` → `{"status":"ok"}`
- [ ] CMS: można się zalogować na `https://cms.paganlinux.eu`
- [ ] Dashboard CMS pokazuje statystyki
- [ ] Webhook Forgejo → CMS skonfigurowany
- [ ] Firewall UFW aktywny (`ufw status`)
- [ ] Fail2ban działa (`systemctl status fail2ban`)
- [ ] PM2 zapisany na starcie (`pm2 save` + `pm2 startup`)
- [ ] Wszystkie serwisy systemd enabled (`systemctl list-unit-files | grep pag-`)
- [ ] Automatyczne odnawianie SSL (`systemctl status certbot.timer`)

---

## 🔗 Przydatne linki

| Zasób | URL |
|-------|-----|
| Główne repo | https://github.com/PaganLinux/pag |
| CMS Frontend | https://cms.paganlinux.eu |
| Dokumentacja | https://paganlinux.eu/docs |
| Porty online | https://pagports.paganlinux.eu |
| Repozytorium | https://repos.paganlinux.eu |
| Git społeczności | https://git.paganlinux.eu |

---

**PaganOS © 2026 — Gotowe do produkcji! 🚀🐧**
