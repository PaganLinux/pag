# 🐺 Pag — PaganLinux Package Manager

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org)
[![Arch](https://img.shields.io/badge/arch-x86__64%20%7C%20aarch64-brightgreen)]()

Wielojęzyczny, w pełni funkcjonalny menedżer pakietów dla dystrybucji **PaganLinux**, zbudowany w języku Rust. Wzorowany na `pacman` (Arch Linux), ale zaprojektowany od podstaw z myślą o nowoczesnych rozwiązaniach: migawkach systemu, weryfikacji podpisów GPG, sprawdzaniu wersji upstream i aktualizacji całego systemu.

## ✨ Główne funkcje

- 📦 **PAGBUILD** — format 1:1 kompatybilny z PKGBUILD Arch Linux
- 🔍 **checkver.toml** — automatyczne sprawdzanie nowych wersji upstream (GitHub, GitLab, PyPI, crates.io, git, URL)
- 🔐 **Podpisy GPG** — weryfikacja `.pag.sig` dla każdego pakietu
- 🗂️ **Baza SQLite** — szybka lokalna baza zainstalowanych pakietów
- 📸 **Migawki systemu** — wsparcie dla Btrfs, ZFS i rsync
- 🌐 **Wielojęzyczność** — Polski, Angielski, Niemiecki, Francuski, Hiszpański
- ⬆️ **Aktualizacja systemu** — `pag upgrade --sys` sprawdza nowe wydania ISO/rootfs
- 🔗 **Rozwiązywanie zależności** — automatyczne pobieranie wymaganych pakietów
- 📊 **Statystyki** — `pag stats` pokazuje stan systemu
- 🧹 **Czyszczenie** — `pag clean` usuwa cache i pliki tymczasowe