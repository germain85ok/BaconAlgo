/**
 * Store exports
 * Central export point for all stores
 */

// Signals store
export {
	signals,
	hotSignals,
	buySignals,
	sellSignals,
	addSignal,
	clearOldSignals,
	initSignalsStore,
	cleanupSignalsStore
} from './signals';

// Donations store
export {
	donations,
	topDonors,
	todaysTotal,
	goalProgress,
	donationAlert,
	recentDonations,
	addDonation,
	showDonationAlert,
	clearDonationAlert,
	initDonationsStore,
	cleanupDonationsStore
} from './donations';

// Market store
export {
	fearGreedIndex,
	vixValue,
	topMovers,
	marketStatus,
	updateFearGreed,
	updateVix,
	updateTopMovers,
	updateMarketStatus,
	isMarketOpen,
	getNextMarketOpen,
	initMarketStore
} from './market';

// Settings store
export {
	settings,
	updateSetting,
	resetSettings,
	toggleSetting,
	setVolume,
	setTheme,
	setLayout
} from './settings';

export type { StreamSettings } from './settings';
export type { TopMover, MarketStatus } from './market';
