import { writable } from 'svelte/store';

export type Language = 'fr' | 'en';
export type Theme = 'dark' | 'light';

export interface User {
	id: string;
	email: string;
	name: string;
	is_admin: boolean;
}

export interface AppState {
	user: User | null;
	language: Language;
	theme: Theme;
	isLoading: boolean;
}

const initialState: AppState = {
	user: null,
	language: 'fr',
	theme: 'dark',
	isLoading: false
};

function createAppStore() {
	const { subscribe, set, update } = writable<AppState>(initialState);

	return {
		subscribe,
		setUser: (user: User | null) => update(state => ({ ...state, user })),
		setLanguage: (language: Language) => update(state => ({ ...state, language })),
		setTheme: (theme: Theme) => update(state => ({ ...state, theme })),
		setLoading: (isLoading: boolean) => update(state => ({ ...state, isLoading })),
		reset: () => set(initialState)
	};
}

export const appStore = createAppStore();
