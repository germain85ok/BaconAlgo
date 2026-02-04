<script lang="ts">
	import { todaysTotal, goalProgress } from '$lib/stores/donations';
	import { DAILY_DONATION_GOAL } from '$lib/config/env';

	$: percentage = ($goalProgress * 100).toFixed(0);
	$: remaining = Math.max(0, DAILY_DONATION_GOAL - $todaysTotal);

	// Milestone markers at 25%, 50%, 75%, 100%
	const milestones = [25, 50, 75, 100];
</script>

<div class="donation-goal-tracker glass-card">
	<div class="header">
		<h3>Daily Goal</h3>
		<span class="goal-amount">${DAILY_DONATION_GOAL}</span>
	</div>

	<div class="progress-container">
		<div class="progress-bar-bg">
			<div class="progress-bar" style="width: {percentage}%">
				<div class="progress-shimmer"></div>
			</div>

			<!-- Milestone markers -->
			{#each milestones as milestone}
				<div class="milestone" style="left: {milestone}%" class:reached={$goalProgress * 100 >= milestone}>
					<div class="milestone-marker"></div>
					<span class="milestone-label">{milestone}%</span>
				</div>
			{/each}
		</div>
	</div>

	<div class="stats">
		<div class="stat">
			<span class="stat-label">Raised</span>
			<span class="stat-value">${$todaysTotal.toFixed(2)}</span>
		</div>
		<div class="stat">
			<span class="stat-label">Remaining</span>
			<span class="stat-value">${remaining.toFixed(2)}</span>
		</div>
	</div>

	<div class="percentage-display">{percentage}%</div>
</div>

<style>
	.donation-goal-tracker {
		width: 100%;
		max-width: 400px;
		padding: 24px;
	}

	.header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 20px;
	}

	h3 {
		font-size: 18px;
		font-weight: 700;
		color: #ffffff;
		margin: 0;
	}

	.goal-amount {
		font-size: 20px;
		font-weight: 700;
		color: var(--brand-accent);
	}

	.progress-container {
		margin-bottom: 20px;
		position: relative;
	}

	.progress-bar-bg {
		width: 100%;
		height: 24px;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		overflow: hidden;
		position: relative;
		box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.2);
	}

	.progress-bar {
		height: 100%;
		background: linear-gradient(90deg, var(--brand-primary) 0%, var(--brand-accent) 100%);
		border-radius: 12px;
		transition: width 0.5s ease;
		position: relative;
		overflow: hidden;
	}

	.progress-shimmer {
		position: absolute;
		top: 0;
		left: -100%;
		width: 100%;
		height: 100%;
		background: linear-gradient(
			90deg,
			transparent,
			rgba(255, 255, 255, 0.3),
			transparent
		);
		animation: shimmer 2s infinite;
	}

	@keyframes shimmer {
		0% {
			left: -100%;
		}
		100% {
			left: 100%;
		}
	}

	.milestone {
		position: absolute;
		top: -8px;
		transform: translateX(-50%);
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 4px;
	}

	.milestone-marker {
		width: 8px;
		height: 40px;
		background: rgba(255, 255, 255, 0.3);
		border-radius: 4px;
		transition: background 0.3s ease;
	}

	.milestone.reached .milestone-marker {
		background: #2ecc71;
		box-shadow: 0 0 10px rgba(46, 204, 113, 0.5);
	}

	.milestone-label {
		font-size: 10px;
		color: rgba(255, 255, 255, 0.5);
		font-weight: 600;
	}

	.milestone.reached .milestone-label {
		color: #2ecc71;
	}

	.stats {
		display: flex;
		justify-content: space-between;
		margin-bottom: 16px;
		gap: 16px;
	}

	.stat {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 4px;
	}

	.stat-label {
		font-size: 12px;
		color: rgba(255, 255, 255, 0.6);
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.stat-value {
		font-size: 18px;
		font-weight: 700;
		color: #ffffff;
	}

	.percentage-display {
		text-align: center;
		font-size: 32px;
		font-weight: 700;
		color: var(--brand-accent);
		text-shadow: 0 2px 10px rgba(247, 147, 30, 0.5);
	}
</style>
