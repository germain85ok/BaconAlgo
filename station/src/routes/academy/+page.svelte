<script lang="ts">
	import Navbar from '$lib/components/ui/Navbar.svelte';
	import Footer from '$lib/components/ui/Footer.svelte';
	import GlassCard from '$lib/components/ui/GlassCard.svelte';

	interface Course {
		id: string;
		title: string;
		level: 'Beginner' | 'Intermediate' | 'Advanced';
		duration: string;
		lessons: number;
		progress: number;
		description: string;
		icon: string;
	}

	const courses: Course[] = [
		{
			id: '1',
			title: 'SMC 101',
			level: 'Beginner',
			duration: '4 hours',
			lessons: 12,
			progress: 0,
			description: 'Master the fundamentals of Smart Money Concepts and institutional trading.',
			icon: '📚'
		},
		{
			id: '2',
			title: 'Order Blocks Masterclass',
			level: 'Intermediate',
			duration: '6 hours',
			lessons: 18,
			progress: 0,
			description: 'Deep dive into identifying and trading order blocks like a professional.',
			icon: '📊'
		},
		{
			id: '3',
			title: 'Reading Confluence',
			level: 'Intermediate',
			duration: '5 hours',
			lessons: 15,
			progress: 0,
			description: 'Learn to combine multiple indicators for high-probability setups.',
			icon: '🎯'
		},
		{
			id: '4',
			title: 'FVG Trading Strategies',
			level: 'Advanced',
			duration: '7 hours',
			lessons: 20,
			progress: 0,
			description: 'Advanced Fair Value Gap strategies for consistent profits.',
			icon: '⚡'
		},
		{
			id: '5',
			title: 'Advanced ICT Concepts',
			level: 'Advanced',
			duration: '8 hours',
			lessons: 24,
			progress: 0,
			description: 'Master Inner Circle Trader methodologies and market structure.',
			icon: '🚀'
		},
		{
			id: '6',
			title: 'Risk Management Fundamentals',
			level: 'Beginner',
			duration: '3 hours',
			lessons: 10,
			progress: 0,
			description: 'Essential risk management techniques to protect your capital.',
			icon: '🛡️'
		}
	];

	function getLevelColor(level: Course['level']) {
		switch (level) {
			case 'Beginner':
				return 'bg-green-500/20 text-green-400 border-green-500/30';
			case 'Intermediate':
				return 'bg-yellow-500/20 text-yellow-400 border-yellow-500/30';
			case 'Advanced':
				return 'bg-red-500/20 text-red-400 border-red-500/30';
		}
	}

	function startCourse(courseId: string) {
		// TODO: Implement course navigation
		console.log('Starting course:', courseId);
	}
</script>

<svelte:head>
	<title>Academy - BaconAlgo</title>
	<meta name="description" content="Learn to trade like a pro with our comprehensive trading courses." />
</svelte:head>

