<script lang="ts">
	import { YOUTUBE_URL, DISCORD_URL } from '$lib/config/env';

	const socialLinks = [
		{
			name: 'YouTube',
			url: YOUTUBE_URL,
			icon: '📺',
			color: '#FF0000'
		},
		{
			name: 'Discord',
			url: DISCORD_URL,
			icon: '💬',
			color: '#5865F2'
		}
	];

	function openLink(url: string) {
		if (typeof window !== 'undefined') {
			window.open(url, '_blank');
		}
	}
</script>

<div class="social-links">
	{#each socialLinks as link}
		<button
			class="social-button glass-button"
			on:click={() => openLink(link.url)}
			style="--link-color: {link.color}"
			title="Visit our {link.name}"
		>
			<span class="icon">{link.icon}</span>
			<span class="label">{link.name}</span>
		</button>
	{/each}
</div>

<style>
	.social-links {
		display: flex;
		gap: 16px;
		justify-content: center;
		align-items: center;
	}

	.social-button {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 12px 24px;
		color: #ffffff;
		font-weight: 600;
		font-size: 16px;
		transition: all 0.3s ease;
		position: relative;
		overflow: hidden;
	}

	.social-button::before {
		content: '';
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background: var(--link-color);
		opacity: 0;
		transition: opacity 0.3s ease;
		z-index: -1;
	}

	.social-button:hover::before {
		opacity: 0.2;
	}

	.social-button:hover {
		border-color: var(--link-color);
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
	}

	.social-button:active {
		transform: translateY(-1px);
	}

	.icon {
		font-size: 24px;
		filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.3));
	}

	.label {
		letter-spacing: 0.5px;
		text-transform: uppercase;
	}

	@media (max-width: 768px) {
		.social-links {
			flex-direction: column;
		}

		.social-button {
			width: 100%;
			justify-content: center;
		}
	}
</style>
