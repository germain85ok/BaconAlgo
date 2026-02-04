<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
	import AnimatedBackground from '$lib/components/AnimatedBackground.svelte';
	import Navbar from '$lib/components/Navbar.svelte';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<title>🥓 BaconAlgo SAAS 2030 - Ultimate Trading Platform</title>
</svelte:head>

<AnimatedBackground />
<Navbar />

<main class="container mx-auto px-6 py-8">
	{@render children()}
</main>
