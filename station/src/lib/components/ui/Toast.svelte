<script lang="ts">
	import { fly, fade } from 'svelte/transition';
	import { onMount } from 'svelte';

	interface Props {
		message: string;
		type?: 'success' | 'error' | 'warning' | 'info';
		duration?: number;
		onClose?: () => void;
	}

	let { message, type = 'info', duration = 3000, onClose }: Props = $props();

	let visible = $state(true);

	const typeStyles = {
		success: {
			bg: 'bg-success-cyan/10 border-success-cyan/30',
			text: 'text-success-cyan',
			icon: '✓'
		},
		error: {
			bg: 'bg-bacon-red/10 border-bacon-red/30',
			text: 'text-bacon-red',
			icon: '✕'
		},
		warning: {
			bg: 'bg-warning-yellow/10 border-warning-yellow/30',
			text: 'text-warning-yellow',
			icon: '⚠'
		},
		info: {
			bg: 'bg-bacon-orange/10 border-bacon-orange/30',
			text: 'text-bacon-orange',
			icon: 'ℹ'
		}
	};

	const style = $derived(typeStyles[type]);

	function handleClose() {
		visible = false;
		setTimeout(() => {
			onClose?.();
		}, 300);
	}

	onMount(() => {
		if (duration > 0) {
			const timer = setTimeout(handleClose, duration);
			return () => clearTimeout(timer);
		}
	});
</script>

{#if visible}
	<div
		class="fixed top-4 right-4 z-50 max-w-sm w-full"
		transition:fly={{ y: -20, duration: 300 }}
		role="alert"
	>
		<div
			class="flex items-start gap-3 p-4 rounded-xl backdrop-blur-md border {style.bg} shadow-lg"
		>
			<span class="text-2xl {style.text} flex-shrink-0">{style.icon}</span>
			<p class="flex-1 text-text-primary font-body">{message}</p>
			<button
				onclick={handleClose}
				class="text-text-secondary hover:text-text-primary transition-colors flex-shrink-0"
				aria-label="Close notification"
			>
				<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M6 18L18 6M6 6l12 12"
					/>
				</svg>
			</button>
		</div>
	</div>
{/if}
