// GitHub API client dla pagports
// Fetch z repozytorium pagports na GitHubie

const GITHUB_API = 'https://api.github.com';
const REPO_OWNER = import.meta.env.PAGPORTS_REPO_OWNER || 'PaganLinux';
const REPO_NAME = import.meta.env.PAGPORTS_REPO_NAME || 'pagports';
const BRANCH = import.meta.env.PAGPORTS_BRANCH || 'main';
const GITHUB_TOKEN = process.env.GITHUB_TOKEN || '';

export interface PortEntry {
  name: string;
  path: string;
  version: string;
  release: number;
  description: string;
  license: string;
  arch: string;
  maintainer: string | null;
  url: string | null;
  depends: string[];
  makedepends: string[];
  sha: string;
  size: number;
}

export interface PortDetail extends PortEntry {
  pagbuildContent: string;
  subpackages: string[];
  provides: string[];
  replaces: string[];
  conflicts: string[];
  source: string[];
  secfixes: string[];
}

const headers: Record<string, string> = {
  'Accept': 'application/vnd.github.v3+json',
  'User-Agent': 'pagports/0.2.0',
};
if (GITHUB_TOKEN) {
  headers['Authorization'] = `Bearer ${GITHUB_TOKEN}`;
}

async function ghFetch<T>(path: string): Promise<T> {
  const url = `${GITHUB_API}${path}`;
  const res = await fetch(url, { headers });
  if (!res.ok) {
    throw new Error(`GitHub API error: ${res.status}`);
  }
  return res.json();
}

interface GhTreeItem {
  path: string;
  type: 'tree' | 'blob';
  sha: string;
  size?: number;
}

interface GhContent {
  content: string;
  encoding: string;
  sha: string;
  size: number;
}

// Pobiera listę portów (katalogów z plikiem pagbuild) — zoptymalizowane
export async function listPorts(dir: string = ''): Promise<PortEntry[]> {
  try {
    // 1. Pobierz całe drzewo (1 zapytanie API zamiast 231)
    const { tree } = await ghFetch<{ tree: GhTreeItem[] }>(
      `/repos/${REPO_OWNER}/${REPO_NAME}/git/trees/${BRANCH}?recursive=1`
    );

    const pagbuildFiles = tree.filter(
      item => item.type === 'blob' && item.path.endsWith('/pagbuild')
    );

    // 2. Mapuj tylko nazwy z drzewa — bez pobierania zawartości
    const ports: PortEntry[] = pagbuildFiles.map(file => {
      const parts = file.path.split('/');
      const portName = parts.length >= 2 ? parts[parts.length - 2] : file.path.replace('/pagbuild', '');

      return {
        name: portName,
        path: file.path,
        version: '?',
        release: 1,
        description: '',
        license: '',
        arch: 'x86_64',
        maintainer: null,
        url: null,
        depends: [],
        makedepends: [],
        sha: file.sha,
        size: file.size || 0,
      };
    });

    return ports.sort((a, b) => a.name.localeCompare(b.name));
  } catch {
    return getFallbackPorts();
  }
}

// Pobiera szczegóły portu (wraz z zawartością pagbuild)
export async function getPort(name: string): Promise<PortDetail | null> {
  try {
    const path = `${name}/pagbuild`;
    const content = await ghFetch<GhContent>(
      `/repos/${REPO_OWNER}/${REPO_NAME}/contents/${path}?ref=${BRANCH}`
    );
    const text = Buffer.from(content.content, 'base64').toString('utf-8');
    const meta = parsePagbuildMeta(text);

    return {
      name,
      path,
      version: meta.pkgver || '?',
      release: meta.pkgrel || 1,
      description: meta.pkgdesc || '',
      license: meta.license || 'custom',
      arch: meta.arch || 'x86_64',
      maintainer: meta.maintainer || null,
      url: meta.url || null,
      depends: meta.depends || [],
      makedepends: meta.makedepends || [],
      sha: content.sha,
      size: content.size,
      pagbuildContent: text,
      subpackages: meta.subpackages || [],
      provides: meta.provides || [],
      replaces: meta.replaces || [],
      conflicts: meta.conflicts || [],
      source: meta.source || [],
      secfixes: meta.secfixes || [],
    };
  } catch {
    const fallback = getFallbackPorts().find(p => p.name === name);
    if (fallback) {
      return {
        ...fallback,
        pagbuildContent: `# ${name} - pagbuild content unavailable\n# Try: git clone https://github.com/${REPO_OWNER}/${REPO_NAME}`,
        subpackages: [],
        provides: [],
        replaces: [],
        conflicts: [],
        source: [],
        secfixes: [],
      };
    }
    return null;
  }
}

