// Proste uwierzytelnianie tokenem dla panelu admin
// W produkcji: zmień na prawdziwy system haseł

const ADMIN_TOKEN = process.env.ADMIN_TOKEN || 'paganadmin';

export function isAuthenticated(request: Request): boolean {
  const cookie = request.headers.get('cookie') || '';
  return cookie.includes(`admin_token=${ADMIN_TOKEN}`);
}

export function getAuthHeaders(): Record<string, string> {
  return {
    'Authorization': `Bearer ${ADMIN_TOKEN}`,
    'Content-Type': 'application/json',
  };
}

// Sprawdza token z POST formularza logowania
export function validateLogin(password: string): boolean {
  return password === ADMIN_TOKEN;
}
