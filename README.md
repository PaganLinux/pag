# pag

`pag` to prosty manager pakietow inspirowany stylem Void Linux: buduje pakiety ze zrodel i instaluje artefakty binarne w lokalnym `rootfs`.

## Obslugiwane komendy

- `pag -f <app-id>`: instalacja aplikacji przez Flatpak.
- `pag -fs <fraza>`: wyszukiwanie przez Flatpak.
- `pag -s <fraza>`: wyszukiwanie pakietow w indeksach repozytoriow `pag`.
- `pag -i <pakiet|alias>`: pobranie zrodla, weryfikacja SHA256, build i instalacja.
- `pag -u`: aktualizacja lokalnej bazy indeksow repozytoriow.
- `pag -uall`: atomowa aktualizacja wszystkich zainstalowanych pakietow.

## Bezpieczenstwo

- Wymuszona weryfikacja `source_sha256` dla zrodel.
- Ochrona przed `path traversal` podczas rozpakowania i instalacji.
- Build uruchamiany przez `sh -eu` z wyczyszczonym srodowiskiem.
- `-uall` buduje nowa generacje i atomowo przełącza symlink `current`.

## Konfiguracja

Plik `pag.conf` (kolejnosc szukania):

1. `PAG_CONFIG`
2. `./pag.conf`
3. `$XDG_CONFIG_HOME/pag/pag.conf`
4. `~/.config/pag/pag.conf`
5. `/etc/pag/pag.conf`
6. `/etc/pag.conf`

Przyklad:

```toml
[general]
flatpak_remote = "flathub"
bubblewrap_level = 3

[[repositories]]
name = "official"
repo_dir = "../pag-repo/packages"

# opcjonalnie nadpisz, jesli chcesz:
# state_dir = "~/.local/state/pag"
# install_root = "~/.local/state/pag/rootfs"
```

`repo_dir` moze byc sciezka absolutna albo wzgledna; wzgledna jest liczona od katalogu pliku `pag.conf`.

## Kolizje nazw pakietow

Jesli dwa rozne template z Void daja ten sam `pkgname`, importer tworzy warianty zamiast nadpisywac plik:

- pierwszy wpis zostaje pod `pkgname`
- kolejne dostaja nazwe `pkgname--<source-dir>`

Przyklad:

- `foo`
- `foo--foo-git`

Domyslnie (bez wpisywania sciezek w `pag.conf`):

- `state_dir`: `$XDG_STATE_HOME/pag` lub fallback `~/.local/state/pag`
- `install_root`: `<state_dir>/rootfs`

## Format repo jak w Void (katalogi pakietow)

Repo moze byc katalogiem, gdzie kazda paczka ma osobny folder i plik `pag.info` (TOML):

```text
pag-repo/
  packages/
    7zip/
      pag.info
    hello-pag/
      pag.info
```

Przykladowy `pag.info`:

```toml
name = "7zip"
version = "26.01"
description = "File archiver with a high compression ratio"
aliases = ["7z", "7za", "7zr", "7zz", "p7zip"]
replaces = ["p7zip"]
distfiles = ["https://www.7-zip.org/a/7z2601-src.tar.xz"]
checksum = "<sha256>"
build_steps = [
  "cd CPP/7zip/Bundles/Alone2 && make -f ../../cmpl_gcc.mak O=b/norar DISABLE_RAR_COMPRESS=1"
]

[[install_map]]
from = "CPP/7zip/Bundles/Alone2/b/norar/7zz"
to = "usr/bin/7zip"
```

Pola zrodla i sumy moga byc podane jako:

- nowe: `source_url` i `source_sha256`
- styl podobny do Void: `distfiles` i `checksum`

W `source_url` i `distfiles` mozna uzywac prostych zmiennych:

- `${version}`
- `${version//./}` (wersja bez kropek, np. `26.01` -> `2601`)
- `${name}`

Przyklad:

```toml
version = "26.01"
distfiles = ["https://www.7-zip.org/a/7z${version//./}-src.tar.xz"]
```

W praktyce zmieniasz tylko `version`, a URL aktualizuje sie automatycznie.

Aliasy (`aliases`), `provides` i `replaces` sa brane pod uwage przy `pag -i`.

Import z Void byl tylko jednorazowym sposobem zasilenia repo w `pag.info`; po imporcie `pag` dziala juz wylacznie na `pag-repo` i nie wymaga lokalnego drzewa `void-pac`.

Przyklad:

```bash
pag -i 7z
```

zostanie rozwiazany do pakietu `7zip`.

## Poziomy Bubblewrap

Ustawiane przez `bubblewrap_level` w `[general]`:

- `0`: bez bubblewrap (tylko czyste env).
- `1`: bubblewrap, izolacja przestrzeni, siec wlaczona.
- `2`: jak 1 + `--clearenv` i minimalne zmienne env.
- `3`: jak 2 + siec wylaczona.
- `4`: jak 3 + `--cap-drop ALL`.
- `5`: jak 4 + `--unshare-user-try`.

## Wielojezycznosc (gettext)

`pag` uzywa katalogow gettext:

- `locale/en/LC_MESSAGES/pag.po`
- `locale/pl/LC_MESSAGES/pag.po`

Kompilacja katalogow do `*.mo`:

```bash
msgfmt locale/en/LC_MESSAGES/pag.po -o locale/en/LC_MESSAGES/pag.mo
msgfmt locale/pl/LC_MESSAGES/pag.po -o locale/pl/LC_MESSAGES/pag.mo
```

Uruchomienie z wybranym jezykiem:

```bash
PAG_LOCALE_DIR=/home/patryk/pagan-repo/pag/locale LANG=en_US.UTF-8 pag -u
PAG_LOCALE_DIR=/home/patryk/pagan-repo/pag/locale LANG=pl_PL.UTF-8 pag -u
```

## Build pod musl

Dla statycznego targetu:

```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

Powstaly binarny plik:

- `target/x86_64-unknown-linux-musl/release/pag`

## Szybki start demo

1. Uruchom lokalny serwer w katalogu z przykladowym repo:

```bash
cd examples/repo
python3 -m http.server 8080
```

2. W drugim terminalu:

```bash
pag -u
pag -s hello
pag -i hello-pag
```
