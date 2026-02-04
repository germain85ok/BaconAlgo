#!/bin/bash

echo "🥓 Initialisation du cockpit Bacon Algo..."

# Stores
mkdir -p src/lib/stores
cat << 'EOF' > src/lib/stores/sentiment.ts
import { writable, derived } from 'svelte/store';

export const sentimentRaw = writable<Record<string, any>>({});

export const sentiment = derived(sentimentRaw, ($raw) =>
  Object.entries($raw).map(([key, data]: any) => ({
    key,
    ...data,
    strength: data.momentum * data.confidence
  }))
);

export const greatScore = derived(sentiment, ($sentiment) => {
  if (!$sentiment.length) return 0;
  const sum = $sentiment.reduce((a, s) => a + s.strength, 0);
  return Math.round((sum / $sentiment.length) * 100);
});
EOF

# Components
mkdir -p src/lib/components
cat << 'EOF' > src/lib/components/SentimentTile.svelte
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
EOF

# Dashboard page
cat << 'EOF' > src/routes/+page.svelte
<script lang="ts">
  import { sentiment, greatScore } from '$lib/stores/sentiment';
  import SentimentTile from '$lib/components/SentimentTile.svelte';
</script>

<section class="grid grid-cols-3 gap-4">
  {#each $sentiment as s}
    <SentimentTile
      label={s.key}
      value={s.value}
      change={s.change}
      strength={s.strength}
    />
  {/each}
</section>

<div class="mt-8 text-center">
  <span class="text-zinc-400 text-sm">Great Score™</span>
  <div class="text-4xl font-bold text-emerald-400">
    {$greatScore}
  </div>
</div>
EOF

echo "✅ Cockpit initialisé avec succès."
