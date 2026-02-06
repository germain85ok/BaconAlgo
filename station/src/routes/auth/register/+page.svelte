<script lang="ts">
	import { auth } from '$lib/stores/auth';
	import { goto } from '$app/navigation';
	
	let email = '';
	let username = '';
	let password = '';
	let confirmPassword = '';
	let promoCode = '';
	let error = '';
	let loading = false;
	let showPromoSuccess = false;
	let detectedTier = '';

	function checkPromoCode() {
		const promoCodes: Record<string, string> = {
			'ILOVEBACON-AND-TEA': 'INSTITUTIONAL',
			'BACONALGO2040': 'PRO',
			'PRO2040': 'PRO'
		};

		if (promoCode && promoCodes[promoCode]) {
			detectedTier = promoCodes[promoCode];
			showPromoSuccess = true;
		} else {
			detectedTier = '';
			showPromoSuccess = false;
		}
	}

	async function handleRegister() {
		if (!email || !username || !password || !confirmPassword) {
			error = 'Veuillez remplir tous les champs';
			return;
		}

		if (password !== confirmPassword) {
			error = 'Les mots de passe ne correspondent pas';
			return;
		}

		if (password.length < 8) {
			error = 'Le mot de passe doit contenir au moins 8 caractères';
			return;
		}

		loading = true;
		error = '';

		const result = await auth.register(email, username, password, promoCode || undefined);

		loading = false;

		if (result.success) {
			goto('/dashboard');
		} else {
			error = result.error || "Erreur d'inscription";
		}
	}

	async function handleOAuth(provider: 'google' | 'discord' | 'github') {
		await auth.oauthLogin(provider);
	}
</script>

<svelte:head>
	<title>Inscription - BaconAlgo 2040</title>
</svelte:head>