<div class="min-h-screen flex flex-col">
	<Navbar />

	<main class="flex-1">
		<!-- Hero Section -->
		<section class="py-20 px-4 sm:px-6 lg:px-8 relative overflow-hidden">
			<div class="absolute inset-0 bg-gradient-to-b from-bacon-orange/10 to-transparent"></div>
			<div class="max-w-7xl mx-auto relative z-10">
				<div class="text-center">
					<h1 class="text-5xl md:text-6xl font-display font-bold mb-6">
						<span class="bg-gradient-to-r from-bacon-orange to-bacon-red bg-clip-text text-transparent">
							Bacon Academy
						</span>
					</h1>
					<p class="text-xl md:text-2xl text-text-secondary mb-8 max-w-3xl mx-auto">
						Learn to Trade Like a Pro
					</p>
					<p class="text-text-secondary max-w-2xl mx-auto">
						Master Smart Money Concepts, order blocks, FVG trading, and advanced ICT strategies with our comprehensive courses.
					</p>
				</div>
			</div>
		</section>

		<!-- Courses Grid -->
		<section class="py-12 px-4 sm:px-6 lg:px-8">
			<div class="max-w-7xl mx-auto">
				<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
					{#each courses as course}
						<GlassCard hover={true} class="flex flex-col">
							<!-- Course Header -->
							<div class="mb-4">
								<div class="flex items-start justify-between mb-3">
									<div class="text-4xl">{course.icon}</div>
									<span class="badge border {getLevelColor(course.level)}">
										{course.level}
									</span>
								</div>
								<h3 class="text-xl font-display font-bold text-text-primary mb-2">
									{course.title}
								</h3>
								<p class="text-text-secondary text-sm mb-4">
									{course.description}
								</p>
							</div>

							<!-- Course Info -->
							<div class="flex items-center gap-4 text-sm text-text-secondary mb-4">
								<div class="flex items-center gap-1">
									<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/>
									</svg>
									{course.duration}
								</div>
								<div class="flex items-center gap-1">
									<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"/>
									</svg>
									{course.lessons} lessons
								</div>
							</div>

							<!-- Progress Bar (shown for logged-in users) -->
							{#if course.progress > 0}
								<div class="mb-4">
									<div class="flex justify-between text-sm text-text-secondary mb-2">
										<span>Progress</span>
										<span>{course.progress}%</span>
									</div>
									<div class="w-full h-2 bg-white/10 rounded-full overflow-hidden">
										<div 
											class="h-full bg-gradient-to-r from-bacon-orange to-bacon-red transition-all duration-500"
											style="width: {course.progress}%"
										></div>
									</div>
								</div>
							{/if}

							<!-- Action Button -->
							<button
								onclick={() => startCourse(course.id)}
								class="w-full mt-auto py-3 rounded-lg font-semibold text-center transition-all bg-gradient-to-r from-bacon-orange to-bacon-red text-white hover:shadow-lg hover:shadow-bacon-orange/30 hover:-translate-y-0.5"
							>
								{course.progress > 0 ? 'Continue Course' : 'Start Course'}
							</button>
						</GlassCard>
					{/each}
				</div>
			</div>
		</section>

		<!-- Certificates Section -->
		<section class="py-16 px-4 sm:px-6 lg:px-8">
			<div class="max-w-7xl mx-auto">
				<GlassCard class="text-center">
					<div class="py-8">
						<div class="text-6xl mb-6">🏆</div>
						<h2 class="text-3xl font-display font-bold mb-4 bg-gradient-to-r from-bacon-orange to-bacon-red bg-clip-text text-transparent">
							Earn Certificates
						</h2>
						<p class="text-text-secondary max-w-2xl mx-auto mb-8">
							Complete courses and earn professional certificates to showcase your trading knowledge. 
							Share them on LinkedIn, Twitter, or add them to your trading portfolio.
						</p>
						<div class="flex flex-wrap items-center justify-center gap-4">
							<div class="glass px-6 py-3 rounded-lg">
								<div class="flex items-center gap-2">
									<svg class="w-5 h-5 text-bacon-orange" fill="currentColor" viewBox="0 0 24 24">
										<path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
									</svg>
									<span class="text-text-primary font-semibold">Verified Skills</span>
								</div>
							</div>
							<div class="glass px-6 py-3 rounded-lg">
								<div class="flex items-center gap-2">
									<svg class="w-5 h-5 text-bacon-orange" fill="currentColor" viewBox="0 0 24 24">
										<path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
									</svg>
									<span class="text-text-primary font-semibold">Shareable</span>
								</div>
							</div>
							<div class="glass px-6 py-3 rounded-lg">
								<div class="flex items-center gap-2">
									<svg class="w-5 h-5 text-bacon-orange" fill="currentColor" viewBox="0 0 24 24">
										<path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
									</svg>
									<span class="text-text-primary font-semibold">Professional</span>
								</div>
							</div>
						</div>
					</div>
				</GlassCard>
			</div>
		</section>

		<!-- CTA Section -->
		<section class="py-16 px-4 sm:px-6 lg:px-8">
			<div class="max-w-4xl mx-auto">
				<GlassCard class="text-center">
					<div class="py-8">
						<h2 class="text-3xl font-display font-bold mb-4 text-text-primary">
							Ready to Start Learning?
						</h2>
						<p class="text-text-secondary mb-8 max-w-2xl mx-auto">
							Join thousands of traders mastering Smart Money Concepts and profitable trading strategies.
						</p>
						<div class="flex flex-col sm:flex-row gap-4 justify-center">
							<a
								href="/register"
								class="px-8 py-3 bg-gradient-to-r from-bacon-orange to-bacon-red text-white rounded-lg font-semibold hover:shadow-lg hover:shadow-bacon-orange/30 transition-all"
							>
								Start Free Trial
							</a>
							<a
								href="/pricing"
								class="px-8 py-3 bg-white/10 text-text-primary rounded-lg font-semibold hover:bg-white/20 transition-all"
							>
								View Pricing
							</a>
						</div>
					</div>
				</GlassCard>
			</div>
		</section>
	</main>

	<Footer />
</div>
