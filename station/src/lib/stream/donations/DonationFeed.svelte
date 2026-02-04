<script lang="ts">
	import { recentDonations } from '$lib/stores/donations';
	import { fade, fly } from 'svelte/transition';

	function formatTimeAgo(dateString: string): string {
		const date = new Date(dateString);
		const seconds = Math.floor((Date.now() - date.getTime()) / 1000);

		if (seconds < 60) return 'just now';
		const minutes = Math.floor(seconds / 60);
		if (minutes < 60) return `${minutes}m ago`;
		const hours = Math.floor(minutes / 60);
		if (hours < 24) return `${hours}h ago`;
		const days = Math.floor(hours / 24);
		return `${days}d ago`;
	}

	function formatAmount(amount: number): string {
		return `$${amount.toFixed(2)}`;
	}
</script>

<div class="donation-feed">
	<div class="header">
		<h3>
			<svg
				width="16"
				height="16"
				viewBox="0 0 16 16"
				fill="none"
				xmlns="http://www.w3.org/2000/svg"
			>
				<path
					d="M8 2C8.82843 2 9.5 2.67157 9.5 3.5V4H6.5V3.5C6.5 2.67157 7.17157 2 8 2Z"
					fill="currentColor"
				/>
				<path
					d="M4 5H12V13C12 13.5523 11.5523 14 11 14H5C4.44772 14 4 13.5523 4 13V5Z"
					fill="currentColor"
				/>
			</svg>
			Recent Donations
		</h3>
		<span class="count">{$recentDonations.length}</span>
	</div>

	<div class="donations-list">
		{#each $recentDonations as donation (donation.id)}
			<div class="donation-item" in:fly={{ y: -20, duration: 300 }} out:fade={{ duration: 200 }}>
				<div class="donor-info">
					<div class="donor-avatar">
						{donation.donor_name?.charAt(0).toUpperCase() || '?'}
					</div>
					<div class="donor-details">
						<div class="donor-name">{donation.donor_name || 'Anonymous'}</div>
						<div class="timestamp">{formatTimeAgo(donation.created_at)}</div>
					</div>
				</div>
				<div class="amount">{formatAmount(donation.amount)}</div>
			</div>
		{/each}

		{#if $recentDonations.length === 0}
			<div class="empty-state">
				<p>No donations yet. Be the first to support!</p>
			</div>
		{/if}
	</div>
</div>

<style>
	.donation-feed {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		padding: 1.5rem;
		min-width: 320px;
		max-height: 450px;
		display: flex;
		flex-direction: column;
	}

	.header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
		padding-bottom: 0.75rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	h3 {
		margin: 0;
		font-size: 1rem;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.9);
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	h3 svg {
		color: #f97316;
	}

	.count {
		background: rgba(249, 115, 22, 0.2);
		color: #f97316;
		font-size: 0.75rem;
		font-weight: 700;
		padding: 0.25rem 0.5rem;
		border-radius: 12px;
		min-width: 24px;
		text-align: center;
	}

	.donations-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		overflow-y: auto;
		flex: 1;
	}

	.donations-list::-webkit-scrollbar {
		width: 6px;
	}

	.donations-list::-webkit-scrollbar-track {
		background: rgba(255, 255, 255, 0.05);
		border-radius: 3px;
	}

	.donations-list::-webkit-scrollbar-thumb {
		background: rgba(249, 115, 22, 0.5);
		border-radius: 3px;
	}

	.donation-item {
		display: flex;
		justify-content: space-between;
		align-items: center;
		background: rgba(255, 255, 255, 0.03);
		border-left: 3px solid #f97316;
		border-radius: 6px;
		padding: 0.75rem;
		transition: all 0.3s ease;
	}

	.donation-item:hover {
		background: rgba(255, 255, 255, 0.08);
		transform: translateX(4px);
	}

	.donor-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.donor-avatar {
		width: 36px;
		height: 36px;
		border-radius: 50%;
		background: linear-gradient(135deg, #f97316, #ff6b35);
		display: flex;
		align-items: center;
		justify-content: center;
		font-weight: 700;
		font-size: 1rem;
		color: white;
		flex-shrink: 0;
	}

	.donor-details {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.donor-name {
		font-size: 0.875rem;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.9);
	}

	.timestamp {
		font-size: 0.75rem;
		color: rgba(255, 255, 255, 0.5);
	}

	.amount {
		font-size: 1rem;
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
