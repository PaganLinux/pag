import { defineConfig } from 'astro/config';
import node from '@astrojs/node';

export default defineConfig({
  site: 'https://repos.paganlinux.eu',
  output: 'server',
  adapter: node({ mode: 'standalone' }),
  server: { port: 3002, host: '0.0.0.0' },
});
