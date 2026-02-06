<script lang="ts">
	import { t } from '$lib/i18n/i18n';
	import LanguageToggle from './LanguageToggle.svelte';

	interface Props {
		isLoggedIn?: boolean;
		user?: { name: string; email: string; is_admin?: boolean } | null;
		onLogin?: () => void;
		onRegister?: () => void;
		onLogout?: () => void;
	}

	let {
		isLoggedIn = false,
		user = null,
		onLogin = () => {},
		onRegister = () => {},
		onLogout = () => {}
	}: Props = $props();

	let mobileMenuOpen = $state(false);
	let userMenuOpen = $state(false);

	function toggleMobileMenu() {
		mobileMenuOpen = !mobileMenuOpen;
	}

	function toggleUserMenu() {
		userMenuOpen = !userMenuOpen;
	}

	function closeMobileMenu() {
		mobileMenuOpen = false;
	}

	// Check if user is admin (germain85@hotmail.com)
	let isAdmin = $derived(user?.email === 'germain85@hotmail.com' || user?.is_admin === true);
</script>

<nav class="sticky top-0 z-40 bg-bg-dark/80 backdrop-blur-md border-b border-white/10">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
		<div class="flex items-center justify-between h-16">
			<!-- Logo -->
			<div class="flex-shrink-0">
				<a href="/" class="flex items-center space-x-2">
					<span
						class="text-2xl font-display font-bold bg-gradient-to-r from-bacon-orange to-bacon-red bg-clip-text text-transparent"
					>
						BaconAlgo
					</span>
				</a>
			</div>

			<!-- Desktop Navigation -->
			<div class="hidden md:flex md:items-center md:space-x-8">
				<a
					href="/"
					class="text-text-secondary hover:text-text-primary transition-colors font-body"
				>
					{$t.nav.home}
				</a>
				<a
					href="/pricing"
					class="text-text-secondary hover:text-text-primary transition-colors font-body"
				>
					{$t.nav.pricing}
				</a>
				<a
					href="/academy"
					class="text-text-secondary hover:text-text-primary transition-colors font-body"
				>
					{$t.nav.academy}
				</a>
				{#if isLoggedIn && user}
					<a
						href="/dashboard"
						class="text-text-secondary hover:text-text-primary transition-colors font-body"
					>
						{$t.nav.dashboard}
					</a>
				{/if}
			</div>

			<!-- Desktop Auth/User Menu -->
			<div class="hidden md:flex md:items-center md:space-x-4">
				<LanguageToggle />
				
				{#if isLoggedIn && user}
					{#if isAdmin}
						<a
							href="/admin"
							class="px-3 py-1.5 rounded-lg bg-gradient-to-r from-bacon-orange to-bacon-red text-white font-semibold text-sm hover:shadow-lg hover:shadow-bacon-orange/30 transition-all"
						>
							{$t.admin.badge}
						</a>
					{/if}
					
					<div class="relative">
						<button
							onclick={toggleUserMenu}
							class="flex items-center space-x-2 text-text-primary hover:text-bacon-orange transition-colors"
						>
							<div
								class="w-8 h-8 rounded-full bg-gradient-to-br from-bacon-orange to-bacon-red flex items-center justify-center text-white font-bold text-sm"
							>
								{user.name.charAt(0).toUpperCase()}
							</div>
							<svg
								class="w-4 h-4 transition-transform {userMenuOpen ? 'rotate-180' : ''}"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M19 9l-7 7-7-7"
								/>
							</svg>
						</button>

						{#if userMenuOpen}
							<div
								class="absolute right-0 mt-2 w-48 bg-bg-dark border border-white/10 rounded-xl shadow-lg py-1 backdrop-blur-md"
							>
								<a
									href="/dashboard"
									class="block px-4 py-2 text-text-secondary hover:bg-white/5 hover:text-text-primary transition-colors"
								>
									{$t.nav.dashboard}
								</a>
								<a
									href="/profile"
									class="block px-4 py-2 text-text-secondary hover:bg-white/5 hover:text-text-primary transition-colors"
								>
									{$t.nav.profile}
								</a>
								{#if isAdmin}
									<a
										href="/admin"
										class="block px-4 py-2 text-text-secondary hover:bg-white/5 hover:text-text-primary transition-colors"
									>
										{$t.nav.admin}
									</a>
								{/if}
								<button
									onclick={onLogout}
									class="block w-full text-left px-4 py-2 text-bacon-red hover:bg-white/5 transition-colors"
								>
									{$t.nav.logout}
								</button>
							</div>
						{/if}
					</div>
				{:else}
					<button
						onclick={onLogin}
						class="px-4 py-2 text-text-primary hover:text-bacon-orange transition-colors font-body"
					>
						{$t.nav.login}
					</button>
					<button
						onclick={onRegister}
						class="px-6 py-2 bg-gradient-to-r from-bacon-orange to-bacon-red text-white rounded-lg hover:shadow-lg hover:shadow-bacon-orange/30 transition-all font-body font-semibold"
					>
						{$t.nav.register}
					</button>
				{/if}
			</div>

			<!-- Mobile menu button -->
			<div class="md:hidden">
				<button
					onclick={toggleMobileMenu}
					class="text-text-secondary hover:text-text-primary transition-colors"
					aria-label="Toggle menu"
				>
					{#if mobileMenuOpen}
						<svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M6 18L18 6M6 6l12 12"
							/>
						</svg>
					{:else}
						<svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M4 6h16M4 12h16M4 18h16"
							/>
						</svg>
					{/if}
				</button>
			</div>
		</div>
	</div>

	<!-- Mobile menu -->
	{#if mobileMenuOpen}
		<div class="md:hidden bg-bg-dark/95 backdrop-blur-md border-t border-white/10">
			<div class="px-2 pt-2 pb-3 space-y-1">
				<div class="px-3 py-2 flex justify-center">
					<LanguageToggle />
				</div>
				
				<a
					href="/"
					onclick={closeMobileMenu}
					class="block px-3 py-2 text-text-secondary hover:bg-white/5 hover:text-text-primary rounded-lg transition-colors"
				>
					{$t.nav.home}
				</a>
				<a
					href="/pricing"
					onclick={closeMobileMenu}
					class="block px-3 py-2 text-text-secondary hover:bg-white/5 hover:text-text-primary rounded-lg transition-colors"
				>
					{$t.nav.pricing}
				</a>
				<a
					href="/academy"
					onclick={closeMobileMenu}
					class="block px-3 py-2 text-text-secondary hover:bg-white/5 hover:text-text-primary rounded-lg transition-colors"
				>
					{$t.nav.academy}
				</a>

				{#if isLoggedIn && user}
					<div class="border-t border-white/10 pt-2 mt-2">
						<a
							href="/dashboard"
							onclick={closeMobileMenu}
							class="block px-3 py-2 text-text-secondary hover:bg-white/5 hover:text-text-primary rounded-lg transition-colors"
						>
							{$t.nav.dashboard}
						</a>
						<a
							href="/profile"
							onclick={closeMobileMenu}
							class="block px-3 py-2 text-text-secondary hover:bg-white/5 hover:text-text-primary rounded-lg transition-colors"
						>
							{$t.nav.profile}
						</a>
						{#if isAdmin}
							<a
								href="/admin"
								onclick={closeMobileMenu}
								class="block px-3 py-2 text-text-secondary hover:bg-white/5 hover:text-text-primary rounded-lg transition-colors"
							>
								{$t.nav.admin} 🥓
							</a>
						{/if}
						<button
							onclick={() => {
								onLogout();
								closeMobileMenu();
							}}
							class="block w-full text-left px-3 py-2 text-bacon-red hover:bg-white/5 rounded-lg transition-colors"
						>
							{$t.nav.logout}
						</button>
					</div>
				{:else}
					<div class="border-t border-white/10 pt-2 mt-2 space-y-2">
						<button
							onclick={() => {
								onLogin();
								closeMobileMenu();
							}}
							class="block w-full px-3 py-2 text-text-primary hover:bg-white/5 rounded-lg transition-colors text-left"
						>
							{$t.nav.login}
						</button>
						<button
							onclick={() => {
								onRegister();
								closeMobileMenu();
							}}
							class="block w-full px-3 py-2 bg-gradient-to-r from-bacon-orange to-bacon-red text-white rounded-lg hover:shadow-lg hover:shadow-bacon-orange/30 transition-all font-semibold"
						>
							{$t.nav.register}
						</button>
					</div>
				{/if}
			</div>
		</div>
	{/if}
</nav>
