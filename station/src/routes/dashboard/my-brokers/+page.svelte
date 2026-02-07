<script lang="ts">
	import { AlpacaBroker } from '$lib/brokers/alpaca';
	import { InteractiveBrokersBroker } from '$lib/brokers/ib';
	import { QuestradeBroker } from '$lib/brokers/questrade';
	import { BitgetBroker } from '$lib/brokers/bitget';
	import { userSettingsStore, type BrokerConfig } from '$lib/stores/userSettings';

	let brokers = $state<BrokerConfig[]>([]);
	let showAddModal = $state(false);
	let selectedBrokerType = $state<'alpaca' | 'ib' | 'questrade' | 'bitget'>('alpaca');
	let apiKey = $state('');
	let apiSecret = $state('');
	let isPaper = $state(true);

	$effect(() => {
		// Subscribe to user settings
		userSettingsStore.subscribe(settings => {
			brokers = settings.brokers;
		});
	});

	async function addBroker() {
		if (!apiKey || !apiSecret) {
			alert('Please enter API credentials');
			return;
		}

		const newBroker: BrokerConfig = {
			name: selectedBrokerType,
			apiKey,
			apiSecret,
			isPaper,
			isConnected: false
		};

		// Test connection
		let broker;
		switch (selectedBrokerType) {
			case 'alpaca':
				broker = new AlpacaBroker();
				break;
			case 'ib':
				broker = new InteractiveBrokersBroker();
				break;
			case 'questrade':
				broker = new QuestradeBroker();
				break;
			case 'bitget':
				broker = new BitgetBroker();
				break;
		}

		const connected = await broker.connect({ apiKey, apiSecret, isPaper });
		newBroker.isConnected = connected;

		userSettingsStore.updateBroker(newBroker);
		
		showAddModal = false;
		apiKey = '';
		apiSecret = '';
		
		if (connected) {
			alert('✅ Broker connected successfully!');
		} else {
			alert('❌ Failed to connect. Please check your credentials.');
		}
	}

	function removeBroker(name: string) {
		if (confirm(`Remove ${name} broker?`)) {
			userSettingsStore.removeBroker(name);
		}
	}
</script>

