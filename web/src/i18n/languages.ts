// Definicje języków — paganlinux.eu
// Dodawanie nowego języka: dodaj tu i w ui.ts

export type Lang = 'pl' | 'en' | 'de' | 'fr' | 'es' | 'it';

export interface LanguageDef {
  code: Lang;
  nativeName: string;    // np. "Polski"
  englishName: string;   // np. "Polish"
  flag: string;          // emoji flagi
  dir: 'ltr' | 'rtl';
}

export const languages: Record<Lang, LanguageDef> = {
  pl: { code: 'pl', nativeName: 'Polski',   englishName: 'Polish',   flag: '🇵🇱', dir: 'ltr' },
  en: { code: 'en', nativeName: 'English',  englishName: 'English',  flag: '🇬🇧', dir: 'ltr' },
  de: { code: 'de', nativeName: 'Deutsch',  englishName: 'German',   flag: '🇩🇪', dir: 'ltr' },
  fr: { code: 'fr', nativeName: 'Français', englishName: 'French',   flag: '🇫🇷', dir: 'ltr' },
  es: { code: 'es', nativeName: 'Español',  englishName: 'Spanish',  flag: '🇪🇸', dir: 'ltr' },
  it: { code: 'it', nativeName: 'Italiano', englishName: 'Italian',  flag: '🇮🇹', dir: 'ltr' },
};

export const defaultLang: Lang = 'pl';
export const supportedLangs: Lang[] = Object.keys(languages) as Lang[];
