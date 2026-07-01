---
// POST /admin/api/build — uruchamia budowanie portu
import { isAuthenticated } from '../../../lib/auth';

export const prerender = false;

if (Astro.request.method !== 'POST') {
  return new Response(JSON.stringify({ error: 'Method not allowed' }), { status: 405 });
}

if (!isAuthenticated(Astro.request)) {
  return new Response(JSON.stringify({ error: 'Unauthorized' }), { status: 401 });
}

const data = await Astro.request.formData();
const portName = data.get('port')?.toString() || '';

if (!portName) {
  return new Response(JSON.stringify({ error: 'Brak nazwy portu' }), { status: 400 });
}

const PAGPORTS_DIR = process.env.PAGPORTS_DIR || '/opt/pagports/main';
const OUTPUT_DIR = process.env.PAG_OUTPUT_DIR || '/var/lib/pag/repo/extra';
const REPO_SERVER = process.env.REPO_SERVER || 'http://127.0.0.1:3001';

// Uruchom budowanie w tle
const cmd = `cd ${PAGPORTS_DIR} && pagbuild build ${portName} --output ${OUTPUT_DIR} 2>&1`;
try {
  const { exec } = await import('node:child_process');
  exec(cmd, (error, stdout, stderr) => {
    // Log wyniku (w produkcji: zapisz do pliku / Bazy)
    console.log(`[build:${portName}] ${stdout} ${stderr}`);
  });

  return Astro.redirect('/admin/builds');
} catch (e: any) {
  return new Response(JSON.stringify({ error: e.message }), { status: 500 });
}
