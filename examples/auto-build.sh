#!/bin/bash
# PaganLinux Auto-Build System
# Automatycznie buduje pakiety .pag z pagports i wysyła do repozytorium
#
# Użycie:
#   ./auto-build.sh [--all] [--repo core|extra|community] [package-name]
#
# Cron (co 6h):
#   0 */6 * * * /opt/pag/examples/auto-build.sh --changed >> /var/log/pag/autobuild.log 2>&1

set -euo pipefail

PAGPORTS_DIR="${PAGPORTS_DIR:-/opt/pagports/main}"
OUTPUT_DIR="${OUTPUT_DIR:-/var/lib/pag/repo}"
REPO_SERVER="${REPO_SERVER:-https://repos.paganlinux.eu}"
API_TOKEN="${PAGBUILD_API_TOKEN:-}"
GPG_KEY="${GPG_KEY:-}"
BUILD_ALL=false
TARGET_REPO="extra"
SPECIFIC_PKG=""
CHANGED_ONLY=false
PARALLEL="${PARALLEL:-2}"

log() { echo "[$(date '+%Y-%m-%d %H:%M:%S')] $*"; }

usage() {
    echo "Usage: $0 [--all] [--changed] [--repo core|extra|community] [--parallel N] [package-name]"
    exit 1
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --all) BUILD_ALL=true; shift ;;
        --changed) CHANGED_ONLY=true; shift ;;
        --repo) TARGET_REPO="$2"; shift 2 ;;
        --parallel) PARALLEL="$2"; shift 2 ;;
        --help|-h) usage ;;
        *) SPECIFIC_PKG="$1"; shift ;;
    esac
done

# ─── Wykrywanie zmienionych pakietów ──────────────────────────
get_packages_to_build() {
    if [[ -n "$SPECIFIC_PKG" ]]; then
        echo "$SPECIFIC_PKG"
        return
    fi

    if $CHANGED_ONLY && [[ -d "$PAGPORTS_DIR/.git" ]]; then
        cd "$PAGPORTS_DIR"
        git fetch origin main -q 2>/dev/null || true
        # Pakiety zmienione od ostatniego builda
        git diff --name-only HEAD~10..HEAD -- '*/pagbuild' 2>/dev/null | \
            sed 's|/pagbuild||' | sort -u | head -50
        return
    fi

    if $BUILD_ALL; then
        find "$PAGPORTS_DIR" -name pagbuild -maxdepth 2 | \
            sed 's|/pagbuild||;s|.*/||' | sort
        return
    fi

    log "Nie wybrano pakietów. Użyj --all, --changed, lub podaj nazwę."
    exit 0
}

# ─── Budowanie pojedynczego pakietu ───────────────────────────
build_package() {
    local pkg="$1"
    local build_dir="$PAGPORTS_DIR/$pkg"

    if [[ ! -f "$build_dir/pagbuild" ]]; then
        log "✗ $pkg — brak pliku pagbuild, pomijam"
        return 1
    fi

    log "🔨 Budowanie: $pkg"

    cd "$PAGPORTS_DIR"
    if pagbuild build "$pkg" --output "$OUTPUT_DIR" 2>&1; then
        log "✓ $pkg — zbudowany"

        # Znajdź wyjściowy plik .pag
        local pag_file=$(find "$OUTPUT_DIR" -name "${pkg}-*.pag" -newer "$build_dir/pagbuild" 2>/dev/null | head -1)

        if [[ -n "$pag_file" ]]; then
            # Podpisz (jeśli mamy klucz)
            if [[ -n "$GPG_KEY" ]]; then
                pagbuild sign -k "$GPG_KEY" "$pag_file" 2>&1 || true
                log "  🔐 Podpisany: $(basename "$pag_file")"
            fi

            # Wyślij do repo
            if [[ -n "$API_TOKEN" ]]; then
                PAGBUILD_API_TOKEN="$API_TOKEN" \
                    pagbuild upload "$pag_file" --repo "$TARGET_REPO" --server "$REPO_SERVER" 2>&1 || true
                log "  📤 Wysłany do $TARGET_REPO"
            fi
        fi
        return 0
    else
        log "✗ $pkg — BŁĄD budowania"
        return 1
    fi
}

# ─── Główna pętla ────────────────────────────────────────────
main() {
    mkdir -p "$OUTPUT_DIR" /var/log/pag

    log "=== Auto-Build Start ==="
    log "Repo: $TARGET_REPO | Parallel: $PARALLEL"

    local packages=()
    while IFS= read -r pkg; do
        [[ -n "$pkg" ]] && packages+=("$pkg")
    done < <(get_packages_to_build)

    log "Pakiety do zbudowania: ${#packages[@]}"

    local built=0 failed=0
    local running=0
    local pids=()

    for pkg in "${packages[@]}"; do
        build_package "$pkg" &
        pids+=($!)
        ((running++))

        if ((running >= PARALLEL)); then
            wait "${pids[0]}" && ((built++)) || ((failed++))
            pids=("${pids[@]:1}")
            ((running--))
        fi
    done

    # Czekaj na resztę
    for pid in "${pids[@]}"; do
        wait "$pid" && ((built++)) || ((failed++))
    done

    log "=== Auto-Build Koniec: $built OK, $failed błędów ==="
}

main "$@"
