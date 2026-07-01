// API client dla repo-server
// W produkcji: https://repos.paganlinux.eu/api/v1
// W developmencie: http://localhost:3001/api/v1

const API_BASE = import.meta.env.PAG_REPO_API || 'http://localhost:3001/api/v1';

export interface PackageEntry {
  name: string;
  version: string;
  release: number;
  arch: string;
  description: string;
  repo: string;
  installed_size: number;
  compressed_size: number;
  download_count?: number;
}

export interface PackageDetail extends PackageEntry {
  filename: string;
  sha512: string;
  blake3: string;
  maintainer: string | null;
  license: string;
  upload_date: number;
  depends: string[];
  provides: string[];
  conflicts: string[];
}

export interface RepoIndex {
  version: number;
  name: string;
  description: string;
  url: string;
  updated: number;
  packages: PackageEntry[];
  total: number;
}

export interface SearchResults {
  results: PackageEntry[];
}

export interface RepoStats {
  total_packages: number;
  total_size_bytes: number;
  total_size_human: string;
  by_repo: { repo: string; count: number; size: number }[];
  last_updated: number;
}

async function fetchJSON<T>(path: string): Promise<T> {
  const url = `${API_BASE}${path}`;
  const res = await fetch(url, {
    headers: { 'Accept': 'application/json' },
    // SSR: brak cache po stronie serwera, klient cache'uje
    next: { revalidate: 60 },
  });

  if (!res.ok) {
    throw new Error(`API error: ${res.status} ${res.statusText}`);
  }

  return res.json();
}

export async function getIndex(): Promise<RepoIndex> {
  return fetchJSON<RepoIndex>('/index.json');
}

export async function searchPackages(query: string): Promise<SearchResults> {
  return fetchJSON<SearchResults>(`/search?q=${encodeURIComponent(query)}`);
}

export async function getPackage(name: string): Promise<PackageDetail | null> {
  try {
    return await fetchJSON<PackageDetail>(`/packages/${encodeURIComponent(name)}`);
  } catch {
    return null;
  }
}

export async function listPackages(repo?: string): Promise<{ packages: PackageEntry[] }> {
  const params = repo ? `?repo=${encodeURIComponent(repo)}` : '';
  return fetchJSON(`/packages${params}`);
}

export async function getStats(): Promise<RepoStats> {
  return fetchJSON<RepoStats>('/stats');
}

// Fallback: dane testowe gdy API nie jest dostępne
export function getFallbackPackages(): PackageEntry[] {
  return [
    { name: 'linux-pagan', version: '6.12.0', release: 0, arch: 'x86_64', description: 'PaganLinux kernel (optimized)', repo: 'core', installed_size: 134217728, compressed_size: 45000000, download_count: 15420 },
    { name: 'glibc', version: '2.40', release: 0, arch: 'x86_64', description: 'GNU C Library', repo: 'core', installed_size: 47185920, compressed_size: 12000000, download_count: 28900 },
    { name: 'systemd', version: '256.2', release: 0, arch: 'x86_64', description: 'System and service manager', repo: 'core', installed_size: 23068672, compressed_size: 8000000, download_count: 27300 },
    { name: 'bash', version: '5.2', release: 1, arch: 'x86_64', description: 'GNU Bourne Again Shell', repo: 'core', installed_size: 2097152, compressed_size: 800000, download_count: 31000 },
    { name: 'openssl', version: '3.3.0', release: 0, arch: 'x86_64', description: 'SSL/TLS toolkit', repo: 'core', installed_size: 8388608, compressed_size: 3000000, download_count: 26500 },
    { name: 'nginx', version: '1.27.0', release: 1, arch: 'x86_64', description: 'High performance HTTP and reverse proxy server', repo: 'extra', installed_size: 2202009, compressed_size: 900000, download_count: 18900 },
    { name: 'postgresql', version: '16.4', release: 0, arch: 'x86_64', description: 'Advanced open-source relational database', repo: 'extra', installed_size: 52428800, compressed_size: 18000000, download_count: 12300 },
    { name: 'rust', version: '1.82.0', release: 0, arch: 'x86_64', description: 'Rust programming language', repo: 'extra', installed_size: 188743680, compressed_size: 70000000, download_count: 9800 },
    { name: 'python', version: '3.12.5', release: 0, arch: 'x86_64', description: 'Python programming language', repo: 'extra', installed_size: 62914560, compressed_size: 22000000, download_count: 21000 },
    { name: 'firefox', version: '130.0', release: 0, arch: 'x86_64', description: 'Mozilla Firefox web browser', repo: 'community', installed_size: 99614720, compressed_size: 55000000, download_count: 14500 },
    { name: 'neovim', version: '0.10.1', release: 0, arch: 'x86_64', description: 'Hyperextensible Vim-based text editor', repo: 'community', installed_size: 12582912, compressed_size: 5000000, download_count: 8700 },
    { name: 'hyprland', version: '0.42.0', release: 0, arch: 'x86_64', description: 'Dynamic tiling Wayland compositor', repo: 'community', installed_size: 8388608, compressed_size: 3200000, download_count: 6200 },
  ];
}
