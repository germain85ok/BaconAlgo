<script lang="ts">
	import { GlassCard, Modal, Toast, Navbar, Footer } from '$lib/components/ui';

	let modalOpen = $state(false);
	let showToast = $state(false);
	let toastType = $state<'success' | 'error' | 'warning' | 'info'>('success');
	let isLoggedIn = $state(false);
	let user = $state<{ name: string; email: string } | null>(null);

	function openModal() {
		modalOpen = true;
	}

	function closeModal() {
		modalOpen = false;
	}

	function showToastMessage(type: 'success' | 'error' | 'warning' | 'info') {
		toastType = type;
		showToast = false;
		setTimeout(() => (showToast = true), 10);
	}

	function handleLogin() {
		isLoggedIn = true;
		user = { name: 'John Doe', email: 'john@example.com' };
		showToastMessage('success');
	}

	function handleRegister() {
		alert('Register clicked');
	}

	function handleLogout() {
		isLoggedIn = false;
		user = null;
		showToastMessage('info');
	}
</script>

<div class="min-h-screen bg-bg-dark">
	<Navbar {isLoggedIn} {user} onLogin={handleLogin} onRegister={handleRegister} onLogout={handleLogout} />

	<main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
		<h1 class="text-4xl font-display font-bold text-text-primary mb-8">
			BaconAlgo UI Components Demo
		</h1>

		<!-- GlassCard Demo -->
		<section class="mb-12">
			<h2 class="text-2xl font-display font-semibold text-text-primary mb-4">GlassCard</h2>
			<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
				<GlassCard padding="sm">
					<h3 class="text-bacon-orange font-display font-semibold mb-2">Small Padding</h3>
					<p class="text-text-secondary text-sm">This card has small padding.</p>
				</GlassCard>

				<GlassCard hover>
					<h3 class="text-bacon-orange font-display font-semibold mb-2">Hover Effect</h3>
					<p class="text-text-secondary text-sm">Hover over this card to see the effect.</p>
				</GlassCard>

				<GlassCard padding="lg" class="border-bacon-orange/30">
					<h3 class="text-bacon-orange font-display font-semibold mb-2">Large + Custom</h3>
					<p class="text-text-secondary text-sm">Large padding with custom border color.</p>
				</GlassCard>
			</div>
		</section>

		<!-- Modal Demo -->
		<section class="mb-12">
			<h2 class="text-2xl font-display font-semibold text-text-primary mb-4">Modal</h2>
			<button
				onclick={openModal}
				class="px-6 py-3 bg-gradient-to-r from-bacon-orange to-bacon-red text-white rounded-lg hover:shadow-lg hover:shadow-bacon-orange/30 transition-all font-body font-semibold"
			>
				Open Modal
			</button>

			<Modal isOpen={modalOpen} onClose={closeModal} title="Example Modal" size="md">
				<div class="space-y-4">
					<p class="text-text-secondary">
						This is a modal component with glassmorphism design. You can close it by clicking the X
						button, clicking outside, or pressing Escape.
					</p>
					<div class="flex justify-end gap-3">
						<button
							onclick={closeModal}
							class="px-4 py-2 text-text-secondary hover:text-text-primary transition-colors"
						>
							Cancel
						</button>
						<button
							onclick={closeModal}
							class="px-4 py-2 bg-gradient-to-r from-bacon-orange to-bacon-red text-white rounded-lg hover:shadow-lg transition-all"
						>
							Confirm
						</button>
					</div>
				</div>
			</Modal>
		</section>

		<!-- Toast Demo -->
		<section class="mb-12">
			<h2 class="text-2xl font-display font-semibold text-text-primary mb-4">Toast</h2>
			<div class="flex flex-wrap gap-3">
				<button
					onclick={() => showToastMessage('success')}
					class="px-4 py-2 bg-success-cyan/20 text-success-cyan rounded-lg hover:bg-success-cyan/30 transition-colors"
				>
					Success Toast
				</button>
				<button
					onclick={() => showToastMessage('error')}
					class="px-4 py-2 bg-bacon-red/20 text-bacon-red rounded-lg hover:bg-bacon-red/30 transition-colors"
				>
					Error Toast
				</button>
				<button
					onclick={() => showToastMessage('warning')}
					class="px-4 py-2 bg-warning-yellow/20 text-warning-yellow rounded-lg hover:bg-warning-yellow/30 transition-colors"
				>
					Warning Toast
				</button>
				<button
					onclick={() => showToastMessage('info')}
					class="px-4 py-2 bg-bacon-orange/20 text-bacon-orange rounded-lg hover:bg-bacon-orange/30 transition-colors"
				>
					Info Toast
				</button>
			</div>

			{#if showToast}
				{#key showToast}
					<Toast
						message="{toastType.charAt(0).toUpperCase() + toastType.slice(1)} message! This is a notification toast."
						type={toastType}
						duration={3000}
						onClose={() => (showToast = false)}
					/>
				{/key}
			{/if}
		</section>

		<!-- Usage Examples -->
		<section class="mb-12">
			<GlassCard>
				<h2 class="text-2xl font-display font-semibold text-text-primary mb-4">Usage</h2>
				<div class="space-y-4 text-text-secondary font-mono text-sm">
					<div>
						<p class="text-bacon-orange mb-2">Import components:</p>
						<pre
							class="bg-black/30 p-4 rounded-lg overflow-x-auto">import {'{ GlassCard, Modal, Toast, Navbar, Footer }'} from '$lib/components/ui';</pre>
					</div>
					<div>
						<p class="text-bacon-orange mb-2">Use in your pages:</p>
						<pre class="bg-black/30 p-4 rounded-lg overflow-x-auto">{`<GlassCard hover padding="lg">
  <h3>Your content here</h3>
</GlassCard>`}</pre>
					</div>
				</div>
			</GlassCard>
		</section>
	</main>

	<Footer />
</div>

<style>
	:global(body) {
		margin: 0;
		background: #0a0a0f;
	}
</style>
