<script lang="ts">
	import SignalScanner from '$lib/components/SignalScanner.svelte';
	
	let showStreamGuide = false;
	
	const baseUrl = typeof window !== 'undefined' ? window.location.origin : 'http://localhost:5173';
	
	const widgetExamples = [
		{ name: 'Full Dashboard', url: '/stream?widget=dashboard', description: 'Complete stream overlay (1920x1080)' },
		{ name: 'Market Countdown', url: '/stream?widget=countdown', description: 'Time until market open/close' },
		{ name: 'Fear & Greed Index', url: '/stream?widget=feargreed', description: 'Market sentiment meter' },
		{ name: 'VIX Meter', url: '/stream?widget=vix', description: 'Volatility index display' },
		{ name: 'Rocket Signals', url: '/stream?widget=signals', description: 'Live trading signals' },
		{ name: 'Market Heatmap', url: '/stream?widget=heatmap', description: 'Sector performance heatmap' },
		{ name: 'Crypto Ticker', url: '/stream?widget=crypto', description: 'Cryptocurrency prices' },
		{ name: 'News Headlines', url: '/stream?widget=news', description: 'Latest market news' },
		{ name: 'Top Movers', url: '/stream?widget=movers', description: 'Biggest gainers/losers' },
		{ name: 'Live Clock', url: '/stream?widget=clock', description: 'Current time display' },
		{ name: 'Viewer Count', url: '/stream?widget=viewers', description: 'Live viewer counter' },
		{ name: 'Music Visualizer', url: '/stream?widget=visualizer', description: 'Audio visualization' },
		{ name: 'Donation Alert', url: '/stream?widget=donations', description: 'Donation notifications' },
		{ name: 'Donation Feed', url: '/stream?widget=donation-feed', description: 'Recent donations list' },
		{ name: 'Donation Tiers', url: '/stream?widget=donation-tiers', description: 'Donation tier information' },
		{ name: 'Donation Goal', url: '/stream?widget=donation-goal', description: 'Goal progress tracker' },
		{ name: 'Donor Leaderboard', url: '/stream?widget=leaderboard', description: 'Top donors ranking' },
		{ name: 'Donation Links', url: '/stream?widget=donation-links', description: 'Ways to donate' },
		{ name: 'Sponsor Banner', url: '/stream?widget=sponsor', description: 'Sponsor advertisements' },
		{ name: 'Social Links', url: '/stream?widget=social', description: 'Social media links' },
		{ name: 'Chat Overlay', url: '/stream?widget=chat', description: 'Live chat messages' },
		{ name: 'Stream Schedule', url: '/stream?widget=schedule', description: 'Upcoming streams' },
		{ name: 'Song Queue', url: '/stream?widget=songs', description: 'Music requests' },
		{ name: 'Bacon Logo', url: '/stream?widget=logo', description: 'BaconAlgo logo' }
	];
	
	function copyToClipboard(text: string) {
		navigator.clipboard.writeText(baseUrl + text);
	}
</script>

