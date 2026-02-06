<script lang="ts">
	export let variant: 'default' | 'bordered' | 'highlight' = 'default';
	export let className: string = '';
</script>

<div
	class="glass-panel {variant} {className}"
>
	<slot />
</div>

<style>
	.glass-panel {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(16px);
		-webkit-backdrop-filter: blur(16px);
		border-radius: 1rem;
		transition: all 0.3s ease;
	}

	.glass-panel.default {
		border: 1px solid rgba(255, 255, 255, 0.1);
	}

	.glass-panel.bordered {
		border: 2px solid rgba(255, 107, 53, 0.3);
		box-shadow: 0 0 20px rgba(255, 107, 53, 0.1);
	}

	.glass-panel.highlight {
		background: rgba(255, 107, 53, 0.1);
		border: 1px solid rgba(255, 107, 53, 0.5);
		box-shadow: 0 0 30px rgba(255, 107, 53, 0.2);
	}

	.glass-panel:hover {
		border-color: rgba(255, 107, 53, 0.4);
	}
</style>