<div class="min-h-screen flex items-center justify-center p-4 py-12">
	<div class="w-full max-w-md">
		<div class="glass-panel p-8">
			<div class="text-center mb-8">
				<h1 class="text-4xl font-display font-bold text-bacon-orange mb-2 glow-text">
					BaconAlgo
				</h1>
				<p class="text-gray-400">Créer votre compte Neural 2040</p>
			</div>

			<form on:submit|preventDefault={handleRegister} class="space-y-4">
				<div>
					<label for="email" class="block text-sm font-medium text-gray-300 mb-2">
						Courriel
					</label>
					<input
						id="email"
						type="email"
						bind:value={email}
						class="input-field w-full"
						placeholder="votre@courriel.com"
						required
					/>
				</div>

				<div>
					<label for="username" class="block text-sm font-medium text-gray-300 mb-2">
						Nom d'utilisateur
					</label>
					<input
						id="username"
						type="text"
						bind:value={username}
						class="input-field w-full"
						placeholder="utilisateur"
						required
					/>
				</div>

				<div>
					<label for="password" class="block text-sm font-medium text-gray-300 mb-2">
						Mot de passe
					</label>
					<input
						id="password"
						type="password"
						bind:value={password}
						class="input-field w-full"
						placeholder="••••••••"
						required
					/>
				</div>

				<div>
					<label for="confirmPassword" class="block text-sm font-medium text-gray-300 mb-2">
						Confirmer le mot de passe
					</label>
					<input
						id="confirmPassword"
						type="password"
						bind:value={confirmPassword}
						class="input-field w-full"
						placeholder="••••••••"
						required
					/>
				</div>

				<div>
					<label for="promoCode" class="block text-sm font-medium text-gray-300 mb-2">
						Code promo (optionnel)
					</label>
					<input
						id="promoCode"
						type="text"
						bind:value={promoCode}
						on:input={checkPromoCode}
						class="input-field w-full"
						placeholder="ILOVEBACON-AND-TEA"
					/>
					{#if showPromoSuccess}
						<p class="mt-2 text-sm text-green-400">
							✓ Code valide! Accès {detectedTier} activé
						</p>
					{/if}
				</div>

				{#if error}
					<div class="bg-red-500/10 border border-red-500/50 rounded-lg p-3">
						<p class="text-red-400 text-sm">{error}</p>
					</div>
				{/if}

				<button
					type="submit"
					disabled={loading}
					class="neon-button w-full"
				>
					{#if loading}
						<span class="inline-block animate-spin">⟳</span>
						Inscription...
					{:else}
						S'inscrire
					{/if}
				</button>
			</form>

			<div class="mt-6">
				<div class="relative">
					<div class="absolute inset-0 flex items-center">
						<div class="w-full border-t border-gray-700"></div>
					</div>
					<div class="relative flex justify-center text-sm">
						<span class="px-2 bg-bg-dark text-gray-400">Ou continuer avec</span>
					</div>
				</div>

				<div class="mt-6 grid grid-cols-3 gap-3">
					<button
						on:click={() => handleOAuth('google')}
						class="glass-button flex items-center justify-center"
					>
						<svg class="w-5 h-5" viewBox="0 0 24 24">
							<path
								fill="currentColor"
								d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"
							/>
						</svg>
					</button>

					<button
						on:click={() => handleOAuth('discord')}
						class="glass-button flex items-center justify-center"
					>
						<svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
							<path
								d="M20.317 4.37a19.791 19.791 0 0 0-4.885-1.515a.074.074 0 0 0-.079.037c-.21.375-.444.864-.608 1.25a18.27 18.27 0 0 0-5.487 0a12.64 12.64 0 0 0-.617-1.25a.077.077 0 0 0-.079-.037A19.736 19.736 0 0 0 3.677 4.37a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057a.082.082 0 0 0 .031.057a19.9 19.9 0 0 0 5.993 3.03a.078.078 0 0 0 .084-.028a14.09 14.09 0 0 0 1.226-1.994a.076.076 0 0 0-.041-.106a13.107 13.107 0 0 1-1.872-.892a.077.077 0 0 1-.008-.128a10.2 10.2 0 0 0 .372-.292a.074.074 0 0 1 .077-.01c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 0 1 .078.01c.12.098.246.198.373.292a.077.077 0 0 1-.006.127a12.299 12.299 0 0 1-1.873.892a.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.028a19.839 19.839 0 0 0 6.002-3.03a.077.077 0 0 0 .032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 0 0-.031-.03z"
							/>
						</svg>
					</button>

					<button
						on:click={() => handleOAuth('github')}
						class="glass-button flex items-center justify-center"
					>
						<svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
							<path
								d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
							/>
						</svg>
					</button>
				</div>
			</div>

			<p class="mt-6 text-center text-sm text-gray-400">
				Vous avez déjà un compte?
				<a href="/auth/login" class="text-bacon-orange hover:text-bacon-accent font-medium">
					Se connecter
				</a>
			</p>
		</div>
	</div>
</div>

<style>
	.glass-panel {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(16px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 1rem;
	}

	.input-field {
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.5rem;
		padding: 0.75rem 1rem;
		color: white;
		transition: all 0.3s;
	}

	.input-field:focus {
		outline: none;
		border-color: #ff6b35;
		box-shadow: 0 0 0 3px rgba(255, 107, 53, 0.1);
	}

	.neon-button {
		background: linear-gradient(to right, #ff6b35, #f97316);
		color: white;
		font-weight: 600;
		padding: 0.75rem 1.5rem;
		border-radius: 0.5rem;
		transition: all 0.3s;
		border: none;
		cursor: pointer;
	}

	.neon-button:hover:not(:disabled) {
		box-shadow: 0 0 20px rgba(255, 107, 53, 0.5);
		transform: translateY(-2px);
	}

	.neon-button:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.glass-button {
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		padding: 0.75rem;
		border-radius: 0.5rem;
		transition: all 0.3s;
		cursor: pointer;
		color: white;
	}

	.glass-button:hover {
		background: rgba(255, 255, 255, 0.1);
		border-color: rgba(255, 107, 53, 0.3);
	}

	.glow-text {
		text-shadow: 0 0 20px rgba(255, 107, 53, 0.5);
	}
</style>
