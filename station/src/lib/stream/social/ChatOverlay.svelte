<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { fly } from 'svelte/transition';

	interface ChatMessage {
		id: string;
		user: string;
		avatar: string;
		message: string;
		timestamp: Date;
	}

	let messages: ChatMessage[] = [
		{
			id: '1',
			user: 'TraderPro',
			avatar: '👨‍💼',
			message: 'Great analysis on that setup!',
			timestamp: new Date()
		},
		{
			id: '2',
			user: 'CryptoKing',
			avatar: '👑',
			message: 'What do you think about BTC?',
			timestamp: new Date()
		},
		{
			id: '3',
			user: 'BaconFan42',
			avatar: '🥓',
			message: 'Love the stream!',
			timestamp: new Date()
		}
	];

	let messagesContainer: HTMLDivElement;
	let interval: ReturnType<typeof setInterval>;

	const mockMessages = [
		'Thanks for the rocket signals!',
		'When is the next trade?',
		'This strategy is amazing',
		'Can you explain the fear & greed index?',
		'Best trading stream ever!',
		'VIX is spiking!',
		'Just donated $5!',
		'Following your trades',
		'Great content as always',
		'Any thoughts on ETH?'
	];

	const mockUsers = [
		{ name: 'Trader123', avatar: '👨' },
		{ name: 'MoonBoy', avatar: '🌙' },
		{ name: 'DiamondHands', avatar: '💎' },
		{ name: 'BullRunner', avatar: '🐂' },
		{ name: 'ChartMaster', avatar: '📊' },
		{ name: 'HodlKing', avatar: '👑' },
		{ name: 'TrendSeeker', avatar: '🔍' },
		{ name: 'AlgoTrader', avatar: '🤖' }
	];

	function addMockMessage() {
		const user = mockUsers[Math.floor(Math.random() * mockUsers.length)];
		const message = mockMessages[Math.floor(Math.random() * mockMessages.length)];

		const newMessage: ChatMessage = {
			id: Date.now().toString(),
			user: user.name,
			avatar: user.avatar,
			message,
			timestamp: new Date()
		};

		messages = [...messages, newMessage];

		if (messages.length > 20) {
			messages = messages.slice(-20);
		}

		setTimeout(() => {
			if (messagesContainer) {
				messagesContainer.scrollTop = messagesContainer.scrollHeight;
			}
		}, 100);
	}

	onMount(() => {
		interval = setInterval(addMockMessage, 8000);
	});

	onDestroy(() => {
		if (interval) clearInterval(interval);
	});
</script>

<div class="chat-overlay">
	<div class="chat-header">
		<h3>
			<svg
				width="16"
				height="16"
				viewBox="0 0 16 16"
				fill="none"
				xmlns="http://www.w3.org/2000/svg"
			>
				<path
					d="M14 2H2C1.44772 2 1 2.44772 1 3V11C1 11.5523 1.44772 12 2 12H5L8 15L11 12H14C14.5523 12 15 11.5523 15 11V3C15 2.44772 14.5523 2 14 2Z"
					fill="currentColor"
				/>
			</svg>
			Live Chat
		</h3>
		<span class="viewer-count">{messages.length} viewers</span>
	</div>

	<div class="messages-container" bind:this={messagesContainer}>
		{#each messages as message (message.id)}
			<div class="message" in:fly={{ x: -20, duration: 300 }}>
				<div class="message-avatar">{message.avatar}</div>
				<div class="message-content">
					<div class="message-header">
						<span class="username">{message.user}</span>
						<span class="timestamp">
							{message.timestamp.toLocaleTimeString('en-US', {
								hour: '2-digit',
								minute: '2-digit'
							})}
						</span>
					</div>
					<div class="message-text">{message.message}</div>
				</div>
			</div>
		{/each}
	</div>
</div>

<style>
	.chat-overlay {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		padding: 1rem;
		min-width: 320px;
		max-height: 500px;
		display: flex;
		flex-direction: column;
	}

	.chat-header {
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

	.viewer-count {
		font-size: 0.75rem;
		color: rgba(255, 255, 255, 0.6);
		background: rgba(255, 255, 255, 0.05);
		padding: 0.25rem 0.5rem;
		border-radius: 12px;
	}

	.messages-container {
		flex: 1;
		overflow-y: auto;
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.messages-container::-webkit-scrollbar {
		width: 6px;
	}

	.messages-container::-webkit-scrollbar-track {
		background: rgba(255, 255, 255, 0.05);
		border-radius: 3px;
	}

	.messages-container::-webkit-scrollbar-thumb {
		background: rgba(249, 115, 22, 0.5);
		border-radius: 3px;
	}

	.message {
		display: flex;
		gap: 0.75rem;
		padding: 0.5rem;
		border-radius: 6px;
		transition: all 0.3s ease;
	}

	.message:hover {
		background: rgba(255, 255, 255, 0.05);
	}

	.message-avatar {
		width: 32px;
		height: 32px;
		border-radius: 50%;
		background: linear-gradient(135deg, #f97316, #ff6b35);
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 1.25rem;
		flex-shrink: 0;
	}

	.message-content {
		flex: 1;
		min-width: 0;
	}

	.message-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.25rem;
	}

	.username {
		font-size: 0.875rem;
		font-weight: 600;
		color: #f97316;
	}

	.timestamp {
		font-size: 0.75rem;
		color: rgba(255, 255, 255, 0.5);
	}

	.message-text {
		font-size: 0.875rem;
		color: rgba(255, 255, 255, 0.85);
		line-height: 1.4;
		word-wrap: break-word;
	}
</style>
