// Klient API — przygotowany pod przyszłe endpointy backendu

const API_BASE = import.meta.env.PUBLIC_API_URL || 'https://api.paganlinux.eu';

interface FetchOptions extends RequestInit {
  locale?: string;
}

export async function apiFetch<T = unknown>(
  path: string,
  options: FetchOptions = {},
): Promise<T> {
  const { locale, ...fetchOpts } = options;

  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    ...(fetchOpts.headers as Record<string, string>),
  };

  if (locale) {
    headers['Accept-Language'] = locale;
  }

  const res = await fetch(`${API_BASE}${path}`, {
    ...fetchOpts,
    headers,
  });

  if (!res.ok) {
    throw new Error(`API error: ${res.status} ${res.statusText}`);
  }

  return res.json();
}

// Przykładowe endpointy — do użycia gdy backend będzie gotowy

export async function getStats() {
  return apiFetch<{
    packages: number;
    repos: number;
    size: string;
  }>('/api/v1/stats');
}

export async function getPackages(params?: { arch?: string; search?: string }) {
  const searchParams = new URLSearchParams();
  if (params?.arch) searchParams.set('arch', params.arch);
  if (params?.search) searchParams.set('q', params.search);
  const qs = searchParams.toString();
  return apiFetch(`/api/v1/packages${qs ? `?${qs}` : ''}`);
}
