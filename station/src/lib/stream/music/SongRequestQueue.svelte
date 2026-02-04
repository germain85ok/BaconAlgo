<script lang="ts">
	import { onMount } from 'svelte';

	interface SongRequest {
		id: string;
		title: string;
		artist: string;
		requester: string;
		isPlaying: boolean;
	}

	let songRequests: SongRequest[] = [
		{
			id: '1',
			title: 'Bulls on Parade',
			artist: 'Rage Against the Machine',
			requester: 'TraderJoe',
			isPlaying: true
		},
		{
			id: '2',
			title: 'Money',
			artist: 'Pink Floyd',
			requester: 'CryptoKing',
			isPlaying: false
		},
		{
			id: '3',
			title: 'Can\'t Stop',
			artist: 'Red Hot Chili Peppers',
			requester: 'DiamondHands',
			isPlaying: false
		},
		{
			id: '4',
			title: 'Uptown Funk',
			artist: 'Bruno Mars',
			requester: 'MoonBoy',
			isPlaying: false
		},
		{
			id: '5',
			title: 'Eye of the Tiger',
			artist: 'Survivor',
			requester: 'BullRunner',
			isPlaying: false
		}
	];

	let currentSongIndex = 0;

	onMount(() => {
		const interval = setInterval(() => {
			currentSongIndex = (currentSongIndex + 1) % songRequests.length;
			songRequests = songRequests.map((song, i) => ({
				...song,
				isPlaying: i === currentSongIndex
			}));
		}, 30000);

		return () => clearInterval(interval);
	});
</script>

<div class="song-queue">
	<div class="header">
		<h3>
			<svg
				width="16"
				height="16"
				viewBox="0 0 16 16"
				fill="none"
				xmlns="http://www.w3.org/2000/svg"
			>
				<circle cx="4" cy="12" r="2" fill="currentColor" />
				<circle cx="12" cy="10" r="2" fill="currentColor" />
				<path d="M6 12V4L14 2V10" stroke="currentColor" stroke-width="2" stroke-linecap="round" />
			</svg>
			Song Queue
		</h3>
		<span class="queue-count">{songRequests.length} songs</span>
	</div>

	<div class="queue-list">
		{#each songRequests as song, index (song.id)}
			<div class="song-item" class:playing={song.isPlaying}>
				<div class="song-number">
					{#if song.isPlaying}
						<div class="equalizer">
							<span></span>
							<span></span>
							<span></span>
						</div>
					{:else}
						{index + 1}
					{/if}
				</div>
				<div class="song-info">
					<div class="song-title">{song.title}</div>
					<div class="song-meta">
						<span class="artist">{song.artist}</span>
						<span class="separator">•</span>
						<span class="requester">Req: {song.requester}</span>
					</div>
				</div>
			</div>
		{/each}
	</div>

	<div class="footer">
		<button class="request-btn">
			<svg
				width="16"
				height="16"
				viewBox="0 0 16 16"
				fill="none"
				xmlns="http://www.w3.org/2000/svg"
			>
				<path d="M8 3V13M3 8H13" stroke="currentColor" stroke-width="2" stroke-linecap="round" />
			</svg>
			Request a Song
		</button>
	</div>
</div>

<style>
	.song-queue {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		padding: 1.5rem;
		min-width: 350px;
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

	.queue-count {
		font-size: 0.75rem;
		color: rgba(255, 255, 255, 0.6);
		background: rgba(255, 255, 255, 0.05);
		padding: 0.25rem 0.5rem;
		border-radius: 12px;
	}

	.queue-list {
		flex: 1;
		overflow-y: auto;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.queue-list::-webkit-scrollbar {
		width: 6px;
	}

	.queue-list::-webkit-scrollbar-track {
		background: rgba(255, 255, 255, 0.05);
		border-radius: 3px;
	}

	.queue-list::-webkit-scrollbar-thumb {
		background: rgba(249, 115, 22, 0.5);
		border-radius: 3px;
	}

	.song-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		background: rgba(255, 255, 255, 0.03);
		border-radius: 6px;
		padding: 0.75rem;
		transition: all 0.3s ease;
	}

	.song-item:hover {
		background: rgba(255, 255, 255, 0.08);
		transform: translateX(4px);
	}

	.song-item.playing {
		background: rgba(249, 115, 22, 0.1);
		border: 1px solid rgba(249, 115, 22, 0.3);
	}

	.song-number {
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 0.875rem;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.6);
		flex-shrink: 0;
	}

	.equalizer {
		display: flex;
		align-items: flex-end;
		gap: 2px;
		height: 16px;
	}

	.equalizer span {
		width: 3px;
		background: #f97316;
		border-radius: 2px;
		animation: bounce 0.8s ease-in-out infinite;
	}

	.equalizer span:nth-child(1) {
		animation-delay: 0s;
	}

	.equalizer span:nth-child(2) {
		animation-delay: 0.2s;
	}

	.equalizer span:nth-child(3) {
		animation-delay: 0.4s;
	}

	@keyframes bounce {
		0%,
		100% {
			height: 4px;
		}
		50% {
			height: 12px;
		}
	}

	.song-info {
		flex: 1;
		min-width: 0;
	}

	.song-title {
		font-size: 0.875rem;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.9);
		margin-bottom: 0.25rem;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.song-meta {
		font-size: 0.75rem;
		color: rgba(255, 255, 255, 0.6);
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.separator {
		opacity: 0.5;
	}

	.footer {
		margin-top: 1rem;
		padding-top: 1rem;
		border-top: 1px solid rgba(255, 255, 255, 0.1);
	}

	.request-btn {
		width: 100%;
		background: linear-gradient(135deg, #f97316, #ff6b35);
		border: none;
		border-radius: 8px;
		padding: 0.75rem;
		color: white;
		font-size: 0.875rem;
		font-weight: 600;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		transition: all 0.3s ease;
	}

	.request-btn:hover {
		transform: translateY(-2px);
		box-shadow: 0 8px 20px rgba(249, 115, 22, 0.4);
	}
</style>
