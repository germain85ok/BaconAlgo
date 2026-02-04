<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { fade } from 'svelte/transition';

	interface Sponsor {
		id: string;
		name: string;
		logo: string;
		website: string;
	}

	const sponsors: Sponsor[] = [
		{
			id: '1',
			name: 'Premium Trading Sponsor',
			logo: '📊',
			website: 'https://example.com'
		},
		{
			id: '2',
			name: 'Elite Analytics Partner',
			logo: '📈',
			website: 'https://example.com'
		},
		{
			id: '3',
			name: 'Pro Market Data',
			logo: '💹',
			website: 'https://example.com'
		}
	];

	let currentIndex = 0;
	let interval: ReturnType<typeof setInterval>;

	function nextSponsor() {
		currentIndex = (currentIndex + 1) % sponsors.length;
	}

	onMount(() => {
		interval = setInterval(nextSponsor, 10000);
	});

	onDestroy(() => {
		if (interval) clearInterval(interval);
	});

	$: currentSponsor = sponsors[currentIndex];
</script>

<div class="sponsor-banner">
	{#key currentSponsor.id}
		<div class="sponsor-content" in:fade={{ duration: 500 }}>
			<div class="sponsor-label">Sponsored by</div>
			<div class="sponsor-main">
				<div class="sponsor-logo">{currentSponsor.logo}</div>
				<div class="sponsor-info">
					<div class="sponsor-name">{currentSponsor.name}</div>
					<a
						href={currentSponsor.website}
						target="_blank"
						rel="noopener noreferrer"
						class="sponsor-link"
					>
						Visit Website →
					</a>
				</div>
			</div>
			<div class="cta-section">
				<button class="cta-button">
					<span>Sponsor This Stream</span>
					<div class="shine" />
				</button>
			</div>
		</div>
	{/key}

	<div class="progress-dots">
		{#each sponsors as sponsor, i (sponsor.id)}
			<button
				class="dot"
				class:active={i === currentIndex}
				on:click={() => (currentIndex = i)}
				aria-label="Go to sponsor {i + 1}"
			/>
		{/each}
	</div>
</div>

<style>
	.sponsor-banner {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		padding: 1.5rem;
		min-width: 350px;
		position: relative;
		overflow: hidden;
	}

	.sponsor-banner::before {
		content: '';
		position: absolute;
		top: -50%;
		left: -50%;
		width: 200%;
		height: 200%;
		background: radial-gradient(circle, rgba(249, 115, 22, 0.1) 0%, transparent 70%);
		animation: rotate 10s linear infinite;
	}

	@keyframes rotate {
		0% {
			transform: rotate(0deg);
		}
		100% {
			transform: rotate(360deg);
		}
	}

	.sponsor-content {
		position: relative;
		z-index: 1;
	}

	.sponsor-label {
		font-size: 0.75rem;
		text-transform: uppercase;
		letter-spacing: 0.1em;
		color: rgba(255, 255, 255, 0.5);
		margin-bottom: 0.75rem;
		text-align: center;
	}

	.sponsor-main {
		display: flex;
		align-items: center;
		gap: 1rem;
		margin-bottom: 1rem;
	}

	.sponsor-logo {
		width: 60px;
		height: 60px;
		background: linear-gradient(135deg, #f97316, #ff6b35);
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 2rem;
		flex-shrink: 0;
		box-shadow: 0 4px 15px rgba(249, 115, 22, 0.3);
	}

	.sponsor-info {
		flex: 1;
	}

	.sponsor-name {
		font-size: 1rem;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.9);
		margin-bottom: 0.5rem;
	}

	.sponsor-link {
		font-size: 0.875rem;
		color: #f97316;
		text-decoration: none;
		display: flex;
		align-items: center;
		gap: 0.25rem;
		transition: all 0.3s ease;
	}

	.sponsor-link:hover {
		color: #ff6b35;
		gap: 0.5rem;
	}

	.cta-section {
		display: flex;
		justify-content: center;
		padding-top: 1rem;
		border-top: 1px solid rgba(255, 255, 255, 0.1);
	}

	.cta-button {
		background: linear-gradient(135deg, #f97316, #ff6b35);
		border: none;
		border-radius: 8px;
		padding: 0.75rem 1.5rem;
		color: white;
		font-size: 0.875rem;
		font-weight: 600;
		cursor: pointer;
		position: relative;
		overflow: hidden;
		transition: all 0.3s ease;
	}

	.cta-button:hover {
		transform: translateY(-2px);
		box-shadow: 0 8px 20px rgba(249, 115, 22, 0.4);
	}

	.shine {
		position: absolute;
		top: 0;
		left: -100%;
		width: 100%;
		height: 100%;
		background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.3), transparent);
		animation: shine 3s infinite;
	}

	@keyframes shine {
		0% {
			left: -100%;
		}
		20%,
		100% {
			left: 100%;
		}
	}

	.progress-dots {
		display: flex;
		justify-content: center;
		gap: 0.5rem;
		margin-top: 1rem;
	}

	.dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: rgba(255, 255, 255, 0.3);
		border: none;
		cursor: pointer;
		padding: 0;
		transition: all 0.3s ease;
	}

	.dot.active {
		background: #f97316;
		width: 24px;
		border-radius: 4px;
	}
</style>
