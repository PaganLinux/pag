#!/usr/bin/env python3
"""Add popular Linux desktop packages to pagports."""
import os

BASE = "/workspaces/pag/examples/pagports/main"

POPULAR = {
    # === BROWSERS ===
    "brave": {
        "ver": "1.68.128", "rel": 0,
        "desc": "Brave web browser",
        "url": "https://brave.com/", "license": "MPL-2.0",
        "makedeps": "rust gcc make nodejs", "deps": "gtk+3.0 nss nspr libx11",
        "source": "https://github.com/brave/brave-browser/archive/vVER.tar.gz",
        "build": "npm install && npm run build",
    },
    "vivaldi": {
        "ver": "6.8.3381", "rel": 0,
        "desc": "Vivaldi web browser",
        "url": "https://vivaldi.com/", "license": "BSD-3-Clause",
        "makedeps": "", "deps": "gtk+3.0 nss nspr libx11 libxcb",
        "source": "https://downloads.vivaldi.com/stable/vivaldi-stable-VER-amd64.deb",
        "build": "",
        "is_binary": True,
    },

    # === COMMUNICATION ===
    "discord": {
        "ver": "0.0.57", "rel": 0,
        "desc": "Discord messaging client",
        "url": "https://discord.com/", "license": "custom:proprietary",
        "makedeps": "", "deps": "gtk+3.0 nss nspr libx11 libxcb libxcomposite libxdamage libxext libxfixes libxrandr libxrender libxtst",
        "source": "https://dl.discordapp.net/apps/linux/VER/discord-VER.tar.gz",
        "build": "", "is_binary": True,
    },
    "telegram-desktop": {
        "ver": "5.3.2", "rel": 0,
        "desc": "Telegram Desktop client",
        "url": "https://desktop.telegram.org/", "license": "GPL-3.0-or-later",
        "makedeps": "cmake gcc make ninja qt6-base-dev qt6-svg-dev qt6-wayland-dev",
        "deps": "qt6-base qt6-svg qt6-wayland openssl",
        "source": "https://github.com/telegramdesktop/tdesktop/archive/vVER.tar.gz",
        "build": "cmake -B build -DCMAKE_INSTALL_PREFIX=/usr -DCMAKE_BUILD_TYPE=Release && cmake --build build",
        "package_cmake": True,
    },
    "signal-desktop": {
        "ver": "7.19.0", "rel": 0,
        "desc": "Signal private messenger desktop",
        "url": "https://signal.org/", "license": "AGPL-3.0-only",
        "makedeps": "nodejs npm git", "deps": "gtk+3.0 nss nspr libx11",
        "source": "https://github.com/signalapp/Signal-Desktop/archive/vVER.tar.gz",
        "build": "npm install && npm run build",
    },

    # === DEVELOPMENT ===
    "vscode": {
        "ver": "1.92.0", "rel": 0,
        "desc": "Visual Studio Code editor",
        "url": "https://code.visualstudio.com/", "license": "MIT",
        "makedeps": "nodejs npm git python3", "deps": "gtk+3.0 nss nspr libx11 libxcb",
        "source": "https://github.com/microsoft/vscode/archive/VER.tar.gz",
        "build": "npm install && npm run compile",
    },
    "neovim": {
        "ver": "0.10.1", "rel": 1,
        "desc": "Hyperextensible Vim-based text editor (updated)",
        "url": "https://neovim.io/", "license": "Apache-2.0",
        "makedeps": "cmake gcc make ninja", "deps": "",
        "source": "https://github.com/neovim/neovim/archive/vVER.tar.gz",
        "build": "cmake -B build -DCMAKE_INSTALL_PREFIX=/usr -DCMAKE_BUILD_TYPE=Release && cmake --build build",
        "package_cmake": True,
    },
    "helix": {
        "ver": "24.07", "rel": 0,
        "desc": "Post-modern modal text editor",
        "url": "https://helix-editor.com/", "license": "MPL-2.0",
        "makedeps": "rust cargo", "deps": "",
        "source": "https://github.com/helix-editor/helix/archive/VER.tar.gz",
        "build": "cargo build --release",
        "package": 'install -Dm755 target/release/hx "$pkgdir"/usr/bin/hx',
    },
    "gh": {
        "ver": "2.55.0", "rel": 0,
        "desc": "GitHub CLI tool",
        "url": "https://cli.github.com/", "license": "MIT",
        "makedeps": "go", "deps": "git",
        "source": "https://github.com/cli/cli/archive/vVER.tar.gz",
        "build": "make",
        "package": 'install -Dm755 bin/gh "$pkgdir"/usr/bin/gh',
    },
    "ripgrep": {
        "ver": "14.1.1", "rel": 0,
        "desc": "Ultra-fast grep alternative",
        "url": "https://github.com/BurntSushi/ripgrep", "license": "MIT OR Unlicense",
        "makedeps": "rust cargo", "deps": "",
        "source": "https://github.com/BurntSushi/ripgrep/archive/VER.tar.gz",
        "build": "cargo build --release",
        "package": 'install -Dm755 target/release/rg "$pkgdir"/usr/bin/rg',
    },
    "fd": {
        "ver": "10.1.0", "rel": 0,
        "desc": "Simple, fast find alternative",
        "url": "https://github.com/sharkdp/fd", "license": "MIT OR Apache-2.0",
        "makedeps": "rust cargo", "deps": "",
        "source": "https://github.com/sharkdp/fd/archive/vVER.tar.gz",
        "build": "cargo build --release",
        "package": 'install -Dm755 target/release/fd "$pkgdir"/usr/bin/fd',
    },
    "bat": {
        "ver": "0.24.0", "rel": 0,
        "desc": "Cat clone with syntax highlighting",
        "url": "https://github.com/sharkdp/bat", "license": "MIT OR Apache-2.0",
        "makedeps": "rust cargo", "deps": "",
        "source": "https://github.com/sharkdp/bat/archive/vVER.tar.gz",
        "build": "cargo build --release",
        "package": 'install -Dm755 target/release/bat "$pkgdir"/usr/bin/bat',
    },
    "eza": {
        "ver": "0.19.0", "rel": 0,
        "desc": "Modern replacement for ls",
        "url": "https://github.com/eza-community/eza", "license": "MIT",
        "makedeps": "rust cargo", "deps": "",
        "source": "https://github.com/eza-community/eza/archive/vVER.tar.gz",
        "build": "cargo build --release",
        "package": 'install -Dm755 target/release/eza "$pkgdir"/usr/bin/eza',
    },
    "delta": {
        "ver": "0.17.0", "rel": 0,
        "desc": "Git diff viewer with syntax highlighting",
        "url": "https://github.com/dandavison/delta", "license": "MIT",
        "makedeps": "rust cargo", "deps": "git",
        "source": "https://github.com/dandavison/delta/archive/VER.tar.gz",
        "build": "cargo build --release",
        "package": 'install -Dm755 target/release/delta "$pkgdir"/usr/bin/delta',
    },
    "lazygit": {
        "ver": "0.43.1", "rel": 0,
        "desc": "Terminal UI for git commands",
        "url": "https://github.com/jesseduffield/lazygit", "license": "MIT",
        "makedeps": "go", "deps": "git",
        "source": "https://github.com/jesseduffield/lazygit/archive/vVER.tar.gz",
        "build": "go build",
        "package": 'install -Dm755 lazygit "$pkgdir"/usr/bin/lazygit',
    },
    "jq": {
        "ver": "1.7.1", "rel": 0,
        "desc": "Command-line JSON processor",
        "url": "https://jqlang.github.io/jq/", "license": "MIT",
        "makedeps": "gcc make", "deps": "",
        "source": "https://github.com/jqlang/jq/archive/jq-VER.tar.gz",
        "build": "autoreconf -fi && ./configure --prefix=/usr && make",
    },
    "lazydocker": {
        "ver": "0.23.3", "rel": 0,
        "desc": "Terminal UI for Docker",
        "url": "https://github.com/jesseduffield/lazydocker", "license": "MIT",
        "makedeps": "go", "deps": "",
        "source": "https://github.com/jesseduffield/lazydocker/archive/vVER.tar.gz",
        "build": "go build",
        "package": 'install -Dm755 lazydocker "$pkgdir"/usr/bin/lazydocker',
    },

    # === MULTIMEDIA ===
    "obs-studio": {
        "ver": "30.2.3", "rel": 0,
        "desc": "OBS Studio streaming/recording software",
        "url": "https://obsproject.com/", "license": "GPL-2.0-or-later",
        "makedeps": "cmake gcc make ninja qt6-base-dev mesa-dev wayland-dev libx11-dev libxcb-dev",
        "deps": "qt6-base mesa wayland libx11 libxcb ffmpeg",
        "source": "https://github.com/obsproject/obs-studio/archive/VER.tar.gz",
        "build": "cmake -B build -DCMAKE_INSTALL_PREFIX=/usr && cmake --build build",
        "package_cmake": True,
    },
    "blender": {
        "ver": "4.2.1", "rel": 0,
        "desc": "3D creation suite",
        "url": "https://www.blender.org/", "license": "GPL-3.0-or-later",
        "makedeps": "cmake gcc make ninja python3 mesa-dev",
        "deps": "mesa python3",
        "source": "https://download.blender.org/source/blender-VER.tar.xz",
        "build": "cmake -B build -DCMAKE_INSTALL_PREFIX=/usr && cmake --build build",
        "package_cmake": True,
    },
    "audacity": {
        "ver": "3.6.1", "rel": 0,
        "desc": "Audio editor and recorder",
        "url": "https://www.audacityteam.org/", "license": "GPL-3.0-or-later",
        "makedeps": "cmake gcc make ninja wxwidgets-dev",
        "deps": "wxwidgets",
        "source": "https://github.com/audacity/audacity/archive/Audacity-VER.tar.gz",
        "build": "cmake -B build -DCMAKE_INSTALL_PREFIX=/usr && cmake --build build",
        "package_cmake": True,
    },
    "kdenlive": {
        "ver": "24.05.2", "rel": 0,
        "desc": "KDE Non-linear video editor",
        "url": "https://kdenlive.org/", "license": "GPL-2.0-or-later",
        "makedeps": "cmake gcc make ninja qt6-base-dev qt6-declarative-dev",
        "deps": "qt6-base qt6-declarative ffmpeg",
        "source": "https://download.kde.org/stable/release-service/24.05.2/src/kdenlive-VER.tar.xz",
        "build": "cmake -B build -DCMAKE_INSTALL_PREFIX=/usr && cmake --build build",
        "package_cmake": True,
    },
    "handbrake": {
        "ver": "1.8.2", "rel": 0,
        "desc": "Video transcoder",
        "url": "https://handbrake.fr/", "license": "GPL-2.0-only",
        "makedeps": "gcc make nasm yasm", "deps": "",
        "source": "https://github.com/HandBrake/HandBrake/archive/VER.tar.gz",
        "build": "./configure --prefix=/usr && make",
    },
    "strawberry": {
        "ver": "1.1.3", "rel": 0,
        "desc": "Music player and collection organizer",
        "url": "https://www.strawberrymusicplayer.org/", "license": "GPL-3.0-or-later",
        "makedeps": "cmake gcc make ninja qt6-base-dev",
        "deps": "qt6-base gstreamer",
        "source": "https://github.com/strawberrymusicplayer/strawberry/archive/VER.tar.gz",
        "build": "cmake -B build -DCMAKE_INSTALL_PREFIX=/usr && cmake --build build",
        "package_cmake": True,
    },
    "spotify": {
        "ver": "1.2.42", "rel": 0,
        "desc": "Spotify music streaming client",
        "url": "https://www.spotify.com/", "license": "custom:proprietary",
        "makedeps": "", "deps": "gtk+3.0 nss nspr libx11",
        "source": "https://repository-origin.spotify.com/pool/non-free/s/spotify-client/spotify-client_VER_amd64.deb",
        "build": "", "is_binary": True,
    },

    # === GAMING ===
    "steam": {
        "ver": "1.0.0.79", "rel": 0,
        "desc": "Steam gaming platform",
        "url": "https://store.steampowered.com/", "license": "custom:proprietary",
        "makedeps": "", "deps": "libx11 libxcb mesa vulkan-loader",
        "source": "https://repo.steampowered.com/steam/pool/steam/s/steam/steam_latest_amd64.deb",
        "build": "", "is_binary": True,
    },
    "lutris": {
        "ver": "0.5.17", "rel": 0,
        "desc": "Open gaming platform",
        "url": "https://lutris.net/", "license": "GPL-3.0-or-later",
        "makedeps": "python3", "deps": "python3 gtk+3.0",
        "source": "https://github.com/lutris/lutris/archive/vVER.tar.gz",
        "build": "python3 setup.py build",
        "package_python": True,
    },
    "wine": {
        "ver": "9.14", "rel": 0,
        "desc": "Windows compatibility layer",
        "url": "https://www.winehq.org/", "license": "LGPL-2.1-or-later",
        "makedeps": "gcc make flex bison", "deps": "libx11 libxcb mesa",
        "source": "https://gitlab.winehq.org/wine/wine/-/archive/wine-VER/wine-wine-VER.tar.gz",
        "build": "./configure --prefix=/usr && make",
    },
    "mangohud": {
        "ver": "0.7.2", "rel": 0,
        "desc": "Vulkan/OpenGL overlay for FPS/metrics",
        "url": "https://github.com/flightlessmango/MangoHud", "license": "MIT",
        "makedeps": "meson ninja gcc python3-mako mesa-dev",
        "deps": "mesa",
        "source": "https://github.com/flightlessmango/MangoHud/archive/vVER.tar.gz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },
    "gamemode": {
        "ver": "1.8.1", "rel": 0,
        "desc": "Optimise Linux system performance for gaming",
        "url": "https://github.com/FeralInteractive/gamemode", "license": "BSD-3-Clause",
        "makedeps": "meson ninja gcc", "deps": "",
        "source": "https://github.com/FeralInteractive/gamemode/archive/VER.tar.gz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },
    "heroic-games-launcher": {
        "ver": "2.14.1", "rel": 0,
        "desc": "Epic/GOG games launcher",
        "url": "https://heroicgameslauncher.com/", "license": "GPL-3.0-or-later",
        "makedeps": "nodejs npm", "deps": "",
        "source": "https://github.com/Heroic-Games-Launcher/HeroicGamesLauncher/archive/vVER.tar.gz",
        "build": "npm install && npm run build",
    },
    "bottles": {
        "ver": "51.13", "rel": 0,
        "desc": "Run Windows software on Linux",
        "url": "https://usebottles.com/", "license": "GPL-3.0-or-later",
        "makedeps": "python3 meson ninja", "deps": "python3 gtk+3.0",
        "source": "https://github.com/bottlesdevs/Bottles/archive/VER.tar.gz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
        "package": 'DESTDIR="$pkgdir" ninja -C build install',
    },

    # === TERMINAL TOOLS ===
    "btop": {
        "ver": "1.3.2", "rel": 0,
        "desc": "Resource monitor (C++ version of bashtop)",
        "url": "https://github.com/aristocratos/btop", "license": "Apache-2.0",
        "makedeps": "gcc make", "deps": "",
        "source": "https://github.com/aristocratos/btop/archive/vVER.tar.gz",
        "build": "make",
        "package": 'install -Dm755 bin/btop "$pkgdir"/usr/bin/btop',
    },
    "fzf": {
        "ver": "0.54.3", "rel": 0,
        "desc": "Command-line fuzzy finder",
        "url": "https://github.com/junegunn/fzf", "license": "MIT",
        "makedeps": "go", "deps": "",
        "source": "https://github.com/junegunn/fzf/archive/vVER.tar.gz",
        "build": "make",
        "package": 'install -Dm755 target/fzf "$pkgdir"/usr/bin/fzf',
    },
    "zoxide": {
        "ver": "0.9.4", "rel": 0,
        "desc": "Smarter cd command",
        "url": "https://github.com/ajeetdsouza/zoxide", "license": "MIT",
        "makedeps": "rust cargo", "deps": "",
        "source": "https://github.com/ajeetdsouza/zoxide/archive/vVER.tar.gz",
        "build": "cargo build --release",
        "package": 'install -Dm755 target/release/zoxide "$pkgdir"/usr/bin/zoxide',
    },
    "starship": {
        "ver": "1.20.1", "rel": 0,
        "desc": "Cross-shell prompt",
        "url": "https://starship.rs/", "license": "ISC",
        "makedeps": "rust cargo", "deps": "",
        "source": "https://github.com/starship/starship/archive/vVER.tar.gz",
        "build": "cargo build --release",
        "package": 'install -Dm755 target/release/starship "$pkgdir"/usr/bin/starship',
    },
    "dust": {
        "ver": "1.1.1", "rel": 0,
        "desc": "More intuitive du",
        "url": "https://github.com/bootandy/dust", "license": "Apache-2.0",
        "makedeps": "rust cargo", "deps": "",
        "source": "https://github.com/bootandy/dust/archive/vVER.tar.gz",
        "build": "cargo build --release",
        "package": 'install -Dm755 target/release/dust "$pkgdir"/usr/bin/dust',
    },
    "ncdu": {
        "ver": "2.5", "rel": 0,
        "desc": "NCurses disk usage analyzer",
        "url": "https://dev.yorhel.nl/ncdu", "license": "MIT",
        "makedeps": "gcc make zig", "deps": "",
        "source": "https://dev.yorhel.nl/download/ncdu-VER.tar.gz",
        "build": "./configure --prefix=/usr && make",
    },
    "wezterm": {
        "ver": "20240203", "rel": 0,
        "desc": "GPU-accelerated terminal emulator",
        "url": "https://wezfurlong.org/wezterm/", "license": "MIT",
        "makedeps": "rust cargo", "deps": "fontconfig mesa wayland libx11 libxcb",
        "source": "https://github.com/wez/wezterm/archive/VER.tar.gz",
        "build": "cargo build --release",
        "package": 'install -Dm755 target/release/wezterm "$pkgdir"/usr/bin/wezterm',
    },
    "kitty": {
        "ver": "0.35.2", "rel": 0,
        "desc": "GPU-accelerated terminal emulator",
        "url": "https://sw.kovidgoyal.net/kitty/", "license": "GPL-3.0-only",
        "makedeps": "gcc make python3 harfbuzz-dev freetype-dev fontconfig-dev wayland-dev libx11-dev libxcb-dev",
        "deps": "harfbuzz freetype fontconfig wayland libx11 libxcb",
        "source": "https://github.com/kovidgoyal/kitty/archive/vVER.tar.gz",
        "build": "make",
        "package": "make DESTDIR=\"$pkgdir\" install",
    },

    # === SYSTEM UTILITIES ===
    "timeshift": {
        "ver": "24.06.1", "rel": 0,
        "desc": "System restore tool",
        "url": "https://github.com/linuxmint/timeshift", "license": "GPL-3.0-or-later",
        "makedeps": "meson ninja gcc", "deps": "",
        "source": "https://github.com/linuxmint/timeshift/archive/VER.tar.gz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },
    "tlp": {
        "ver": "1.7.0", "rel": 0,
        "desc": "Power management for Linux",
        "url": "https://linrunner.de/tlp", "license": "GPL-2.0-or-later",
        "makedeps": "", "deps": "",
        "source": "https://github.com/linrunner/TLP/archive/VER.tar.gz",
        "build": "make",
    },
    "brightnessctl": {
        "ver": "0.5.1", "rel": 0,
        "desc": "Screen brightness control",
        "url": "https://github.com/Hummer12007/brightnessctl", "license": "MIT",
        "makedeps": "gcc make", "deps": "",
        "source": "https://github.com/Hummer12007/brightnessctl/archive/VER.tar.gz",
        "build": "make",
        "package": 'install -Dm755 brightnessctl "$pkgdir"/usr/bin/brightnessctl',
    },
    "lm_sensors": {
        "ver": "3.6.0", "rel": 0,
        "desc": "Hardware monitoring tools",
        "url": "https://github.com/lm-sensors/lm-sensors", "license": "GPL-2.0-or-later",
        "makedeps": "gcc make bison flex", "deps": "",
        "source": "https://github.com/lm-sensors/lm-sensors/archive/V3-6-0.tar.gz",
        "build": "make",
    },

    # === NETWORK ===
    "wireguard-tools": {
        "ver": "1.0.20210914", "rel": 0,
        "desc": "WireGuard VPN tools",
        "url": "https://www.wireguard.com/", "license": "GPL-2.0-only",
        "makedeps": "gcc make", "deps": "",
        "source": "https://git.zx2c4.com/wireguard-tools/snapshot/wireguard-tools-VER.tar.xz",
        "build": "make",
    },
    "tailscale": {
        "ver": "1.72.0", "rel": 0,
        "desc": "Tailscale VPN mesh network",
        "url": "https://tailscale.com/", "license": "BSD-3-Clause",
        "makedeps": "go", "deps": "",
        "source": "https://github.com/tailscale/tailscale/archive/vVER.tar.gz",
        "build": "go build -o tailscale ./cmd/tailscale && go build -o tailscaled ./cmd/tailscaled",
    },
    "nmap": {
        "ver": "7.95", "rel": 0,
        "desc": "Network discovery and security auditing",
        "url": "https://nmap.org/", "license": "GPL-2.0-only",
        "makedeps": "gcc make openssl-dev", "deps": "openssl",
        "source": "https://nmap.org/dist/nmap-VER.tar.bz2",
        "build": "./configure --prefix=/usr && make",
    },
    "openvpn": {
        "ver": "2.6.12", "rel": 0,
        "desc": "OpenVPN client and server",
        "url": "https://openvpn.net/", "license": "GPL-2.0-only",
        "makedeps": "gcc make openssl-dev", "deps": "openssl",
        "source": "https://swupdate.openvpn.org/community/releases/openvpn-VER.tar.gz",
        "build": "./configure --prefix=/usr && make",
    },
    "rsync": {
        "ver": "3.3.0", "rel": 0,
        "desc": "Fast file synchronization tool",
        "url": "https://rsync.samba.org/", "license": "GPL-3.0-or-later",
        "makedeps": "gcc make", "deps": "",
        "source": "https://rsync.samba.org/ftp/rsync/rsync-VER.tar.gz",
        "build": "./configure --prefix=/usr && make",
    },
    "wget": {
        "ver": "1.24.5", "rel": 0,
        "desc": "Network downloader",
        "url": "https://www.gnu.org/software/wget/", "license": "GPL-3.0-or-later",
        "makedeps": "gcc make openssl-dev", "deps": "openssl",
        "source": "https://ftp.gnu.org/gnu/wget/wget-VER.tar.gz",
        "build": "./configure --prefix=/usr --with-ssl=openssl && make",
    },

    # === CONTAINERS / VIRTUALIZATION ===
    "docker": {
        "ver": "27.1.1", "rel": 0,
        "desc": "Docker container runtime",
        "url": "https://www.docker.com/", "license": "Apache-2.0",
        "makedeps": "go", "deps": "",
        "sub": "docker-compose",
        "source": "https://github.com/moby/moby/archive/vVER.tar.gz",
        "build": "make",
    },
    "podman": {
        "ver": "5.1.2", "rel": 0,
        "desc": "Podman container engine",
        "url": "https://podman.io/", "license": "Apache-2.0",
        "makedeps": "go gcc make", "deps": "",
        "source": "https://github.com/containers/podman/archive/vVER.tar.gz",
        "build": "make",
    },
    "distrobox": {
        "ver": "1.7.2", "rel": 0,
        "desc": "Use any Linux distribution inside terminal",
        "url": "https://github.com/89luca89/distrobox", "license": "GPL-3.0-only",
        "makedeps": "", "deps": "podman",
        "source": "https://github.com/89luca89/distrobox/archive/VER.tar.gz",
        "build": "./install --prefix=/usr",
    },
    "qemu": {
        "ver": "9.0.2", "rel": 0,
        "desc": "QEMU machine emulator and virtualizer",
        "url": "https://www.qemu.org/", "license": "GPL-2.0-only",
        "makedeps": "gcc make ninja python3 mesa-dev libcap-dev",
        "deps": "mesa",
        "source": "https://download.qemu.org/qemu-VER.tar.xz",
        "build": "./configure --prefix=/usr && make",
    },

    # === XDG / DESKTOP PORTALS ===
    "xdg-desktop-portal": {
        "ver": "1.18.4", "rel": 0,
        "desc": "Desktop integration portal",
        "url": "https://github.com/flatpak/xdg-desktop-portal", "license": "LGPL-2.1-or-later",
        "makedeps": "meson ninja gcc", "deps": "",
        "source": "https://github.com/flatpak/xdg-desktop-portal/archive/VER.tar.gz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },
    "xdg-desktop-portal-wlr": {
        "ver": "0.8.0", "rel": 0,
        "desc": "XDG Desktop Portal for wlroots",
        "url": "https://github.com/emersion/xdg-desktop-portal-wlr", "license": "MIT",
        "makedeps": "meson ninja gcc wayland-dev", "deps": "wayland xdg-desktop-portal",
        "source": "https://github.com/emersion/xdg-desktop-portal-wlr/archive/vVER.tar.gz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },
    "xdg-desktop-portal-gtk": {
        "ver": "1.15.1", "rel": 0,
        "desc": "XDG Desktop Portal GTK backend",
        "url": "https://github.com/flatpak/xdg-desktop-portal-gtk", "license": "LGPL-2.1-or-later",
        "makedeps": "meson ninja gcc gtk+3.0-dev", "deps": "gtk+3.0 xdg-desktop-portal",
        "source": "https://github.com/flatpak/xdg-desktop-portal-gtk/archive/VER.tar.gz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },
    "xdg-desktop-portal-kde": {
        "ver": "6.1.3", "rel": 0,
        "desc": "XDG Desktop Portal KDE backend",
        "url": "https://invent.kde.org/plasma/xdg-desktop-portal-kde", "license": "LGPL-2.0-or-later",
        "makedeps": "cmake gcc make ninja qt6-base-dev", "deps": "qt6-base xdg-desktop-portal",
        "source": "https://download.kde.org/stable/plasma/6.1.3/xdg-desktop-portal-kde-VER.tar.xz",
        "build": "cmake -B build -DCMAKE_INSTALL_PREFIX=/usr && cmake --build build",
        "package_cmake": True,
    },

    # === DESKTOP ENVIRONMENT (GNOME basics) ===
    "nautilus": {
        "ver": "46.2", "rel": 0,
        "desc": "GNOME file manager",
        "url": "https://apps.gnome.org/Nautilus/", "license": "GPL-3.0-or-later",
        "makedeps": "meson ninja gcc gtk+3.0-dev", "deps": "gtk+3.0",
        "source": "https://download.gnome.org/sources/nautilus/46/nautilus-VER.tar.xz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },

    # === FONTS ===
    "fira-code": {
        "ver": "6.2", "rel": 0,
        "desc": "Fira Code monospace font with ligatures",
        "url": "https://github.com/tonsky/FiraCode", "license": "OFL-1.1",
        "makedeps": "", "deps": "",
        "source": "https://github.com/tonsky/FiraCode/releases/download/VER/Fira_Code_vVER.zip",
        "build": "",
        "package": 'mkdir -p "$pkgdir"/usr/share/fonts/fira-code && unzip -o "$srcdir"/Fira_Code_vVER.zip -d "$pkgdir"/usr/share/fonts/fira-code/',
    },
    "jetbrains-mono": {
        "ver": "2.304", "rel": 0,
        "desc": "JetBrains Mono font",
        "url": "https://www.jetbrains.com/lp/mono/", "license": "OFL-1.1",
        "makedeps": "", "deps": "",
        "source": "https://github.com/JetBrains/JetBrainsMono/releases/download/vVER/JetBrainsMono-VER.zip",
        "build": "",
        "package": 'mkdir -p "$pkgdir"/usr/share/fonts/jetbrains-mono && unzip -o "$srcdir"/JetBrainsMono-VER.zip -d "$pkgdir"/usr/share/fonts/jetbrains-mono/',
    },

    # === Window Managers (WM) ===
    "i3": {
        "ver": "4.23", "rel": 0,
        "desc": "Improved tiling window manager",
        "url": "https://i3wm.org/", "license": "BSD-3-Clause",
        "makedeps": "meson ninja gcc libx11-dev libxcb-dev xorgproto cairo-dev pango-dev",
        "deps": "libx11 libxcb xorgproto cairo pango",
        "sub": "i3status i3lock",
        "source": "https://i3wm.org/downloads/i3-VER.tar.xz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },
    "polybar": {
        "ver": "3.7.1", "rel": 0,
        "desc": "Fast and easy-to-use status bar",
        "url": "https://polybar.github.io/", "license": "MIT",
        "makedeps": "cmake gcc make ninja libx11-dev libxcb-dev cairo-dev",
        "deps": "libx11 libxcb cairo",
        "source": "https://github.com/polybar/polybar/archive/VER.tar.gz",
        "build": "cmake -B build -DCMAKE_INSTALL_PREFIX=/usr && cmake --build build",
        "package_cmake": True,
    },
    "rofi": {
        "ver": "1.7.5", "rel": 0,
        "desc": "Window switcher and application launcher",
        "url": "https://github.com/davatorium/rofi", "license": "MIT",
        "makedeps": "meson ninja gcc libx11-dev libxcb-dev cairo-dev pango-dev",
        "deps": "libx11 libxcb cairo pango",
        "source": "https://github.com/davatorium/rofi/archive/VER.tar.gz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },
    "picom": {
        "ver": "12.3", "rel": 0,
        "desc": "Lightweight X11 compositor",
        "url": "https://github.com/yshui/picom", "license": "MIT",
        "makedeps": "meson ninja gcc libx11-dev libxcb-dev libxext-dev pixman-dev",
        "deps": "libx11 libxcb libxext pixman mesa",
        "source": "https://github.com/yshui/picom/archive/vVER.tar.gz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },
    "dunst": {
        "ver": "1.9.2", "rel": 0,
        "desc": "Lightweight notification daemon",
        "url": "https://dunst-project.org/", "license": "BSD-3-Clause",
        "makedeps": "gcc make libx11-dev libxext-dev libxrandr-dev pango-dev cairo-dev",
        "deps": "libx11 libxext libxrandr pango cairo",
        "source": "https://github.com/dunst-project/dunst/archive/vVER.tar.gz",
        "build": "make",
        "package": 'make PREFIX=/usr DESTDIR="$pkgdir" install',
    },

    # === Other ===
    "calibre": {
        "ver": "7.16.0", "rel": 0,
        "desc": "E-book library manager",
        "url": "https://calibre-ebook.com/", "license": "GPL-3.0-only",
        "makedeps": "python3", "deps": "python3 qt6-base",
        "source": "https://github.com/kovidgoyal/calibre/archive/vVER.tar.gz",
        "build": "python3 setup.py build",
        "package_python": True,
    },
    "zathura": {
        "ver": "0.5.6", "rel": 0,
        "desc": "Minimalistic document viewer",
        "url": "https://pwmt.org/projects/zathura/", "license": "Zlib",
        "makedeps": "meson ninja gcc gtk+3.0-dev", "deps": "gtk+3.0",
        "source": "https://pwmt.org/projects/zathura/download/zathura-VER.tar.xz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },
    "flatpak": {
        "ver": "1.15.9", "rel": 0,
        "desc": "Flatpak application framework",
        "url": "https://flatpak.org/", "license": "LGPL-2.1-or-later",
        "makedeps": "meson ninja gcc", "deps": "",
        "source": "https://github.com/flatpak/flatpak/archive/VER.tar.gz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },
    "appstream": {
        "ver": "1.0.3", "rel": 0,
        "desc": "AppStream metadata library",
        "url": "https://www.freedesktop.org/wiki/Distributions/AppStream/", "license": "LGPL-2.1-or-later",
        "makedeps": "meson ninja gcc", "deps": "",
        "source": "https://github.com/ximion/appstream/archive/vVER.tar.gz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },

    # === FlatHub / extra runtime ===
    "gstreamer": {
        "ver": "1.24.6", "rel": 0,
        "desc": "GStreamer multimedia framework",
        "url": "https://gstreamer.freedesktop.org/", "license": "LGPL-2.1-or-later",
        "makedeps": "meson ninja gcc", "deps": "",
        "sub": "gstreamer-dev",
        "source": "https://gstreamer.freedesktop.org/src/gstreamer/gstreamer-VER.tar.xz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },
    "wxwidgets": {
        "ver": "3.2.5", "rel": 0,
        "desc": "wxWidgets GUI toolkit",
        "url": "https://www.wxwidgets.org/", "license": "wxWindows",
        "makedeps": "gcc make gtk+3.0-dev", "deps": "gtk+3.0",
        "sub": "wxwidgets-dev",
        "source": "https://github.com/wxWidgets/wxWidgets/archive/vVER.tar.gz",
        "build": "./configure --prefix=/usr && make",
    },
    "zig": {
        "ver": "0.13.0", "rel": 0,
        "desc": "Zig programming language",
        "url": "https://ziglang.org/", "license": "MIT",
        "makedeps": "cmake gcc make ninja", "deps": "",
        "source": "https://ziglang.org/download/VER/zig-VER.tar.xz",
        "build": "cmake -B build -DCMAKE_INSTALL_PREFIX=/usr && cmake --build build",
        "package_cmake": True,
    },
}


def generate(name, info):
    ver = info["ver"]
    lines = [
        f"# PaganLinux Build Script (pagbuild)",
        f"# {info['desc']}",
        "",
        f'maintainer="PaganLinux Team <team@paganlinux.eu>"',
        f'pkgname={name}',
        f'pkgver={ver}',
        f'pkgrel={info["rel"]}',
        f'pkgdesc="{info["desc"]}"',
        f'url="{info["url"]}"',
        f'arch="x86_64"',
        f'license="{info["license"]}"',
    ]
    lines.append(f'makedepends="{info.get("makedeps", "")}"')
    lines.append(f'depends="{info.get("deps", "")}"')
    sub = info.get("sub", "")
    if sub: lines.append(f'subpackages="{sub}"')
    src = info["source"].replace("VER", ver)
    lines.append(f'source="{src}"')
    lines.append(f'builddir="$srcdir"')
    lines.append("")

    build = info.get("build", "")
    if build and not info.get("is_binary"):
        lines.append("build() {")
        lines.append(f"    {build}")
        lines.append("}")
        lines.append("")

    if info.get("is_binary"):
        lines.append("# Binary package — no build step")
        lines.append("build() {")
        lines.append('    mkdir -p "$pkgdir"/usr/bin')
        if "deb" in src:
            lines.append('    ar x "$srcdir"/*.deb && tar -xf data.tar.* -C "$pkgdir"/')
        else:
            lines.append('    cp -a * "$pkgdir"/usr/')
        lines.append("}")
    elif info.get("package_cmake"):
        lines.append("package() {")
        lines.append('    cmake --install build --prefix="$pkgdir"/usr')
        lines.append("}")
    elif info.get("package_python"):
        lines.append("package() {")
        lines.append('    python3 setup.py install --root="$pkgdir"')
        lines.append("}")
    elif "package" in info:
        lines.append("package() {")
        lines.append(f"    {info['package']}")
        lines.append("}")
    elif build:
        lines.append("package() {")
        lines.append('    make DESTDIR="$pkgdir" install')
        lines.append("}")
    else:
        lines.append("package() {")
        lines.append('    mkdir -p "$pkgdir"/usr && cp -a * "$pkgdir"/usr/')
        lines.append("}")

    lines.append("")
    lines.append('# sha512sums=""')
    lines.append("")
    return "\n".join(lines)


def main():
    created = 0
    for name, info in POPULAR.items():
        dirpath = os.path.join(BASE, name)
        os.makedirs(dirpath, exist_ok=True)
        filepath = os.path.join(dirpath, "pagbuild")
        with open(filepath, "w") as f:
            f.write(generate(name, info))
        created += 1
    print(f"Created {created} popular package pagbuild files")

if __name__ == "__main__":
    main()
