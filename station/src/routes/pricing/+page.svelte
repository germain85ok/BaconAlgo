<script lang="ts">
	import Navbar from '$lib/components/ui/Navbar.svelte';
	import Footer from '$lib/components/ui/Footer.svelte';
	import GlassCard from '$lib/components/ui/GlassCard.svelte';
	import { validateCoupon, calculatePriceWithDiscount, PRICING, type CouponValidationResult } from '$lib/services/subscriptions';

	let billingPeriod = $state<'monthly' | 'yearly'>('monthly');
	let promoCode = $state('');
	let activeFaq = $state<number | null>(null);
	let couponValidation = $state<CouponValidationResult | null>(null);
	let couponMessage = $state<string>('');
	let isValidatingCoupon = $state(false);

	const faqs = [
		{
			question: 'Can I cancel my subscription at any time?',
			answer: 'Yes! You can cancel your subscription at any time from your account settings. No questions asked.'
		},
		{
			question: 'What payment methods do you accept?',
			answer: 'We accept PayPal, credit/debit cards via Stripe, and cryptocurrency (BTC, ETH, USDT).'
		},
		{
			question: 'Is there a free trial?',
			answer: 'Yes! All new users get a 7-day free trial on any paid plan. No credit card required.'
		},
		{
			question: 'Can I upgrade or downgrade my plan?',
			answer: 'Absolutely! You can change your plan at any time. Upgrades are immediate, downgrades take effect at the next billing cycle.'
		},
		{
			question: 'Do you offer refunds?',
			answer: 'We offer a 14-day money-back guarantee. If you\'re not satisfied, contact us for a full refund.'
		}
	];

	const tiers = [
		{
			name: 'FREE',
			price: { monthly: 0, yearly: 0 },
			popular: false,
			features: [
				'YouTube Live access',
				'Basic dashboard',
				'Community Discord access',
				'Weekly market reports',
				'Limited signal history'
			]
		},
		{
			name: 'INDICATEUR',
			price: { monthly: 20, yearly: 192 }, // 20% discount: 20 * 12 * 0.8 = 192
			popular: false,
			features: [
				'TradingView indicators',
				'Basic signals',
				'Discord alerts',
				'Real-time data',
				'Email support',
				'No ads'
			]
		},
		{
			name: 'SCANNER',
			price: { monthly: 30, yearly: 288 }, // 20% discount: 30 * 12 * 0.8 = 288
			popular: true,
			features: [
				'Stock scanner',
				'Advanced filters',
				'Custom alerts',
				'Watchlists',
				'All indicators',
				'Priority support',
				'API access (limited)',
				'No ads'
			]
		},
		{
			name: 'STATION',
			price: { monthly: 50, yearly: 480 }, // 20% discount: 50 * 12 * 0.8 = 480
			popular: false,
			features: [
				'Full trading station',
				'Auto-trade functionality',
				'All broker integrations',
				'Portfolio management',
				'Full API access',
				'Unlimited everything',
				'Priority support',
				'Early access features',
				'White-label options'
			]
		}
	];

	function getPrice(tier: typeof tiers[0]) {
		const price = billingPeriod === 'yearly' ? tier.price.yearly : tier.price.monthly;
		if (billingPeriod === 'yearly') {
			return `$${Math.floor(price / 12)}/mo`;
		}
		return `$${price}/mo`;
	}

	function getYearlyPrice(tier: typeof tiers[0]) {
		return `$${tier.price.yearly}/yr`;
	}

	function toggleFaq(index: number) {
		activeFaq = activeFaq === index ? null : index;
	}

	async function applyPromoCode() {
		if (!promoCode.trim()) {
			couponMessage = 'Please enter a promo code';
			couponValidation = null;
			return;
		}

		isValidatingCoupon = true;
		couponMessage = '';

		try {
			// Validate against a sample plan (we'll validate per-plan at checkout)
			const result = await validateCoupon(promoCode.toUpperCase(), 'STATION');
			couponValidation = result;
			couponMessage = result.message;
		} catch (err) {
			couponMessage = 'Error validating coupon';
			couponValidation = null;
		} finally {
			isValidatingCoupon = false;
		}
	}

	function getPriceWithCoupon(tier: typeof tiers[0]) {
		const basePrice = billingPeriod === 'yearly' ? tier.price.yearly : tier.price.monthly;
		
		if (couponValidation?.valid && tier.name !== 'FREE') {
			const discountedPrice = calculatePriceWithDiscount(
				tier.name as keyof typeof PRICING,
				billingPeriod,
				couponValidation
			);
			
			if (billingPeriod === 'yearly') {
				return {
					display: `$${Math.floor(discountedPrice / 12)}/mo`,
					original: `$${Math.floor(basePrice / 12)}/mo`,
					savings: basePrice - discountedPrice
				};
			}
			return {
				display: `$${discountedPrice.toFixed(0)}/mo`,
				original: `$${basePrice}/mo`,
				savings: basePrice - discountedPrice
			};
		}
		
		if (billingPeriod === 'yearly') {
			return {
				display: `$${Math.floor(basePrice / 12)}/mo`,
				original: null,
				savings: 0
			};
		}
		return {
			display: `$${basePrice}/mo`,
			original: null,
			savings: 0
		};
	}
