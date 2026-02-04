<script lang="ts">
	import { DONATION_TIERS } from '$lib/config/env';

	const tiers = [
		{
			name: 'Bacon Bit',
			amount: DONATION_TIERS.BACON_BIT,
			emoji: '🥓',
			color: '#FFA500',
			gradient: 'linear-gradient(135deg, #FFA500 0%, #FF8C00 100%)'
		},
		{
			name: 'Crispy',
			amount: DONATION_TIERS.CRISPY,
			emoji: '✨',
			color: '#F7931E',
			gradient: 'linear-gradient(135deg, #F7931E 0%, #E67E00 100%)'
		},
		{
			name: 'Sizzling',
			amount: DONATION_TIERS.SIZZLING,
			emoji: '🔥',
			color: '#FF6B35',
			gradient: 'linear-gradient(135deg, #FF6B35 0%, #E85A24 100%)'
		},
		{
			name: 'Legendary',
			amount: DONATION_TIERS.LEGENDARY,
			emoji: '🏆',
			color: '#FFD700',
			gradient: 'linear-gradient(135deg, #FFD700 0%, #FFA500 100%)'
		}
	];
</script>

<div class="donation-tiers">
	<h3 class="tiers-title">Support the Stream 💰</h3>

	<div class="tiers-grid">
		{#each tiers as tier}
			<div class="tier-card glass-card" style="border-color: {tier.color}">
				<div class="tier-emoji">{tier.emoji}</div>
				<div class="tier-name">{tier.name}</div>
				<div class="tier-amount" style="color: {tier.color}">
					${tier.amount}
				</div>
			</div>
		{/each}
	</div>

	<div class="donation-info">
		<p>Every donation helps keep the stream running!</p>
	</div>
</div>

<style>
	.donation-tiers {
		width: 100%;
		max-width: 400px;
	}

	.tiers-title {
		font-size: 18px;
		font-weight: 700;
		color: #ffffff;
		margin: 0 0 16px 0;
		text-align: center;
	}

	.tiers-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 12px;
		margin-bottom: 16px;
	}

	.tier-card {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 20px 16px;
		transition: all 0.3s ease;
		cursor: pointer;
		border: 2px solid transparent;
	}

	.tier-card:hover {
		transform: translateY(-4px) scale(1.05);
		box-shadow: 0 8px 24px rgba(255, 107, 53, 0.3);
		background: rgba(255, 255, 255, 0.08);
	}

	.tier-emoji {
		font-size: 32px;
		margin-bottom: 8px;
		filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.3));
	}

	.tier-name {
		font-size: 14px;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.8);
		margin-bottom: 6px;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.tier-amount {
		font-size: 20px;
		font-weight: 700;
		text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
	}

	.donation-info {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		padding: 12px 16px;
		text-align: center;
	}

	.donation-info p {
		font-size: 13px;
		color: rgba(255, 255, 255, 0.7);
		margin: 0;
		line-height: 1.4;
	}
</style>
