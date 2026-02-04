<script lang="ts">
  export let rows = [];
</script>

<table class="w-full text-sm text-zinc-300">
  <thead>
    <tr class="text-zinc-500">
      <th class="text-left">Symbol</th>
      <th>Score</th>
      <th>Setup</th>
    </tr>
  </thead>
  <tbody>
    {#each rows as r}
      <tr class="border-t border-zinc-800">
        <td>{r.symbol}</td>
        <td class="text-emerald-400">{r.score}</td>
        <td>{r.setup}</td>
      </tr>
    {/each}
  </tbody>
</table>