<div class="brokers-page">
	<header class="page-header">
		<div>
			<h1>🏦 My Brokers</h1>
			<p class="subtitle">Connect and manage your trading brokers</p>
		</div>
		<button class="add-btn" onclick={() => showAddModal = true}>
			➕ Add Broker
		</button>
	</header>

	<!-- Connected Brokers -->
	<div class="section">
		<h2 class="section-title">Connected Brokers</h2>
		{#if brokers.length === 0}
			<div class="empty-state">
				<div class="empty-icon">🏦</div>
				<h3>No Brokers Connected</h3>
				<p>Connect a broker to start trading</p>
				<button class="btn-primary" onclick={() => showAddModal = true}>Add Your First Broker</button>
			</div>
		{:else}
			<div class="brokers-grid">
				{#each brokers as broker}
					<div class="broker-card">
						<div class="broker-header">
							<div class="broker-name">
								{#if broker.name === 'alpaca'}
									📈 Alpaca
								{:else if broker.name === 'ib'}
									📊 Interactive Brokers
								{:else if broker.name === 'questrade'}
									🇨🇦 Questrade
								{:else}
									₿ Bitget
								{/if}
							</div>
							<button class="remove-btn" onclick={() => removeBroker(broker.name)}>
								❌
							</button>
						</div>
						<div class="broker-status">
							<span class="status-dot" class:connected={broker.isConnected}></span>
							<span>{broker.isConnected ? 'Connected' : 'Disconnected'}</span>
						</div>
						<div class="broker-mode">
							{broker.isPaper ? '📄 Paper Trading' : '💰 Live Trading'}
						</div>
						<div class="broker-actions">
							<button class="btn-secondary">⚙️ Settings</button>
							<button class="btn-secondary">📊 View Account</button>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</div>

	<!-- Supported Brokers -->
	<div class="section">
		<h2 class="section-title">Supported Brokers</h2>
		<div class="supported-grid">
			<div class="supported-card">
				<div class="supported-icon">📈</div>
				<div class="supported-name">Alpaca</div>
				<div class="supported-desc">Commission-free US stocks & crypto</div>
				<div class="supported-status implemented">✅ Implemented</div>
			</div>

			<div class="supported-card">
				<div class="supported-icon">📊</div>
				<div class="supported-name">Interactive Brokers</div>
				<div class="supported-desc">Global markets & advanced tools</div>
				<div class="supported-status planned">🔜 Planned</div>
			</div>

			<div class="supported-card">
				<div class="supported-icon">🇨🇦</div>
				<div class="supported-name">Questrade</div>
				<div class="supported-desc">Canadian stocks & ETFs</div>
				<div class="supported-status planned">🔜 Planned</div>
			</div>

			<div class="supported-card">
				<div class="supported-icon">₿</div>
				<div class="supported-name">Bitget</div>
				<div class="supported-desc">Crypto futures & spot trading</div>
				<div class="supported-status planned">🔜 Planned</div>
			</div>
		</div>
	</div>
</div>

<!-- Add Broker Modal -->
{#if showAddModal}
	<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
	<div class="modal-overlay" role="button" tabindex="-1" onclick={() => showAddModal = false} onkeydown={(e) => { if (e.key === 'Escape') showAddModal = false; }}>
		<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
		<div class="modal" role="dialog" aria-modal="true" aria-labelledby="modal-title" tabindex="0" onclick={(e) => e.stopPropagation()} onkeydown={(e) => { if (e.key === 'Escape') showAddModal = false; e.stopPropagation(); }}>
			<h2 id="modal-title">Add Broker</h2>
			
			<div class="form-group">
				<label>Broker
					<select bind:value={selectedBrokerType} class="select">
						<option value="alpaca">Alpaca</option>
						<option value="ib">Interactive Brokers</option>
						<option value="questrade">Questrade</option>
						<option value="bitget">Bitget</option>
					</select>
				</label>
			</div>

			<div class="form-group">
				<label>API Key
					<input type="text" bind:value={apiKey} class="input" placeholder="Enter your API key" />
				</label>
			</div>

			<div class="form-group">
				<label>API Secret
					<input type="password" bind:value={apiSecret} class="input" placeholder="Enter your API secret" />
				</label>
			</div>

			<div class="form-group">
				<label class="checkbox-label">
					<input type="checkbox" bind:checked={isPaper} />
					<span>Paper Trading (Recommended for testing)</span>
				</label>
			</div>

			<div class="modal-actions">
				<button class="btn-secondary" onclick={() => showAddModal = false}>Cancel</button>
				<button class="btn-primary" onclick={addBroker}>Connect Broker</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.brokers-page {
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

	.add-btn {
		padding: 0.75rem 1.5rem;
		background: linear-gradient(135deg, #ff6b35 0%, #fbbf24 100%);
		border: none;
		border-radius: 0.5rem;
		color: #fff;
		font-weight: 700;
		cursor: pointer;
		transition: all 0.3s;
	}

	.add-btn:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 25px rgba(255, 107, 53, 0.4);
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

	.empty-state {
		text-align: center;
		padding: 4rem 2rem;
		background: rgba(255, 255, 255, 0.03);
		border: 2px dashed rgba(255, 255, 255, 0.1);
		border-radius: 1rem;
	}

	.empty-icon {
		font-size: 4rem;
		margin-bottom: 1rem;
	}

	.empty-state h3 {
		font-size: 1.5rem;
		color: #f9fafb;
		margin-bottom: 0.5rem;
	}

	.empty-state p {
		color: #9ca3af;
		margin-bottom: 2rem;
	}

	.btn-primary, .btn-secondary {
		padding: 0.75rem 1.5rem;
		border-radius: 0.5rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
	}

	.btn-primary {
		background: linear-gradient(135deg, #ff6b35 0%, #fbbf24 100%);
		color: #fff;
	}

	.btn-secondary {
		background: rgba(255, 255, 255, 0.05);
		color: #e5e7eb;
		border: 1px solid rgba(255, 255, 255, 0.1);
	}

	.brokers-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
		gap: 1.5rem;
	}

	.broker-card {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 1rem;
		padding: 1.5rem;
	}

	.broker-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.broker-name {
		font-size: 1.25rem;
		font-weight: 700;
		color: #f9fafb;
	}

	.remove-btn {
		background: none;
		border: none;
		cursor: pointer;
		font-size: 1rem;
		opacity: 0.5;
		transition: opacity 0.2s;
	}

	.remove-btn:hover {
		opacity: 1;
	}

	.broker-status {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.75rem;
		font-size: 0.875rem;
		color: #9ca3af;
	}

	.status-dot {
		width: 10px;
		height: 10px;
		border-radius: 50%;
		background: #6b7280;
	}

	.status-dot.connected {
		background: #10b981;
		animation: pulse-dot 2s ease-in-out infinite;
	}

	@keyframes pulse-dot {
		0%, 100% { box-shadow: 0 0 0 0 rgba(16, 185, 129, 0.7); }
		50% { box-shadow: 0 0 10px 5px rgba(16, 185, 129, 0); }
	}

	.broker-mode {
		font-size: 0.875rem;
		color: #fbbf24;
		margin-bottom: 1rem;
	}

	.broker-actions {
		display: flex;
		gap: 0.5rem;
	}

	.broker-actions .btn-secondary {
		flex: 1;
		font-size: 0.875rem;
		padding: 0.625rem;
	}

	.supported-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
		gap: 1.5rem;
	}

	.supported-card {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.75rem;
		padding: 1.5rem;
		text-align: center;
	}

	.supported-icon {
		font-size: 3rem;
		margin-bottom: 0.75rem;
	}

	.supported-name {
		font-size: 1.125rem;
		font-weight: 700;
		color: #f9fafb;
		margin-bottom: 0.5rem;
	}

	.supported-desc {
		font-size: 0.875rem;
		color: #9ca3af;
		margin-bottom: 1rem;
	}

	.supported-status {
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 700;
		display: inline-block;
	}

	.supported-status.implemented {
		background: rgba(16, 185, 129, 0.2);
		color: #10b981;
	}

	.supported-status.planned {
		background: rgba(251, 191, 36, 0.2);
		color: #fbbf24;
	}

	.modal-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(0, 0, 0, 0.8);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
	}

	.modal {
		background: #1e293b;
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 1rem;
		padding: 2rem;
		max-width: 500px;
		width: 90%;
	}

	.modal h2 {
		color: #f9fafb;
		margin: 0 0 1.5rem 0;
	}

	.form-group {
		margin-bottom: 1.5rem;
	}

	.form-group label {
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

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
	}

	.checkbox-label input[type="checkbox"] {
		width: auto;
	}

	.modal-actions {
		display: flex;
		gap: 1rem;
		justify-content: flex-end;
	}

	@media (max-width: 768px) {
		.brokers-grid, .supported-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
