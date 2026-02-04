<script lang="ts">
	import { fade, scale } from 'svelte/transition';

	interface Props {
		isOpen: boolean;
		onClose: () => void;
		title?: string;
		size?: 'sm' | 'md' | 'lg' | 'xl';
		children?: any;
	}

	let { isOpen = $bindable(), onClose, title = '', size = 'md', children }: Props = $props();

	const sizeClasses = {
		sm: 'max-w-md',
		md: 'max-w-lg',
		lg: 'max-w-2xl',
		xl: 'max-w-4xl'
	};

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape' && isOpen) {
			onClose();
		}
	}

	function handleBackdropClick(e: MouseEvent) {
		if (e.target === e.currentTarget) {
			onClose();
		}
	}

	function handleBackdropKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' || e.key === ' ') {
			handleBackdropClick(e as any);
		}
	}

	$effect(() => {
		if (isOpen) {
			window.addEventListener('keydown', handleKeydown);
			document.body.style.overflow = 'hidden';
		} else {
			document.body.style.overflow = '';
		}

		return () => {
			window.removeEventListener('keydown', handleKeydown);
			document.body.style.overflow = '';
		};
	});
</script>

{#if isOpen}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center p-4"
		transition:fade={{ duration: 200 }}
		onclick={handleBackdropClick}
		onkeydown={handleBackdropKeydown}
		role="dialog"
		aria-modal="true"
		aria-labelledby={title ? 'modal-title' : undefined}
		tabindex="-1"
	>
		<div class="absolute inset-0 bg-black/60 backdrop-blur-sm"></div>

		<div
			class="relative w-full {sizeClasses[
				size
			]} bg-bg-dark border border-white/10 rounded-2xl shadow-2xl shadow-bacon-orange/10"
			transition:scale={{ duration: 200, start: 0.95 }}
		>
			{#if title}
				<div class="flex items-center justify-between p-6 border-b border-white/10">
					<h2 id="modal-title" class="text-2xl font-display font-bold text-text-primary">
						{title}
					</h2>
					<button
						onclick={onClose}
						class="text-text-secondary hover:text-text-primary transition-colors"
						aria-label="Close modal"
					>
						<svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M6 18L18 6M6 6l12 12"
							/>
						</svg>
					</button>
				</div>
			{:else}
				<button
					onclick={onClose}
					class="absolute top-4 right-4 text-text-secondary hover:text-text-primary transition-colors z-10"
					aria-label="Close modal"
				>
					<svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M6 18L18 6M6 6l12 12"
						/>
					</svg>
				</button>
			{/if}

			<div class="p-6">
				{@render children?.()}
			</div>
		</div>
	</div>
{/if}
