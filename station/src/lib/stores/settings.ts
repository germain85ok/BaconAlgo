/**
 * Settings Store
 * Manages user preferences with localStorage persistence
 */

import { writable } from 'svelte/store';
import { browser } from '$app/environment';

export interface StreamSettings {
	theme: 'dark' | 'light' | 'bacon';
	volume: number;
	showChat: boolean;
	showDonations: boolean;
	showSignals: boolean;
	showMarketData: boolean;
	streamLayout: 'default' | 'minimal' | 'full';
	enableSounds: boolean;
	enableNotifications: boolean;
	autoHideAlerts: boolean;
}

// Default settings
const defaultSettings: StreamSettings = {
	theme: 'dark',
	volume: 0.5,
	showChat: true,
	showDonations: true,
	showSignals: true,
	showMarketData: true,
	streamLayout: 'default',
	enableSounds: true,
	enableNotifications: true,
	autoHideAlerts: true
};

// Storage key
const STORAGE_KEY = 'baconalgo_settings';

/**
 * Load settings from localStorage
 */
function loadSettings(): StreamSettings {
	if (!browser) return defaultSettings;

	try {
		const stored = localStorage.getItem(STORAGE_KEY);
		if (stored) {
			const parsed = JSON.parse(stored);
			return { ...defaultSettings, ...parsed };
		}
	} catch (error) {
		console.error('Failed to load settings from localStorage:', error);
	}

	return defaultSettings;
}

/**
 * Save settings to localStorage
 */
function saveSettings(settings: StreamSettings) {
	if (!browser) return;

	try {
		localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
	} catch (error) {
		console.error('Failed to save settings to localStorage:', error);
	}
}

// Create the store with initial value from localStorage
export const settings = writable<StreamSettings>(loadSettings());

// Subscribe to changes and save to localStorage
if (browser) {
	settings.subscribe((value) => {
		saveSettings(value);
	});
}

/**
 * Update a specific setting
 */
export function updateSetting<K extends keyof StreamSettings>(
	key: K,
	value: StreamSettings[K]
) {
	settings.update((current) => ({
		...current,
		[key]: value
	}));
}

/**
 * Reset settings to defaults
 */
export function resetSettings() {
	settings.set(defaultSettings);
}

/**
 * Toggle a boolean setting
 */
export function toggleSetting(key: keyof StreamSettings) {
	settings.update((current) => ({
		...current,
		[key]: !current[key]
	}));
}

/**
 * Update volume (0-1)
 */
export function setVolume(volume: number) {
	updateSetting('volume', Math.max(0, Math.min(1, volume)));
}

/**
 * Update theme
 */
export function setTheme(theme: StreamSettings['theme']) {
	updateSetting('theme', theme);
}

/**
 * Update layout
 */
export function setLayout(layout: StreamSettings['streamLayout']) {
	updateSetting('streamLayout', layout);
}
