<script lang="ts">
  export let symbol;
  export let score;
  export let state;
</script>

<div class="bg-zinc-900 rounded-xl p-4 border border-zinc-800">
  <div class="flex justify-between">
    <span class="font-semibold">{symbol}</span>
    <span class="text-xs text-zinc-400">{state}</span>
  </div>
  <div class="mt-2 text-emerald-400 font-bold">
    Confluence {score}
  </div>
</div>
