// API client dla strony głównej - pobiera statystyki z repo API

const REPO_API = import.meta.env.PAG_REPO_API || 'http://localhost:3001/api/v1';
const PORTS_API = import.meta.env.PAG_PORTS_API || 'http://localhost:3003';

export interface LiveStats {
  total_packages: number;
  total_size_human: string;
  last_updated: number;
}

export async function getLiveStats(): Promise<LiveStats> {
  try {
    const res = await fetch(`${REPO_API}/stats`, {
      headers: { 'Accept': 'application/json' },
      signal: AbortSignal.timeout(5000),
    });
    if (res.ok) {
      const data = await res.json();
      return {
        total_packages: data.total_packages || 5000,
        total_size_human: data.total_size_human || '45 GiB',
        last_updated: data.last_updated || Math.floor(Date.now() / 1000),
      };
    }
  } catch {
    // Fallback
  }
  return {
    total_packages: 5000,
    total_size_human: '45 GiB',
    last_updated: Math.floor(Date.now() / 1000),
  };
}
