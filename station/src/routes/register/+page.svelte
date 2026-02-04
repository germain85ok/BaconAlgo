<script lang="ts">
	import { goto } from '$app/navigation';
	import { getSupabaseClient } from '$lib/supabase/client';
	import GlassCard from '$lib/components/ui/GlassCard.svelte';

	let email = $state('');
	let username = $state('');
	let password = $state('');
	let confirmPassword = $state('');
	let referralCode = $state('');
	let termsAccepted = $state(false);
	let loading = $state(false);
	let error = $state('');
	let success = $state('');

	const supabase = getSupabaseClient();

	function validateForm() {
		if (!email || !username || !password || !confirmPassword) {
			error = 'Please fill in all required fields';
			return false;
		}

		if (!email.includes('@')) {
			error = 'Please enter a valid email address';
			return false;
		}

		if (username.length < 3) {
			error = 'Username must be at least 3 characters long';
			return false;
		}

		if (password.length < 8) {
			error = 'Password must be at least 8 characters long';
			return false;
		}

		if (password !== confirmPassword) {
			error = 'Passwords do not match';
			return false;
		}

		if (!termsAccepted) {
			error = 'You must accept the Terms & Conditions';
			return false;
		}

		return true;
	}

	async function handleRegister() {
		error = '';
		success = '';

		if (!validateForm()) return;

		loading = true;
		try {
			const { data, error: signUpError } = await supabase.auth.signUp({
				email,
				password,
				options: {
					data: {
						username,
						referral_code: referralCode || null
					}
				}
			});

			if (signUpError) throw signUpError;

			if (data.user) {
				success = 'Account created! Please check your email to verify your account.';
				setTimeout(() => goto('/login'), 3000);
			}
		} catch (e) {
			error = e instanceof Error ? e.message : 'Registration failed';
		} finally {
			loading = false;
		}
	}

	async function handleGoogleRegister() {
		if (!termsAccepted) {
			error = 'You must accept the Terms & Conditions';
			return;
		}

		error = '';
		loading = true;
		try {
			const { error: signInError } = await supabase.auth.signInWithOAuth({
				provider: 'google',
				options: {
					redirectTo: `${window.location.origin}/dashboard`
				}
			});

			if (signInError) throw signInError;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Google registration failed';
			loading = false;
		}
	}

	async function handleDiscordRegister() {
		if (!termsAccepted) {
			error = 'You must accept the Terms & Conditions';
			return;
		}

		error = '';
		loading = true;
		try {
			const { error: signInError } = await supabase.auth.signInWithOAuth({
				provider: 'discord',
				options: {
					redirectTo: `${window.location.origin}/dashboard`
				}
			});

			if (signInError) throw signInError;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Discord registration failed';
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>Register - BaconAlgo</title>
	<meta name="description" content="Create your BaconAlgo account and start trading smarter" />
</svelte:head>

<div class="min-h-screen flex items-center justify-center px-4 py-12">
	<!-- Background decoration -->
	<div class="absolute inset-0 overflow-hidden pointer-events-none">
		<div class="absolute top-1/4 right-1/4 w-96 h-96 bg-bacon-orange/10 rounded-full blur-3xl"></div>
		<div class="absolute bottom-1/4 left-1/4 w-96 h-96 bg-bacon-red/10 rounded-full blur-3xl"></div>
	</div>

	<div class="w-full max-w-md relative z-10">
		<GlassCard padding="xl">
			<!-- Logo/Header -->
			<div class="text-center mb-8">
				<h1 class="text-3xl font-display font-bold bg-gradient-to-r from-bacon-orange to-bacon-red bg-clip-text text-transparent mb-2">
					Join BaconAlgo
				</h1>
				<p class="text-text-secondary">Start your 7-day free trial today</p>
			</div>

			<!-- Error/Success Messages -->
			{#if error}
				<div class="mb-4 p-4 bg-red-500/20 border border-red-500/30 rounded-lg text-red-400 text-sm animate-shake">
					{error}
				</div>
			{/if}
			{#if success}
				<div class="mb-4 p-4 bg-green-500/20 border border-green-500/30 rounded-lg text-green-400 text-sm">
					{success}
				</div>
			{/if}

			<!-- Registration Form -->
			<form onsubmit={(e) => { e.preventDefault(); handleRegister(); }} class="space-y-4 mb-6">
				<div>
					<label for="email" class="block text-text-primary text-sm font-semibold mb-2">
						Email *
					</label>
					<input
						id="email"
						type="email"
						bind:value={email}
						placeholder="you@example.com"
						class="w-full bg-white/5 border border-white/10 rounded-lg px-4 py-3 text-white placeholder-text-secondary focus:outline-none focus:ring-2 focus:ring-bacon-orange/50 transition-all"
						required
					/>
				</div>

				<div>
					<label for="username" class="block text-text-primary text-sm font-semibold mb-2">
						Username *
					</label>
					<input
						id="username"
						type="text"
						bind:value={username}
						placeholder="trader_pro"
						class="w-full bg-white/5 border border-white/10 rounded-lg px-4 py-3 text-white placeholder-text-secondary focus:outline-none focus:ring-2 focus:ring-bacon-orange/50 transition-all"
						required
					/>
				</div>

				<div>
					<label for="password" class="block text-text-primary text-sm font-semibold mb-2">
						Password *
					</label>
					<input
						id="password"
						type="password"
						bind:value={password}
						placeholder="••••••••"
						class="w-full bg-white/5 border border-white/10 rounded-lg px-4 py-3 text-white placeholder-text-secondary focus:outline-none focus:ring-2 focus:ring-bacon-orange/50 transition-all"
						required
					/>
					<p class="text-text-secondary text-xs mt-1">Minimum 8 characters</p>
				</div>

				<div>
					<label for="confirmPassword" class="block text-text-primary text-sm font-semibold mb-2">
						Confirm Password *
					</label>
					<input
						id="confirmPassword"
						type="password"
						bind:value={confirmPassword}
						placeholder="••••••••"
						class="w-full bg-white/5 border border-white/10 rounded-lg px-4 py-3 text-white placeholder-text-secondary focus:outline-none focus:ring-2 focus:ring-bacon-orange/50 transition-all"
						required
					/>
				</div>

				<div>
					<label for="referral" class="block text-text-primary text-sm font-semibold mb-2">
						Referral Code (Optional)
					</label>
					<input
						id="referral"
						type="text"
						bind:value={referralCode}
						placeholder="BACON2024"
						class="w-full bg-white/5 border border-white/10 rounded-lg px-4 py-3 text-white placeholder-text-secondary focus:outline-none focus:ring-2 focus:ring-bacon-orange/50 transition-all"
					/>
				</div>

				<!-- Terms & Conditions -->
				<div class="flex items-start">
					<input
						id="terms"
						type="checkbox"
						bind:checked={termsAccepted}
						class="w-4 h-4 mt-1 rounded border-white/10 bg-white/5 text-bacon-orange focus:ring-bacon-orange/50 focus:ring-offset-0"
					/>
					<label for="terms" class="ml-2 text-text-secondary text-sm">
						I agree to the
						<a href="/terms" class="text-bacon-orange hover:underline">Terms & Conditions</a>
						and
						<a href="/privacy" class="text-bacon-orange hover:underline">Privacy Policy</a>
					</label>
				</div>

				<!-- Register Button -->
				<button
					type="submit"
					disabled={loading || !termsAccepted}
					class="w-full py-3 bg-gradient-to-r from-bacon-orange to-bacon-red text-white rounded-lg font-semibold hover:shadow-lg hover:shadow-bacon-orange/30 transition-all disabled:opacity-50 disabled:cursor-not-allowed"
				>
					{loading ? 'Creating account...' : 'Start 7-Day Free Trial'}
				</button>

				<p class="text-text-secondary text-xs text-center">
					No credit card required • Cancel anytime
				</p>
			</form>

			<!-- Divider -->
			<div class="relative my-6">
				<div class="absolute inset-0 flex items-center">
					<div class="w-full border-t border-white/10"></div>
				</div>
				<div class="relative flex justify-center text-sm">
					<span class="px-4 bg-bg-dark text-text-secondary">OR</span>
				</div>
			</div>

			<!-- OAuth Buttons -->
			<div class="space-y-3 mb-6">
				<button
					onclick={handleGoogleRegister}
					disabled={loading}
					class="w-full py-3 bg-white/10 border border-white/10 text-text-primary rounded-lg font-semibold hover:bg-white/20 transition-all flex items-center justify-center gap-3 disabled:opacity-50 disabled:cursor-not-allowed"
				>
					<svg class="w-5 h-5" viewBox="0 0 24 24" fill="currentColor">
						<path d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z" fill="#4285F4"/>
						<path d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z" fill="#34A853"/>
						<path d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z" fill="#FBBC05"/>
						<path d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z" fill="#EA4335"/>
					</svg>
					Continue with Google
				</button>

				<button
					onclick={handleDiscordRegister}
					disabled={loading}
					class="w-full py-3 bg-[#5865F2]/20 border border-[#5865F2]/30 text-[#5865F2] rounded-lg font-semibold hover:bg-[#5865F2]/30 transition-all flex items-center justify-center gap-3 disabled:opacity-50 disabled:cursor-not-allowed"
				>
					<svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
						<path d="M20.317 4.37a19.791 19.791 0 0 0-4.885-1.515a.074.074 0 0 0-.079.037c-.21.375-.444.864-.608 1.25a18.27 18.27 0 0 0-5.487 0a12.64 12.64 0 0 0-.617-1.25a.077.077 0 0 0-.079-.037A19.736 19.736 0 0 0 3.677 4.37a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057a.082.082 0 0 0 .031.057a19.9 19.9 0 0 0 5.993 3.03a.078.078 0 0 0 .084-.028a14.09 14.09 0 0 0 1.226-1.994a.076.076 0 0 0-.041-.106a13.107 13.107 0 0 1-1.872-.892a.077.077 0 0 1-.008-.128a10.2 10.2 0 0 0 .372-.292a.074.074 0 0 1 .077-.01c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 0 1 .078.01c.12.098.246.198.373.292a.077.077 0 0 1-.006.127a12.299 12.299 0 0 1-1.873.892a.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.028a19.839 19.839 0 0 0 6.002-3.03a.077.077 0 0 0 .032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 0 0-.031-.03zM8.02 15.33c-1.183 0-2.157-1.085-2.157-2.419c0-1.333.956-2.419 2.157-2.419c1.21 0 2.176 1.096 2.157 2.42c0 1.333-.956 2.418-2.157 2.418zm7.975 0c-1.183 0-2.157-1.085-2.157-2.419c0-1.333.955-2.419 2.157-2.419c1.21 0 2.176 1.096 2.157 2.42c0 1.333-.946 2.418-2.157 2.418z"/>
					</svg>
					Continue with Discord
				</button>
			</div>

			<!-- Login Link -->
			<div class="text-center">
				<p class="text-text-secondary text-sm">
					Already have an account?
					<a href="/login" class="text-bacon-orange hover:underline font-semibold">
						Login
					</a>
				</p>
			</div>
		</GlassCard>
	</div>
</div>

<style>
	@keyframes shake {
		0%, 100% { transform: translateX(0); }
		25% { transform: translateX(-5px); }
		75% { transform: translateX(5px); }
	}

	.animate-shake {
		animation: shake 0.3s ease-in-out;
	}
</style>
