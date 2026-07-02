# 🚀 PaganLinux — Deploy na Contabo VPS (Debian 13)

Kompletna instrukcja: od świeżego Debiana 13 do działającego ekosystemu.
Wszystkie komendy — kopiuj i wklejaj. ⏱️ ~30 minut.

## 📋 Co instalujemy

| Subdomena | Co to | Port |
|-----------|-------|------|
| `paganlinux.eu` | Strona główna (Astro SSR) | 3004 |
| `cms.paganlinux.eu` | Panel admina (React SPA) | nginx static |
| `pagports.paganlinux.eu` | Lista portów | przez nginx → 3004 |
| `packages.paganlinux.eu` | Repozytorium .pag | przez nginx → 3004 |
| `build.paganlinux.eu` | Kolejka buildów | przez nginx → 3004 |
| `api.paganlinux.eu` | Backend API (Rust) | 3000 |
| `git.paganlinux.eu` | Gitea | 3001 |

---

## 🟢 Krok 1: SSH i sprawdzenie systemu

```bash
ssh root@<IP_SERWERA>

cat /etc/debian_version   # powinno: 13.x
df -h                      # sprawdź miejsce
free -h                    # sprawdź RAM
```

---

## 🟢 Krok 2: Aktualizacja i narzędzia

```bash
apt update && apt upgrade -y

apt install -y \
  curl wget git build-essential \
  pkg-config libssl-dev \
  nginx certbot python3-certbot-nginx \
  ufw
```

---

## 🟢 Krok 3: Instalacja Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
rustc --version   # 1.80+
```

---

## 🟢 Krok 4: Instalacja Node.js 22

```bash
curl -fsSL https://deb.nodesource.com/setup_22.x | bash -
apt install -y nodejs
node --version    # 22.x
```

---

## 🟢 Krok 5: DNS (ustaw w panelu domeny)

Wszystkie rekordy A → IP serwera:

```
paganlinux.eu
www.paganlinux.eu
cms.paganlinux.eu
pagports.paganlinux.eu
packages.paganlinux.eu
build.paganlinux.eu
api.paganlinux.eu
git.paganlinux.eu
```

---

## 🟢 Krok 6: Firewall

```bash
ufw default deny incoming
ufw default allow outgoing
ufw allow 22/tcp
ufw allow 80/tcp
ufw allow 443/tcp
ufw --force enable
```

---

## 🟢 Krok 7: Klonowanie repozytorium

```bash
cd /opt
git clone https://github.com/PaganLinux/pag.git
cd pag
```

---

## 🟢 Krok 8: Budowanie backendu API (Rust)

```bash
cd /opt/pag/cms/backend

# Wygeneruj silny secret JWT
export JWT_SECRET=$(openssl rand -hex 64)

# Kompilacja (może potrwać ~5-10 min na VPS)
cargo build --release

# Katalogi
mkdir -p /var/lib/pagancms/data

# Plik .env
cat > /var/lib/pagancms/.env << EOF
JWT_SECRET=$JWT_SECRET
JWT_EXPIRY_HOURS=72
DATABASE_URL=sqlite:///var/lib/pagancms/data/pagancms.db
SERVER_ADDR=127.0.0.1:3000
CORS_ORIGIN=https://cms.paganlinux.eu
GITEA_API_URL=https://git.paganlinux.eu/api/v1
GITEA_TOKEN=
EOF

chmod 600 /var/lib/pagancms/.env

# Systemd service
cat > /etc/systemd/system/pagancms-api.service << 'SYSTEMD'
[Unit]
Description=PaganCMS Backend API
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/opt/pag/cms/backend
EnvironmentFile=/var/lib/pagancms/.env
ExecStart=/opt/pag/cms/backend/target/release/pagan-cms
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
SYSTEMD

systemctl daemon-reload
systemctl enable pagancms-api
systemctl start pagancms-api

# Diagnostyka — sprawdź czy działa
sleep 3
echo "=== Status serwisu ==="
systemctl status pagancms-api --no-pager -l

echo "=== Test API ==="
curl -v http://127.0.0.1:3000/api/v1/stats 2>&1

echo "=== Jeśli nic nie zwraca, sprawdź logi ==="
journalctl -u pagancms-api --no-pager -n 30

# Najczęstsze problemy:
# 1. "Address already in use" →kill $(lsof -t -i:3000)
# 2. "Permission denied" →chown -R root:root /var/lib/pagancms
# 3. "no such table" → API tworzy tabele automatycznie przy starcie
```

---

## 🟢 Krok 9: Budowanie Panelu Admina (React)

```bash
cd /opt/pag/cms/frontend
npm install
npm run build

