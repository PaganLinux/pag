# 🚀 PaganLinux — Deploy na Contabo VPS (Debian 13)

Kompletna instrukcja: od czystego Debiana 13 do działającego ekosystemu PaganLinux.
Wszystkie komendy kopiuj-wklejaj. ⏱️ ~30 minut.

---

## 📋 Architektura

| Subdomena | Co | Jak |
|-----------|-----|-----|
| `paganlinux.eu` | Strona główna | Astro SSR → port 3004 |
| `cms.paganlinux.eu` | Panel admina | React SPA → nginx static |
| `pagports.paganlinux.eu` | Lista portów PAGBUILD | nginx → Astro |
| `packages.paganlinux.eu` | Repozytorium .pag | nginx → Astro |
| `build.paganlinux.eu` | Kolejka buildów | nginx → Astro |
| `api.paganlinux.eu` | Backend API | Rust → port 3000 |
| `git.paganlinux.eu` | Gitea (opcjonalnie) | Docker → port 3001 |

---

## 🟢 Krok 1: SSH

```bash
ssh root@<IP_SERWERA>
cat /etc/debian_version   # 13.x
df -h && free -h           # sprawdź zasoby
```

---

## 🟢 Krok 2: Pakiety systemowe

```bash
apt update && apt upgrade -y
apt install -y curl wget git build-essential pkg-config libssl-dev nginx ufw
```

---

## 🟢 Krok 3: Rust + Node.js

```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# Node.js 22
curl -fsSL https://deb.nodesource.com/setup_22.x | bash -
apt install -y nodejs

# Weryfikacja
rustc --version && node --version
```

---

## 🟢 Krok 4: DNS

W panelu domeny ustaw rekordy A → IP serwera:

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

## 🟢 Krok 5: Firewall

```bash
ufw default deny incoming
ufw default allow outgoing
ufw allow 22/tcp
ufw allow 80/tcp
ufw allow 443/tcp
ufw --force enable
```

---

## 🟢 Krok 6: Klonowanie

```bash
cd /opt
git clone https://github.com/PaganLinux/pag.git
cd pag
```

---

## 🟢 Krok 7: Backend API (Rust)

```bash
# Katalog na dane
mkdir -p /var/lib/pagancms
export JWT_SECRET=$(openssl rand -hex 64)

# Plik .env
cat > /var/lib/pagancms/.env << EOF
JWT_SECRET=$JWT_SECRET
JWT_EXPIRY_HOURS=72
DATABASE_URL=sqlite:///var/lib/pagancms/pagancms.db
SERVER_ADDR=127.0.0.1:3000
CORS_ORIGIN=https://cms.paganlinux.eu
GITEA_API_URL=https://git.paganlinux.eu/api/v1
GITEA_TOKEN=
EOF

chmod 600 /var/lib/pagancms/.env

# Sprawdź czy plik poprawny
cat /var/lib/pagancms/.env | grep DATABASE_URL

# Kompilacja (~5-10 min)
cd /opt/pag/cms/backend
cargo build --release

# Sprawdź czy binarek działa
/opt/pag/cms/backend/target/release/pagan-cms --help 2>&1 || true

# Systemd
cat > /etc/systemd/system/pagancms-api.service << 'SYSTEMD'
[Unit]
Description=PaganCMS API
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
systemctl stop pagancms-api 2>/dev/null
systemctl enable --now pagancms-api
sleep 3

# Test
curl -s http://127.0.0.1:3000/api/v1/stats
echo ""

# Jeśli pusto — debug:
journalctl -u pagancms-api -n 10 --no-pager
```

---

## 🟢 Krok 8: Panel Admina (React)

```bash
cd /opt/pag/cms/frontend
npm install
npm run build
mkdir -p /var/www/cms
cp -r dist/* /var/www/cms/
```

---

## 🟢 Krok 9: Strona WWW (Astro)

```bash
cd /opt/pag/web
npm install
npm run build

cat > /etc/systemd/system/paganlinux-web.service << 'SYSTEMD'
[Unit]
Description=PaganLinux Web
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
systemctl enable --now paganlinux-web
sleep 2
curl -s http://127.0.0.1:3004/ | head -3
```

---

## 🟢 Krok 10: Worker buildów

```bash
cd /opt/pag/cms/worker
cargo build --release
mkdir -p /var/lib/pagancms/build-workspace

cat > /etc/systemd/system/pagancms-worker.service << 'SYSTEMD'
[Unit]
Description=PaganCMS Worker
After=pagancms-api.service

[Service]
Type=simple
User=root
ExecStart=/opt/pag/cms/worker/target/release/pagancms-worker
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
SYSTEMD

systemctl daemon-reload
systemctl enable --now pagancms-worker
```

