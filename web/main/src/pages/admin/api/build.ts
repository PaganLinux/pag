// POST /admin/api/build — uruchamia budowanie portu
import type { APIRoute } from 'astro';
import { isAuthenticated } from '../../../lib/auth';
import { exec } from 'node:child_process';

export const prerender = false;

export const POST: APIRoute = async ({ request, redirect }) => {
  if (!isAuthenticated(request)) {
    return new Response(JSON.stringify({ error: 'Unauthorized' }), { status: 401 });
  }

  const data = await request.formData();
  const portName = data.get('port')?.toString() || '';

  if (!portName) {
    return new Response(JSON.stringify({ error: 'Brak nazwy portu' }), { status: 400 });
  }

  const pagportsDir = process.env.PAGPORTS_DIR || '/opt/pagports/main';
  const outputDir = process.env.PAG_OUTPUT_DIR || '/var/lib/pag/repo/extra';
  const cmd = `cd ${pagportsDir} && pagbuild build ${portName} --output ${outputDir} 2>&1`;

  exec(cmd, (err, stdout, stderr) => {
    console.log(`[build:${portName}] ${stdout} ${stderr}`);
  });

  return redirect('/admin/builds');
};