</script>

<svelte:head>
	<title>Pricing - BaconAlgo</title>
	<meta name="description" content="Choose the perfect plan for your trading needs. Free to pro plans with advanced features." />
</svelte:head>

<div class="min-h-screen flex flex-col">
	<Navbar />

	<main class="flex-1 py-16 px-4 sm:px-6 lg:px-8">
		<div class="max-w-7xl mx-auto">
			<!-- Header -->
			<div class="text-center mb-12">
				<h1 class="text-4xl md:text-5xl font-display font-bold mb-4 bg-gradient-to-r from-bacon-orange to-bacon-red bg-clip-text text-transparent">
					Choose Your Plan
				</h1>
				<p class="text-text-secondary text-lg max-w-2xl mx-auto">
					Start free, upgrade when ready. All plans include 7-day free trial.
				</p>
			</div>

			<!-- Billing Toggle -->
			<div class="flex items-center justify-center mb-12">
				<div class="glass p-1 rounded-full inline-flex">
					<button
						onclick={() => billingPeriod = 'monthly'}
						class="px-6 py-2 rounded-full font-semibold transition-all {billingPeriod === 'monthly' ? 'bg-gradient-to-r from-bacon-orange to-bacon-red text-white' : 'text-text-secondary'}"
					>
						Monthly
					</button>
					<button
						onclick={() => billingPeriod = 'yearly'}
						class="px-6 py-2 rounded-full font-semibold transition-all {billingPeriod === 'yearly' ? 'bg-gradient-to-r from-bacon-orange to-bacon-red text-white' : 'text-text-secondary'}"
					>
						Yearly
						<span class="ml-2 text-xs bg-green-500 text-white px-2 py-1 rounded-full">Save 20%</span>
					</button>
				</div>
			</div>

			<!-- Pricing Cards -->
			<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-12">
				{#each tiers as tier}
					<GlassCard 
						hover={true}
						class="flex flex-col {tier.popular ? 'ring-2 ring-bacon-orange relative' : ''}"
					>
						{#if tier.popular}
							<div class="absolute -top-4 left-1/2 -translate-x-1/2 bg-gradient-to-r from-bacon-orange to-bacon-red text-white px-4 py-1 rounded-full text-sm font-bold">
								Most Popular
							</div>
						{/if}

						<div class="text-center mb-6">
							<h3 class="text-2xl font-display font-bold text-text-primary mb-2">{tier.name}</h3>
							<div class="mb-2">
								{#if getPriceWithCoupon(tier).original}
									<div class="text-sm text-text-secondary line-through mb-1">
										{getPriceWithCoupon(tier).original}
									</div>
									<span class="text-4xl font-bold bg-gradient-to-r from-green-400 to-green-600 bg-clip-text text-transparent">
										{getPriceWithCoupon(tier).display}
									</span>
									<div class="text-xs text-green-400 mt-1">
										Save ${getPriceWithCoupon(tier).savings.toFixed(0)}!
									</div>
								{:else}
									<span class="text-4xl font-bold bg-gradient-to-r from-bacon-orange to-bacon-red bg-clip-text text-transparent">
										{getPriceWithCoupon(tier).display}
									</span>
								{/if}
							</div>
							{#if billingPeriod === 'yearly' && tier.price.yearly > 0}
								<p class="text-text-secondary text-sm">{getYearlyPrice(tier)} billed annually</p>
							{/if}
						</div>

						<ul class="space-y-3 mb-8 flex-1">
							{#each tier.features as feature}
								<li class="flex items-start text-text-secondary text-sm">
									<svg class="w-5 h-5 text-bacon-orange mr-2 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
									</svg>
									{feature}
								</li>
							{/each}
						</ul>

						<a
							href="/register"
							class="w-full py-3 rounded-lg font-semibold text-center transition-all {tier.popular 
								? 'bg-gradient-to-r from-bacon-orange to-bacon-red text-white hover:shadow-lg hover:shadow-bacon-orange/30' 
								: 'bg-white/10 text-text-primary hover:bg-white/20'}"
						>
							Get Started
						</a>
					</GlassCard>
				{/each}
			</div>

			<!-- Promo Code -->
			<div class="max-w-md mx-auto mb-12">
				<GlassCard>
					<div class="flex gap-2">
						<input
							type="text"
							bind:value={promoCode}
							placeholder="Enter promo code"
							class="flex-1 bg-white/5 border border-white/10 rounded-lg px-4 py-3 text-white placeholder-text-secondary focus:outline-none focus:ring-2 focus:ring-bacon-orange/50"
						/>
						<button
							onclick={applyPromoCode}
							disabled={isValidatingCoupon}
							class="px-6 py-3 bg-gradient-to-r from-bacon-orange to-bacon-red text-white rounded-lg font-semibold hover:shadow-lg hover:shadow-bacon-orange/30 transition-all disabled:opacity-50"
						>
							{isValidatingCoupon ? 'Checking...' : 'Apply'}
						</button>
					</div>
					{#if couponMessage}
						<div class="mt-3 text-sm {couponValidation?.valid ? 'text-green-400' : 'text-red-400'}">
							{couponMessage}
						</div>
					{/if}
				</GlassCard>
			</div>

			<!-- Payment Methods -->
			<div class="text-center mb-12">
				<p class="text-text-secondary mb-4">Accepted Payment Methods</p>
				<div class="flex items-center justify-center gap-6 flex-wrap">
					<div class="glass px-6 py-3 rounded-lg">
						<span class="text-text-primary font-semibold">💳 PayPal</span>
					</div>
					<div class="glass px-6 py-3 rounded-lg">
						<span class="text-text-primary font-semibold">💳 Stripe</span>
					</div>
					<div class="glass px-6 py-3 rounded-lg">
						<span class="text-text-primary font-semibold">₿ Crypto</span>
					</div>
				</div>
			</div>

			<!-- FAQ -->
			<div class="max-w-3xl mx-auto">
				<h2 class="text-3xl font-display font-bold text-center mb-8 bg-gradient-to-r from-bacon-orange to-bacon-red bg-clip-text text-transparent">
					Frequently Asked Questions
				</h2>
				<div class="space-y-4">
					{#each faqs as faq, i}
						<GlassCard class="cursor-pointer" hover={true}>
							<button
								onclick={() => toggleFaq(i)}
								class="w-full text-left flex items-center justify-between"
							>
								<h3 class="text-text-primary font-semibold pr-4">{faq.question}</h3>
								<svg
									class="w-5 h-5 text-bacon-orange transition-transform {activeFaq === i ? 'rotate-180' : ''}"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
								>
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
								</svg>
							</button>
							{#if activeFaq === i}
								<div class="mt-4 pt-4 border-t border-white/10">
									<p class="text-text-secondary">{faq.answer}</p>
								</div>
							{/if}
						</GlassCard>
					{/each}
				</div>
			</div>
		</div>
	</main>

	<Footer />
</div>

<style>
	/* Smooth animations */
	:global(.fade-in) {
		animation: fadeIn 0.3s ease-out;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
			transform: translateY(-10px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}
</style>
