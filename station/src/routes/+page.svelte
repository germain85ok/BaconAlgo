<script lang="ts">
	import { onMount } from 'svelte';
	import AnimatedBackground from '$lib/components/ui/AnimatedBackground.svelte';
	import Navbar from '$lib/components/ui/Navbar.svelte';
	import Footer from '$lib/components/ui/Footer.svelte';
	
	let email = '';
	let emailSubmitted = false;
	
	const features = [
		{
			icon: '🎯',
			title: 'Smart Money Concepts',
			description: 'Advanced FVG, Order Blocks, BOS/CHoCH detection with institutional-grade analysis'
		},
		{
			icon: '⚡',
			title: 'Ultra-fast Rust Scanner',
			description: 'Process 1000+ symbols per second with our blazing-fast Rust engine'
		},
		{
			icon: '🤖',
			title: 'AI-Powered Predictions',
			description: 'Machine learning models trained on millions of data points for accuracy'
		},
		{
			icon: '📊',
			title: 'Real-time Dashboard',
			description: 'Live confluence scores, multi-timeframe analysis, and instant notifications'
		},
		{
			icon: '💎',
			title: 'Professional Tiers',
			description: 'From free tier to institutional-grade plans for every trader level'
		},
		{
			icon: '🔒',
			title: 'Bank-Grade Security',
			description: 'Enterprise-level encryption and secure infrastructure you can trust'
		}
	];
	
	const stats = [
		{ value: '10,000+', label: 'Signals Generated' },
		{ value: '95%', label: 'Win Rate' },
		{ value: '24/7', label: 'Market Monitoring' }
	];
	
	const testimonials = [
		{
			quote: "BaconAlgo transformed my trading. The Smart Money analysis is unmatched!",
			name: "Alex Chen",
			role: "Day Trader",
			avatar: "AC"
		},
		{
			quote: "Finally, a platform that understands institutional flows. Game changer!",
			name: "Sarah Martinez",
			role: "Swing Trader",
			avatar: "SM"
		},
		{
			quote: "The AI predictions are incredibly accurate. Best investment I've made.",
			name: "Mike Johnson",
			role: "Professional Trader",
			avatar: "MJ"
		},
		{
			quote: "Real-time alerts saved me from multiple bad trades. Worth every penny!",
			name: "Emma Wilson",
			role: "Crypto Trader",
			avatar: "EW"
		}
	];
	
	let observedElements: HTMLElement[] = [];
	
	onMount(() => {
		const observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						entry.target.classList.add('fade-in-visible');
					}
				});
			},
			{ threshold: 0.1 }
		);
		
		observedElements = Array.from(document.querySelectorAll('.fade-in'));
		observedElements.forEach((el) => observer.observe(el));
		
		return () => observer.disconnect();
	});
	
	function handleEmailSubmit(e: Event) {
		e.preventDefault();
		if (email && email.includes('@')) {
			emailSubmitted = true;
			setTimeout(() => {
				email = '';
				emailSubmitted = false;
			}, 3000);
		}
	}
</script>

<svelte:head>
	<title>BaconAlgo - Make Bacon Great Again!</title>
	<meta name="description" content="Smart Money Concepts meet AI-Powered Trading Signals. Ultra-fast scanning, real-time analysis, institutional-grade tools." />
</svelte:head>

