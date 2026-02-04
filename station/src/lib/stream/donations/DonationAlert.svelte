<script lang="ts">
	import { donationAlert, clearDonationAlert } from '$lib/stores/donations';
	import { getDonationTier, ALERT_DURATION } from '$lib/config/env';
	import { onDestroy } from 'svelte';
	import { fly, scale } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';

	let showAlert = false;
	let timeout: ReturnType<typeof setTimeout>;

	$: if ($donationAlert && !showAlert) {
		showAlert = true;
		// Auto-dismiss after ALERT_DURATION seconds
		timeout = setTimeout(() => {
			showAlert = false;
			setTimeout(() => clearDonationAlert(), 500);
		}, ALERT_DURATION * 1000);
	}

	$: tier = $donationAlert ? getDonationTier($donationAlert.amount) : null;
	$: isLegendary = $donationAlert && $donationAlert.amount >= 20;

	onDestroy(() => {
		if (timeout) clearTimeout(timeout);
	});

	function createConfetti() {
		const confettiCount = isLegendary ? 50 : 20;
		const colors = ['#FFD700', '#FF6B35', '#F7931E', '#FFA500', '#FF1493'];
		const confetti = [];

		for (let i = 0; i < confettiCount; i++) {
			confetti.push({
				x: Math.random() * 100,
				delay: Math.random() * 0.5,
				duration: 2 + Math.random() * 2,
				color: colors[Math.floor(Math.random() * colors.length)]
			});
		}
		return confetti;
	}

	$: confetti = showAlert && (isLegendary || ($donationAlert && $donationAlert.amount >= 10))
		? createConfetti()
		: [];
</script>

{#if showAlert && $donationAlert}
	<div class="donation-alert-overlay">
		<div
			class="donation-alert"
			class:legendary={isLegendary}
			transition:fly={{ y: -100, duration: 500, easing: quintOut }}
		>
			<!-- Confetti -->
			{#if confetti.length > 0}
				<div class="confetti-container">
					{#each confetti as piece, i (i)}
						<div
							class="confetti"
							style="left: {piece.x}%; animation-delay: {piece.delay}s; animation-duration: {piece.duration}s; background: {piece.color};"
						></div>
					{/each}
				</div>
			{/if}

			<!-- Alert Content -->
			<div class="alert-content">
				<div class="alert-header">
					<span class="emoji" transition:scale={{ duration: 300, delay: 200 }}>
						{tier?.emoji || '🎉'}
					</span>
					<h2 class="title">
						{#if isLegendary}
							⚡ LEGENDARY DONATION! ⚡
						{:else}
							New Donation!
						{/if}
					</h2>
				</div>

				<div class="donor-info">
					<p class="donor-name">{$donationAlert.donor_name}</p>
					<p class="amount" style="color: {tier?.color}">
						${$donationAlert.amount.toFixed(2)}
					</p>
					{#if tier}
						<span class="tier-badge" style="background: {tier.color}">
							{tier.name}
						</span>
					{/if}
				</div>

				{#if $donationAlert.message}
					<div class="message">
						<p>"{$donationAlert.message}"</p>
					</div>
				{/if}

				<div class="thank-you">Thank you for your support! 🙏</div>
			</div>
		</div>
	</div>
{/if}

<style>
	.donation-alert-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		pointer-events: none;
		z-index: 9999;
	}

	.donation-alert {
		position: relative;
		background: rgba(0, 0, 0, 0.9);
		backdrop-filter: blur(20px);
		border: 2px solid var(--brand-primary);
		border-radius: 24px;
		padding: 40px 60px;
		max-width: 700px;
		box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5), 0 0 40px rgba(255, 107, 53, 0.3);
		pointer-events: all;
	}

	.donation-alert.legendary {
		border-color: #ffd700;
		animation: legendary-pulse 1s ease-in-out infinite;
		box-shadow:
			0 20px 60px rgba(0, 0, 0, 0.5),
			0 0 60px rgba(255, 215, 0, 0.5),
			inset 0 0 40px rgba(255, 215, 0, 0.1);
	}

	@keyframes legendary-pulse {
		0%,
		100% {
			box-shadow:
				0 20px 60px rgba(0, 0, 0, 0.5),
				0 0 60px rgba(255, 215, 0, 0.5),
				inset 0 0 40px rgba(255, 215, 0, 0.1);
		}
		50% {
			box-shadow:
				0 20px 60px rgba(0, 0, 0, 0.5),
				0 0 80px rgba(255, 215, 0, 0.7),
				inset 0 0 60px rgba(255, 215, 0, 0.2);
		}
	}

	.confetti-container {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		overflow: hidden;
		pointer-events: none;
	}

	.confetti {
		position: absolute;
		width: 10px;
		height: 10px;
		top: -10px;
		animation: confetti-fall linear forwards;
	}

	@keyframes confetti-fall {
		to {
			transform: translateY(600px) rotate(720deg);
			opacity: 0;
		}
	}

	.alert-content {
		position: relative;
		z-index: 1;
		text-align: center;
	}

	.alert-header {
		margin-bottom: 24px;
	}

	.emoji {
		display: block;
		font-size: 80px;
		margin-bottom: 16px;
		filter: drop-shadow(0 4px 8px rgba(0, 0, 0, 0.3));
	}

	.title {
		font-size: 36px;
		font-weight: 700;
		color: #ffffff;
		margin: 0;
		text-transform: uppercase;
		letter-spacing: 2px;
		text-shadow: 0 2px 10px rgba(0, 0, 0, 0.5);
	}

	.donor-info {
		margin-bottom: 24px;
	}

	.donor-name {
		font-size: 28px;
		font-weight: 700;
		color: #ffffff;
		margin: 0 0 12px 0;
	}

	.amount {
		font-size: 48px;
		font-weight: 700;
		margin: 0 0 12px 0;
		text-shadow: 0 2px 10px rgba(0, 0, 0, 0.5);
	}

	.tier-badge {
		display: inline-block;
		padding: 8px 20px;
		font-size: 16px;
		font-weight: 700;
		color: #000;
		border-radius: 20px;
		text-transform: uppercase;
		letter-spacing: 1px;
	}

	.message {
		background: rgba(255, 255, 255, 0.05);
		border-radius: 16px;
		padding: 20px 28px;
		margin-bottom: 24px;
		border: 1px solid rgba(255, 255, 255, 0.1);
	}

	.message p {
		font-size: 18px;
		color: rgba(255, 255, 255, 0.9);
		margin: 0;
		font-style: italic;
		line-height: 1.5;
	}

	.thank-you {
		font-size: 20px;
		color: var(--brand-accent);
		font-weight: 600;
	}
</style>