mkdir -p /var/www/cms
cp -r dist/* /var/www/cms/
```

---

## 🟢 Krok 10: Budowanie Stron (Astro)

```bash
cd /opt/pag/web
npm install
npm run build

# Uruchomienie przez systemd
cat > /etc/systemd/system/paganlinux-web.service << 'SYSTEMD'
[Unit]
Description=PaganLinux Web Server
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/opt/pag/web
Environment="HOST=127.0.0.1"
Environment="PORT=3004"
Environment="PUBLIC_API_URL=http://127.0.0.1:3000/api/v1"
ExecStart=/usr/bin/node /opt/pag/web/dist/server/entry.mjs
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
SYSTEMD

systemctl daemon-reload
systemctl enable paganlinux-web
systemctl start paganlinux-web

# Sprawdź
sleep 2
curl -s http://127.0.0.1:3004/ | head -3
```

---

## 🟢 Krok 11: Worker buildów

```bash
cd /opt/pag/cms/worker
cargo build --release

mkdir -p /var/lib/pagancms/build-workspace

cat > /etc/systemd/system/pagancms-worker.service << 'SYSTEMD'
[Unit]
Description=PaganCMS Build Worker
After=network.target pagancms-api.service

[Service]
Type=simple
User=root
Environment="API_URL=http://127.0.0.1:3000/api/v1"
Environment="BUILD_WORKSPACE=/var/lib/pagancms/build-workspace"
ExecStart=/opt/pag/cms/worker/target/release/pagancms-worker
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
SYSTEMD

systemctl daemon-reload
systemctl enable pagancms-worker
systemctl start pagancms-worker
```

---

## 🟢 Krok 12: Nginx (reverse proxy)

```bash
rm -f /etc/nginx/sites-enabled/default

cat > /etc/nginx/sites-available/paganlinux << 'NGINX'
# ─── paganlinux.eu ────────────────────────────
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

# ─── cms.paganlinux.eu ────────────────────────
server {
    listen 80;
    server_name cms.paganlinux.eu;

    root /var/www/cms;
    index index.html;

    location / {
        try_files $uri $uri/ /index.html;
    }

    location /api/ {
        proxy_pass http://127.0.0.1:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}

# ─── pagports.paganlinux.eu ──────────────────
server {
    listen 80;
    server_name pagports.paganlinux.eu;
    location / {
        proxy_pass http://127.0.0.1:3004;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}

# ─── packages.paganlinux.eu ──────────────────
server {
    listen 80;
    server_name packages.paganlinux.eu;
    location / {
        proxy_pass http://127.0.0.1:3004;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}

# ─── build.paganlinux.eu ─────────────────────
server {
    listen 80;
    server_name build.paganlinux.eu;
    location / {
        proxy_pass http://127.0.0.1:3004;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}

# ─── api.paganlinux.eu ───────────────────────
server {
    listen 80;
    server_name api.paganlinux.eu;
    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}

# ─── git.paganlinux.eu ───────────────────────
server {
    listen 80;
    server_name git.paganlinux.eu;
    location / {
        proxy_pass http://127.0.0.1:3001;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}
NGINX

ln -sf /etc/nginx/sites-available/paganlinux /etc/nginx/sites-enabled/paganlinux
nginx -t && systemctl reload nginx
```

---

## 🟢 Krok 13: SSL (Let's Encrypt)

```bash
certbot --nginx -d paganlinux.eu -d www.paganlinux.eu
certbot --nginx -d cms.paganlinux.eu
certbot --nginx -d pagports.paganlinux.eu
certbot --nginx -d packages.paganlinux.eu
certbot --nginx -d build.paganlinux.eu
certbot --nginx -d api.paganlinux.eu
certbot --nginx -d git.paganlinux.eu

# Auto-renewal
echo "0 3 * * * certbot renew --quiet --post-hook 'systemctl reload nginx'" | crontab -
```

---

## 🟢 Krok 14: Weryfikacja

```bash
systemctl status pagancms-api pagancms-worker paganlinux-web nginx

# Test API
curl -s https://api.paganlinux.eu/api/v1/stats

# Test stron
curl -s https://paganlinux.eu | head -3
curl -s https://cms.paganlinux.eu | head -3
curl -s https://pagports.paganlinux.eu | head -3
```

---

## 🟢 Krok 15: Pierwsze konto admina CMS

```bash
curl -X POST https://api.paganlinux.eu/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "email": "admin@paganlinux.eu",
    "password": "Patryk1991!/"
  }'
```

Teraz otwórz `https://cms.paganlinux.eu` i zaloguj się.

---

## 🛠️ Zarządzanie na co dzień

```bash
# Logi na żywo
journalctl -u pagancms-api -f
journalctl -u paganlinux-web -f
journalctl -u pagancms-worker -f

# Restart
systemctl restart pagancms-api
systemctl restart paganlinux-web
systemctl restart nginx

# Porty — co nasłuchuje
ss -tlnp | grep -E '3000|3004|80|443'

# Backup bazy
cp /var/lib/pagancms/data/pagancms.db /root/backup-$(date +%Y%m%d).db
```

---

## ⚠️ Checklist przed produkcją

- [ ] Zmień `JWT_SECRET` w `/var/lib/pagancms/.env`
- [ ] Zmień hasło admina CMS
- [ ] Dodaj swap: `fallocate -l 2G /swapfile && chmod 600 /swapfile && mkswap /swapfile && swapon /swapfile`
- [ ] Włącz auto-aktualizacje: `apt install unattended-upgrades -y`
- [ ] Skonfiguruj Gitea (jeśli potrzebne): `docker run -d --name gitea -p 3001:3000 -v /var/lib/gitea:/data gitea/gitea:latest`
- [ ] Ustaw limity logów: `journalctl --vacuum-size=500M`