<div class="page-container">
	<!-- Stream Setup Button -->
	<div class="stream-setup-header">
		<button class="stream-guide-toggle" on:click={() => showStreamGuide = !showStreamGuide}>
			{showStreamGuide ? '📊 Hide Stream Setup' : '🎥 OBS Stream Setup'}
		</button>
	</div>
	
	{#if showStreamGuide}
		<div class="stream-guide">
			<h1>🚀 BaconAlgo OBS Stream Setup</h1>
			
			<section class="setup-section">
				<h2>Quick Start Guide</h2>
				<ol>
					<li>Open OBS Studio</li>
					<li>Add a new <strong>Browser Source</strong></li>
					<li>Copy one of the URLs below</li>
					<li>Set width/height based on widget (Full Dashboard: 1920x1080)</li>
					<li>Enable "Shutdown source when not visible" for better performance</li>
					<li>Optional: Use Chroma Key filter if widget has background</li>
				</ol>
			</section>
			
			<section class="setup-section">
				<h2>📋 Available Widgets</h2>
				<div class="widget-grid">
					{#each widgetExamples as widget}
						<div class="widget-card">
							<div class="widget-header">
								<h3>{widget.name}</h3>
								<p class="widget-description">{widget.description}</p>
							</div>
							<div class="widget-url">
								<code>{baseUrl}{widget.url}</code>
								<button 
									class="copy-btn"
									on:click={() => copyToClipboard(widget.url)}
									title="Copy URL"
								>
									📋
								</button>
							</div>
							<a 
								href={widget.url} 
								target="_blank" 
								rel="noopener noreferrer"
								class="preview-link"
							>
								Preview →
							</a>
						</div>
					{/each}
				</div>
			</section>
			
			<section class="setup-section">
				<h2>💡 Pro Tips</h2>
				<ul class="tips-list">
					<li><strong>Performance:</strong> Use individual widgets instead of full dashboard when possible</li>
					<li><strong>Positioning:</strong> Individual widgets have no padding - position them freely in OBS</li>
					<li><strong>Updates:</strong> Widgets auto-refresh with real-time data</li>
					<li><strong>Transparency:</strong> Most widgets have transparent backgrounds for easy overlay</li>
					<li><strong>Testing:</strong> Preview widgets in browser before adding to OBS</li>
				</ul>
			</section>
		</div>
	{/if}
	
	<!-- Main Scanner -->
	<SignalScanner />
</div>

<style>
	.page-container {
		min-height: 100vh;
		background: linear-gradient(135deg, #0a0e27 0%, #1a1f3a 100%);
	}
	
	.stream-setup-header {
		position: sticky;
		top: 0;
		z-index: 1000;
		padding: 1rem;
		background: rgba(10, 14, 39, 0.95);
		backdrop-filter: blur(10px);
		border-bottom: 1px solid rgba(255, 107, 53, 0.2);
	}
	
	.stream-guide-toggle {
		background: linear-gradient(135deg, #ff6b35 0%, #f7931e 100%);
		color: white;
		border: none;
		padding: 0.75rem 1.5rem;
		font-size: 1rem;
		font-weight: 600;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.3s ease;
		box-shadow: 0 4px 15px rgba(255, 107, 53, 0.3);
	}
	
	.stream-guide-toggle:hover {
		transform: translateY(-2px);
		box-shadow: 0 6px 20px rgba(255, 107, 53, 0.4);
	}
	
	.stream-guide {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
		color: white;
	}
	
	.stream-guide h1 {
		font-size: 2.5rem;
		margin-bottom: 2rem;
		background: linear-gradient(135deg, #ff6b35 0%, #f7931e 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}
	
	.setup-section {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 16px;
		padding: 2rem;
		margin-bottom: 2rem;
	}
	
	.setup-section h2 {
		color: #ff6b35;
		margin-bottom: 1.5rem;
		font-size: 1.75rem;
	}
	
	.setup-section ol,
	.tips-list {
		line-height: 1.8;
		font-size: 1.1rem;
	}
	
	.setup-section ol li {
		margin-bottom: 0.5rem;
	}
	
	.tips-list {
		list-style: none;
		padding: 0;
	}
	
	.tips-list li {
		padding: 0.75rem;
		background: rgba(255, 255, 255, 0.05);
		border-left: 3px solid #ff6b35;
		margin-bottom: 0.75rem;
		border-radius: 4px;
	}
	
	.widget-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
		gap: 1.5rem;
		margin-top: 1.5rem;
	}
	
	.widget-card {
		background: rgba(255, 255, 255, 0.08);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		padding: 1.5rem;
		transition: all 0.3s ease;
	}
	
	.widget-card:hover {
		background: rgba(255, 255, 255, 0.12);
		border-color: rgba(255, 107, 53, 0.4);
		transform: translateY(-4px);
		box-shadow: 0 8px 20px rgba(255, 107, 53, 0.2);
	}
	
	.widget-header h3 {
		color: #ff6b35;
		margin: 0 0 0.5rem 0;
		font-size: 1.25rem;
	}
	
	.widget-description {
		color: rgba(255, 255, 255, 0.7);
		font-size: 0.9rem;
		margin: 0 0 1rem 0;
	}
	
	.widget-url {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		background: rgba(0, 0, 0, 0.3);
		padding: 0.75rem;
		border-radius: 6px;
		margin-bottom: 1rem;
	}
	
	.widget-url code {
		flex: 1;
		color: #4af;
		font-size: 0.85rem;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}
	
	.copy-btn {
		background: rgba(255, 107, 53, 0.2);
		border: 1px solid rgba(255, 107, 53, 0.4);
		color: white;
		padding: 0.5rem 0.75rem;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.2s ease;
		font-size: 1rem;
	}
	
	.copy-btn:hover {
		background: rgba(255, 107, 53, 0.4);
		border-color: rgba(255, 107, 53, 0.6);
	}
	
	.preview-link {
		display: inline-block;
		color: #4af;
		text-decoration: none;
		font-weight: 500;
		transition: color 0.2s ease;
	}
	
	.preview-link:hover {
		color: #6cf;
		text-decoration: underline;
	}
</style>
