// Helper do tłumaczeń w komponentach Astro
import { translations, type Lang, type TranslationKey, defaultLang } from './translations';

/**
 * Zwraca przetłumaczony string dla danego języka.
 * Użycie: t(locale, 'klucz')
 */
export function t(locale: string | undefined, key: TranslationKey): string {
  const lang = (locale || defaultLang) as Lang;
  const langTranslations = translations[lang];
  if (langTranslations && key in langTranslations) {
    return langTranslations[key as keyof typeof langTranslations];
  }
  // Fallback do polskiego
  return translations[defaultLang][key] || key;
}

/**
 * Zwraca URL dla danego języka
 */
export function getLocalizedPath(currentPath: string, targetLang: Lang): string {
  // Usuń istniejący prefix językowy
  let path = currentPath;
  for (const lang of ['pl', 'en', 'de', 'fr', 'es', 'it']) {
    if (path.startsWith(`/${lang}/`)) {
      path = path.slice(lang.length + 1);
      break;
    }
  }

  // Dla domyślnego (pl) nie dodajemy prefixu
  if (targetLang === defaultLang) {
    return path || '/';
  }

  return `/${targetLang}${path}`;
}

/**
 * Pobiera aktualny locale z URL (dla Astro SSR)
 */
export function getLocaleFromUrl(url: URL): Lang {
  const path = url.pathname;
  for (const lang of ['en', 'de', 'fr', 'es', 'it'] as Lang[]) {
    if (path.startsWith(`/${lang}/`) || path === `/${lang}`) {
      return lang;
    }
  }
  return defaultLang;
}
