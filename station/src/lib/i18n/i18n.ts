// i18n Store and Helper Functions
import { writable, derived, get } from 'svelte/store';
import { translations, type Language } from './translations';
import { browser } from '$app/environment';

// Locale store
function createLocaleStore() {
	// Default to French
	const defaultLang: Language = 'fr';
	
	// Get initial language from localStorage if in browser
	const initialLang: Language = browser 
		? (localStorage.getItem('baconalgo-lang') as Language) || defaultLang
		: defaultLang;
	
	const { subscribe, set } = writable<Language>(initialLang);

	return {
		subscribe,
		set: (lang: Language) => {
			if (browser) {
				localStorage.setItem('baconalgo-lang', lang);
			}
			set(lang);
		},
		toggle: () => {
			const currentLang = get(locale);
			const newLang: Language = currentLang === 'fr' ? 'en' : 'fr';
			if (browser) {
				localStorage.setItem('baconalgo-lang', newLang);
			}
			set(newLang);
		}
	};
}

export const locale = createLocaleStore();

// Derived store for current translations
export const t = derived(locale, ($locale) => translations[$locale]);

// Helper function to get nested translation
export function getTranslation(key: string, lang?: Language): string {
	const currentLang = lang || get(locale);
	const keys = key.split('.');
	let value: any = translations[currentLang];
	
	for (const k of keys) {
		if (value && typeof value === 'object' && k in value) {
			value = value[k];
		} else {
			console.warn(`Translation key not found: ${key}`);
			return key;
		}
	}
	
	return typeof value === 'string' ? value : key;
}
