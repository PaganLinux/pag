# 🚀 PaganLinux — Poradnik Deployu na Nowy VPS Contabo (Ubuntu)

Kompletna instrukcja: od świeżego Ubuntu 24.04 na Contabo do w pełni działającego ekosystemu PaganLinux.
Wszystkie komendy — kopiuj i wklejaj po kolei. Zero zbędnych kroków.

> ⏱️ **Czas instalacji:** ~20 minut (zależy od szybkości VPS)

**Przegląd krok po kroku:**
1. [Logowanie i sprawdzenie](#-krok-1-logowanie-i-pierwsze-sprawdzenie)
2. [Aktualizacja + Rust + Node.js + PM2](#-krok-2-aktualizacja-systemu-i-podstawowe-narzędzia)
3. [DNS + Firewall](#-krok-3-konfiguracja-dns-i-firewalla)
4. [Klonowanie repo + budowanie](#-krok-4-klonowanie-i-budowanie-projektu)
5. [Serwer repozytorium (API)](#-krok-5-uruchomienie-serwera-repozytorium)
6. [Strony WWW (Astro SSR)](#-krok-6-budowanie-i-uruchomienie-stron-www)
7. [Nginx + SSL](#-krok-7-konfiguracja-nginx-reverse-proxy--ssl)
8. [Weryfikacja](#-krok-8-weryfikacja)
9. [Pierwszy pakiet .pag](#-krok-9-dodawanie-pakietów-do-repozytorium)

---

## 📋 Wymagania

| Składnik | Minimum | Zalecane |
|----------|---------|----------|
| Serwer | Contabo VPS S (4 vCPU, 8 GB RAM) | Contabo VPS M/L |
| Dysk | 50 GB SSD | 100+ GB SSD (na pakiety .pag) |
| System | **Ubuntu 24.04 LTS** (świeża instalacja) | Ubuntu 24.04 LTS |
| Domeny | paganlinux.eu, repos.paganlinux.eu, pagports.paganlinux.eu |

---

## 🟢 Krok 1: Logowanie i pierwsze sprawdzenie

```bash
# Zaloguj się przez SSH (hasło jest w mailu od Contabo)
ssh root@<IP_TWOJEGO_SERWERA>

# Przy pierwszym logowaniu Ubuntu poprosi o zmianę hasła — zmień na silne

# Sprawdź czy system jest świeży
lsb_release -a        # powinno: Ubuntu 24.04 LTS
df -h                  # sprawdź miejsce na dysku
free -h                # sprawdź RAM
uname -a               # wersja kernela
```

---

## 🟢 Krok 2: Aktualizacja systemu i podstawowe narzędzia

```bash
# Aktualizacja systemu
apt update && apt upgrade -y

# Niezbędne narzędzia
apt install -y \
  curl wget git build-essential \
  pkg-config libssl-dev \
  gpg gnupg2 \
  bash make cmake \
  nginx certbot python3-certbot-nginx \
  ufw

# === Instalacja Rust (dla pag, pagbuild, repo-server) ===
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# === Instalacja Node.js 22 LTS (dla stron Astro) ===
curl -fsSL https://deb.nodesource.com/setup_22.x | bash -
apt install -y nodejs

# === Instalacja PM2 (process manager dla Node) ===
npm install -g pm2

# === Weryfikacja ===
rustc --version        # powinno: 1.80+
cargo --version
node --version         # powinno: 22.x
npm --version
pm2 --version
```

---

## 🟢 Krok 3: Konfiguracja DNS i Firewalla

### DNS (w panelu Contabo / Cloudflare / gdziekolwiek):

```
A     paganlinux.eu        → <IP twojego serwera>
A     repos.paganlinux.eu  → <IP twojego serwera>
A     pagports.paganlinux.eu → <IP twojego serwera>
```

### Firewall:

```bash
ufw default deny incoming
ufw default allow outgoing
ufw allow 22/tcp    # SSH
ufw allow 80/tcp    # HTTP
ufw allow 443/tcp   # HTTPS
ufw --force enable
ufw status
```

---

## 🟢 Krok 4: Klonowanie i budowanie projektu

```bash
# Sklonuj repozytorium
cd /opt
git clone https://github.com/PaganLinux/pag.git
cd pag

# === 4a. Budowanie menedżera pag ===
cd cli
cargo build --release
cp target/release/pag /usr/local/bin/
chmod +x /usr/local/bin/pag

# Inicjalizacja konfiguracji
mkdir -p /etc/pag
pag init -c /etc/pag/config.toml

cd /opt/pag

# === 4b. Budowanie pagbuild ===
cd pagbuild
cargo build --release
cp target/release/pagbuild /usr/local/bin/
cd /opt/pag

# === 4c. Budowanie repo-servera ===
cd repo-server
cargo build --release
cp target/release/pag-repo /usr/local/bin/
cd /opt/pag

# Weryfikacja
pag --version
pagbuild --version
pag-repo --version
```

---

## 🟢 Krok 5: Uruchomienie serwera repozytorium

```bash
# Katalog na pakiety
mkdir -p /var/lib/pag/repo/{core,extra,community}

# Plik z tokenami API (do uploadu pakietów)
echo "twoj-super-tajny-token-api" > /etc/pag/api-tokens.conf
chmod 600 /etc/pag/api-tokens.conf

# Uruchom przez PM2
cat > /etc/systemd/system/pag-repo.service << 'EOF'
[Unit]
Description=PaganLinux Repository Server
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/opt/pag/repo-server
Environment="PAG_REPO_DIR=/var/lib/pag/repo"
Environment="PAG_REPO_BIND=127.0.0.1:3001"
Environment="PAG_API_TOKENS=/etc/pag/api-tokens.conf"
ExecStart=/usr/local/bin/pag-repo
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
EOF

systemctl daemon-reload
systemctl enable pag-repo
systemctl start pag-repo
systemctl status pag-repo
```

---

## 🟢 Krok 6: Budowanie i uruchomienie stron WWW

```bash
cd /opt/pag/web

# === 6a. paganlinux.eu (port 3001) ===
cd main
npm install
npm run build
cd /opt/pag/web

# === 6b. repos.paganlinux.eu (port 3002) ===
cd repos
npm install
npm run build
cd /opt/pag/web

# === 6c. pagports.paganlinux.eu (port 3003) ===
cd ports
npm install
npm run build
cd /opt/pag/web

# === Uruchom wszystkie przez PM2 ===
pm2 start web/main/dist/server/entry.mjs --name paganlinux-main -- --port 3001
pm2 start web/repos/dist/server/entry.mjs --name paganlinux-repos -- --port 3002
pm2 start web/ports/dist/server/entry.mjs --name paganlinux-ports -- --port 3003

pm2 save
pm2 startup systemd
```

---

## 🟢 Krok 7: Konfiguracja Nginx (reverse proxy + SSL)

```bash
# === paganlinux.eu ===
cat > /etc/nginx/sites-available/paganlinux-main << 'EOF'
server {
    listen 80;
    server_name paganlinux.eu www.paganlinux.eu;

    location / {
        proxy_pass http://127.0.0.1:3001;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;
    }

    # Pliki statyczne (ISO, obrazy)
    location /iso/ {
        alias /var/www/paganlinux/iso/;
        expires 7d;
        add_header Cache-Control "public, immutable";
    }
}
EOF

# === repos.paganlinux.eu ===
cat > /etc/nginx/sites-available/paganlinux-repos << 'EOF'
server {
    listen 80;
    server_name repos.paganlinux.eu;

    # Przekieruj zapytania API bezpośrednio do repo-servera
    location /api/ {
        proxy_pass http://127.0.0.1:3001;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }

    # Pliki .pag serwowane bezpośrednio z dysku
    location /core/ { alias /var/lib/pag/repo/core/; }
    location /extra/ { alias /var/lib/pag/repo/extra/; }
    location /community/ { alias /var/lib/pag/repo/community/; }

    # Strona WWW
    location / {
        proxy_pass http://127.0.0.1:3002;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;
    }
}
EOF

# === pagports.paganlinux.eu ===
cat > /etc/nginx/sites-available/paganlinux-ports << 'EOF'
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
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;
    }
}
EOF

# === Aktywuj strony ===
ln -sf /etc/nginx/sites-available/paganlinux-main /etc/nginx/sites-enabled/
ln -sf /etc/nginx/sites-available/paganlinux-repos /etc/nginx/sites-enabled/
ln -sf /etc/nginx/sites-available/paganlinux-ports /etc/nginx/sites-enabled/

rm -f /etc/nginx/sites-enabled/default
nginx -t
systemctl reload nginx

# === SSL (Let's Encrypt) ===
certbot --nginx -d paganlinux.eu -d www.paganlinux.eu
certbot --nginx -d repos.paganlinux.eu
certbot --nginx -d pagports.paganlinux.eu

# Auto-renewal
echo "0 3 * * * certbot renew --quiet --post-hook 'systemctl reload nginx'" | crontab -
```

---

## 🟢 Krok 8: Weryfikacja

```bash
# Sprawdź czy wszystkie serwisy działają
systemctl status pag-repo
pm2 status

# Test HTTP
curl -s https://paganlinux.eu | head -5
curl -s https://repos.paganlinux.eu/api/v1/stats | python3 -m json.tool
curl -s https://pagports.paganlinux.eu | head -5

# Sprawdź czy menedżer pag działa
pag --version
pag stats
```

---

## 🟢 Krok 9: Dodawanie pakietów do repozytorium

```bash
# Zbuduj pakiet z pagports
cd /opt/pagports/main
pagbuild build 7zip

# Podpisz
pagbuild sign -k TWÓJ_KLUCZ_GPG packages/7zip-*.pag

# Wyślij do repo (token z /etc/pag/api-tokens.conf)
PAGBUILD_API_TOKEN="twoj-super-tajny-token-api" \
  pagbuild upload packages/7zip-*.pag --repo extra --server https://repos.paganlinux.eu
```

---

## 📊 Podsumowanie portów i endpointów

| Serwis | Port | URL |
|--------|------|-----|
| paganlinux.eu (Astro SSR) | 3001 | https://paganlinux.eu |
| repos.paganlinux.eu (Astro SSR) | 3002 | https://repos.paganlinux.eu |
| pagports.paganlinux.eu (Astro SSR) | 3003 | https://pagports.paganlinux.eu |
| repo-server API | 3001 (internal) | https://repos.paganlinux.eu/api/v1/ |
| Nginx | 80, 443 | Reverse proxy + SSL |

---

## 🔧 Przydatne komendy

```bash
# Logi
pm2 logs                          # wszystkie logi
pm2 logs paganlinux-main          # konkretny serwis
journalctl -u pag-repo -f         # logi repo-servera
tail -f /var/log/nginx/access.log # logi nginx

# Restart
pm2 restart all                   # restart stron
systemctl restart pag-repo        # restart repo API
systemctl reload nginx            # przeładuj konfig nginx

# Aktualizacja kodu
cd /opt/pag && git pull
cd cli && cargo build --release && cp target/release/pag /usr/local/bin/
cd ../pagbuild && cargo build --release && cp target/release/pagbuild /usr/local/bin/
cd ../repo-server && cargo build --release && cp target/release/pag-repo /usr/local/bin/
systemctl restart pag-repo

# Aktualizacja stron
cd /opt/pag/web/main && npm install && npm run build
cd /opt/pag/web/repos && npm install && npm run build
cd /opt/pag/web/ports && npm install && npm run build
pm2 restart all
```

---

PaganLinux © 2026 — Gotowe do produkcji! 🚀
