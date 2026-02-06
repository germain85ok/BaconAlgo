<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { getSupabaseClient } from '$lib/supabase/client';
	import type { Profile } from '$lib/supabase/types';

	interface Props {
		data: {
			session: any;
			profile: Profile;
		};
	}

	let { data }: Props = $props();
	let { profile } = data;

	const supabase = getSupabaseClient();

	let sidebarOpen = $state(false);

	const toggleSidebar = () => {
		sidebarOpen = !sidebarOpen;
	};

	const handleLogout = async () => {
		await supabase.auth.signOut();
		goto('/login');
	};

	const planBadgeColor = $derived.by(() => {
		switch (profile.plan) {
			case 'station':
				return 'from-purple-500 to-pink-500';
			case 'scanner':
				return 'from-orange-500 to-yellow-500';
			case 'indicator':
				return 'from-blue-500 to-cyan-500';
			default:
				return 'from-gray-500 to-gray-600';
		}
	});

	const planLabel = $derived.by(() => {
		switch (profile.plan) {
			case 'station':
				return '🚀 Station';
			case 'scanner':
				return '🔍 Scanner';
			case 'indicator':
				return '📊 Indicator';
			default:
				return '🥓 Free';
		}
	});

	const canAccessRoute = (route: string): boolean => {
		if (route === 'free') return true;
		if (route === 'indicator') return ['indicator', 'scanner', 'station'].includes(profile.plan);
		if (route === 'scanner') return ['scanner', 'station'].includes(profile.plan);
		if (route === 'station') return profile.plan === 'station';
		return false;
	};

	const navItems = [
		{ path: '/dashboard', label: '🏠 Dashboard', plan: 'free' },
		{ path: '/dashboard/markets', label: '🌍 Markets', plan: 'free' },
		{ path: '/dashboard/scanner', label: '🔍 Scanner', plan: 'scanner' },
		{ path: '/dashboard/risk', label: '⚠️ Risk', plan: 'scanner' },
		{ path: '/dashboard/orderflow', label: '📊 Order Flow', plan: 'station' },
		{ path: '/dashboard/auto-trade', label: '🤖 Auto-Trade', plan: 'station' },
		{ path: '/dashboard/my-brokers', label: '🏦 Brokers', plan: 'station' },
		{ path: '/dashboard/station', label: '🚀 Station', plan: 'station' }
	];
</script>

