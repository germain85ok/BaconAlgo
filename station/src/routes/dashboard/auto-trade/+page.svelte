<script lang="ts">
	import { onMount } from 'svelte';
	import { getCurrentUser } from '$lib/services/supabase';
	import {
		getAutoTradeSettings,
		upsertAutoTradeSettings,
		activateKillSwitch,
		deactivateKillSwitch,
		getBrokerConnections,
		getTradeStats,
		type AutoTradeSettings,
		type BrokerConnection
	} from '$lib/services/autoTrade';
	import GlassCard from '$lib/components/ui/GlassCard.svelte';

	let userId = $state<string | null>(null);
	let settings = $state<AutoTradeSettings | null>(null);
	let brokerConnections = $state<BrokerConnection[]>([]);
	let tradeStats = $state<any>(null);
	let loading = $state(true);
	let saving = $state(false);

	// Form state
	let formData = $state({
		enabled: false,
		mode: 'paper' as 'paper' | 'semi-auto' | 'full-auto',
		broker_connection_id: null as string | null,
		max_daily_loss: 500,
		max_position_size_percent: 10,
		max_concurrent_positions: 5,
		auto_execute_grades: ['S', 'A'] as string[],
		symbol_blacklist: [] as string[],
		trading_hours_start: null as string | null,
		trading_hours_end: null as string | null,
		discord_webhook_url: null as string | null,
		telegram_chat_id: null as string | null,
		email_notifications: true,
		kill_switch_activated: false
	});

	let symbolBlacklistInput = $state('');
	let selectedBroker = $state('');
	let maxPositionSize = $state(1000);
	let maxDailyLoss = $state(500);
	let minSignalScore = $state(70);
	let allowedSymbols = $state('');

	const gradeOptions = ['S', 'A', 'B', 'C'];

	onMount(async () => {
		await loadData();
	});

	async function loadData() {
		loading = true;
		const { user } = await getCurrentUser();
		if (!user) {
			loading = false;
			return;
		}

		userId = user.id;

		// Load settings
		const { data: settingsData } = await getAutoTradeSettings(user.id);
		if (settingsData) {
			settings = settingsData;
			formData = {
				enabled: settingsData.enabled,
				mode: settingsData.mode,
				broker_connection_id: settingsData.broker_connection_id,
				max_daily_loss: settingsData.max_daily_loss,
				max_position_size_percent: settingsData.max_position_size_percent,
				max_concurrent_positions: settingsData.max_concurrent_positions,
				auto_execute_grades: settingsData.auto_execute_grades || ['S', 'A'],
				symbol_blacklist: settingsData.symbol_blacklist || [],
				trading_hours_start: settingsData.trading_hours_start,
				trading_hours_end: settingsData.trading_hours_end,
				discord_webhook_url: settingsData.discord_webhook_url,
				telegram_chat_id: settingsData.telegram_chat_id,
				email_notifications: settingsData.email_notifications,
				kill_switch_activated: settingsData.kill_switch_activated
			};
			symbolBlacklistInput = settingsData.symbol_blacklist.join(', ');
		}

		// Load broker connections
		const { data: brokersData } = await getBrokerConnections(user.id);
		if (brokersData) {
			brokerConnections = brokersData;
		}

		// Load trade stats
		const stats = await getTradeStats(user.id);
		tradeStats = stats;

		loading = false;
	}
</script>

<div class="auto-trade-page">
	<header class="page-header">
		<div>
			<h1>🤖 Auto-Trading</h1>
			<p class="subtitle">Automate your trading based on signals</p>
		</div>
		<button class="toggle-btn" class:active={autoTradingEnabled} onclick={toggleAutoTrading}>
			{autoTradingEnabled ? '🟢 ENABLED' : '🔴 DISABLED'}
		</button>
	</header>

	<!-- Configuration Panel -->
	<div class="section">
		<h2 class="section-title">⚙️ Configuration</h2>
		<div class="config-grid">
			<div class="config-card">
				<label class="label">Broker
					<select bind:value={selectedBroker} class="select">
						<option value="">Select Broker</option>
						<option value="alpaca">Alpaca</option>
						<option value="ib">Interactive Brokers</option>
						<option value="questrade">Questrade</option>
						<option value="bitget">Bitget</option>
					</select>
				</label>
			</div>

			<div class="config-card">
				<label class="label">Max Position Size
					<input type="number" bind:value={maxPositionSize} class="input" />
				</label>
				<span class="help-text">Maximum $ per position</span>
			</div>

			<div class="config-card">
				<label class="label">Max Daily Loss
					<input type="number" bind:value={maxDailyLoss} class="input" />
				</label>
				<span class="help-text">Stop trading after this loss</span>
			</div>

			<div class="config-card">
				<label class="label">Min Signal Score
					<input type="number" bind:value={minSignalScore} min="0" max="100" class="input" />
				</label>
				<span class="help-text">Only trade signals above this score</span>
			</div>
		</div>

		<div class="config-card full-width">
			<label class="label">Allowed Symbols (comma-separated)
				<input type="text" bind:value={allowedSymbols} class="input" placeholder="SPY,QQQ,NVDA,TSLA" />
			</label>
			<span class="help-text">Only trade these symbols</span>
		</div>

		<button class="save-btn" onclick={saveSettings}>💾 Save Settings</button>
	</div>

	<!-- Trading Rules -->
	<div class="section">
		<h2 class="section-title">📋 Trading Rules</h2>
		<div class="rules-grid">
			<div class="rule-card">
				<div class="rule-icon">✅</div>
				<div class="rule-name">Signal Quality</div>
				<div class="rule-value">Score ≥ {minSignalScore}</div>
			</div>

			<div class="rule-card">
				<div class="rule-icon">💰</div>
				<div class="rule-name">Position Sizing</div>
				<div class="rule-value">Max ${maxPositionSize.toLocaleString()}</div>
			</div>

			<div class="rule-card">
				<div class="rule-icon">🛑</div>
				<div class="rule-name">Daily Loss Limit</div>
				<div class="rule-value">-${maxDailyLoss.toLocaleString()}</div>
			</div>

			<div class="rule-card">
				<div class="rule-icon">📊</div>
				<div class="rule-name">Risk/Reward</div>
				<div class="rule-value">Min 2:1</div>
			</div>

			<div class="rule-card">
				<div class="rule-icon">⏰</div>
				<div class="rule-name">Trading Hours</div>
				<div class="rule-value">9:30 AM - 4:00 PM EST</div>
			</div>

			<div class="rule-card">
				<div class="rule-icon">🎯</div>
				<div class="rule-name">Max Positions</div>
				<div class="rule-value">5 concurrent</div>
			</div>
		</div>
	</div>

	<!-- Status -->
	<div class="section">
		<h2 class="section-title">📊 Status</h2>
		<div class="status-card">
			<div class="status-item">
				<span class="status-label">Status:</span>
				<span class="status-value" class:active={autoTradingEnabled}>
					{autoTradingEnabled ? '🟢 Active' : '🔴 Inactive'}
				</span>
			</div>
			<div class="status-item">
				<span class="status-label">Broker Connection:</span>
				<span class="status-value">{selectedBroker || 'Not configured'}</span>
			</div>
			<div class="status-item">
				<span class="status-label">Today's Trades:</span>
				<span class="status-value">0</span>
			</div>
			<div class="status-item">
				<span class="status-label">Today's P&L:</span>
				<span class="status-value positive">$0.00</span>
			</div>
		</div>
	</div>
