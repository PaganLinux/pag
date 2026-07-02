# PaganCMS — Full CMS for PaganLinux

## Architecture

```
cms/
├── backend/          # Rust API (Axum + SQLite + JWT)
│   ├── src/
│   │   ├── main.rs           # Server entry point
│   │   ├── config.rs         # Environment config
│   │   ├── db.rs             # Database init & migrations
│   │   ├── models/           # Data structures
│   │   ├── services/         # Business logic
│   │   ├── handlers/         # HTTP handlers
│   │   └── middleware/        # JWT auth, CORS
│   └── migrations/           # SQL schema
├── frontend/         # React Admin Panel (Vite + Tailwind)
│   └── src/
│       ├── api/              # API client
│       ├── components/       # Reusable UI
│       ├── context/          # Auth state
│       └── pages/            # Dashboard, Packages, etc.
├── docker-compose.yml
└── .env.example
```

## Quick Start

### 1. Backend

```bash
cd cms/backend
cargo run
# API starts on http://localhost:3000
```

### 2. Frontend

```bash
cd cms/frontend
npm install
npm run dev
# Admin panel on http://localhost:5173
```

### 3. Docker (full stack)

```bash
cd cms
cp .env.example .env
# Edit .env with your settings
docker-compose up -d
```

## API Endpoints

### Public
| Method | Path | Description |
|--------|------|-------------|
| POST | `/api/v1/auth/register` | Register user |
| POST | `/api/v1/auth/login` | Login, get JWT |
| GET  | `/api/v1/stats` | Dashboard statistics |
| GET  | `/api/v1/packages` | List packages |
| GET  | `/api/v1/packages/{id}` | Package details |
| GET  | `/api/v1/builds` | Build queue |
| POST | `/api/v1/webhook/gitea` | Gitea webhook |
| GET  | `/api/v1/ports` | List ports |
| GET  | `/api/v1/repos` | List repos |

### Protected (JWT required)
| Method | Path | Description |
|--------|------|-------------|
| GET  | `/api/v1/auth/me` | Current user |
| POST | `/api/v1/packages` | Create package |
| POST | `/api/v1/packages/upload` | Upload PAGBUILD |
| PUT  | `/api/v1/packages/{id}` | Update package |
| POST | `/api/v1/builds` | Trigger build |
| POST | `/api/v1/ports` | Create port |
| PUT  | `/api/v1/ports/{id}` | Update port |
| DELETE | `/api/v1/ports/{id}` | Delete port |
| POST | `/api/v1/repos` | Create Gitea repo |

## Adding a New Language to the Web

1. Add language code to `web/src/i18n/languages.ts`
2. Add translation block in `web/src/i18n/ui.ts`
3. Add locale to `web/astro.config.mjs` i18n.locales array

## Domains

- `paganlinux.eu` — Main landing page (web/)
- `cms.paganlinux.eu` — Admin panel (cms/frontend)
- `api.paganlinux.eu` — Backend API (cms/backend)
- `git.paganlinux.eu` — Gitea instance
- `pagports.paganlinux.eu` — Ports listing
- `packages.paganlinux.eu` — Package repository
- `build.paganlinux.eu` — Build logs & status
