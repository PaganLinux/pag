import { defineConfig } from 'astro/config';
import node from '@astrojs/node';

export default defineConfig({
  site: 'https://paganlinux.eu',
  output: 'hybrid',
  adapter: node({ mode: 'standalone' }),
  server: { port: 3001, host: '0.0.0.0' },
  i18n: {
    defaultLocale: 'pl',
    locales: ['pl', 'en', 'de', 'fr', 'es', 'it'],
    routing: { prefixDefaultLocale: false },
  },
});
