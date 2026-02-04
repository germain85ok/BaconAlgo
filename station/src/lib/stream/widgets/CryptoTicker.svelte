<script lang="ts">
	import { onMount, onDestroy } from 'svelte';

	interface CryptoPrice {
		symbol: string;
		name: string;
		price: number;
		change24h: number;
	}

	let cryptos: CryptoPrice[] = [
		{ symbol: 'BTC', name: 'Bitcoin', price: 43250, change24h: 2.3 },
		{ symbol: 'ETH', name: 'Ethereum', price: 2280, change24h: 1.8 },
		{ symbol: 'SOL', name: 'Solana', price: 98.5, change24h: 5.2 },
		{ symbol: 'AVAX', name: 'Avalanche', price: 34.2, change24h: -1.5 },
		{ symbol: 'MATIC', name: 'Polygon', price: 0.82, change24h: 3.1 },
		{ symbol: 'ADA', name: 'Cardano', price: 0.54, change24h: -0.8 },
		{ symbol: 'DOT', name: 'Polkadot', price: 7.2, change24h: 1.2 },
		{ symbol: 'LINK', name: 'Chainlink', price: 14.8, change24h: 2.7 },
		{ symbol: 'UNI', name: 'Uniswap', price: 6.3, change24h: -2.1 },
		{ symbol: 'ATOM', name: 'Cosmos', price: 9.8, change24h: 4.5 },
		{ symbol: 'XRP', name: 'Ripple', price: 0.52, change24h: 0.9 },
		{ symbol: 'DOGE', name: 'Dogecoin', price: 0.085, change24h: 1.5 }
	];

	let interval: ReturnType<typeof setInterval>;

	function updatePrices() {
		cryptos = cryptos.map((crypto) => ({
			...crypto,
			price: crypto.price * (1 + (Math.random() - 0.5) * 0.01),
			change24h: crypto.change24h + (Math.random() - 0.5) * 0.5
		}));
	}

	function formatPrice(price: number): string {
		if (price >= 1000) return price.toLocaleString('en-US', { maximumFractionDigits: 0 });
		if (price >= 1) return price.toFixed(2);
		return price.toFixed(4);
	}

	onMount(() => {
		interval = setInterval(updatePrices, 3000);
	});

	onDestroy(() => {
		if (interval) clearInterval(interval);
	});
</script>

<div class="crypto-ticker">
	<div class="ticker-container">
		<div class="ticker-track">
			{#each [...cryptos, ...cryptos] as crypto, i (i)}
				<div class="ticker-item">
					<span class="crypto-symbol">{crypto.symbol}</span>
					<span class="crypto-price">${formatPrice(crypto.price)}</span>
					<span class="crypto-change" class:positive={crypto.change24h >= 0}>
						{crypto.change24h >= 0 ? '↑' : '↓'}
						{Math.abs(crypto.change24h).toFixed(2)}%
					</span>
				</div>
			{/each}
		</div>
	</div>
</div>

<style>
	.crypto-ticker {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		overflow: hidden;
		position: relative;
	}

	.crypto-ticker::before,
	.crypto-ticker::after {
		content: '';
		position: absolute;
		top: 0;
		bottom: 0;
		width: 50px;
		z-index: 1;
		pointer-events: none;
	}

	.crypto-ticker::before {
		left: 0;
		background: linear-gradient(to right, rgba(0, 0, 0, 0.3), transparent);
	}

	.crypto-ticker::after {
		right: 0;
		background: linear-gradient(to left, rgba(0, 0, 0, 0.3), transparent);
	}

	.ticker-container {
		overflow: hidden;
		padding: 1rem 0;
	}

	.ticker-track {
		display: flex;
		gap: 2rem;
		animation: scroll 40s linear infinite;
		will-change: transform;
	}

	.ticker-track:hover {
		animation-play-state: paused;
	}

	@keyframes scroll {
		0% {
			transform: translateX(0);
		}
		100% {
			transform: translateX(-50%);
		}
	}

	.ticker-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		white-space: nowrap;
		padding: 0 1rem;
		border-right: 1px solid rgba(255, 255, 255, 0.1);
	}

	.crypto-symbol {
		font-weight: 700;
		font-size: 0.875rem;
		color: rgba(255, 255, 255, 0.9);
	}

	.crypto-price {
		font-weight: 600;
		font-size: 0.875rem;
		color: rgba(255, 255, 255, 0.7);
	}

	.crypto-change {
		font-size: 0.75rem;
		font-weight: 600;
		color: #ef4444;
		padding: 0.25rem 0.5rem;
		border-radius: 4px;
		background: rgba(239, 68, 68, 0.1);
	}

	.crypto-change.positive {
		color: #10b981;
		background: rgba(16, 185, 129, 0.1);
	}
</style>
