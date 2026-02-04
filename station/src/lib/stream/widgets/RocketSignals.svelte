<script lang="ts">
	import { hotSignals } from '$lib/stores/signals';
	import type { Signal } from '$lib/supabase/client';
	import { fly, fade } from 'svelte/transition';

	function getGradeColor(grade: string): string {
		switch (grade) {
			case 'S':
				return 'var(--grade-s)';
			case 'A':
				return 'var(--grade-a)';
			case 'B':
				return 'var(--grade-b)';
			case 'C':
				return 'var(--grade-c)';
			default:
				return '#95a5a6';
		}
	}

	function getDirectionColor(direction: string): string {
		return direction === 'BUY' ? 'var(--buy-color)' : 'var(--sell-color)';
	}

	function formatTime(dateString: string): string {
		const date = new Date(dateString);
		return date.toLocaleTimeString('en-US', {
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function getGradeEmoji(grade: string): string {
		switch (grade) {
			case 'S':
				return '🚀';
			case 'A':
				return '⭐';
			case 'B':
				return '📈';
			case 'C':
				return '📊';
			default:
				return '📉';
		}
	}
</script>

<div class="rocket-signals glass-card">
	<div class="header">
		<h3>🚀 Hot Signals</h3>
		<span class="live-badge">
			<span class="live-dot"></span>
			LIVE
		</span>
	</div>

	<div class="signals-list">
		{#if $hotSignals.length === 0}
			<div class="empty-state">
				<span class="empty-icon">🔍</span>
				<p>Scanning for signals...</p>
			</div>
		{:else}
			{#each $hotSignals as signal (signal.id)}
				<div class="signal-card" transition:fly={{ x: -20, duration: 300 }}>
					<div class="signal-header">
						<div class="ticker-section">
							<span class="grade-badge" style="background: {getGradeColor(signal.grade)}">
								{signal.grade}
							</span>
							<span class="ticker">{signal.ticker}</span>
							<span class="grade-emoji">{getGradeEmoji(signal.grade)}</span>
						</div>
						<span
							class="direction-badge {signal.direction.toLowerCase()}"
							style="background: {getDirectionColor(signal.direction)}"
						>
							{signal.direction}
						</span>
					</div>

					<div class="signal-details">
						<div class="score">
							<span class="score-label">Score:</span>
							<span class="score-value">{signal.score.toFixed(1)}</span>
						</div>
						<div class="timestamp">{formatTime(signal.created_at)}</div>
					</div>
				</div>
			{/each}
		{/if}
	</div>
</div>

<style>
	.rocket-signals {
		width: 100%;
		max-width: 600px;
		padding: 24px;
		max-height: 600px;
		display: flex;
		flex-direction: column;
	}

	.header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 20px;
		padding-bottom: 16px;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	h3 {
		font-size: 20px;
		font-weight: 700;
		color: #ffffff;
		margin: 0;
	}

	.live-badge {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 12px;
		font-weight: 700;
		color: #ff6b35;
		text-transform: uppercase;
		letter-spacing: 1px;
	}

	.live-dot {
		width: 6px;
		height: 6px;
		border-radius: 50%;
		background: #ff6b35;
		animation: pulse 2s infinite;
	}

	.signals-list {
		display: flex;
		flex-direction: column;
		gap: 12px;
		overflow-y: auto;
		flex: 1;
		padding-right: 8px;
	}

	.signals-list::-webkit-scrollbar {
		width: 6px;
	}

	.signals-list::-webkit-scrollbar-track {
		background: rgba(255, 255, 255, 0.05);
		border-radius: 3px;
	}

	.signals-list::-webkit-scrollbar-thumb {
		background: rgba(255, 255, 255, 0.2);
		border-radius: 3px;
	}

	.signal-card {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		padding: 16px;
		transition: all 0.3s ease;
	}

	.signal-card:hover {
		background: rgba(255, 255, 255, 0.05);
		border-color: rgba(255, 255, 255, 0.2);
		transform: translateX(4px);
	}

	.signal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 12px;
	}

	.ticker-section {
		display: flex;
		align-items: center;
		gap: 10px;
	}

	.grade-badge {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 16px;
		font-weight: 700;
		color: #000;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
	}

	.ticker {
		font-size: 20px;
		font-weight: 700;
		color: #ffffff;
	}

	.grade-emoji {
		font-size: 18px;
	}

	.direction-badge {
		padding: 6px 12px;
		font-size: 12px;
		font-weight: 700;
		color: #ffffff;
		border-radius: 6px;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.signal-details {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.score {
		display: flex;
		align-items: baseline;
		gap: 6px;
	}

	.score-label {
		font-size: 12px;
		color: rgba(255, 255, 255, 0.6);
	}

	.score-value {
		font-size: 16px;
		font-weight: 700;
		color: #f7931e;
	}

	.timestamp {
		font-size: 12px;
		color: rgba(255, 255, 255, 0.5);
		font-variant-numeric: tabular-nums;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 60px 20px;
		gap: 12px;
	}

	.empty-icon {
		font-size: 48px;
		opacity: 0.5;
	}

	.empty-state p {
		color: rgba(255, 255, 255, 0.5);
		font-size: 14px;
		margin: 0;
	}
</style>
