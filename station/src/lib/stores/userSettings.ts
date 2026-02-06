import { writable } from 'svelte/store';

export interface BrokerConfig {
	name: 'alpaca' | 'ib' | 'questrade' | 'bitget';
	apiKey: string;
	apiSecret: string;
	isPaper: boolean;
	isConnected: boolean;
}

export interface AutoTradingSettings {
	enabled: boolean;
	maxPositionSize: number;
	maxDailyLoss: number;
	minSignalScore: number;
	allowedSymbols: string[];
	broker: string;
}

export interface AlertSettings {
	email: boolean;
	discord: boolean;
	sound: boolean;
	minSignalScore: number;
}

export interface UserSettings {
	brokers: BrokerConfig[];
	autoTrading: AutoTradingSettings;
	alerts: AlertSettings;
	riskLimits: {
		maxPositions: number;
		maxDrawdown: number;
		maxLeverage: number;
	};
}

const defaultSettings: UserSettings = {
	brokers: [],
	autoTrading: {
		enabled: false,
		maxPositionSize: 10000,
		maxDailyLoss: 500,
		minSignalScore: 70,
		allowedSymbols: [],
		broker: ''
	},
	alerts: {
		email: true,
		discord: false,
		sound: true,
		minSignalScore: 60
	},
	riskLimits: {
		maxPositions: 5,
		maxDrawdown: 0.1,
		maxLeverage: 1
	}
};

function createUserSettingsStore() {
	const { subscribe, set, update } = writable<UserSettings>(defaultSettings);

	return {
		subscribe,
		updateBroker: (broker: BrokerConfig) => update(state => {
			const index = state.brokers.findIndex(b => b.name === broker.name);
			if (index >= 0) {
				state.brokers[index] = broker;
			} else {
				state.brokers.push(broker);
			}
			return { ...state };
		}),
		removeBroker: (name: string) => update(state => ({
			...state,
			brokers: state.brokers.filter(b => b.name !== name)
		})),
		updateAutoTrading: (settings: Partial<AutoTradingSettings>) => update(state => ({
			...state,
			autoTrading: { ...state.autoTrading, ...settings }
		})),
		updateAlerts: (settings: Partial<AlertSettings>) => update(state => ({
			...state,
			alerts: { ...state.alerts, ...settings }
		})),
		updateRiskLimits: (limits: Partial<UserSettings['riskLimits']>) => update(state => ({
			...state,
			riskLimits: { ...state.riskLimits, ...limits }
		})),
		reset: () => set(defaultSettings)
	};
}

export const userSettingsStore = createUserSettingsStore();
