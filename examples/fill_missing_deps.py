#!/usr/bin/env python3
"""Generate missing dependency pagbuild files to complete the pagports tree."""
import os

BASE = "/workspaces/pag/examples/pagports/main"

# Missing standalone packages (not subpackages)
MISSING = {
    # === Build essentials ===
    "bison": {
        "ver": "3.8.2", "rel": 0,
        "desc": "GNU parser generator",
        "url": "https://www.gnu.org/software/bison/",
        "license": "GPL-3.0-or-later",
        "makedeps": "gcc make",
        "source": "https://ftp.gnu.org/gnu/bison/bison-VER.tar.xz",
        "build": "./configure --prefix=/usr && make",
    },
    "gperf": {
        "ver": "3.1", "rel": 0,
        "desc": "Perfect hash function generator",
        "url": "https://www.gnu.org/software/gperf/",
        "license": "GPL-3.0-or-later",
        "makedeps": "gcc make",
        "source": "https://ftp.gnu.org/gnu/gperf/gperf-VER.tar.gz",
        "build": "./configure --prefix=/usr && make",
    },
    "linux-headers": {
        "ver": "6.10", "rel": 0,
        "desc": "Linux kernel headers",
        "url": "https://kernel.org/",
        "license": "GPL-2.0-only",
        "makedeps": "gcc make",
        "source": "https://cdn.kernel.org/pub/linux/kernel/v6.x/linux-VER.tar.xz",
        "build": "make headers_install INSTALL_HDR_PATH=/usr",
    },
    "make": {
        "ver": "4.4.1", "rel": 0,
        "desc": "GNU Make build tool",
        "url": "https://www.gnu.org/software/make/",
        "license": "GPL-3.0-or-later",
        "makedeps": "gcc",
        "source": "https://ftp.gnu.org/gnu/make/make-VER.tar.gz",
        "build": "./configure --prefix=/usr && make",
    },
    "ninja": {
        "ver": "1.12.1", "rel": 0,
        "desc": "Small build system with a focus on speed",
        "url": "https://ninja-build.org/",
        "license": "Apache-2.0",
        "makedeps": "cmake gcc make python3",
        "source": "https://github.com/ninja-build/ninja/archive/vVER.tar.gz",
        "build": "cmake -B build -DCMAKE_INSTALL_PREFIX=/usr && cmake --build build",
        "package_cmake": True,
    },
    "pkgconfig": {
        "ver": "2.1.1", "rel": 0,
        "desc": "pkg-config implementation (pkgconf)",
        "url": "https://github.com/pkgconf/pkgconf",
        "license": "ISC",
        "makedeps": "meson ninja gcc",
        "source": "https://github.com/pkgconf/pkgconf/archive/pkgconf-VER.tar.gz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },

    # === Compression / Archiving ===
    "zlib": {
        "ver": "1.3.1", "rel": 0,
        "desc": "Compression library",
        "url": "https://zlib.net/",
        "license": "Zlib",
        "makedeps": "gcc make",
        "deps": "",
        "sub": "zlib-dev",
        "source": "https://zlib.net/zlib-VER.tar.gz",
        "build": "./configure --prefix=/usr && make",
    },
    "expat": {
        "ver": "2.6.2", "rel": 0,
        "desc": "XML parser library",
        "url": "https://libexpat.github.io/",
        "license": "MIT",
        "makedeps": "gcc make",
        "deps": "",
        "sub": "expat-dev",
        "source": "https://github.com/libexpat/libexpat/releases/download/R_2_6_2/expat-VER.tar.xz",
        "build": "./configure --prefix=/usr && make",
    },

    # === Libs: PNG, XML, JSON, SSL ===
    "libpng": {
        "ver": "1.6.43", "rel": 0,
        "desc": "PNG reference library",
        "url": "http://www.libpng.org/pub/png/libpng.html",
        "license": "libpng",
        "makedeps": "gcc make zlib-dev",
        "deps": "zlib",
        "sub": "libpng-dev",
        "source": "https://download.sourceforge.net/libpng/libpng-VER.tar.xz",
        "build": "./configure --prefix=/usr && make",
    },
    "pixman": {
        "ver": "0.43.4", "rel": 0,
        "desc": "Pixel manipulation library",
        "url": "https://pixman.org/",
        "license": "MIT",
        "makedeps": "meson ninja gcc",
        "deps": "",
        "sub": "pixman-dev",
        "source": "https://cairographics.org/releases/pixman-VER.tar.gz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },
    "libxml2": {
        "ver": "2.13.3", "rel": 0,
        "desc": "XML parsing library",
        "url": "https://gitlab.gnome.org/GNOME/libxml2",
        "license": "MIT",
        "makedeps": "meson ninja gcc",
        "deps": "",
        "sub": "libxml2-dev",
        "source": "https://download.gnome.org/sources/libxml2/2.13/libxml2-VER.tar.xz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },
    "json-c": {
        "ver": "0.17", "rel": 0,
        "desc": "JSON implementation in C",
        "url": "https://github.com/json-c/json-c",
        "license": "MIT",
        "makedeps": "cmake gcc make",
        "deps": "",
        "sub": "json-c-dev",
        "source": "https://github.com/json-c/json-c/archive/json-c-VER.tar.gz",
        "build": "cmake -B build -DCMAKE_INSTALL_PREFIX=/usr && cmake --build build",
        "package_cmake": True,
    },
    "jsoncpp": {
        "ver": "1.9.5", "rel": 0,
        "desc": "C++ JSON library",
        "url": "https://github.com/open-source-parsers/jsoncpp",
        "license": "MIT",
        "makedeps": "meson ninja gcc",
        "deps": "",
        "sub": "jsoncpp-dev",
        "source": "https://github.com/open-source-parsers/jsoncpp/archive/VER.tar.gz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },
    "openssl": {
        "ver": "3.3.1", "rel": 0,
        "desc": "SSL/TLS toolkit",
        "url": "https://www.openssl.org/",
        "license": "Apache-2.0",
        "makedeps": "gcc make perl",
        "deps": "",
        "sub": "openssl-dev openssl-doc",
        "source": "https://www.openssl.org/source/openssl-VER.tar.gz",
        "build": "./Configure --prefix=/usr --openssldir=/etc/ssl shared && make",
    },
    "curl": {
        "ver": "8.9.1", "rel": 0,
        "desc": "URL retrieval library",
        "url": "https://curl.se/",
        "license": "curl",
        "makedeps": "gcc make openssl-dev",
        "deps": "openssl",
        "sub": "curl-dev",
        "source": "https://curl.se/download/curl-VER.tar.xz",
        "build": "./configure --prefix=/usr --with-openssl && make",
    },

    # === X11 libs ===
    "xcb-proto": {
        "ver": "1.17.0", "rel": 0,
        "desc": "XCB protocol descriptions",
        "url": "https://xcb.freedesktop.org/",
        "license": "MIT",
        "makedeps": "python3",
        "deps": "",
        "sub": "",
        "source": "https://xorg.freedesktop.org/archive/individual/proto/xcb-proto-VER.tar.xz",
        "build": "./configure --prefix=/usr && make",
    },
    "libxau": {
        "ver": "1.0.11", "rel": 0,
        "desc": "X11 authorisation library",
        "url": "https://www.x.org/",
        "license": "MIT",
        "makedeps": "gcc make xorgproto",
        "deps": "xorgproto",
        "sub": "libxau-dev",
        "source": "https://www.x.org/releases/individual/lib/libXau-VER.tar.xz",
        "build": "./configure --prefix=/usr && make",
    },
    "libxext": {
        "ver": "1.3.6", "rel": 0,
        "desc": "X11 miscellaneous extensions library",
        "url": "https://www.x.org/",
        "license": "MIT",
        "makedeps": "gcc make xorgproto libx11-dev",
        "deps": "xorgproto libx11",
        "sub": "libxext-dev",
        "source": "https://www.x.org/releases/individual/lib/libXext-VER.tar.xz",
        "build": "./configure --prefix=/usr && make",
    },
    "libxrender": {
        "ver": "0.9.11", "rel": 0,
        "desc": "X Rendering Extension library",
        "url": "https://www.x.org/",
        "license": "MIT",
        "makedeps": "gcc make xorgproto libx11-dev",
        "deps": "xorgproto libx11",
        "sub": "libxrender-dev",
        "source": "https://www.x.org/releases/individual/lib/libXrender-VER.tar.xz",
        "build": "./configure --prefix=/usr && make",
    },
    "libxrandr": {
        "ver": "1.5.4", "rel": 0,
        "desc": "X RandR extension library",
        "url": "https://www.x.org/",
        "license": "MIT",
        "makedeps": "gcc make xorgproto libx11-dev libxrender-dev libxext-dev",
        "deps": "xorgproto libx11 libxrender libxext",
        "sub": "libxrandr-dev",
        "source": "https://www.x.org/releases/individual/lib/libXrandr-VER.tar.xz",
        "build": "./configure --prefix=/usr && make",
    },
    "libxfixes": {
        "ver": "6.0.1", "rel": 0,
        "desc": "X Fixes extension library",
        "url": "https://www.x.org/",
        "license": "MIT",
        "makedeps": "gcc make xorgproto libx11-dev",
        "deps": "xorgproto libx11",
        "sub": "libxfixes-dev",
        "source": "https://www.x.org/releases/individual/lib/libXfixes-VER.tar.xz",
        "build": "./configure --prefix=/usr && make",
    },
    "libxi": {
        "ver": "1.8.1", "rel": 0,
        "desc": "X Input extension library",
        "url": "https://www.x.org/",
        "license": "MIT",
        "makedeps": "gcc make xorgproto libx11-dev libxext-dev libxfixes-dev",
        "deps": "xorgproto libx11 libxext libxfixes",
        "sub": "libxi-dev",
        "source": "https://www.x.org/releases/individual/lib/libXi-VER.tar.xz",
        "build": "./configure --prefix=/usr && make",
    },
    "libxtst": {
        "ver": "1.2.5", "rel": 0,
        "desc": "X Test extension library",
        "url": "https://www.x.org/",
        "license": "MIT",
        "makedeps": "gcc make xorgproto libx11-dev libxext-dev libxi-dev",
        "deps": "xorgproto libx11 libxext libxi",
        "sub": "libxtst-dev",
        "source": "https://www.x.org/releases/individual/lib/libXtst-VER.tar.xz",
        "build": "./configure --prefix=/usr && make",
    },
    "libxdamage": {
        "ver": "1.1.6", "rel": 0,
        "desc": "X Damage extension library",
        "url": "https://www.x.org/",
        "license": "MIT",
        "makedeps": "gcc make xorgproto libx11-dev libxfixes-dev",
        "deps": "xorgproto libx11 libxfixes",
        "sub": "libxdamage-dev",
        "source": "https://www.x.org/releases/individual/lib/libXdamage-VER.tar.xz",
        "build": "./configure --prefix=/usr && make",
    },
    "libxcomposite": {
        "ver": "0.4.6", "rel": 0,
        "desc": "X Composite extension library",
        "url": "https://www.x.org/",
        "license": "MIT",
        "makedeps": "gcc make xorgproto libx11-dev libxfixes-dev",
        "deps": "xorgproto libx11 libxfixes",
        "sub": "libxcomposite-dev",
        "source": "https://www.x.org/releases/individual/lib/libXcomposite-VER.tar.xz",
        "build": "./configure --prefix=/usr && make",
    },
    "libxcursor": {
        "ver": "1.2.2", "rel": 0,
        "desc": "X Cursor library",
        "url": "https://www.x.org/",
        "license": "MIT",
        "makedeps": "gcc make xorgproto libx11-dev libxrender-dev libxfixes-dev",
        "deps": "xorgproto libx11 libxrender libxfixes",
        "sub": "libxcursor-dev",
        "source": "https://www.x.org/releases/individual/lib/libXcursor-VER.tar.xz",
        "build": "./configure --prefix=/usr && make",
    },
    "libxinerama": {
        "ver": "1.1.5", "rel": 0,
        "desc": "Xinerama extension library",
        "url": "https://www.x.org/",
        "license": "MIT",
        "makedeps": "gcc make xorgproto libx11-dev libxext-dev",
        "deps": "xorgproto libx11 libxext",
        "sub": "libxinerama-dev",
        "source": "https://www.x.org/releases/individual/lib/libXinerama-VER.tar.xz",
        "build": "./configure --prefix=/usr && make",
    },
    "libxxf86vm": {
        "ver": "1.1.5", "rel": 0,
        "desc": "X11 XFree86 video mode extension library",
        "url": "https://www.x.org/",
        "license": "MIT",
        "makedeps": "gcc make xorgproto libx11-dev libxext-dev",
        "deps": "xorgproto libx11 libxext",
        "sub": "libxxf86vm-dev",
        "source": "https://www.x.org/releases/individual/lib/libXxf86vm-VER.tar.xz",
        "build": "./configure --prefix=/usr && make",
    },
    "libxkbfile": {
        "ver": "1.1.3", "rel": 0,
        "desc": "X keyboard file manipulation library",
        "url": "https://www.x.org/",
        "license": "MIT",
        "makedeps": "gcc make xorgproto libx11-dev",
        "deps": "xorgproto libx11",
        "sub": "libxkbfile-dev",
        "source": "https://www.x.org/releases/individual/lib/libxkbfile-VER.tar.xz",
        "build": "./configure --prefix=/usr && make",
    },
    "libxscrnsaver": {
        "ver": "1.2.4", "rel": 0,
        "desc": "X11 Screen Saver extension library",
        "url": "https://www.x.org/",
        "license": "MIT",
        "makedeps": "gcc make xorgproto libx11-dev libxext-dev",
        "deps": "xorgproto libx11 libxext",
        "sub": "libxscrnsaver-dev",
        "source": "https://www.x.org/releases/individual/lib/libXScrnSaver-VER.tar.xz",
        "build": "./configure --prefix=/usr && make",
    },
    "libxpm": {
        "ver": "3.5.17", "rel": 0,
        "desc": "X Pixmap library",
        "url": "https://www.x.org/",
        "license": "MIT",
        "makedeps": "gcc make xorgproto libx11-dev",
        "deps": "xorgproto libx11",
        "sub": "libxpm-dev",
        "source": "https://www.x.org/releases/individual/lib/libXpm-VER.tar.xz",
        "build": "./configure --prefix=/usr && make",
    },
    "xkeyboard-config": {
        "ver": "2.42", "rel": 0,
        "desc": "X Keyboard Configuration Database",
        "url": "https://www.freedesktop.org/wiki/Software/XKeyboardConfig/",
        "license": "MIT",
        "makedeps": "meson ninja",
        "deps": "xorgproto",
        "sub": "",
        "source": "https://www.x.org/releases/individual/data/xkeyboard-config/xkeyboard-config-VER.tar.xz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },

    # === Video / Acceleration ===
    "libvdpau": {
        "ver": "1.5", "rel": 0,
        "desc": "Video Decode and Presentation API",
        "url": "https://gitlab.freedesktop.org/vdpau/libvdpau",
        "license": "MIT",
        "makedeps": "meson ninja gcc xorgproto libx11-dev libxext-dev",
        "deps": "xorgproto libx11 libxext",
        "sub": "libvdpau-dev",
        "source": "https://gitlab.freedesktop.org/vdpau/libvdpau/-/archive/VER/libvdpau-VER.tar.gz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },
    "libva": {
        "ver": "2.21.0", "rel": 0,
        "desc": "Video Acceleration API",
        "url": "https://github.com/intel/libva",
        "license": "MIT",
        "makedeps": "meson ninja gcc libdrm-dev libx11-dev",
        "deps": "libdrm libx11",
        "sub": "libva-dev",
        "source": "https://github.com/intel/libva/archive/VER.tar.gz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },

    # === Input ===
    "mtdev": {
        "ver": "1.1.7", "rel": 0,
        "desc": "Multitouch Protocol Translation Library",
        "url": "https://github.com/whot/mtdev",
        "license": "MIT",
        "makedeps": "gcc make",
        "deps": "",
        "sub": "mtdev-dev",
        "source": "https://github.com/whot/mtdev/archive/vVER.tar.gz",
        "build": "./autogen.sh --prefix=/usr && make",
    },

    # === Audio ===
    "pulseaudio": {
        "ver": "17.0", "rel": 0,
        "desc": "PulseAudio sound server",
        "url": "https://www.freedesktop.org/wiki/Software/PulseAudio/",
        "license": "LGPL-2.1-or-later",
        "makedeps": "meson ninja gcc dbus-dev",
        "deps": "dbus",
        "sub": "pulseaudio-dev",
        "source": "https://www.freedesktop.org/software/pulseaudio/releases/pulseaudio-VER.tar.xz",
        "build": "meson setup build -Dprefix=/usr -Ddaemon=false && ninja -C build",
    },

    # === Build tools ===
    "nasm": {
        "ver": "2.16.03", "rel": 0,
        "desc": "NASM assembler",
        "url": "https://www.nasm.us/",
        "license": "BSD-2-Clause",
        "makedeps": "gcc make",
        "deps": "",
        "sub": "",
        "source": "https://www.nasm.us/pub/nasm/releasebuilds/VER/nasm-VER.tar.xz",
        "build": "./configure --prefix=/usr && make",
    },
    "yasm": {
        "ver": "1.3.0", "rel": 0,
        "desc": "Yasm assembler",
        "url": "https://yasm.tortall.net/",
        "license": "BSD",
        "makedeps": "gcc make",
        "deps": "",
        "sub": "",
        "source": "https://www.tortall.net/projects/yasm/releases/yasm-VER.tar.gz",
        "build": "./configure --prefix=/usr && make",
    },
    "perl": {
        "ver": "5.40.0", "rel": 0,
        "desc": "Perl programming language",
        "url": "https://www.perl.org/",
        "license": "GPL-1.0-or-later OR Artistic-1.0",
        "makedeps": "gcc make",
        "deps": "",
        "sub": "perl-doc",
        "source": "https://www.cpan.org/src/5.0/perl-VER.tar.gz",
        "build": "./Configure -des -Dprefix=/usr && make",
    },
    "python3-mako": {
        "ver": "1.3.5", "rel": 0,
        "desc": "Mako template library for Python",
        "url": "https://www.makotemplates.org/",
        "license": "MIT",
        "makedeps": "python3",
        "deps": "python3",
        "sub": "",
        "source": "https://files.pythonhosted.org/packages/source/M/Mako/Mako-VER.tar.gz",
        "build": "python3 setup.py build",
        "package_python": True,
    },

    # === System libs ===
    "libcap": {
        "ver": "2.70", "rel": 0,
        "desc": "POSIX capabilities library",
        "url": "https://sites.google.com/site/fullycapable/",
        "license": "GPL-2.0-only",
        "makedeps": "gcc make",
        "deps": "",
        "sub": "libcap-dev",
        "source": "https://git.kernel.org/pub/scm/linux/kernel/git/morgan/libcap.git/snapshot/libcap-VER.tar.gz",
        "build": "make prefix=/usr",
    },
    "util-linux": {
        "ver": "2.40.2", "rel": 0,
        "desc": "Linux system utilities",
        "url": "https://github.com/util-linux/util-linux",
        "license": "GPL-2.0-or-later",
        "makedeps": "gcc make",
        "deps": "",
        "sub": "util-linux-dev",
        "source": "https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.40/util-linux-VER.tar.xz",
        "build": "./configure --prefix=/usr && make",
    },
    "audit": {
        "ver": "4.0.1", "rel": 0,
        "desc": "Linux audit framework",
        "url": "https://people.redhat.com/sgrubb/audit/",
        "license": "GPL-2.0-or-later",
        "makedeps": "gcc make",
        "deps": "",
        "sub": "audit-dev",
        "source": "https://people.redhat.com/sgrubb/audit/audit-VER.tar.gz",
        "build": "./configure --prefix=/usr && make",
    },

    # === GTK ===
    "fribidi": {
        "ver": "1.0.15", "rel": 0,
        "desc": "Free Implementation of the Unicode Bidirectional Algorithm",
        "url": "https://github.com/fribidi/fribidi",
        "license": "LGPL-2.1-or-later",
        "makedeps": "meson ninja gcc",
        "deps": "",
        "sub": "fribidi-dev",
        "source": "https://github.com/fribidi/fribidi/archive/vVER.tar.gz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },
    "gtk+2.0": {
        "ver": "2.24.33", "rel": 0,
        "desc": "GTK+2 GUI toolkit",
        "url": "https://www.gtk.org/",
        "license": "LGPL-2.1-or-later",
        "makedeps": "gcc make pkgconfig pango-dev cairo-dev",
        "deps": "pango cairo",
        "sub": "gtk+2.0-dev",
        "source": "https://download.gnome.org/sources/gtk+/2.24/gtk+-VER.tar.xz",
        "build": "./configure --prefix=/usr && make",
    },
    "gtk+3.0": {
        "ver": "3.24.43", "rel": 0,
        "desc": "GTK+3 GUI toolkit",
        "url": "https://www.gtk.org/",
        "license": "LGPL-2.1-or-later",
        "makedeps": "meson ninja gcc pkgconfig pango-dev cairo-dev",
        "deps": "pango cairo",
        "sub": "gtk+3.0-dev",
        "source": "https://download.gnome.org/sources/gtk+/3.24/gtk+-VER.tar.xz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },

    # === Security ===
    "nss": {
        "ver": "3.103", "rel": 0,
        "desc": "Network Security Services",
        "url": "https://developer.mozilla.org/docs/Mozilla/Projects/NSS",
        "license": "MPL-2.0",
        "makedeps": "gcc make",
        "deps": "nspr",
        "sub": "nss-dev",
        "source": "https://ftp.mozilla.org/pub/security/nss/releases/NSS_3_103_RTM/src/nss-VER.tar.gz",
        "build": "make -C nss BUILD_OPT=1 USE_SYSTEM_ZLIB=1 NSS_USE_SYSTEM_SQLITE=1",
    },
    "nspr": {
        "ver": "4.35", "rel": 0,
        "desc": "Netscape Portable Runtime",
        "url": "https://developer.mozilla.org/docs/Mozilla/Projects/NSPR",
        "license": "MPL-2.0",
        "makedeps": "gcc make",
        "deps": "",
        "sub": "nspr-dev",
        "source": "https://ftp.mozilla.org/pub/nspr/releases/vVER/src/nspr-VER.tar.gz",
        "build": "./configure --prefix=/usr && make",
    },

    # === Wayland / Hyprland deps ===
    "hyprlang": {
        "ver": "0.5.3", "rel": 0,
        "desc": "Hyprland configuration language",
        "url": "https://github.com/hyprwm/hyprlang",
        "license": "BSD-3-Clause",
        "makedeps": "cmake gcc make",
        "deps": "",
        "sub": "hyprlang-dev",
        "source": "https://github.com/hyprwm/hyprlang/archive/vVER.tar.gz",
        "build": "cmake -B build -DCMAKE_INSTALL_PREFIX=/usr && cmake --build build",
        "package_cmake": True,
    },
    "hyprcursor": {
        "ver": "0.1.10", "rel": 0,
        "desc": "Hyprland cursor utility",
        "url": "https://github.com/hyprwm/hyprcursor",
        "license": "BSD-3-Clause",
        "makedeps": "cmake gcc make",
        "deps": "",
        "sub": "hyprcursor-dev",
        "source": "https://github.com/hyprwm/hyprcursor/archive/vVER.tar.gz",
        "build": "cmake -B build -DCMAKE_INSTALL_PREFIX=/usr && cmake --build build",
        "package_cmake": True,
    },
    "hyprgraphics": {
        "ver": "0.1.1", "rel": 0,
        "desc": "Hyprland graphics utilities",
        "url": "https://github.com/hyprwm/hyprgraphics",
        "license": "BSD-3-Clause",
        "makedeps": "cmake gcc make",
        "deps": "cairo pixman",
        "sub": "hyprgraphics-dev",
        "source": "https://github.com/hyprwm/hyprgraphics/archive/vVER.tar.gz",
        "build": "cmake -B build -DCMAKE_INSTALL_PREFIX=/usr && cmake --build build",
        "package_cmake": True,
    },
    "hyprutils": {
        "ver": "0.2.3", "rel": 0,
        "desc": "Hyprland utility library",
        "url": "https://github.com/hyprwm/hyprutils",
        "license": "BSD-3-Clause",
        "makedeps": "cmake gcc make",
        "deps": "",
        "sub": "hyprutils-dev",
        "source": "https://github.com/hyprwm/hyprutils/archive/vVER.tar.gz",
        "build": "cmake -B build -DCMAKE_INSTALL_PREFIX=/usr && cmake --build build",
        "package_cmake": True,
    },
    "aquamarine": {
        "ver": "0.4.2", "rel": 0,
        "desc": "Hyprland rendering backend library",
        "url": "https://github.com/hyprwm/aquamarine",
        "license": "BSD-3-Clause",
        "makedeps": "cmake gcc make wayland-dev",
        "deps": "wayland",
        "sub": "aquamarine-dev",
        "source": "https://github.com/hyprwm/aquamarine/archive/vVER.tar.gz",
        "build": "cmake -B build -DCMAKE_INSTALL_PREFIX=/usr && cmake --build build",
        "package_cmake": True,
    },
    "swaybg": {
        "ver": "1.2.1", "rel": 0,
        "desc": "Wallpaper daemon for Wayland compositors",
        "url": "https://github.com/swaywm/swaybg",
        "license": "MIT",
        "makedeps": "meson ninja gcc wayland-dev wayland-protocols cairo-dev",
        "deps": "wayland cairo",
        "sub": "",
        "source": "https://github.com/swaywm/swaybg/archive/vVER.tar.gz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },

    # === KDE Frameworks ===
    "kio": {
        "ver": "6.4.0", "rel": 0,
        "desc": "KDE Input/Output framework",
        "url": "https://api.kde.org/frameworks/kio/html/",
        "license": "LGPL-2.0-or-later",
        "makedeps": "cmake gcc make ninja qt6-base-dev",
        "deps": "qt6-base",
        "sub": "kio-dev",
        "source": "https://download.kde.org/stable/frameworks/6.4.0/kio-VER.tar.xz",
        "build": "cmake -B build -DCMAKE_INSTALL_PREFIX=/usr && cmake --build build",
        "package_cmake": True,
    },
    "qt6-declarative": {
        "ver": "6.7.2", "rel": 0,
        "desc": "Qt6 QML and declarative engine",
        "url": "https://www.qt.io/",
        "license": "LGPL-3.0-only OR GPL-3.0-only",
        "makedeps": "cmake gcc make ninja qt6-base-dev",
        "deps": "qt6-base",
        "sub": "qt6-declarative-dev",
        "source": "https://download.qt.io/official_releases/qt/6.7/6.7.2/submodules/qtdeclarative-everywhere-src-VER.tar.xz",
        "build": "cmake -B build -DCMAKE_INSTALL_PREFIX=/usr && cmake --build build",
        "package_cmake": True,
    },
    "qt6-svg": {
        "ver": "6.7.2", "rel": 0,
        "desc": "Qt6 SVG module",
        "url": "https://www.qt.io/",
        "license": "LGPL-3.0-only OR GPL-3.0-only",
        "makedeps": "cmake gcc make ninja qt6-base-dev",
        "deps": "qt6-base",
        "sub": "",
        "source": "https://download.qt.io/official_releases/qt/6.7/6.7.2/submodules/qtsvg-everywhere-src-VER.tar.xz",
        "build": "cmake -B build -DCMAKE_INSTALL_PREFIX=/usr && cmake --build build",
        "package_cmake": True,
    },
    "qt6-tools": {
        "ver": "6.7.2", "rel": 0,
        "desc": "Qt6 development tools",
        "url": "https://www.qt.io/",
        "license": "LGPL-3.0-only OR GPL-3.0-only",
        "makedeps": "cmake gcc make ninja qt6-base-dev qt6-declarative-dev",
        "deps": "qt6-base qt6-declarative",
        "sub": "",
        "source": "https://download.qt.io/official_releases/qt/6.7/6.7.2/submodules/qttools-everywhere-src-VER.tar.xz",
        "build": "cmake -B build -DCMAKE_INSTALL_PREFIX=/usr && cmake --build build",
        "package_cmake": True,
    },
    "qt6ct": {
        "ver": "0.9", "rel": 0,
        "desc": "Qt6 Configuration Tool",
        "url": "https://github.com/trialuser02/qt6ct",
        "license": "BSD-2-Clause",
        "makedeps": "cmake gcc make qt6-base-dev qt6-tools",
        "deps": "qt6-base",
        "sub": "",
        "source": "https://github.com/trialuser02/qt6ct/archive/VER.tar.gz",
        "build": "cmake -B build -DCMAKE_INSTALL_PREFIX=/usr && cmake --build build",
        "package_cmake": True,
    },

    # === Plasma addons ===
    "plasma-pa": {
        "ver": "6.1.3", "rel": 0,
        "desc": "KDE Plasma audio volume applet",
        "url": "https://kde.org/plasma-desktop/",
        "license": "GPL-2.0-or-later",
        "makedeps": "cmake gcc make ninja qt6-base-dev plasma-workspace-dev",
        "deps": "qt6-base plasma-workspace pulseaudio",
        "sub": "",
        "source": "https://download.kde.org/stable/plasma/6.1.3/plasma-pa-VER.tar.xz",
        "build": "cmake -B build -DCMAKE_INSTALL_PREFIX=/usr && cmake --build build",
        "package_cmake": True,
    },
    "plasma-nm": {
        "ver": "6.1.3", "rel": 0,
        "desc": "KDE Plasma Network Manager applet",
        "url": "https://kde.org/plasma-desktop/",
        "license": "GPL-2.0-or-later",
        "makedeps": "cmake gcc make ninja qt6-base-dev plasma-workspace-dev networkmanager-dev",
        "deps": "qt6-base plasma-workspace networkmanager",
        "sub": "",
        "source": "https://download.kde.org/stable/plasma/6.1.3/plasma-nm-VER.tar.xz",
        "build": "cmake -B build -DCMAKE_INSTALL_PREFIX=/usr && cmake --build build",
        "package_cmake": True,
    },
    "plasma-browser-integration": {
        "ver": "6.1.3", "rel": 0,
        "desc": "KDE Plasma browser integration",
        "url": "https://kde.org/plasma-desktop/",
        "license": "GPL-3.0-or-later",
        "makedeps": "cmake gcc make ninja qt6-base-dev",
        "deps": "qt6-base",
        "sub": "",
        "source": "https://download.kde.org/stable/plasma/6.1.3/plasma-browser-integration-VER.tar.xz",
        "build": "cmake -B build -DCMAKE_INSTALL_PREFIX=/usr && cmake --build build",
        "package_cmake": True,
    },

    # === More Wayland tools ===
    "wdisplays": {
        "ver": "1.1.1", "rel": 0,
        "desc": "GUI display configurator for wlroots",
        "url": "https://github.com/artizirk/wdisplays",
        "license": "MIT",
        "makedeps": "meson ninja gcc wayland-dev wlroots-dev gtk+3.0-dev",
        "deps": "wayland wlroots gtk+3.0",
        "source": "https://github.com/artizirk/wdisplays/archive/VER.tar.gz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },
    "wlr-randr": {
        "ver": "0.4.1", "rel": 0,
        "desc": "xrandr clone for wlroots compositors",
        "url": "https://sr.ht/~emersion/wlr-randr/",
        "license": "MIT",
        "makedeps": "meson ninja gcc wayland-dev",
        "deps": "wayland",
        "source": "https://git.sr.ht/~emersion/wlr-randr/archive/vVER.tar.gz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },
    "mako": {
        "ver": "1.9.0", "rel": 0,
        "desc": "Lightweight Wayland notification daemon",
        "url": "https://github.com/emersion/mako",
        "license": "MIT",
        "makedeps": "meson ninja gcc wayland-dev pango-dev cairo-dev",
        "deps": "wayland pango cairo",
        "source": "https://github.com/emersion/mako/archive/vVER.tar.gz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },
    "wlsunset": {
        "ver": "0.4.0", "rel": 0,
        "desc": "Day/night gamma adjustments for Wayland",
        "url": "https://sr.ht/~kennylevinsen/wlsunset/",
        "license": "MIT",
        "makedeps": "meson ninja gcc wayland-dev",
        "deps": "wayland",
        "source": "https://git.sr.ht/~kennylevinsen/wlsunset/archive/VER.tar.gz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },
    "swaylock": {
        "ver": "1.7.2", "rel": 0,
        "desc": "Screen locker for Wayland",
        "url": "https://github.com/swaywm/swaylock",
        "license": "MIT",
        "makedeps": "meson ninja gcc wayland-dev wayland-protocols cairo-dev pango-dev",
        "deps": "wayland cairo pango",
        "source": "https://github.com/swaywm/swaylock/archive/vVER.tar.gz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },
    "swayidle": {
        "ver": "1.8.0", "rel": 0,
        "desc": "Idle management daemon for Wayland",
        "url": "https://github.com/swaywm/swayidle",
        "license": "MIT",
        "makedeps": "meson ninja gcc wayland-dev wayland-protocols",
        "deps": "wayland",
        "source": "https://github.com/swaywm/swayidle/archive/VER.tar.gz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },
    "kanshi": {
        "ver": "1.7.0", "rel": 0,
        "desc": "Dynamic display configuration for Wayland",
        "url": "https://sr.ht/~emersion/kanshi/",
        "license": "MIT",
        "makedeps": "meson ninja gcc wayland-dev",
        "deps": "wayland",
        "source": "https://git.sr.ht/~emersion/kanshi/archive/vVER.tar.gz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },

    # === Display manager ===
    "lightdm": {
        "ver": "1.32.0", "rel": 0,
        "desc": "Lightweight Display Manager",
        "url": "https://github.com/canonical/lightdm",
        "license": "GPL-3.0-or-later",
        "makedeps": "gcc make pkgconfig",
        "deps": "",
        "sub": "",
        "source": "https://github.com/canonical/lightdm/archive/VER.tar.gz",
        "build": "./configure --prefix=/usr && make",
    },

    # === Networking ===
    "iwd": {
        "ver": "2.19", "rel": 0,
        "desc": "Wireless daemon for Linux",
        "url": "https://iwd.wiki.kernel.org/",
        "license": "LGPL-2.1-or-later",
        "makedeps": "gcc make",
        "deps": "",
        "sub": "",
        "source": "https://git.kernel.org/pub/scm/network/wireless/iwd.git/snapshot/iwd-VER.tar.gz",
        "build": "./configure --prefix=/usr && make",
    },
    "bluez": {
        "ver": "5.77", "rel": 0,
        "desc": "Bluetooth protocol stack for Linux",
        "url": "https://www.bluez.org/",
        "license": "GPL-2.0-or-later",
        "makedeps": "gcc make",
        "deps": "",
        "sub": "bluez-dev",
        "source": "https://mirrors.edge.kernel.org/pub/linux/bluetooth/bluez-VER.tar.xz",
        "build": "./configure --prefix=/usr && make",
    },

    # === Multimedia ===
    "pavucontrol": {
        "ver": "5.0", "rel": 0,
        "desc": "PulseAudio Volume Control",
        "url": "https://freedesktop.org/software/pulseaudio/pavucontrol/",
        "license": "GPL-2.0-or-later",
        "makedeps": "meson ninja gcc gtk+3.0-dev pulseaudio-dev",
        "deps": "gtk+3.0 pulseaudio",
        "source": "https://freedesktop.org/software/pulseaudio/pavucontrol/pavucontrol-VER.tar.xz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },

    # === Font utilities ===
    "libjpeg-turbo": {
        "ver": "3.0.3", "rel": 0,
        "desc": "JPEG image codec library",
        "url": "https://libjpeg-turbo.org/",
        "license": "IJG",
        "makedeps": "cmake gcc make nasm",
        "deps": "",
        "sub": "libjpeg-turbo-dev",
        "source": "https://github.com/libjpeg-turbo/libjpeg-turbo/archive/VER.tar.gz",
        "build": "cmake -B build -DCMAKE_INSTALL_PREFIX=/usr && cmake --build build",
        "package_cmake": True,
    },

    # === Desktop utils ===
    "fastfetch": {
        "ver": "2.21.3", "rel": 0,
        "desc": "Fast system information tool",
        "url": "https://github.com/fastfetch-cli/fastfetch",
        "license": "MIT",
        "makedeps": "cmake gcc make pkgconfig",
        "deps": "",
        "source": "https://github.com/fastfetch-cli/fastfetch/archive/VER.tar.gz",
        "build": "cmake -B build -DCMAKE_INSTALL_PREFIX=/usr && cmake --build build",
        "package_cmake": True,
    },

    # === KDE extras ===
    "oxygen": {
        "ver": "6.1.3", "rel": 0,
        "desc": "KDE Oxygen theme",
        "url": "https://kde.org/plasma-desktop/",
        "license": "GPL-2.0-or-later",
        "makedeps": "cmake gcc make ninja qt6-base-dev",
        "deps": "qt6-base",
        "source": "https://download.kde.org/stable/plasma/6.1.3/oxygen-VER.tar.xz",
        "build": "cmake -B build -DCMAKE_INSTALL_PREFIX=/usr && cmake --build build",
        "package_cmake": True,
    },
    "breeze-gtk": {
        "ver": "6.1.3", "rel": 0,
        "desc": "KDE Breeze GTK theme",
        "url": "https://kde.org/plasma-desktop/",
        "license": "GPL-2.0-or-later",
        "makedeps": "cmake gcc make ninja gtk+3.0-dev",
        "deps": "gtk+3.0 breeze",
        "source": "https://download.kde.org/stable/plasma/6.1.3/breeze-gtk-VER.tar.xz",
        "build": "cmake -B build -DCMAKE_INSTALL_PREFIX=/usr && cmake --build build",
        "package_cmake": True,
    },
    "kwrite": {
        "ver": "24.05.2", "rel": 0,
        "desc": "KDE Simple Text Editor",
        "url": "https://apps.kde.org/kwrite/",
        "license": "LGPL-2.0-or-later",
        "makedeps": "cmake gcc make ninja qt6-base-dev",
        "deps": "qt6-base",
        "source": "https://download.kde.org/stable/release-service/24.05.2/src/kwrite-VER.tar.xz",
        "build": "cmake -B build -DCMAKE_INSTALL_PREFIX=/usr && cmake --build build",
        "package_cmake": True,
    },

    # === Other missing deps ===
    "dbus-glib": {
        "ver": "0.112", "rel": 0,
        "desc": "GLib bindings for D-Bus",
        "url": "https://dbus.freedesktop.org/doc/dbus-glib/",
        "license": "AFL-2.1 OR GPL-2.0-or-later",
        "makedeps": "gcc make pkgconfig dbus-dev",
        "deps": "dbus",
        "sub": "dbus-glib-dev",
        "source": "https://dbus.freedesktop.org/releases/dbus-glib/dbus-glib-VER.tar.gz",
        "build": "./configure --prefix=/usr && make",
    },
    "elogind": {
        "ver": "255.5", "rel": 0,
        "desc": "Standalone logind implementation",
        "url": "https://github.com/elogind/elogind",
        "license": "LGPL-2.1-or-later",
        "makedeps": "meson ninja gcc",
        "deps": "",
        "sub": "elogind-dev",
        "source": "https://github.com/elogind/elogind/archive/vVER.tar.gz",
        "build": "meson setup build -Dprefix=/usr && ninja -C build",
    },
    "libwayland": {
        "ver": "1.23.0", "rel": 0,
        "desc": "Wayland C library (libwayland-client/server)",
        "url": "https://wayland.freedesktop.org/",
        "license": "MIT",
        "makedeps": "meson ninja gcc expat-dev libxml2-dev",
        "deps": "expat libxml2",
        "sub": "libwayland-dev",
        "source": "https://gitlab.freedesktop.org/wayland/wayland/-/releases/VER/downloads/wayland-VER.tar.xz",
        "build": "meson setup build -Dprefix=/usr -Ddocumentation=false && ninja -C build",
    },
    "kde-applications": {
        "ver": "26.04.2", "rel": 0,
        "desc": "KDE Applications meta package",
        "url": "https://kde.org/applications/",
        "license": "GPL-3.0-or-later",
        "makedeps": "",
        "deps": "kde-applications-accessibility kde-applications-admin kde-applications-base kde-applications-edu kde-applications-games kde-applications-graphics kde-applications-multimedia kde-applications-network kde-applications-pim kde-applications-sdk kde-applications-utils kde-applications-webdev glibc systemd",
        "source": "",
        "build": "",
    },

    # dbus-x11 subpackage handled by dbus itself
    # X11 xdmcp
    "libxdmcp": {
        "ver": "1.1.5", "rel": 0,
        "desc": "X Display Manager Control Protocol library",
        "url": "https://www.x.org/",
        "license": "MIT",
        "makedeps": "gcc make xorgproto",
        "deps": "xorgproto",
        "sub": "libxdmcp-dev",
        "source": "https://www.x.org/releases/individual/lib/libXdmcp-VER.tar.xz",
        "build": "./configure --prefix=/usr && make",
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

    makedeps = info.get("makedeps", "")
    lines.append(f'makedepends="{makedeps}"')

    deps = info.get("deps", "")
    lines.append(f'depends="{deps}"')

    sub = info.get("sub", "")
    if sub:
        lines.append(f'subpackages="{sub}"')

    source = info["source"].replace("VER", ver)
    lines.append(f'source="{source}"')
    lines.append(f'builddir="$srcdir"')
    lines.append("")

    build = info.get("build", "")
    if build:
        lines.append("build() {")
        lines.append(f"    {build}")
        lines.append("}")
        lines.append("")

    if info.get("package_cmake"):
        lines.append("package() {")
        lines.append('    cmake --install build --prefix="$pkgdir"/usr')
        lines.append("}")
    elif info.get("package_python"):
        lines.append("package() {")
        lines.append('    python3 setup.py install --root="$pkgdir"')
        lines.append("}")
    elif name == "kde-applications":
        lines.append("package() {")
        lines.append('    mkdir -p "$pkgdir"')
        lines.append("}")
    elif build:
        lines.append("package() {")
        lines.append('    make DESTDIR="$pkgdir" install')
        lines.append("}")
    elif not build:
        lines.append("package() {")
        lines.append('    mkdir -p "$pkgdir"/usr && cp -a * "$pkgdir"/usr/')
        lines.append("}")

    lines.append("")
    lines.append('# sha512sums=""')
    lines.append("")
    return "\n".join(lines)


def main():
    created = 0
    for name, info in MISSING.items():
        dirpath = os.path.join(BASE, name)
        os.makedirs(dirpath, exist_ok=True)
        filepath = os.path.join(dirpath, "pagbuild")
        # Skip if already exists and not in our MISSING dict as new
        if os.path.exists(filepath) and name not in MISSING:
            continue
        content = generate(name, info)
        with open(filepath, "w") as f:
            f.write(content)
        created += 1
    print(f"Created {created} new pagbuild files")

if __name__ == "__main__":
    main()
