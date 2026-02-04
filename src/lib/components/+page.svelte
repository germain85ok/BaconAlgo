<script>
  import '$lib/styles/bacon.css';
  import HeaderOPUS from '$lib/components/HeaderOPUS.svelte';
  import SentimentOPUS from '$lib/components/SentimentOPUS.svelte';
  import RocketsOPUS from '$lib/components/RocketsOPUS.svelte';
  import ScannerOPUS from '$lib/components/ScannerOPUS.svelte';
  import ChartOPUS from '$lib/components/ChartOPUS.svelte';
</script>

<div class="bg-ultimate">
  <div class="grid-perspective"></div>
</div>

<HeaderOPUS />

<div style="display:grid;grid-template-columns:1fr 2fr 1fr;gap:1rem;padding:1rem;">
  <SentimentOPUS />
  <ChartOPUS />
  <div style="display:flex;flex-direction:column;gap:1rem;">
    <RocketsOPUS />
    <ScannerOPUS />
  </div>
</div>