<div class="landing-page">
	<AnimatedBackground />
	
	<Navbar />
	
	<!-- Hero Section -->
	<section class="hero-section">
		<div class="hero-content fade-in">
			<h1 class="hero-title">
				Make Bacon <span class="gradient-text">Great Again!</span>
			</h1>
			<p class="hero-subtitle">
				Smart Money Concepts meet AI-Powered Trading Signals
			</p>
			<div class="hero-cta">
				<a href="/register" class="btn btn-primary">Start Free Trial</a>
				<a href="/pricing" class="btn btn-secondary">View Pricing</a>
			</div>
		</div>
		<div class="hero-visual fade-in">
			<div class="chart-mockup">
				<div class="chart-header">
					<div class="chart-indicator bullish">BUY SIGNAL</div>
					<div class="chart-confidence">Confidence: 95%</div>
				</div>
				<div class="chart-bars">
					{#each Array(20) as _, i}
						<div class="chart-bar" style="height: {Math.random() * 80 + 20}%; animation-delay: {i * 0.05}s;"></div>
					{/each}
				</div>
				<div class="chart-indicators">
					<span class="indicator-badge">FVG ✓</span>
					<span class="indicator-badge">Order Block ✓</span>
					<span class="indicator-badge">BOS ✓</span>
				</div>
			</div>
		</div>
	</section>
	
	<!-- Features Section -->
	<section class="features-section">
		<div class="container">
			<h2 class="section-title fade-in">Powerful Features for Smart Traders</h2>
			<div class="features-grid">
				{#each features as feature, i}
					<div class="feature-card fade-in" style="animation-delay: {i * 0.1}s;">
						<div class="feature-icon">{feature.icon}</div>
						<h3 class="feature-title">{feature.title}</h3>
						<p class="feature-description">{feature.description}</p>
					</div>
				{/each}
			</div>
		</div>
	</section>
	
	<!-- Social Proof Section -->
	<section class="social-proof-section">
		<div class="container">
			<h2 class="section-title fade-in">Trusted by Traders Worldwide</h2>
			
			<div class="stats-grid fade-in">
				{#each stats as stat}
					<div class="stat-card">
						<div class="stat-value">{stat.value}</div>
						<div class="stat-label">{stat.label}</div>
					</div>
				{/each}
			</div>
			
			<div class="testimonials-grid">
				{#each testimonials as testimonial, i}
					<div class="testimonial-card fade-in" style="animation-delay: {i * 0.15}s;">
						<div class="testimonial-quote">"{testimonial.quote}"</div>
						<div class="testimonial-author">
							<div class="author-avatar">{testimonial.avatar}</div>
							<div class="author-info">
								<div class="author-name">{testimonial.name}</div>
								<div class="author-role">{testimonial.role}</div>
							</div>
						</div>
					</div>
				{/each}
			</div>
		</div>
	</section>
	
	<!-- CTA Section -->
	<section class="cta-section fade-in">
		<div class="container">
			<h2 class="cta-title">Try Free for 7 Days</h2>
			<p class="cta-subtitle">No credit card required. Cancel anytime.</p>
			
			{#if !emailSubmitted}
				<form class="email-form" on:submit={handleEmailSubmit}>
					<input 
						type="email" 
						bind:value={email}
						placeholder="Enter your email"
						class="email-input"
						required
					/>
					<button type="submit" class="btn btn-primary">Start Now</button>
				</form>
			{:else}
				<div class="success-message">
					✓ Thanks! Check your email for next steps.
				</div>
			{/if}
			
			<p class="cta-link">
				Or <a href="/register">create an account</a> to get started
			</p>
		</div>
	</section>
	
	<Footer />
</div>

<style>
	:global(body) {
		margin: 0;
		padding: 0;
		overflow-x: hidden;
	}
	
	.landing-page {
		position: relative;
		min-height: 100vh;
		color: white;
	}
	
	/* Fade-in animations */
	.fade-in {
		opacity: 0;
		transform: translateY(30px);
		transition: opacity 0.8s ease, transform 0.8s ease;
	}
	
	:global(.fade-in-visible) {
		opacity: 1 !important;
		transform: translateY(0) !important;
	}
	
	.container {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 2rem;
	}
	
	.section-title {
		font-size: 3rem;
		font-weight: 700;
		text-align: center;
		margin-bottom: 4rem;
		background: linear-gradient(135deg, #ff6b35 0%, #f7931e 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}
	
	/* Hero Section */
	.hero-section {
		position: relative;
		z-index: 1;
		min-height: 90vh;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 8rem 2rem 4rem;
		gap: 4rem;
		flex-wrap: wrap;
	}
	
	.hero-content {
		flex: 1;
		min-width: 300px;
		max-width: 600px;
	}
	
	.hero-title {
		font-size: 4.5rem;
		font-weight: 900;
		line-height: 1.1;
		margin-bottom: 1.5rem;
		color: white;
	}
	
	.gradient-text {
		background: linear-gradient(135deg, #ff6b35 0%, #f7931e 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}
	
	.hero-subtitle {
		font-size: 1.5rem;
		color: rgba(255, 255, 255, 0.8);
		margin-bottom: 2.5rem;
		line-height: 1.6;
	}
	
	.hero-cta {
		display: flex;
		gap: 1rem;
		flex-wrap: wrap;
	}
	
	.btn {
		padding: 1rem 2.5rem;
		font-size: 1.1rem;
		font-weight: 600;
		border-radius: 12px;
		text-decoration: none;
		transition: all 0.3s ease;
		cursor: pointer;
		border: none;
		display: inline-block;
	}
	
	.btn-primary {
		background: linear-gradient(135deg, #ff6b35 0%, #f7931e 100%);
		color: white;
		box-shadow: 0 8px 25px rgba(255, 107, 53, 0.4);
	}
	
	.btn-primary:hover {
		transform: translateY(-3px);
		box-shadow: 0 12px 35px rgba(255, 107, 53, 0.5);
	}
	
	.btn-secondary {
		background: rgba(255, 255, 255, 0.1);
		color: white;
		border: 2px solid rgba(255, 107, 53, 0.5);
		backdrop-filter: blur(10px);
	}
	
	.btn-secondary:hover {
		background: rgba(255, 107, 53, 0.2);
		border-color: rgba(255, 107, 53, 0.8);
		transform: translateY(-3px);
	}
	
	.hero-visual {
		flex: 1;
		min-width: 300px;
		max-width: 500px;
	}
	
	.chart-mockup {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(20px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 20px;
		padding: 2rem;
		box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
	}
	
	.chart-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
	}
	
	.chart-indicator {
		padding: 0.5rem 1rem;
		border-radius: 8px;
		font-weight: 700;
		font-size: 0.9rem;
	}
	
	.chart-indicator.bullish {
		background: rgba(34, 197, 94, 0.2);
		color: #22c55e;
		border: 1px solid rgba(34, 197, 94, 0.5);
	}
	
	.chart-confidence {
		color: rgba(255, 255, 255, 0.7);
		font-size: 0.9rem;
	}
	
	.chart-bars {
		display: flex;
		align-items: flex-end;
		gap: 0.4rem;
		height: 200px;
		margin-bottom: 1.5rem;
	}
	
	.chart-bar {
		flex: 1;
		background: linear-gradient(180deg, #ff6b35 0%, #f7931e 100%);
		border-radius: 4px 4px 0 0;
		animation: barGrow 1s ease forwards;
		opacity: 0.8;
		transition: opacity 0.3s ease;
	}
	
	.chart-bar:hover {
		opacity: 1;
	}
	
	@keyframes barGrow {
		from {
			height: 0;
			opacity: 0;
		}
		to {
			opacity: 0.8;
		}
	}
	
	.chart-indicators {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}
	
	.indicator-badge {
		padding: 0.4rem 0.8rem;
		background: rgba(34, 197, 94, 0.2);
		color: #22c55e;
		border: 1px solid rgba(34, 197, 94, 0.3);
		border-radius: 6px;
		font-size: 0.8rem;
		font-weight: 600;
	}
	
	/* Features Section */
	.features-section {
		position: relative;
		z-index: 1;
		padding: 6rem 0;
	}
	
	.features-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
		gap: 2rem;
	}
	
	.feature-card {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(20px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 20px;
		padding: 2.5rem;
		transition: all 0.4s ease;
		cursor: pointer;
	}
	
	.feature-card:hover {
		background: rgba(255, 255, 255, 0.08);
		border-color: rgba(255, 107, 53, 0.5);
		transform: translateY(-10px);
		box-shadow: 0 20px 40px rgba(255, 107, 53, 0.2);
	}
	
	.feature-icon {
		font-size: 3.5rem;
		margin-bottom: 1.5rem;
	}
	
	.feature-title {
		font-size: 1.5rem;
		font-weight: 700;
		margin-bottom: 1rem;
		color: #ff6b35;
	}
	
	.feature-description {
		color: rgba(255, 255, 255, 0.7);
		line-height: 1.6;
		font-size: 1.05rem;
	}
	
	/* Social Proof Section */
	.social-proof-section {
		position: relative;
		z-index: 1;
		padding: 6rem 0;
		background: rgba(0, 0, 0, 0.2);
	}
	
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		gap: 2rem;
		margin-bottom: 4rem;
	}
	
	.stat-card {
		text-align: center;
		padding: 2rem;
	}
	
	.stat-value {
		font-size: 3.5rem;
		font-weight: 900;
		background: linear-gradient(135deg, #ff6b35 0%, #f7931e 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
		margin-bottom: 0.5rem;
	}
	
	.stat-label {
		font-size: 1.1rem;
		color: rgba(255, 255, 255, 0.7);
		font-weight: 500;
	}
	
	.testimonials-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: 2rem;
	}
	
	.testimonial-card {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(20px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 16px;
		padding: 2rem;
		transition: all 0.3s ease;
	}
	
	.testimonial-card:hover {
		background: rgba(255, 255, 255, 0.08);
		border-color: rgba(255, 107, 53, 0.3);
		transform: translateY(-5px);
	}
	
	.testimonial-quote {
		font-size: 1.1rem;
		line-height: 1.6;
		color: rgba(255, 255, 255, 0.9);
		margin-bottom: 1.5rem;
		font-style: italic;
	}
	
	.testimonial-author {
		display: flex;
		align-items: center;
		gap: 1rem;
	}
	
	.author-avatar {
		width: 50px;
		height: 50px;
		border-radius: 50%;
		background: linear-gradient(135deg, #ff6b35 0%, #f7931e 100%);
		display: flex;
		align-items: center;
		justify-content: center;
		font-weight: 700;
		font-size: 1.1rem;
	}
	
	.author-name {
		font-weight: 600;
		color: white;
	}
	
	.author-role {
		font-size: 0.9rem;
		color: rgba(255, 255, 255, 0.6);
	}
	
	/* CTA Section */
	.cta-section {
		position: relative;
		z-index: 1;
		padding: 8rem 0;
		text-align: center;
	}
	
	.cta-title {
		font-size: 3.5rem;
		font-weight: 900;
		margin-bottom: 1rem;
		background: linear-gradient(135deg, #ff6b35 0%, #f7931e 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}
	
	.cta-subtitle {
		font-size: 1.3rem;
		color: rgba(255, 255, 255, 0.7);
		margin-bottom: 3rem;
	}
	
	.email-form {
		display: flex;
		gap: 1rem;
		max-width: 600px;
		margin: 0 auto 2rem;
		flex-wrap: wrap;
		justify-content: center;
	}
	
	.email-input {
		flex: 1;
		min-width: 250px;
		padding: 1rem 1.5rem;
		font-size: 1.1rem;
		background: rgba(255, 255, 255, 0.1);
		backdrop-filter: blur(10px);
		border: 2px solid rgba(255, 255, 255, 0.2);
		border-radius: 12px;
		color: white;
		transition: all 0.3s ease;
	}
	
	.email-input::placeholder {
		color: rgba(255, 255, 255, 0.5);
	}
	
	.email-input:focus {
		outline: none;
		border-color: #ff6b35;
		background: rgba(255, 255, 255, 0.15);
	}
	
	.success-message {
		padding: 1.5rem 2rem;
		background: rgba(34, 197, 94, 0.2);
		border: 2px solid rgba(34, 197, 94, 0.5);
		border-radius: 12px;
		color: #22c55e;
		font-size: 1.2rem;
		font-weight: 600;
		max-width: 600px;
		margin: 0 auto 2rem;
	}
	
	.cta-link {
		font-size: 1.1rem;
		color: rgba(255, 255, 255, 0.7);
	}
	
	.cta-link a {
		color: #ff6b35;
		text-decoration: none;
		font-weight: 600;
		transition: color 0.3s ease;
	}
	
	.cta-link a:hover {
		color: #f7931e;
		text-decoration: underline;
	}
	
	/* Mobile Responsive */
	@media (max-width: 768px) {
		.hero-title {
			font-size: 3rem;
		}
		
		.hero-subtitle {
			font-size: 1.2rem;
		}
		
		.section-title {
			font-size: 2.2rem;
		}
		
		.cta-title {
			font-size: 2.5rem;
		}
		
		.hero-section {
			padding: 6rem 1rem 2rem;
		}
		
		.features-grid,
		.testimonials-grid {
			grid-template-columns: 1fr;
		}
		
		.stats-grid {
			grid-template-columns: 1fr;
		}
		
		.stat-value {
			font-size: 2.5rem;
		}
		
		.email-form {
			flex-direction: column;
		}
		
		.email-input {
			width: 100%;
		}
	}
</style>