</div>

<style>
	.auto-trade-page {
		max-width: 1400px;
		margin: 0 auto;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2rem;
	}

	h1 {
		font-size: 2rem;
		font-weight: 700;
		color: #f9fafb;
		margin: 0;
	}

	.subtitle {
		color: #9ca3af;
		margin: 0.5rem 0 0 0;
	}

	.toggle-btn {
		padding: 1rem 2rem;
		border: 2px solid;
		border-radius: 0.5rem;
		font-weight: 700;
		font-size: 1rem;
		cursor: pointer;
		transition: all 0.3s;
	}

	.toggle-btn:not(.active) {
		background: rgba(239, 68, 68, 0.1);
		border-color: #ef4444;
		color: #ef4444;
	}

	.toggle-btn.active {
		background: linear-gradient(135deg, #10b981 0%, #059669 100%);
		border-color: #10b981;
		color: #fff;
		animation: pulse-green 2s ease-in-out infinite;
	}

	@keyframes pulse-green {
		0%, 100% { box-shadow: 0 0 0 0 rgba(16, 185, 129, 0.7); }
		50% { box-shadow: 0 0 20px 10px rgba(16, 185, 129, 0); }
	}

	.section {
		margin-bottom: 3rem;
	}

	.section-title {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f9fafb;
		margin-bottom: 1.5rem;
	}

	.config-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		gap: 1.5rem;
		margin-bottom: 1.5rem;
	}

	.config-card {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.75rem;
		padding: 1.25rem;
	}

	.config-card.full-width {
		grid-column: 1 / -1;
	}

	.label {
		display: block;
		font-size: 0.875rem;
		font-weight: 600;
		color: #e5e7eb;
		margin-bottom: 0.5rem;
	}

	.input, .select {
		width: 100%;
		padding: 0.75rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.5rem;
		color: #f9fafb;
		font-size: 1rem;
	}

	.help-text {
		display: block;
		font-size: 0.75rem;
		color: #6b7280;
		margin-top: 0.5rem;
	}

	.save-btn {
		padding: 0.875rem 2rem;
		background: linear-gradient(135deg, #ff6b35 0%, #fbbf24 100%);
		border: none;
		border-radius: 0.5rem;
		color: #fff;
		font-weight: 700;
		font-size: 1rem;
		cursor: pointer;
		transition: all 0.3s;
	}

	.save-btn:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 25px rgba(255, 107, 53, 0.4);
	}

	.rules-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
		gap: 1rem;
	}

	.rule-card {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.75rem;
		padding: 1.25rem;
		text-align: center;
	}

	.rule-icon {
		font-size: 2rem;
		margin-bottom: 0.75rem;
	}

	.rule-name {
		font-size: 0.875rem;
		color: #9ca3af;
		margin-bottom: 0.5rem;
	}

	.rule-value {
		font-size: 1.125rem;
		font-weight: 700;
		color: #f9fafb;
	}

	.status-card {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.75rem;
		padding: 1.5rem;
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		gap: 1.5rem;
	}

	.status-item {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.status-label {
		color: #9ca3af;
		font-weight: 600;
	}

	.status-value {
		color: #f9fafb;
		font-weight: 700;
	}

	.status-value.active {
		color: #10b981;
	}

	.status-value.positive {
		color: #10b981;
	}

	@media (max-width: 768px) {
		.config-grid, .rules-grid {
			grid-template-columns: 1fr;
		}

		.status-card {
			grid-template-columns: 1fr;
		}
	}
</style>
