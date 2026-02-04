<script>
  export let active = true;
</script>

<nav class="fixed top-0 left-0 right-0 z-50 glass-card border-b border-primary/20">
  <div class="container mx-auto px-6 py-4">
    <div class="flex items-center justify-between">
      <!-- Logo -->
      <a href="/" class="flex items-center space-x-3">
        <div class="text-3xl">🥓</div>
        <span class="text-2xl font-display font-bold text-gradient">BaconAlgo</span>
        {#if active}
          <span class="badge badge-bullish animate-pulse">AI</span>
        {/if}
      </a>
      
      <!-- Navigation Links -->
      <div class="hidden md:flex items-center space-x-6">
        <a href="/" class="hover:text-primary transition">Dashboard</a>
        <a href="/scanner" class="hover:text-primary transition">Scanner</a>
        <a href="/rockets" class="hover:text-primary transition">Rockets</a>
        <a href="/portfolio" class="hover:text-primary transition">Portfolio</a>
        <a href="/signals" class="hover:text-primary transition">Signals</a>
        <a href="/pricing" class="hover:text-primary transition">Pricing</a>
      </div>
      
      <!-- Right side -->
      <div class="flex items-center space-x-4">
        <a href="/settings" class="hover:text-primary transition">⚙️</a>
        <a href="/auth/login" class="btn-primary">Login</a>
      </div>
    </div>
  </div>
</nav>

<!-- Spacer to prevent content from going under navbar -->
<div class="h-20"></div>
