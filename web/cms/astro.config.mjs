import { defineConfig } from 'astro/config';
import node from '@astrojs/node';

export default defineConfig({
  site: 'https://cms.paganlinux.eu',
  output: 'server',
  adapter: node({ mode: 'standalone' }),
  server: { port: 3006, host: '0.0.0.0' },
});
