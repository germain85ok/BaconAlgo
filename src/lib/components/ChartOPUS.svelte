<script>
  import Chart from '$lib/components/Chart.svelte';
  import { focusSymbol } from '$lib/stores/focus';
</script>

<Chart symbol={$focusSymbol} />
