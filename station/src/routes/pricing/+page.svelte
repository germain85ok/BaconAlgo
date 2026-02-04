<script>
  import GlassCard from '$lib/components/GlassCard.svelte';
  
  let promoCode = '';
  let promoMessage = '';
  let promoValid = false;
  
  const plans = [
    {
      name: 'Indicator',
      price: 20,
      features: [
        'BaconAlgo TradingView Indicator',
        'Basic Smart Money analysis',
        'Support & updates'
      ],
      paypalButtonId: 'YGZZHDVETM3AL'
    },
    {
      name: 'Scanner',
      price: 30,
      features: [
        'Everything in Indicator',
        'Multi-symbol scanner',
        'Real-time alerts',
        'Desktop & Mobile'
      ],
      paypalButtonId: '5VM6UAHY34BZS',
      popular: true
    },
    {
      name: 'Station',
      price: 50,
      features: [
        'Everything in Scanner',
        'Portfolio manager',
        'ML/AI predictions',
        'Broker integrations (IBKR, Bitget)',
        'Priority support'
      ],
      paypalButtonId: 'D8XKK24KV74GC'
    }
  ];
  
  async function verifyPromo() {
    try {
      const response = await fetch('http://localhost:3000/api/auth/verify-promo', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ code: promoCode })
      });
      
      const result = await response.json();
      promoValid = result.valid;
      promoMessage = result.message;
    } catch (error) {
      promoMessage = 'Failed to verify promo code';
      promoValid = false;
    }
  }
</script>

<div class="space-y-8">
  <div class="text-center">
    <h1 class="text-5xl font-display font-bold text-gradient mb-4">Pricing</h1>
    <p class="text-xl text-gray-300">Choose the perfect plan for your trading</p>
  </div>
  
  <!-- Promo Code Section -->
  <GlassCard className="p-6 max-w-md mx-auto">
    <h3 class="text-xl font-display font-bold mb-4">Have a Promo Code?</h3>
    <div class="flex gap-2">
      <input 
        type="text" 
        bind:value={promoCode}
        placeholder="Enter code (e.g., FREEBACON)"
        class="input flex-1"
      />
      <button on:click={verifyPromo} class="btn-primary">Verify</button>
    </div>
    {#if promoMessage}
      <p class="mt-3 text-sm {promoValid ? 'text-success' : 'text-negative'}">
        {promoMessage}
      </p>
    {/if}
    <div class="mt-4 text-xs text-gray-400 space-y-1">
      <p>💎 <strong>ILOVEBACON&TEA</strong> - Full Station access</p>
      <p>🆓 <strong>FREEBACON</strong> - 7 days free trial</p>
      <p>💰 <strong>BACON20</strong> - 20% discount</p>
      <p>🔥 <strong>BACON50</strong> - 50% discount</p>
    </div>
  </GlassCard>
  
  <!-- Pricing Cards -->
  <div class="grid grid-cols-1 md:grid-cols-3 gap-8 max-w-6xl mx-auto">
    {#each plans as plan}
      <GlassCard hover={true} className="p-8 relative {plan.popular ? 'ring-2 ring-primary' : ''}">
        {#if plan.popular}
          <div class="absolute -top-3 left-1/2 -translate-x-1/2">
            <span class="badge badge-bullish">MOST POPULAR</span>
          </div>
        {/if}
        
        <h3 class="text-2xl font-display font-bold mb-2">{plan.name}</h3>
        <div class="mb-6">
          <span class="text-5xl font-display font-bold text-gradient">${plan.price}</span>
          <span class="text-gray-400">/month</span>
        </div>
        
        <ul class="space-y-3 mb-8">
          {#each plan.features as feature}
            <li class="flex items-start">
              <span class="text-success mr-2">✓</span>
              <span class="text-sm">{feature}</span>
            </li>
          {/each}
        </ul>
        
        <button class="btn-primary w-full">
          Subscribe - ${plan.price}/mo
        </button>
        
        <p class="text-xs text-gray-400 text-center mt-4">
          PayPal ID: {plan.paypalButtonId}
        </p>
      </GlassCard>
    {/each}
  </div>
  
  <!-- Donation -->
  <GlassCard className="p-8 max-w-md mx-auto text-center">
    <div class="text-4xl mb-4">☕</div>
    <h3 class="text-xl font-display font-bold mb-2">Support Development</h3>
    <p class="text-gray-400 mb-4">Help us improve BaconAlgo</p>
    <button class="btn-secondary">
      Donate via PayPal
    </button>
    <p class="text-xs text-gray-400 mt-2">ID: KWVC5YDHC6QNU</p>
  </GlassCard>
</div>
