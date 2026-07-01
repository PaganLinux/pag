# PaganLinux Package Ecosystem

Kompletny ekosystem menedżera pakietów dla dystrybucji PaganLinux.

## 🏗️ Struktura projektu

```
pag/
├── cli/                    # Menedżer pakietów `pag` (Rust)
│   ├── src/
│   │   ├── main.rs         # Punkt wejścia, CLI (clap)
│   │   ├── commands/       # Implementacje komend
│   │   │   ├── install.rs  # pag install
│   │   │   ├── remove.rs   # pag remove
│   │   │   ├── update.rs   # pag update
│   │   │   ├── search.rs   # pag search
│   │   │   ├── info.rs     # pag info
│   │   │   ├── list.rs     # pag list
│   │   │   ├── download.rs # pag download
│   │   │   ├── check.rs    # pag check
│   │   │   ├── clean.rs    # pag clean
│   │   │   ├── repo.rs     # pag repo
│   │   │   ├── flatpak.rs  # pag flatpak
│   │   │   ├── key.rs      # pag key
│   │   │   ├── query.rs    # pag query
│   │   │   ├── stats.rs    # pag stats
│   │   │   └── init.rs     # pag init
│   │   ├── package/        # Format .pag
│   │   ├── db/             # Baza SQLite
│   │   ├── deps/           # Solver zależności
│   │   ├── crypto/         # GPG, hashe
│   │   ├── flatpak/        # Integracja Flatpak
│   │   ├── repo/           # Klient HTTP repo
│   │   ├── i18n/           # 10 języków
│   │   └── config/         # Konfiguracja TOML
│   └── Cargo.toml
├── pagbuild/               # System budowania `pagbuild` (Rust)
│   ├── src/
│   │   ├── main.rs         # CLI build system
│   │   ├── parser/         # Parser pagbuild
│   │   ├── builder/        # Budowanie (chroot/kontener)
│   │   ├── signer/         # Podpisywanie GPG
│   │   └── repo/           # Upload do repo
│   └── Cargo.toml
├── repo-server/            # Serwer repozytorium (Rust/Axum)
│   ├── src/main.rs         # API REST
│   └── Cargo.toml
├── web/                    # Strony WWW (Astro)
│   ├── main/               # paganlinux.eu
│   ├── repos/              # repos.paganlinux.eu
│   └── ports/              # pagports.paganlinux.eu
├── docs/                   # Dokumentacja
├── examples/
│   └── pagports/main/7zip/pagbuild  # Przykładowy pagbuild
└── README.md
```

## 🚀 Szybki start

### Instalacja menedżera `pag`

```bash
# Sklonuj repozytorium
git clone https://github.com/PaganLinux/pag.git
cd pag/cli

# Zbuduj (wymagany Rust 1.80+)
cargo build --release

# Zainstaluj
sudo cp target/release/pag /usr/local/bin/

# Inicjalizuj konfigurację
sudo pag init
pag update
```

### Podstawowe komendy

```bash
# Instalacja
pag install nginx postgresql redis

# Wyszukiwanie
pag search firefox
pag search --verbose python

# Aktualizacja
pag update            # odśwież listę i zaktualizuj
pag update -y         # tylko odśwież indeksy

# Usuwanie
pag remove nginx
pag remove --recursive   # usuń też niepotrzebne zależności

# Informacje
pag info nginx
pag query owner /usr/bin/ssh
pag query files openssh

# Flatpak
pag flatpak install org.mozilla.firefox
pag flatpak search libreoffice
pag flatpak list

# Statystyki
pag stats
pag list --orphans

# Bezpieczeństwo
pag check              # sprawdź integralność
pag check --fix        # napraw uszkodzone pakiety
pag key import klucz.gpg
```

### Budowanie pakietów z `pagbuild`

```bash
cd pagports/main

# Zbuduj pakiet
pagbuild build 7zip

# Zbuduj z czyszczeniem katalogów
pagbuild build --clean nginx

# Podpisz pakiet
pagbuild sign 7zip

# Wyślij do repozytorium
PAGBUILD_API_TOKEN=twoj_token pagbuild upload packages/*.pag --repo extra

# Utwórz nowy szablon
pagbuild new moj-pakiet -v 1.0.0
```

## 📦 Format .pag

Pliki `.pag` to binarne archiwa z następującą strukturą:

| Offset | Rozmiar | Opis |
|--------|---------|------|
| 0 | 4 bajty | Magic: `PAG\x01` |
| 4 | 4 bajty | Rozmiar nagłówka (u32 LE) |
| 8 | N | Nagłówek JSON |
| 8+N | M | Payload (tar.zst) |
| 8+N+M | K | Sygnatura GPG (opcjonalnie) |

Nagłówek JSON zawiera: nazwę, wersję, architekturę, zależności, provides, konflikty, listę plików, checksumy i skrypty.

## 🌍 Wielojęzyczność

`pag` obsługuje 10 języków: 🇵🇱 PL · 🇬🇧 EN · 🇩🇪 DE · 🇫🇷 FR · 🇪🇸 ES · 🇮🇹 IT · 🇷🇺 RU · 🇨🇿 CS · 🇯🇵 JA · 🇨🇳 ZH

```bash
pag --lang pl install nginx
pag --lang en search firefox
```

## 🏗️ Struktura pliku pagbuild

```bash
maintainer="Imię <email>"    # Opiekun pakietu
pkgname=nazwa                 # Nazwa pakietu
pkgver=1.0.0                  # Wersja
pkgrel=1                      # Wydanie
pkgdesc="Opis"                # Opis
url="https://..."             # Strona projektu
arch="x86_64"                 # Architektura (lub "all")
license="MIT"                 # Licencja
makedepends="gcc make"        # Zależności build-time
depends="glibc"               # Zależności runtime
provides="virtual-name"       # Wirtualne pakiety
source="url-do-źródła"        # Źródła

build() { ... }               # Funkcja budowania
check() { ... }               # Funkcja testowania
package() { ... }             # Funkcja pakowania
sha512sums="..."              # Sumy kontrolne
```

## 🔐 Bezpieczeństwo

- **Podpisy GPG**: Każdy pakiet jest opcjonalnie podpisany GPG
- **Weryfikacja checksum**: SHA512 + BLAKE3 dla każdego pliku
- **Sandboxing**: Opcjonalna izolacja podczas instalacji
- **Klucze**: Wbudowane zarządzanie kluczami (`pag key`)
- **CVE**: Wsparcie dla śledzenia podatności (`secfixes` w pagbuild)

## 🌐 Strony

| Strona | URL | Opis |
|--------|-----|------|
| Główna | https://paganlinux.eu | Landing page, download, docs |
| Repozytorium | https://repos.paganlinux.eu | Przeglądarka pakietów .pag |
| Porty | https://pagports.paganlinux.eu | System portów / źródła |

## 📋 Wymagania

**`pag`**: Rust 1.80+, Linux z glibc i systemd, GPG, Flatpak (opcjonalnie)

**`pagbuild`**: Rust 1.80+, Bash, GCC/Make, Docker/Podman (opcjonalnie)

**`pag-repo-server`**: Rust 1.80+, 10GB+ przestrzeni

## 📄 Licencja

GPL-2.0-only

---

PaganLinux © 2026 — Zbudowane z pasją dla społeczności open-source.