<div class="dashboard-container">
	<!-- Mobile Header -->
	<header class="mobile-header">
		<button class="hamburger" onclick={toggleSidebar} aria-label="Toggle menu">
			<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<path d="M3 12h18M3 6h18M3 18h18" />
			</svg>
		</button>
		<div class="mobile-logo">BaconAlgo</div>
		<div class="mobile-avatar">
			{#if profile.avatar_url}
				<img src={profile.avatar_url} alt={profile.username || 'User'} />
			{:else}
				<div class="avatar-placeholder">{(profile.username || 'U')[0].toUpperCase()}</div>
			{/if}
		</div>
	</header>

	<!-- Sidebar -->
	<aside class="sidebar" class:open={sidebarOpen}>
		<!-- Logo -->
		<div class="sidebar-logo">
			<div class="logo-icon">🥓</div>
			<div class="logo-text">
				<span class="logo-main">BaconAlgo</span>
				<span class="logo-sub">Trading Station</span>
			</div>
		</div>

		<!-- User Profile -->
		<div class="user-profile">
			<div class="avatar">
				{#if profile.avatar_url}
					<img src={profile.avatar_url} alt={profile.username || 'User'} />
				{:else}
					<div class="avatar-placeholder">{(profile.username || 'U')[0].toUpperCase()}</div>
				{/if}
			</div>
			<div class="user-info">
				<div class="username">{profile.username || 'User'}</div>
				<div class="plan-badge" style="background: linear-gradient(135deg, {planBadgeColor});">
					{planLabel}
				</div>
			</div>
		</div>

		<!-- Bacon Points & Streak -->
		<div class="stats-section">
			<div class="stat-item">
				<div class="stat-icon">🥓</div>
				<div class="stat-value">{profile.bacon_points}</div>
				<div class="stat-label">Bacon Points</div>
			</div>
			<div class="stat-item">
				<div class="stat-icon">🔥</div>
				<div class="stat-value">{profile.streak_days}</div>
				<div class="stat-label">Day Streak</div>
			</div>
		</div>

		<!-- Navigation -->
		<nav class="sidebar-nav">
			{#each navItems as item}
				{#if canAccessRoute(item.plan)}
					<a
						href={item.path}
						class="nav-item"
						class:active={$page.url.pathname === item.path}
						onclick={() => (sidebarOpen = false)}
					>
						{item.label}
					</a>
				{:else}
					<div class="nav-item locked" title="Upgrade to unlock">
						{item.label}
						<svg
							width="16"
							height="16"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
						>
							<rect x="3" y="11" width="18" height="11" rx="2" ry="2" />
							<path d="M7 11V7a5 5 0 0 1 10 0v4" />
						</svg>
					</div>
				{/if}
			{/each}
		</nav>

		<!-- Upgrade CTA (if not on highest plan) -->
		{#if profile.plan !== 'station'}
			<div class="upgrade-cta">
				<div class="upgrade-icon">🚀</div>
				<h4>Upgrade Now!</h4>
				<p>Unlock all features with Station plan</p>
				<a href="/pricing" class="upgrade-btn">View Plans</a>
			</div>
		{/if}

		<!-- Logout -->
		<button class="logout-btn" onclick={handleLogout}>
			<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" />
				<polyline points="16 17 21 12 16 7" />
				<line x1="21" y1="12" x2="9" y2="12" />
			</svg>
			Logout
		</button>
	</aside>

	<!-- Overlay (mobile) -->
	{#if sidebarOpen}
		<div class="overlay" onclick={toggleSidebar}></div>
	{/if}

	<!-- Main Content -->
	<main class="main-content">
		<slot />
	</main>
</div>

<style>
	.dashboard-container {
		display: flex;
		min-height: 100vh;
		background: linear-gradient(135deg, #0a0a0f 0%, #1a1f3a 100%);
	}

	/* Mobile Header */
	.mobile-header {
		display: none;
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		height: 60px;
		background: rgba(17, 24, 39, 0.95);
		backdrop-filter: blur(12px);
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		padding: 0 1rem;
		align-items: center;
		justify-content: space-between;
		z-index: 100;
	}

	.hamburger {
		background: none;
		border: none;
		color: #f9fafb;
		cursor: pointer;
		padding: 0.5rem;
	}

	.mobile-logo {
		font-size: 1.25rem;
		font-weight: 700;
		background: linear-gradient(135deg, #ff6b35 0%, #fbbf24 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.mobile-avatar {
		width: 36px;
		height: 36px;
		border-radius: 50%;
		overflow: hidden;
	}

	.mobile-avatar img,
	.mobile-avatar .avatar-placeholder {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	/* Sidebar */
	.sidebar {
		width: 280px;
		background: rgba(17, 24, 39, 0.8);
		backdrop-filter: blur(12px);
		border-right: 1px solid rgba(255, 255, 255, 0.1);
		padding: 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
		position: sticky;
		top: 0;
		height: 100vh;
		overflow-y: auto;
	}

	.sidebar::-webkit-scrollbar {
		width: 6px;
	}

	.sidebar::-webkit-scrollbar-track {
		background: rgba(0, 0, 0, 0.2);
	}

	.sidebar::-webkit-scrollbar-thumb {
		background: rgba(255, 107, 53, 0.3);
		border-radius: 3px;
	}

	.sidebar-logo {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding-bottom: 1rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.05);
	}

	.logo-icon {
		font-size: 2rem;
	}

	.logo-text {
		display: flex;
		flex-direction: column;
	}

	.logo-main {
		font-size: 1.25rem;
		font-weight: 700;
		background: linear-gradient(135deg, #ff6b35 0%, #fbbf24 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.logo-sub {
		font-size: 0.75rem;
		color: #9ca3af;
	}

	.user-profile {
		display: flex;
		gap: 0.875rem;
		padding: 1rem;
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.05);
		border-radius: 0.75rem;
	}

	.avatar {
		width: 48px;
		height: 48px;
		border-radius: 50%;
		overflow: hidden;
		flex-shrink: 0;
	}

	.avatar img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.avatar-placeholder {
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #ff6b35 0%, #fbbf24 100%);
		color: #1f2937;
		font-weight: 700;
		font-size: 1.25rem;
	}

	.user-info {
		display: flex;
		flex-direction: column;
		justify-content: center;
		gap: 0.375rem;
		min-width: 0;
	}

	.username {
		font-size: 1rem;
		font-weight: 600;
		color: #f9fafb;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.plan-badge {
		padding: 0.25rem 0.625rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 700;
		color: #fff;
		text-align: center;
		width: fit-content;
	}

	.stats-section {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 0.75rem;
	}

	.stat-item {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 0.875rem;
		background: rgba(255, 107, 53, 0.05);
		border: 1px solid rgba(255, 107, 53, 0.2);
		border-radius: 0.75rem;
	}

	.stat-icon {
		font-size: 1.5rem;
		margin-bottom: 0.25rem;
	}

	.stat-value {
		font-size: 1.25rem;
		font-weight: 700;
		color: #fbbf24;
	}

	.stat-label {
		font-size: 0.625rem;
		color: #9ca3af;
		text-align: center;
	}

	.sidebar-nav {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		flex: 1;
	}

	.nav-item {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.875rem 1rem;
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.05);
		border-radius: 0.5rem;
		color: #e5e7eb;
		font-size: 0.875rem;
		font-weight: 600;
		text-decoration: none;
		transition: all 0.2s ease;
		cursor: pointer;
	}

	.nav-item:hover:not(.locked) {
		background: rgba(255, 107, 53, 0.1);
		border-color: rgba(255, 107, 53, 0.3);
		color: #ff6b35;
		transform: translateX(4px);
	}

	.nav-item.active {
		background: linear-gradient(135deg, rgba(255, 107, 53, 0.2) 0%, rgba(251, 191, 36, 0.2) 100%);
		border-color: rgba(255, 107, 53, 0.5);
		color: #fbbf24;
	}

	.nav-item.locked {
		opacity: 0.4;
		cursor: not-allowed;
	}

	.upgrade-cta {
		padding: 1.25rem;
		background: linear-gradient(135deg, rgba(168, 85, 247, 0.1) 0%, rgba(236, 72, 153, 0.1) 100%);
		border: 1px solid rgba(168, 85, 247, 0.3);
		border-radius: 0.75rem;
		text-align: center;
	}

	.upgrade-icon {
		font-size: 2rem;
		margin-bottom: 0.5rem;
	}

	.upgrade-cta h4 {
		margin: 0 0 0.375rem 0;
		font-size: 1rem;
		font-weight: 700;
		color: #f9fafb;
	}

	.upgrade-cta p {
		margin: 0 0 0.875rem 0;
		font-size: 0.75rem;
		color: #9ca3af;
	}

	.upgrade-btn {
		display: inline-block;
		padding: 0.625rem 1.25rem;
		background: linear-gradient(135deg, #a855f7 0%, #ec4899 100%);
		border-radius: 0.5rem;
		color: #fff;
		font-size: 0.875rem;
		font-weight: 700;
		text-decoration: none;
		transition: all 0.3s ease;
	}

	.upgrade-btn:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 25px rgba(168, 85, 247, 0.4);
	}

	.logout-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.875rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 0.5rem;
		color: #ef4444;
		font-size: 0.875rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.logout-btn:hover {
		background: rgba(239, 68, 68, 0.2);
		border-color: rgba(239, 68, 68, 0.5);
	}

	.overlay {
		display: none;
	}

	.main-content {
		flex: 1;
		padding: 2rem;
		overflow-y: auto;
	}

	/* Responsive */
	@media (max-width: 1024px) {
		.mobile-header {
			display: flex;
		}

		.sidebar {
			position: fixed;
			top: 0;
			left: -280px;
			height: 100vh;
			z-index: 200;
			transition: left 0.3s ease;
		}

		.sidebar.open {
			left: 0;
		}

		.overlay {
			display: block;
			position: fixed;
			top: 0;
			left: 0;
			right: 0;
			bottom: 0;
			background: rgba(0, 0, 0, 0.6);
			z-index: 150;
		}

		.main-content {
			padding-top: 80px;
		}
	}

	@media (max-width: 640px) {
		.main-content {
			padding: 1rem;
			padding-top: 70px;
		}

		.stats-section {
			grid-template-columns: 1fr;
		}
	}
</style>
