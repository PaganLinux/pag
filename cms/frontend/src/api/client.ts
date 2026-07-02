const API_BASE = import.meta.env.VITE_API_URL || '/api/v1';

function getToken(): string | null {
  return localStorage.getItem('token');
}

async function request<T>(
  path: string,
  options: RequestInit = {},
): Promise<T> {
  const token = getToken();
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    ...(options.headers as Record<string, string>),
  };

  if (token) {
    headers['Authorization'] = `Bearer ${token}`;
  }

  const res = await fetch(`${API_BASE}${path}`, { ...options, headers });

  if (res.status === 401) {
    localStorage.removeItem('token');
    window.location.href = '/login';
    throw new Error('Unauthorized');
  }

  if (!res.ok) {
    const err = await res.text();
    throw new Error(err || `HTTP ${res.status}`);
  }

  if (res.status === 204) return undefined as T;
  return res.json();
}

// ─── Auth ────────────────────────────────────────────────
export const auth = {
  login: (username: string, password: string) =>
    request<{ token: string; user: User }>('/auth/login', {
      method: 'POST',
      body: JSON.stringify({ username, password }),
    }),
  register: (username: string, email: string, password: string) =>
    request<{ token: string; user: User }>('/auth/register', {
      method: 'POST',
      body: JSON.stringify({ username, email, password }),
    }),
  me: () => request<User>('/auth/me'),
};

// ─── Packages ────────────────────────────────────────────
export const packages = {
  list: (params?: { arch?: string; status?: string; search?: string; page?: number }) => {
    const qs = new URLSearchParams();
    if (params?.arch) qs.set('arch', params.arch);
    if (params?.status) qs.set('status', params.status);
    if (params?.search) qs.set('search', params.search);
    if (params?.page) qs.set('page', String(params.page));
    return request<PackageListResponse>(`/packages${qs.toString() ? `?${qs}` : ''}`);
  },
  get: (id: number) => request<Package>('/packages/' + id),
  create: (data: CreatePackage) =>
    request<Package>('/packages', { method: 'POST', body: JSON.stringify(data) }),
  update: (id: number, data: Partial<Package>) =>
    request<Package>('/packages/' + id, { method: 'PUT', body: JSON.stringify(data) }),
  upload: (data: { name: string; version: string; arch?: string; description?: string; pagbuild?: string }) =>
    request<Package>('/packages/upload', { method: 'POST', body: JSON.stringify(data) }),
};

// ─── Builds ──────────────────────────────────────────────
export const builds = {
  list: () => request<BuildListResponse>('/builds'),
  get: (id: number) => request<Build>('/builds/' + id),
  create: (packageId: number, arch?: string) =>
    request<Build>('/builds', {
      method: 'POST',
      body: JSON.stringify({ package_id: packageId, arch }),
    }),
  forPackage: (packageId: number) => request<Build[]>('/builds/package/' + packageId),
};

// ─── Ports ───────────────────────────────────────────────
export const ports = {
  list: () => request<Port[]>('/ports'),
  get: (id: number) => request<Port>('/ports/' + id),
  create: (data: CreatePort) =>
    request<Port>('/ports', { method: 'POST', body: JSON.stringify(data) }),
  update: (id: number, data: Partial<Port>) =>
    request<Port>('/ports/' + id, { method: 'PUT', body: JSON.stringify(data) }),
  delete: (id: number) => request<void>('/ports/' + id, { method: 'DELETE' }),
};

// ─── Repos ───────────────────────────────────────────────
export const repos = {
  list: () => request<Repo[]>('/repos'),
  get: (id: number) => request<Repo>('/repos/' + id),
  create: (data: CreateRepo) =>
    request<Repo>('/repos', { method: 'POST', body: JSON.stringify(data) }),
};

// ─── Stats ───────────────────────────────────────────────
export const stats = {
  get: () => request<Stats>('/stats'),
};

// ─── Types ───────────────────────────────────────────────
export interface User {
  id: number; username: string; email: string; role: string;
  avatar_url?: string; created_at: string;
}

export interface Package {
  id: number; name: string; version: string; release: string;
  description?: string; arch: string; maintainer_id?: number;
  build_status: string; pkg_url?: string; pkg_size?: number;
  created_at: string; updated_at: string;
}

export interface CreatePackage {
  name: string; version: string; release?: string;
  description?: string; arch?: string;
}

export interface PackageListResponse {
  packages: Package[]; total: number; page: number; total_pages: number;
}

export interface Build {
  id: number; package_id: number; job_id: string; status: string;
  arch: string; log_path?: string; started_at?: string;
  finished_at?: string; created_at: string;
  package_name?: string; package_version?: string;
}

export interface BuildListResponse {
  builds: Build[]; total: number;
}

export interface Port {
  id: number; name: string; category?: string; description?: string;
  version?: string; maintainer_id?: number; pagbuild_path: string;
  status: string; created_at: string; updated_at: string;
}

export interface CreatePort {
  name: string; category?: string; description?: string;
  version?: string; pagbuild_path: string;
}

export interface Repo {
  id: number; name: string; full_name: string; owner: string;
  description?: string; gitea_id?: number; clone_url?: string;
  webhook_url?: string; active: boolean; created_at: string;
}

export interface CreateRepo {
  name: string; description?: string; owner?: string;
}

export interface Stats {
  total_packages: number; total_ports: number; total_builds: number;
  builds_today: number; successful_builds: number; failed_builds: number;
}
