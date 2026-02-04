<script lang="ts">
	import { topDonors } from '$lib/stores/donations';

	function getMedalColor(rank: number): string {
		if (rank === 0) return 'linear-gradient(135deg, #FFD700, #FFA500)';
		if (rank === 1) return 'linear-gradient(135deg, #C0C0C0, #A8A8A8)';
		if (rank === 2) return 'linear-gradient(135deg, #CD7F32, #B87333)';
		return 'rgba(255, 255, 255, 0.1)';
	}

	function getTrophyIcon(rank: number): string {
		if (rank === 0) return '🥇';
		if (rank === 1) return '🥈';
		if (rank === 2) return '🥉';
		return '';
	}
</script>

<div class="leaderboard">
	<div class="header">
		<h3>
			<svg
				width="20"
				height="20"
				viewBox="0 0 20 20"
				fill="none"
				xmlns="http://www.w3.org/2000/svg"
			>
				<path
					d="M10 2L12.5 7.5L18 8.5L14 13L15 18.5L10 15.5L5 18.5L6 13L2 8.5L7.5 7.5L10 2Z"
					fill="currentColor"
				/>
			</svg>
			Top Donors
		</h3>
	</div>

	<div class="donors-list">
		{#each $topDonors as donor, index (donor.id)}
			<div class="donor-row" class:top-three={index < 3}>
				<div class="rank-badge" style="background: {getMedalColor(index)}">
					{#if index < 3}
						<span class="trophy">{getTrophyIcon(index)}</span>
					{:else}
						<span class="rank-number">#{index + 1}</span>
					{/if}
				</div>

				<div class="donor-info">
					<div class="donor-name">{donor.name || 'Anonymous'}</div>
					<div class="donor-stats">
						<span class="donation-count">{donor.donation_count} donations</span>
					</div>
				</div>

				<div class="total-amount">${donor.total_donated.toFixed(2)}</div>
			</div>
		{/each}

		{#if $topDonors.length === 0}
			<div class="empty-state">
				<p>No donors yet. Be the first!</p>
			</div>
		{/if}
	</div>
</div>

<style>
	.leaderboard {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		padding: 1.5rem;
		min-width: 380px;
		max-height: 500px;
		display: flex;
		flex-direction: column;
	}

	.header {
		margin-bottom: 1rem;
		padding-bottom: 0.75rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	h3 {
		margin: 0;
		font-size: 1.125rem;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.9);
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	h3 svg {
		color: #ffd700;
		filter: drop-shadow(0 0 10px rgba(255, 215, 0, 0.5));
	}

	.donors-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		overflow-y: auto;
		flex: 1;
	}

	.donors-list::-webkit-scrollbar {
		width: 6px;
	}

	.donors-list::-webkit-scrollbar-track {
		background: rgba(255, 255, 255, 0.05);
		border-radius: 3px;
	}

	.donors-list::-webkit-scrollbar-thumb {
		background: rgba(249, 115, 22, 0.5);
		border-radius: 3px;
	}

	.donor-row {
		display: flex;
		align-items: center;
		gap: 1rem;
		background: rgba(255, 255, 255, 0.03);
		border-radius: 8px;
		padding: 0.875rem;
		transition: all 0.3s ease;
	}

	.donor-row:hover {
		background: rgba(255, 255, 255, 0.08);
		transform: translateX(4px);
	}

	.donor-row.top-three {
		border: 1px solid rgba(255, 215, 0, 0.3);
		box-shadow: 0 0 20px rgba(255, 215, 0, 0.1);
	}

	.rank-badge {
		width: 40px;
		height: 40px;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		font-weight: 700;
		flex-shrink: 0;
		box-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
	}

	.trophy {
		font-size: 1.25rem;
	}

	.rank-number {
		font-size: 0.875rem;
		color: rgba(255, 255, 255, 0.7);
	}

	.donor-info {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.donor-name {
		font-size: 0.9375rem;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.9);
	}

	.donor-stats {
		font-size: 0.75rem;
		color: rgba(255, 255, 255, 0.5);
	}

	.total-amount {
		font-size: 1.125rem;
		font-weight: 700;
		color: #f97316;
		text-shadow: 0 0 10px rgba(249, 115, 22, 0.3);
	}

	.empty-state {
		text-align: center;
		padding: 2rem;
		color: rgba(255, 255, 255, 0.5);
	}

	.empty-state p {
		margin: 0;
		font-size: 0.875rem;
	}
</style>
