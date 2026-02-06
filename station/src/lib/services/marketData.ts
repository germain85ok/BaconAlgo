export interface MarketIndex {
	symbol: string;
	name: string;
	price: number;
	change: number;
	changePercent: number;
}

export interface CryptoAsset {
	symbol: string;
	name: string;
	price: number;
	change24h: number;
	changePercent24h: number;
	volume24h: number;
	marketCap: number;
}

export interface Commodity {
	symbol: string;
	name: string;
	price: number;
	change: number;
	changePercent: number;
}

export interface TopMover {
	symbol: string;
	name: string;
	price: number;
	changePercent: number;
	volume: number;
}

export interface MarketSummary {
	indices: MarketIndex[];
	crypto: CryptoAsset[];
	commodities: Commodity[];
	gainers: TopMover[];
	losers: TopMover[];
	lastUpdated: Date;
}

class MarketDataService {
	private apiBaseUrl = '/api';
	private updateInterval: number | null = null;

	async fetchIndices(): Promise<MarketIndex[]> {
		// Mock data for now - will be replaced with real API calls
		return [
			{ symbol: 'SPY', name: 'S&P 500 ETF', price: 450.25, change: 5.32, changePercent: 1.19 },
			{ symbol: 'QQQ', name: 'Nasdaq 100 ETF', price: 380.15, change: 4.18, changePercent: 1.11 },
			{ symbol: 'DIA', name: 'Dow Jones ETF', price: 350.88, change: 2.95, changePercent: 0.85 },
			{ symbol: 'XIU.TO', name: 'TSX 60 ETF', price: 32.45, change: 0.15, changePercent: 0.47 }
		];
	}

	async fetchCrypto(): Promise<CryptoAsset[]> {
		// Mock data - top 20 crypto
		const cryptos = [
			'BTC', 'ETH', 'BNB', 'SOL', 'XRP', 'ADA', 'DOGE', 'AVAX', 'DOT', 'MATIC',
			'LINK', 'UNI', 'ATOM', 'LTC', 'NEAR', 'APT', 'ARB', 'OP', 'SUI', 'INJ'
		];
		
		return cryptos.map((symbol, i) => ({
			symbol,
			name: `${symbol} Token`,
			price: Math.random() * 50000,
			change24h: (Math.random() - 0.5) * 5000,
			changePercent24h: (Math.random() - 0.5) * 10,
			volume24h: Math.random() * 1000000000,
			marketCap: Math.random() * 100000000000
		}));
	}

	async fetchCommodities(): Promise<Commodity[]> {
		return [
			{ symbol: 'GC=F', name: 'Gold', price: 2050.30, change: 12.45, changePercent: 0.61 },
			{ symbol: 'CL=F', name: 'Crude Oil', price: 78.22, change: -1.33, changePercent: -1.67 },
			{ symbol: 'SI=F', name: 'Silver', price: 24.15, change: 0.38, changePercent: 1.60 }
		];
	}

	async fetchTopMovers(): Promise<{ gainers: TopMover[], losers: TopMover[] }> {
		// Mock data
		const gainers: TopMover[] = [
			{ symbol: 'NVDA', name: 'NVIDIA', price: 875.50, changePercent: 8.5, volume: 45000000 },
			{ symbol: 'AMD', name: 'AMD', price: 165.30, changePercent: 7.2, volume: 38000000 },
			{ symbol: 'TSLA', name: 'Tesla', price: 245.80, changePercent: 6.8, volume: 52000000 }
		];

		const losers: TopMover[] = [
			{ symbol: 'META', name: 'Meta', price: 380.20, changePercent: -5.3, volume: 28000000 },
			{ symbol: 'AAPL', name: 'Apple', price: 178.50, changePercent: -3.9, volume: 65000000 },
			{ symbol: 'GOOGL', name: 'Alphabet', price: 138.75, changePercent: -3.2, volume: 42000000 }
		];

		return { gainers, losers };
	}

	async fetchMarketSummary(): Promise<MarketSummary> {
		const [indices, crypto, commodities, movers] = await Promise.all([
			this.fetchIndices(),
			this.fetchCrypto(),
			this.fetchCommodities(),
			this.fetchTopMovers()
		]);

		return {
			indices,
			crypto,
			commodities,
			gainers: movers.gainers,
			losers: movers.losers,
			lastUpdated: new Date()
		};
	}

	startAutoUpdate(callback: (data: MarketSummary) => void, intervalMs: number = 10000) {
		if (this.updateInterval) {
			this.stopAutoUpdate();
		}

		// Initial fetch
		this.fetchMarketSummary().then(callback);

		// Set up interval
		this.updateInterval = window.setInterval(async () => {
			const data = await this.fetchMarketSummary();
			callback(data);
		}, intervalMs);
	}

	stopAutoUpdate() {
		if (this.updateInterval) {
			clearInterval(this.updateInterval);
			this.updateInterval = null;
		}
	}
}

export const marketDataService = new MarketDataService();
