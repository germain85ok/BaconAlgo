<script lang="ts">
	import { locale } from '$lib/i18n/i18n';
</script>

<div class="language-toggle">
	<button
		class="lang-btn {$locale === 'fr' ? 'active' : ''}"
		onclick={() => locale.set('fr')}
		aria-label="Switch to French"
	>
		FR
	</button>
	<span class="separator">|</span>
	<button
		class="lang-btn {$locale === 'en' ? 'active' : ''}"
		onclick={() => locale.set('en')}
		aria-label="Switch to English"
	>
		EN
	</button>
</div>

<style>
	.language-toggle {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		font-weight: 600;
	}

	.lang-btn {
		background: none;
		border: none;
		color: rgba(255, 255, 255, 0.6);
		cursor: pointer;
		transition: color 0.2s ease;
		padding: 0.25rem 0.5rem;
		font-weight: 600;
	}

	.lang-btn:hover {
		color: rgba(255, 255, 255, 0.9);
	}

	.lang-btn.active {
		color: #ff6b35;
	}

	.separator {
		color: rgba(255, 255, 255, 0.3);
	}
</style>