// Parsuje metadane z pliku pagbuild (uproszczone)
function parsePagbuildMeta(text: string): Record<string, any> {
  const meta: Record<string, any> = {};

  const simpleVars = ['pkgname', 'pkgver', 'pkgdesc', 'url', 'arch', 'license', 'maintainer', 'patch_args'];
  const listVars = ['depends', 'makedepends', 'optdepends', 'subpackages', 'provides', 'replaces', 'conflicts', 'source'];

  for (const line of text.split('\n')) {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith('#')) continue;

    // Simple vars: key="value" or key=value
    for (const varName of simpleVars) {
      const match = trimmed.match(new RegExp(`^${varName}=(?:"([^"]*)"|(\\S+))`));
      if (match) {
        meta[varName] = (match[1] || match[2] || '').trim();
      }
    }

    // List vars: key="item1 item2"
    for (const varName of listVars) {
      const match = trimmed.match(new RegExp(`^${varName}="([^"]*)"`));
      if (match) {
        meta[varName] = match[1].split(/\s+/).filter(Boolean);
      }
    }

    // pkgrel=number
    const relMatch = trimmed.match(/^pkgrel=(\d+)/);
    if (relMatch) meta.pkgrel = parseInt(relMatch[1]);
  }

  return meta;
}

// Fallback: dane demonstracyjne
function getFallbackPorts(): PortEntry[] {
  return [
    { name: '7zip', path: 'main/7zip/pagbuild', version: '26.01', release: 2, description: 'File archiver with a high compression ratio', license: 'LGPL-2.0-only', arch: 'all', maintainer: 'Achill Gilgenast', url: 'https://7-zip.org/', depends: [], makedepends: ['make'], sha: 'abc123', size: 2048 },
    { name: 'nginx', path: 'main/nginx/pagbuild', version: '1.27.0', release: 1, description: 'High performance HTTP and reverse proxy server', license: 'BSD-2-Clause', arch: 'x86_64', maintainer: 'PaganLinux Team', url: 'https://nginx.org/', depends: ['pcre2', 'openssl', 'zlib'], makedepends: ['gcc', 'make', 'pcre2-dev'], sha: 'def456', size: 3100 },
    { name: 'neovim', path: 'community/neovim/pagbuild', version: '0.10.1', release: 0, description: 'Hyperextensible Vim-based text editor', license: 'Apache-2.0', arch: 'x86_64', maintainer: 'PaganLinux Team', url: 'https://neovim.io/', depends: ['libluv', 'libtermkey', 'libvterm', 'msgpack-c', 'tree-sitter', 'unibilium'], makedepends: ['cmake', 'gcc', 'make', 'ninja'], sha: 'ghi789', size: 2800 },
    { name: 'hyprland', path: 'community/hyprland/pagbuild', version: '0.42.0', release: 0, description: 'Dynamic tiling Wayland compositor', license: 'BSD-3-Clause', arch: 'x86_64', maintainer: null, url: 'https://hyprland.org/', depends: ['aquamarine', 'hyprcursor', 'hyprgraphics', 'hyprlang', 'hyprutils', 'wayland'], makedepends: ['cmake', 'gcc', 'make', 'pkgconfig', 'wayland-protocols'], sha: 'jkl012', size: 3500 },
    { name: 'postgresql', path: 'main/postgresql/pagbuild', version: '16.4', release: 0, description: 'Advanced open-source relational database', license: 'PostgreSQL', arch: 'x86_64', maintainer: 'PaganLinux Team', url: 'https://postgresql.org/', depends: ['glibc', 'openssl', 'zlib', 'readline'], makedepends: ['bison', 'flex', 'gcc', 'make', 'perl'], sha: 'mno345', size: 4200 },
    { name: 'linux-pagan', path: 'main/linux-pagan/pagbuild', version: '6.12.0', release: 0, description: 'Optimized PaganLinux kernel', license: 'GPL-2.0', arch: 'x86_64', maintainer: 'PaganLinux Team', url: 'https://kernel.org/', depends: [], makedepends: ['bc', 'bison', 'elfutils-dev', 'flex', 'gcc', 'make', 'openssl-dev', 'perl'], sha: 'pqr678', size: 5600 },
    { name: 'firefox', path: 'community/firefox/pagbuild', version: '130.0', release: 0, description: 'Mozilla Firefox web browser', license: 'MPL-2.0', arch: 'x86_64', maintainer: null, url: 'https://mozilla.org/firefox/', depends: ['gtk+3', 'libx11', 'nss', 'pulseaudio'], makedepends: ['cargo', 'clang', 'gcc', 'make', 'nodejs', 'rust'], sha: 'stu901', size: 4800 },
    { name: 'rust', path: 'main/rust/pagbuild', version: '1.82.0', release: 0, description: 'Rust programming language', license: 'MIT/Apache-2.0', arch: 'x86_64', maintainer: 'PaganLinux Team', url: 'https://rust-lang.org/', depends: ['gcc', 'libssh2', 'libssl', 'zlib'], makedepends: ['cmake', 'gcc', 'make', 'python3'], sha: 'vwx234', size: 5200 },
  ];
}
