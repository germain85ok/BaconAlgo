<script lang="ts">
  export let label;
  export let value;
  export let change;
  export let strength;
</script>

<div class="bg-zinc-900 rounded-xl p-4 flex flex-col gap-1">
  <span class="text-xs text-zinc-400">{label}</span>
  <span class="text-xl font-semibold">{value}</span>
  <span class="text-sm"
    class:text-emerald-400={change > 0}
    class:text-rose-400={change < 0}>
    {change > 0 ? '+' : ''}{change}%
  </span>

  <div class="h-1 bg-zinc-800 rounded overflow-hidden mt-2">
    <div
      class="h-full bg-emerald-500 transition-all duration-300"
      style="width: {Math.min(Math.abs(strength) * 100, 100)}%">
    </div>
  </div>
</div>
