<script lang="ts">
	import favicon from '$lib/assets/favicon.svg';
	import { page } from '$app/stores';

	let { children } = $props();
	
	const navItems = [
		{ path: '/', label: 'Dashboard', icon: '🏠' },
		{ path: '/scanner', label: 'Scanner', icon: '🔍' },
		{ path: '/markets', label: 'Marchés', icon: '📊' },
		{ path: '/orderflow', label: 'Order Flow', icon: '💹' },
		{ path: '/auction', label: 'Auction', icon: '⚖️' },
		{ path: '/quantum', label: 'Quantum', icon: '⚛️' },
		{ path: '/psychology', label: 'Psychologie', icon: '🧠' },
		{ path: '/risk', label: 'Risque', icon: '🛡️' }
	];
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="app">
	<nav class="sidebar">
		<div class="logo">
			<span class="logo-text">🥓 BaconAlgo</span>
		</div>
		<ul class="nav-list">
			{#each navItems as item}
				<li>
					<a 
						href={item.path} 
						class:active={$page.url.pathname === item.path}
					>
						<span class="icon">{item.icon}</span>
						<span class="label">{item.label}</span>
					</a>
				</li>
			{/each}
		</ul>
	</nav>
	
	<main class="content">
		{@render children()}
	</main>
</div>

<style>
	:global(body) {
		margin: 0;
		padding: 0;
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
		background: #0b0e11;
		color: #d1d4dc;
	}
	
	.app {
		display: flex;
		height: 100vh;
	}
	
	.sidebar {
		width: 240px;
		background: rgba(15, 20, 25, 0.8);
		backdrop-filter: blur(10px);
		border-right: 1px solid rgba(255, 107, 53, 0.2);
		padding: 1.5rem 0;
	}
	
	.logo {
		padding: 0 1.5rem 2rem;
		border-bottom: 1px solid rgba(255, 107, 53, 0.2);
		margin-bottom: 1rem;
	}
	
	.logo-text {
		font-size: 1.5rem;
		font-weight: 700;
		color: #ff6b35;
		text-shadow: 0 0 20px rgba(255, 107, 53, 0.5);
	}
	
	.nav-list {
		list-style: none;
		padding: 0;
		margin: 0;
	}
	
	.nav-list li a {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 0.875rem 1.5rem;
		color: #d1d4dc;
		text-decoration: none;
		transition: all 0.2s;
		border-left: 3px solid transparent;
	}
	
	.nav-list li a:hover {
		background: rgba(255, 107, 53, 0.1);
		color: #ff6b35;
		border-left-color: #ff6b35;
	}
	
	.nav-list li a.active {
		background: rgba(255, 107, 53, 0.15);
		color: #ff6b35;
		border-left-color: #ff6b35;
		font-weight: 600;
	}
	
	.icon {
		font-size: 1.25rem;
	}
	
	.content {
		flex: 1;
		overflow-y: auto;
		padding: 2rem;
	}
</style>
