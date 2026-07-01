const API_BASE = import.meta.env.CMS_API_URL || 'http://localhost:3005';

export interface DashboardStats {
  total_packages: number;
  pending_submissions: number;
  active_builds: number;
  completed_builds_today: number;
  failed_builds_today: number;
  published_packages: number;
  disk_usage_mb: number;
}

export interface Submission {
  id: number;
  forgejo_pr_id: number;
  forgejo_pr_url: string;
  package_name: string;
  package_version: string;
  description: string;
  submitter: string;
  build_script: string;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface BuildJob {
  id: number;
  job_uuid: string;
  submission_id: number | null;
  package_name: string;
  package_version: string;
  build_script: string;
  status: string;
  log_output: string;
  started_at: string | null;
  finished_at: string | null;
  exit_code: number | null;
  artifact_path: string | null;
  created_at: string;
}

export interface CmsSetting {
  key: string;
  value: string;
  updated_at: string;
}

export interface UserInfo {
  id: number;
  username: string;
  role: string;
}

class CmsApi {
  private token: string | null = null;

  constructor() {
    if (typeof window !== 'undefined') {
      this.token = localStorage.getItem('cms_token');
    }
  }

  setToken(token: string | null) {
    this.token = token;
    if (typeof window !== 'undefined') {
      if (token) localStorage.setItem('cms_token', token);
      else localStorage.removeItem('cms_token');
    }
  }

  private async request<T>(path: string, options: RequestInit = {}): Promise<T> {
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
      ...(options.headers as Record<string, string> || {}),
    };
    if (this.token) {
      headers['Authorization'] = `Bearer ${this.token}`;
    }

    const res = await fetch(`${API_BASE}${path}`, { ...options, headers });
    if (!res.ok) {
      const body = await res.json().catch(() => ({ error: res.statusText }));
      throw new Error(body.error || `HTTP ${res.status}`);
    }
    return res.json();
  }

  // Auth
  async login(username: string, password: string) {
    const data = await this.request<{ token: string; user: UserInfo }>(
      '/api/v1/auth/login',
      { method: 'POST', body: JSON.stringify({ username, password }) }
    );
    this.setToken(data.token);
    return data;
  }

  async logout() {
    await this.request('/api/v1/auth/logout', { method: 'POST' });
    this.setToken(null);
  }

  async me(): Promise<UserInfo> {
    return this.request('/api/v1/auth/me');
  }

  isLoggedIn(): boolean {
    return this.token !== null;
  }

  // Dashboard
  async getStats(): Promise<DashboardStats> {
    return this.request('/api/v1/dashboard/stats');
  }

  // Submissions
  async getSubmissions(status?: string): Promise<Submission[]> {
    const qs = status ? `?status=${status}` : '';
    return this.request(`/api/v1/submissions${qs}`);
  }

  async getSubmission(id: number): Promise<Submission> {
    return this.request(`/api/v1/submissions/${id}`);
  }

  async updateSubmission(id: number, data: { status?: string; build_script?: string }) {
    return this.request(`/api/v1/submissions/${id}`, {
      method: 'PATCH', body: JSON.stringify(data),
    });
  }

  async approveAndBuild(id: number) {
    return this.request(`/api/v1/submissions/${id}/approve-build`, { method: 'POST' });
  }

  // Builds
  async getBuilds(status?: string): Promise<BuildJob[]> {
    const qs = status ? `?status=${status}` : '';
    return this.request(`/api/v1/builds${qs}`);
  }

  async getBuild(uuid: string): Promise<BuildJob> {
    return this.request(`/api/v1/builds/${uuid}`);
  }

  async createBuild(data: { package_name: string; package_version: string; build_script: string }) {
    return this.request('/api/v1/builds', {
      method: 'POST', body: JSON.stringify(data),
    });
  }

  async cancelBuild(uuid: string) {
    return this.request(`/api/v1/builds/${uuid}/cancel`, { method: 'POST' });
  }

  // Settings
  async getSettings(): Promise<CmsSetting[]> {
    return this.request('/api/v1/settings');
  }

  async updateSettings(settings: { key: string; value: string }[]) {
    return this.request('/api/v1/settings', {
      method: 'PUT', body: JSON.stringify({ settings }),
    });
  }

  // WebSocket URL for live logs
  getBuildWsUrl(uuid: string): string {
    const base = API_BASE.replace('http', 'ws');
    return `${base}/api/v1/builds/${uuid}/ws`;
  }
}

export const api = new CmsApi();
