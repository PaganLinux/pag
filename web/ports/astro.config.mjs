import { defineConfig } from 'astro/config';
import node from '@astrojs/node';

export default defineConfig({
  site: 'https://pagports.paganlinux.eu',
  output: 'server',
  adapter: node({ mode: 'standalone' }),
  server: { port: 3003, host: '0.0.0.0' },
});
