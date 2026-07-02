// Helper do tłumaczeń w komponentach Astro
// Używa Astro.currentLocale z wbudowanego systemu i18n Astro

import { ui, type TranslationKey } from './ui';
import { defaultLang, type Lang, languages } from './languages';

/**
 * Zwraca przetłumaczony string dla danego locale.
 * Użycie: t(locale, 'klucz')
 */
export function t(locale: string | undefined, key: TranslationKey): string {
  const lang = (locale || defaultLang) as Lang;
  const langData = ui[lang];
  if (langData && key in langData) {
    return langData[key];
  }
  // Fallback do polskiego
  const fallback = ui[defaultLang];
  if (fallback && key in fallback) {
    return fallback[key];
  }
  return key;
}

/**
 * Zwraca URL dla danego języka, używając Astro url helper.
 */
export function getLocalizedUrl(currentUrl: URL, targetLang: Lang): string {
  const currentPath = currentUrl.pathname;

  // Usuń istniejący prefix językowy z URL
  let path = currentPath;
  for (const lang of Object.keys(languages)) {
    if (path.startsWith(`/${lang}/`)) {
      path = path.slice(lang.length + 1);
      break;
    }
    if (path === `/${lang}`) {
      path = '/';
      break;
    }
  }

  // Dla domyślnego języka nie dodajemy prefixu
  if (targetLang === defaultLang) {
    return path || '/';
  }

  return `/${targetLang}${path}`;
}

/**
 * Pobiera nazwę języka w jego natywnym zapisie.
 */
export function getNativeName(code: Lang): string {
  return languages[code]?.nativeName || code;
}

/**
 * Pobiera flagę języka.
 */
export function getFlag(code: Lang): string {
  return languages[code]?.flag || '🌐';
}
