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
	let notificationsOpen = $state(false);
	let userMenuOpen = $state(false);

	const toggleSidebar = () => {
		sidebarOpen = !sidebarOpen;
	};

	const handleLogout = async () => {
		await supabase.auth.signOut();
		goto('/login');
	};

	const adminNavItems = [
		{ path: '/admin', label: '📊 Dashboard', icon: '📊' },
		{ path: '/admin/users', label: '👥 Users', icon: '👥' },
		{ path: '/admin/coupons', label: '🎟️ Coupons', icon: '🎟️' },
		{ path: '/admin/subscriptions', label: '💳 Subscriptions', icon: '💳' },
		{ path: '/admin/promos', label: '🎁 Promo Codes', icon: '🎁' },
		{ path: '/admin/discord', label: '💬 Discord Alerts', icon: '💬' },
		{ path: '/admin/signals', label: '📡 Manual Signals', icon: '📡' },
		{ path: '/admin/analytics', label: '📈 Analytics', icon: '📈' }
	];

	const isActiveRoute = (path: string): boolean => {
		if (path === '/admin') {
			return $page.url.pathname === '/admin';
		}
		return $page.url.pathname.startsWith(path);
	};
</script>

<div class="admin-container">
	<!-- Mobile Header -->
	<header class="admin-mobile-header">
		<button class="hamburger" onclick={toggleSidebar} aria-label="Toggle menu">
			<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<path d="M3 12h18M3 6h18M3 18h18" />
			</svg>
		</button>
		<div class="mobile-logo">🔐 Admin Panel</div>
	</header>

	<!-- Sidebar -->
	<aside class="admin-sidebar" class:open={sidebarOpen}>
		<!-- Logo -->
		<div class="sidebar-logo">
			<div class="logo-icon">🔐</div>
			<div class="logo-text">
				<span class="logo-main">Admin Panel</span>
				<span class="logo-sub">BaconAlgo Control</span>
			</div>
		</div>

		<!-- Navigation -->
		<nav class="sidebar-nav">
			{#each adminNavItems as item}
				<a
					href={item.path}
					class="nav-item"
					class:active={isActiveRoute(item.path)}
					onclick={() => (sidebarOpen = false)}
				>
					<span class="nav-icon">{item.icon}</span>
					<span class="nav-label">{item.label}</span>
				</a>
			{/each}
		</nav>

		<!-- Back to Dashboard -->
		<div class="sidebar-footer">
			<a href="/dashboard" class="back-link">
				<span>←</span>
				<span>Back to Dashboard</span>
			</a>
		</div>
	</aside>

	<!-- Overlay for mobile -->
	{#if sidebarOpen}
		<div class="sidebar-overlay" onclick={toggleSidebar}></div>
	{/if}

	<!-- Main Content -->
	<main class="admin-main">
		<!-- Top Bar -->
		<div class="top-bar">
			<div class="top-bar-left">
				<h1 class="page-title">
					{#if $page.url.pathname === '/admin'}
						Dashboard Overview
					{:else if $page.url.pathname.includes('/users')}
						User Management
					{:else if $page.url.pathname.includes('/promos')}
						Promo Codes
					{:else if $page.url.pathname.includes('/discord')}
						Discord Alerts
					{:else if $page.url.pathname.includes('/signals')}
						Manual Signals
					{:else if $page.url.pathname.includes('/analytics')}
						Analytics
					{/if}
				</h1>
			</div>
			<div class="top-bar-right">
				<!-- Admin Badge -->
				<div class="admin-badge">
					<span class="badge-icon">👑</span>
					<span class="badge-text">Admin</span>
				</div>

				<!-- Notifications -->
				<button class="icon-btn" onclick={() => (notificationsOpen = !notificationsOpen)}>
					<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9" />
						<path d="M13.73 21a2 2 0 0 1-3.46 0" />
					</svg>
					<span class="notification-dot"></span>
				</button>

				<!-- User Menu -->
				<div class="user-menu-container">
					<button class="user-menu-btn" onclick={() => (userMenuOpen = !userMenuOpen)}>
						{#if profile.avatar_url}
							<img src={profile.avatar_url} alt={profile.username || 'Admin'} class="user-avatar" />
						{:else}
							<div class="avatar-placeholder">{(profile.username || 'A')[0].toUpperCase()}</div>
						{/if}
						<span class="username">{profile.username || profile.email}</span>
					</button>

					{#if userMenuOpen}
						<div class="user-menu-dropdown">
							<a href="/dashboard" class="menu-item">
								<span>🏠</span>
								<span>Dashboard</span>
							</a>
							<button onclick={handleLogout} class="menu-item">
								<span>🚪</span>
								<span>Logout</span>
							</button>
						</div>
					{/if}
				</div>
			</div>
		</div>

		<!-- Page Content -->
		<div class="admin-content">
			<slot />
		</div>
	</main>
</div>

<style>
	.admin-container {
		display: flex;
		min-height: 100vh;
		background: linear-gradient(135deg, #0a0a0f 0%, #1a1a2e 100%);
		color: #fff;
	}

	/* Mobile Header */
	.admin-mobile-header {
		display: none;
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		height: 60px;
		background: rgba(20, 20, 30, 0.95);
		backdrop-filter: blur(10px);
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		padding: 0 1rem;
		align-items: center;
		gap: 1rem;
		z-index: 100;
	}

	.hamburger {
		background: none;
		border: none;
		color: #fff;
		cursor: pointer;
		padding: 0.5rem;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.mobile-logo {
		font-size: 1.125rem;
		font-weight: 700;
	}

	/* Sidebar */
	.admin-sidebar {
		position: fixed;
		left: 0;
		top: 0;
		width: 280px;
		height: 100vh;
		background: rgba(20, 20, 30, 0.8);
		backdrop-filter: blur(20px);
		border-right: 1px solid rgba(255, 255, 255, 0.1);
		display: flex;
		flex-direction: column;
		z-index: 101;
		transition: transform 0.3s ease;
	}

	.sidebar-logo {
		padding: 2rem 1.5rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		display: flex;
		align-items: center;
		gap: 1rem;
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
		background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.logo-sub {
		font-size: 0.75rem;
		color: rgba(255, 255, 255, 0.6);
	}

	.sidebar-nav {
		flex: 1;
		padding: 1.5rem 1rem;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		overflow-y: auto;
	}

	.nav-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.875rem 1rem;
		border-radius: 12px;
		color: rgba(255, 255, 255, 0.7);
		text-decoration: none;
		transition: all 0.2s ease;
		cursor: pointer;
	}

	.nav-item:hover {
		background: rgba(255, 255, 255, 0.05);
		color: #fff;
	}

	.nav-item.active {
		background: linear-gradient(135deg, rgba(240, 147, 251, 0.2) 0%, rgba(245, 87, 108, 0.2) 100%);
		color: #fff;
		border: 1px solid rgba(240, 147, 251, 0.3);
	}

	.nav-icon {
		font-size: 1.25rem;
	}

	.nav-label {
		font-size: 0.9375rem;
		font-weight: 500;
	}

	.sidebar-footer {
		padding: 1.5rem;
		border-top: 1px solid rgba(255, 255, 255, 0.1);
	}

	.back-link {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		border-radius: 12px;
		background: rgba(255, 255, 255, 0.05);
		color: rgba(255, 255, 255, 0.7);
		text-decoration: none;
		font-size: 0.875rem;
		transition: all 0.2s ease;
	}

	.back-link:hover {
		background: rgba(255, 255, 255, 0.1);
		color: #fff;
	}

	/* Main Content */
	.admin-main {
		flex: 1;
		margin-left: 280px;
		display: flex;
		flex-direction: column;
	}

	.top-bar {
		position: sticky;
		top: 0;
		height: 70px;
		background: rgba(20, 20, 30, 0.8);
		backdrop-filter: blur(20px);
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0 2rem;
		z-index: 50;
	}

	.page-title {
		font-size: 1.5rem;
		font-weight: 700;
		margin: 0;
	}

	.top-bar-right {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.admin-badge {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		border-radius: 20px;
		background: linear-gradient(135deg, #ffd700 0%, #ffed4e 100%);
		color: #000;
		font-weight: 600;
		font-size: 0.875rem;
	}

	.badge-icon {
		font-size: 1rem;
	}

	.icon-btn {
		position: relative;
		background: rgba(255, 255, 255, 0.1);
		border: none;
		border-radius: 10px;
		padding: 0.625rem;
		color: #fff;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		transition: all 0.2s ease;
	}

	.icon-btn:hover {
		background: rgba(255, 255, 255, 0.15);
	}

	.notification-dot {
		position: absolute;
		top: 8px;
		right: 8px;
		width: 8px;
		height: 8px;
		background: #f5576c;
		border-radius: 50%;
		border: 2px solid rgba(20, 20, 30, 0.8);
	}

	.user-menu-container {
		position: relative;
	}

	.user-menu-btn {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.5rem 1rem;
		border-radius: 12px;
		background: rgba(255, 255, 255, 0.1);
		border: none;
		color: #fff;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.user-menu-btn:hover {
		background: rgba(255, 255, 255, 0.15);
	}

	.user-avatar,
	.avatar-placeholder {
		width: 32px;
		height: 32px;
		border-radius: 50%;
		object-fit: cover;
	}

	.avatar-placeholder {
		background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
		display: flex;
		align-items: center;
		justify-content: center;
		font-weight: 600;
	}

	.username {
		font-size: 0.875rem;
		font-weight: 500;
	}

	.user-menu-dropdown {
		position: absolute;
		top: calc(100% + 0.5rem);
		right: 0;
		min-width: 200px;
		background: rgba(30, 30, 40, 0.95);
		backdrop-filter: blur(20px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		padding: 0.5rem;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
		box-shadow: 0 10px 40px rgba(0, 0, 0, 0.5);
	}

	.menu-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		border-radius: 8px;
		background: none;
		border: none;
		color: #fff;
		text-decoration: none;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s ease;
		width: 100%;
		text-align: left;
	}

	.menu-item:hover {
		background: rgba(255, 255, 255, 0.1);
	}

	.admin-content {
		flex: 1;
		padding: 2rem;
		overflow-y: auto;
	}

	.sidebar-overlay {
		display: none;
	}

	/* Mobile Styles */
	@media (max-width: 768px) {
		.admin-mobile-header {
			display: flex;
		}

		.admin-sidebar {
			transform: translateX(-100%);
		}

		.admin-sidebar.open {
			transform: translateX(0);
		}

		.sidebar-overlay {
			display: block;
			position: fixed;
			inset: 0;
			background: rgba(0, 0, 0, 0.5);
			z-index: 100;
		}

		.admin-main {
			margin-left: 0;
			padding-top: 60px;
		}

		.top-bar {
			padding: 0 1rem;
		}

		.page-title {
			font-size: 1.25rem;
		}

		.username {
			display: none;
		}

		.admin-content {
			padding: 1rem;
		}
	}
</style>
