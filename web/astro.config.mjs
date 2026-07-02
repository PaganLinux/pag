import { defineConfig } from 'astro/config';
import node from '@astrojs/node';

export default defineConfig({
  site: 'https://paganlinux.eu',
  output: 'server',
  adapter: node({ mode: 'standalone' }),
  server: { port: 3004, host: '0.0.0.0' },
  i18n: {
    defaultLocale: 'pl',
    locales: ['pl', 'en', 'de', 'fr', 'es', 'it'],
    routing: {
      prefixDefaultLocale: false,
    },
  },
});