---

## 🟢 Krok 11: Nginx

```bash
rm -f /etc/nginx/sites-enabled/default

cat > /etc/nginx/sites-available/paganlinux << 'NGINX'
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
    }
}

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
    }
}

server {
    listen 80;
    server_name pagports.paganlinux.eu;
    location / {
        proxy_pass http://127.0.0.1:3004;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}

server {
    listen 80;
    server_name packages.paganlinux.eu;
    location / {
        proxy_pass http://127.0.0.1:3004;
        proxy_set_header Host $host;
    }
}

server {
    listen 80;
    server_name build.paganlinux.eu;
    location / {
        proxy_pass http://127.0.0.1:3004;
        proxy_set_header Host $host;
    }
}

server {
    listen 80;
    server_name api.paganlinux.eu;
    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}

server {
    listen 80;
    server_name git.paganlinux.eu;
    location / {
        proxy_pass http://127.0.0.1:3001;
        proxy_set_header Host $host;
    }
}
NGINX

ln -sf /etc/nginx/sites-available/paganlinux /etc/nginx/sites-enabled/paganlinux
nginx -t && systemctl reload nginx
```

---

## 🟢 Krok 12: SSL

```bash
# Spróbuj pluginu nginx (jeśli nie działa → metoda standalone poniżej)
apt install -y python3-certbot-nginx 2>/dev/null

certbot --nginx -d paganlinux.eu -d www.paganlinux.eu 2>/dev/null || {
    # Fallback — metoda standalone
    systemctl stop nginx
    for domain in paganlinux.eu cms.paganlinux.eu pagports.paganlinux.eu packages.paganlinux.eu build.paganlinux.eu api.paganlinux.eu git.paganlinux.eu; do
        certbot certonly --standalone -d $domain --non-interactive --agree-tos -m admin@paganlinux.eu
    done
    systemctl start nginx
    # Dodaj ręcznie listen 443 ssl do każdego bloku server w nginx
}

# Auto-renewal
echo "0 3 * * * certbot renew --quiet --post-hook 'systemctl reload nginx'" | crontab -
```

---

## 🟢 Krok 13: Weryfikacja

```bash
systemctl status pagancms-api paganlinux-web pagancms-worker nginx --no-pager

# API (powinno zwrócić JSON)
curl -s http://127.0.0.1:3000/api/v1/stats

# Strona główna
curl -s http://127.0.0.1:3004/ | head -3

# CMS
curl -s http://127.0.0.1/cms/ 2>/dev/null || echo "CMS: otwórz https://cms.paganlinux.eu"
```

---

## 🟢 Krok 14: Konto admina

```bash
curl -X POST http://127.0.0.1:3000/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","email":"admin@paganlinux.eu","password":"TWOJE_SILNE_HASLO"}'
```

Otwórz `https://cms.paganlinux.eu` i zaloguj się.

---

## 🛠️ Codzienne zarządzanie

```bash
# Status wszystkich serwisów
systemctl status pagancms-api paganlinux-web pagancms-worker nginx

# Logi
journalctl -u pagancms-api -f
journalctl -u paganlinux-web -f

# Restart
systemctl restart pagancms-api
systemctl restart paganlinux-web

# Aktualizacja kodu
cd /opt/pag && git pull
cd cms/backend && cargo build --release && systemctl restart pagancms-api
cd cms/frontend && npm install && npm run build && cp -r dist/* /var/www/cms/
cd ../web && npm install && npm run build && systemctl restart paganlinux-web

# Backup bazy
cp /var/lib/pagancms/pagancms.db /root/backup-$(date +%Y%m%d).db

# Porty
ss -tlnp | grep -E '3000|3004|80|443'
```

---

## ⚠️ Checklist przed produkcją

- [ ] `JWT_SECRET` ustawiony na silny losowy string
- [ ] Hasło admina zmienione
- [ ] Swap: `fallocate -l 2G /swapfile && chmod 600 /swapfile && mkswap /swapfile && swapon /swapfile`
- [ ] Auto-aktualizacje: `apt install unattended-upgrades -y`
- [ ] Logi: `journalctl --vacuum-size=500M`
- [ ] Gitea (opcjonalnie): `docker run -d --name gitea -p 3001:3000 -v /var/lib/gitea:/data gitea/gitea:latest`
