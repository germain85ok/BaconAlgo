<script lang="ts">
import { onMount } from 'svelte';
import SignalCard from '$lib/components/dashboard/SignalCard.svelte';
import FilterSidebar from '$lib/components/dashboard/FilterSidebar.svelte';
import type { Signal } from '$lib/components/dashboard/SignalCard.svelte';

interface Position {
id: string;
symbol: string;
entry: number;
currentPrice: number;
quantity: number;
pnl: number;
pnlPercent: number;
}

interface PaperPosition extends Position {
side: 'long' | 'short';
stopLoss: number;
takeProfit: number;
status: 'open' | 'closed';
openedAt: string;
closedAt?: string;
}

let activeTab = $state<'scanner' | 'portfolio' | 'paper' | 'auto' | 'analytics'>('scanner');
let signals = $state<Signal[]>([]);
let filters = $state<any>({
horizon: 'all',
direction: 'all',
minScore: 0,
minRR: 0,
assetTypes: { stocks: true, crypto: true, forex: true, futures: true },
smcFilters: {
nearFVG: false,
nearOB: false,
bosConfirmed: false,
choch: false,
liquidity: false,
imbalance: false
}
});

// Portfolio state
let portfolio = $state<Position[]>([
{
id: '1',
symbol: 'AAPL',
entry: 170.0,
currentPrice: 175.5,
quantity: 100,
pnl: 550,
pnlPercent: 3.24
},
{
id: '2',
symbol: 'TSLA',
entry: 250.0,
currentPrice: 245.0,
quantity: 50,
pnl: -250,
pnlPercent: -2.0
}
]);

// Paper trading state
let paperPositions = $state<PaperPosition[]>([
{
id: '1',
symbol: 'NVDA',
side: 'long',
entry: 720.0,
currentPrice: 735.0,
quantity: 10,
stopLoss: 710.0,
takeProfit: 750.0,
pnl: 150,
pnlPercent: 2.08,
status: 'open',
openedAt: new Date(Date.now() - 2 * 60 * 60 * 1000).toISOString()
}
]);

// Auto-trade state
let autoTradeEnabled = $state(false);
let brokerConnected = $state(false);
let maxPositionSize = $state(5000);
let dailyLossLimit = $state(500);

const totalPortfolioValue = $derived(
portfolio.reduce((sum, p) => sum + p.currentPrice * p.quantity, 0)
);

const totalPortfolioPnL = $derived(portfolio.reduce((sum, p) => sum + p.pnl, 0));

const paperStats = $derived({
openPositions: paperPositions.filter((p) => p.status === 'open').length,
totalPnL: paperPositions.reduce((sum, p) => sum + p.pnl, 0),
winRate: 75.5
});

const handleAddPosition = (e: Event) => {
e.preventDefault();
alert('Position added!');
};

const handleOpenPaperTrade = (e: Event) => {
e.preventDefault();
alert('Paper trade opened!');
};

const handleConnectBroker = () => {
brokerConnected = !brokerConnected;
};
</script>

<div class="station-dashboard">
<!-- Plan Badge -->
<div class="plan-badge-header">
<div class="badge">
<span class="badge-icon">🚀</span>
<span class="badge-text">Station Plan - Ultimate Access</span>
</div>
</div>

<!-- Tab Navigation -->
<div class="tab-nav">
<button
class="tab-btn"
class:active={activeTab === 'scanner'}
onclick={() => (activeTab = 'scanner')}
>
🔍 Scanner
</button>
<button
class="tab-btn"
class:active={activeTab === 'portfolio'}
onclick={() => (activeTab = 'portfolio')}
>
💼 Portfolio
</button>
<button
class="tab-btn"
class:active={activeTab === 'paper'}
onclick={() => (activeTab = 'paper')}
>
📄 Paper Trading
</button>
<button
class="tab-btn"
class:active={activeTab === 'auto'}
onclick={() => (activeTab = 'auto')}
>
🤖 Auto-Trade
</button>
<button
class="tab-btn"
class:active={activeTab === 'analytics'}
onclick={() => (activeTab = 'analytics')}
>
📊 Analytics
</button>
</div>

<!-- Tab Content -->
<div class="tab-content">
{#if activeTab === 'scanner'}
<div class="scanner-tab">
<h2>Full Signal Scanner</h2>
<div class="scanner-layout">
<div class="sidebar-container">
<FilterSidebar bind:filters />
</div>
<div class="signals-area">
<p class="info-text">Real-time scanner with unlimited signals and advanced filters</p>
<div class="signals-grid">
<!-- Mock signals -->
<div class="placeholder-card">
<p>Scanner signals would appear here</p>
<small>Connected to real-time feed</small>
</div>
</div>
</div>
</div>
</div>
{:else if activeTab === 'portfolio'}
<div class="portfolio-tab">
<h2>Portfolio Tracker</h2>

<!-- Portfolio Stats -->
<div class="portfolio-stats">
<div class="stat-card">
<div class="stat-label">Total Value</div>
<div class="stat-value">${totalPortfolioValue.toFixed(2)}</div>
</div>
<div class="stat-card">
<div class="stat-label">Total P&L</div>
<div class="stat-value" class:profit={totalPortfolioPnL >= 0} class:loss={totalPortfolioPnL < 0}>
${totalPortfolioPnL >= 0 ? '+' : ''}{totalPortfolioPnL.toFixed(2)}
</div>
</div>
<div class="stat-card">
<div class="stat-label">Positions</div>
<div class="stat-value">{portfolio.length}</div>
</div>
</div>

<!-- Add Position Form -->
<div class="add-position-form">
<h3>Add Position</h3>
<form onsubmit={handleAddPosition}>
<div class="form-row">
<input type="text" placeholder="Symbol" required />
<input type="number" placeholder="Entry Price" step="0.01" required />
<input type="number" placeholder="Quantity" required />
<button type="submit" class="add-btn">Add Position</button>
</div>
</form>
</div>

<!-- Position Cards -->
<div class="positions-grid">
{#each portfolio as position}
<div class="position-card">
<div class="position-header">
<h4>{position.symbol}</h4>
<span class="position-qty">{position.quantity} shares</span>
</div>
<div class="position-prices">
<div class="price-item">
<span class="label">Entry</span>
<span class="value">${position.entry.toFixed(2)}</span>
</div>
<div class="price-item">
<span class="label">Current</span>
<span class="value">${position.currentPrice.toFixed(2)}</span>
</div>
</div>
<div class="position-pnl">
<div class="pnl-value" class:profit={position.pnl >= 0} class:loss={position.pnl < 0}>
${position.pnl >= 0 ? '+' : ''}{position.pnl.toFixed(2)}
</div>
<div class="pnl-percent" class:profit={position.pnlPercent >= 0} class:loss={position.pnlPercent < 0}>
{position.pnlPercent >= 0 ? '+' : ''}{position.pnlPercent.toFixed(2)}%
</div>
</div>
</div>
{/each}
</div>
</div>
{:else if activeTab === 'paper'}
<div class="paper-tab">
<h2>Paper Trading</h2>

<!-- Paper Stats -->
<div class="paper-stats">
<div class="stat-card">
<div class="stat-label">Open Positions</div>
<div class="stat-value">{paperStats.openPositions}</div>
</div>
<div class="stat-card">
<div class="stat-label">Total P&L</div>
<div class="stat-value" class:profit={paperStats.totalPnL >= 0} class:loss={paperStats.totalPnL < 0}>
${paperStats.totalPnL >= 0 ? '+' : ''}{paperStats.totalPnL.toFixed(2)}
</div>
</div>
<div class="stat-card">
<div class="stat-label">Win Rate</div>
<div class="stat-value">{paperStats.winRate.toFixed(1)}%</div>
</div>
</div>

<!-- Open Trade Form -->
<div class="paper-form">
<h3>Open Paper Trade</h3>
<form onsubmit={handleOpenPaperTrade}>
<div class="form-grid">
<input type="text" placeholder="Symbol" required />
<select required>
<option value="">Side</option>
<option value="long">Long</option>
<option value="short">Short</option>
</select>
<input type="number" placeholder="Entry Price" step="0.01" required />
<input type="number" placeholder="Quantity" required />
<input type="number" placeholder="Stop Loss" step="0.01" />
<input type="number" placeholder="Take Profit" step="0.01" />
</div>
<button type="submit" class="open-trade-btn">Open Trade</button>
</form>
</div>

<!-- Active Positions -->
<h3>Active Positions</h3>
<div class="paper-positions">
{#each paperPositions.filter((p) => p.status === 'open') as position}
<div class="paper-position-card">
<div class="position-header">
<h4>{position.symbol}</h4>
<span class="side-badge {position.side}">{position.side.toUpperCase()}</span>
</div>
<div class="position-details">
<div class="detail">
<span class="label">Entry</span>
<span class="value">${position.entry.toFixed(2)}</span>
</div>
<div class="detail">
<span class="label">Current</span>
<span class="value">${position.currentPrice.toFixed(2)}</span>
</div>
<div class="detail">
<span class="label">P&L</span>
<span class="value" class:profit={position.pnl >= 0} class:loss={position.pnl < 0}>
${position.pnl >= 0 ? '+' : ''}{position.pnl.toFixed(2)}
</span>
</div>
</div>
<button class="close-position-btn">Close Position</button>
</div>
{/each}
</div>
</div>
{:else if activeTab === 'auto'}
<div class="auto-tab">
<h2>Auto-Trading</h2>

<!-- Broker Connection -->
<div class="broker-section">
<h3>Broker Connection</h3>
<div class="broker-cards">
<div class="broker-card">
<div class="broker-icon">🏦</div>
<h4>Interactive Brokers</h4>
<button class="connect-btn" onclick={handleConnectBroker}>
{brokerConnected ? '✓ Connected' : 'Connect'}
</button>
</div>
<div class="broker-card">
<div class="broker-icon">₿</div>
<h4>Bitget</h4>
<button class="connect-btn disabled" disabled>Coming Soon</button>
</div>
</div>
</div>

<!-- Auto-Trade Settings -->
<div class="auto-settings">
<h3>Auto-Trade Settings</h3>

<div class="setting-item">
<label class="toggle-label">
<input type="checkbox" bind:checked={autoTradeEnabled} disabled={!brokerConnected} />
<span class="toggle-text">Enable Auto-Trading</span>
</label>
{#if !brokerConnected}
<small class="warning">Connect broker first</small>
{/if}
</div>

<div class="setting-item">
<label for="maxPos">Max Position Size ($)</label>
<input id="maxPos" type="number" bind:value={maxPositionSize} step="100" />
</div>

<div class="setting-item">
<label for="dailyLimit">Daily Loss Limit ($)</label>
<input id="dailyLimit" type="number" bind:value={dailyLossLimit} step="50" />
</div>
</div>

<!-- Trade Log -->
<div class="trade-log">
<h3>Trade Log</h3>
<div class="log-table">
<table>
<thead>
<tr>
<th>Time</th>
<th>Symbol</th>
<th>Action</th>
<th>Price</th>
<th>Status</th>
</tr>
</thead>
<tbody>
<tr>
<td colspan="5" class="no-data">No trades yet</td>
</tr>
</tbody>
</table>
</div>
</div>
</div>
{:else if activeTab === 'analytics'}
<div class="analytics-tab">
<h2>Performance Analytics</h2>

<!-- Analytics Stats -->
<div class="analytics-stats">
<div class="stat-card">
<div class="stat-label">Win Rate</div>
<div class="stat-value">75.5%</div>
</div>
<div class="stat-card">
<div class="stat-label">Sharpe Ratio</div>
<div class="stat-value">1.85</div>
</div>
<div class="stat-card">
<div class="stat-label">Max Drawdown</div>
<div class="stat-value loss">-8.3%</div>
</div>
<div class="stat-card">
<div class="stat-label">Total Return</div>
<div class="stat-value profit">+24.7%</div>
</div>
</div>

<!-- Equity Curve -->
<div class="chart-section">
<h3>Equity Curve</h3>
<div class="chart-placeholder">
<p>📈 Equity curve chart would be rendered here using Chart.js</p>
<small>Track your account growth over time</small>
</div>
</div>

<!-- Best/Worst Trades -->
<div class="trades-section">
<div class="trades-column">
<h3>Best Trades</h3>
<div class="trade-list">
<div class="trade-item profit">
<span class="trade-symbol">NVDA</span>
<span class="trade-return">+15.2%</span>
</div>
<div class="trade-item profit">
<span class="trade-symbol">AAPL</span>
<span class="trade-return">+12.8%</span>
</div>
<div class="trade-item profit">
<span class="trade-symbol">TSLA</span>
<span class="trade-return">+10.5%</span>
</div>
</div>
</div>

<div class="trades-column">
<h3>Worst Trades</h3>
<div class="trade-list">
<div class="trade-item loss">
<span class="trade-symbol">SPY</span>
<span class="trade-return">-5.3%</span>
</div>
<div class="trade-item loss">
<span class="trade-symbol">META</span>
<span class="trade-return">-4.1%</span>
</div>
<div class="trade-item loss">
<span class="trade-symbol">AMZN</span>
<span class="trade-return">-3.8%</span>
</div>
</div>
</div>
</div>
</div>
{/if}
</div>
</div>

<style>
.station-dashboard {
max-width: 1800px;
margin: 0 auto;
}

.plan-badge-header {
margin-bottom: 1.5rem;
}

.badge {
display: inline-flex;
align-items: center;
gap: 0.5rem;
padding: 0.75rem 1.5rem;
background: linear-gradient(135deg, #a855f7 0%, #ec4899 100%);
border-radius: 9999px;
color: #fff;
font-weight: 700;
font-size: 1rem;
box-shadow: 0 0 30px rgba(168, 85, 247, 0.5);
}

.tab-nav {
display: flex;
gap: 0.5rem;
margin-bottom: 2rem;
overflow-x: auto;
padding-bottom: 0.5rem;
}

.tab-btn {
padding: 0.875rem 1.5rem;
background: rgba(255, 255, 255, 0.05);
border: 1px solid rgba(255, 255, 255, 0.1);
border-radius: 0.75rem;
color: #e5e7eb;
font-weight: 600;
cursor: pointer;
transition: all 0.2s ease;
white-space: nowrap;
}

.tab-btn:hover {
background: rgba(255, 255, 255, 0.1);
border-color: rgba(255, 255, 255, 0.2);
}

.tab-btn.active {
background: linear-gradient(135deg, rgba(255, 107, 53, 0.2) 0%, rgba(251, 191, 36, 0.2) 100%);
border-color: rgba(255, 107, 53, 0.5);
color: #fbbf24;
}

.tab-content {
padding: 2rem;
background: rgba(17, 24, 39, 0.7);
backdrop-filter: blur(12px);
border: 1px solid rgba(255, 255, 255, 0.1);
border-radius: 1rem;
}

.tab-content h2 {
margin: 0 0 1.5rem 0;
font-size: 1.75rem;
font-weight: 700;
color: #f9fafb;
}

.tab-content h3 {
margin: 0 0 1rem 0;
font-size: 1.25rem;
font-weight: 700;
color: #f9fafb;
}

.scanner-layout {
display: grid;
grid-template-columns: 280px 1fr;
gap: 1.5rem;
}

.info-text {
margin-bottom: 1.5rem;
color: #9ca3af;
}

.placeholder-card {
padding: 4rem;
background: rgba(255, 255, 255, 0.02);
border: 2px dashed rgba(255, 255, 255, 0.1);
border-radius: 1rem;
text-align: center;
color: #6b7280;
}

.placeholder-card small {
display: block;
margin-top: 0.5rem;
font-size: 0.875rem;
color: #4b5563;
}

.portfolio-stats,
.paper-stats,
.analytics-stats {
display: grid;
grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
gap: 1rem;
margin-bottom: 2rem;
}

.stat-card {
padding: 1.25rem;
background: rgba(0, 0, 0, 0.3);
border: 1px solid rgba(255, 255, 255, 0.1);
border-radius: 0.75rem;
}

.stat-label {
font-size: 0.875rem;
color: #9ca3af;
margin-bottom: 0.5rem;
}

.stat-value {
font-size: 1.75rem;
font-weight: 700;
color: #f9fafb;
}

.stat-value.profit {
color: #10b981;
}

.stat-value.loss {
color: #ef4444;
}

.add-position-form,
.paper-form,
.auto-settings {
padding: 1.5rem;
background: rgba(0, 0, 0, 0.2);
border: 1px solid rgba(255, 255, 255, 0.1);
border-radius: 0.75rem;
margin-bottom: 2rem;
}

.form-row {
display: grid;
grid-template-columns: 1fr 1fr 1fr auto;
gap: 1rem;
}

.form-grid {
display: grid;
grid-template-columns: repeat(3, 1fr);
gap: 1rem;
margin-bottom: 1rem;
}

input,
select {
padding: 0.75rem;
background: rgba(255, 255, 255, 0.05);
border: 1px solid rgba(255, 255, 255, 0.1);
border-radius: 0.5rem;
color: #f9fafb;
font-size: 0.875rem;
}

.add-btn,
.open-trade-btn {
padding: 0.75rem 1.5rem;
background: linear-gradient(135deg, #10b981 0%, #059669 100%);
border: none;
border-radius: 0.5rem;
color: #fff;
font-weight: 700;
cursor: pointer;
transition: all 0.3s ease;
}

.add-btn:hover,
.open-trade-btn:hover {
transform: translateY(-2px);
box-shadow: 0 10px 25px rgba(16, 185, 129, 0.4);
}

.positions-grid,
.paper-positions {
display: grid;
grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
gap: 1rem;
}

.position-card,
.paper-position-card {
padding: 1.25rem;
background: rgba(0, 0, 0, 0.3);
border: 1px solid rgba(255, 255, 255, 0.1);
border-radius: 0.75rem;
}

.position-header {
display: flex;
align-items: center;
justify-content: space-between;
margin-bottom: 1rem;
}

.position-header h4 {
margin: 0;
font-size: 1.25rem;
font-weight: 700;
color: #f9fafb;
}

.position-qty {
font-size: 0.875rem;
color: #9ca3af;
}

.position-prices,
.position-details {
display: flex;
flex-direction: column;
gap: 0.75rem;
margin-bottom: 1rem;
}

.price-item,
.detail {
display: flex;
justify-content: space-between;
padding: 0.625rem;
background: rgba(255, 255, 255, 0.03);
border-radius: 0.5rem;
}

.price-item .label,
.detail .label {
font-size: 0.875rem;
color: #9ca3af;
}

.price-item .value,
.detail .value {
font-size: 0.875rem;
font-weight: 700;
color: #f9fafb;
}

.detail .value.profit {
color: #10b981;
}

.detail .value.loss {
color: #ef4444;
}

.position-pnl {
display: flex;
justify-content: space-between;
align-items: center;
padding: 0.875rem;
background: rgba(255, 255, 255, 0.05);
border-radius: 0.5rem;
}

.pnl-value,
.pnl-percent {
font-weight: 700;
}

.pnl-value.profit,
.pnl-percent.profit {
color: #10b981;
}

.pnl-value.loss,
.pnl-percent.loss {
color: #ef4444;
}

.side-badge {
padding: 0.25rem 0.625rem;
border-radius: 9999px;
font-size: 0.75rem;
font-weight: 700;
}

.side-badge.long {
background: rgba(16, 185, 129, 0.2);
color: #10b981;
}

.side-badge.short {
background: rgba(239, 68, 68, 0.2);
color: #ef4444;
}

.close-position-btn {
width: 100%;
padding: 0.625rem;
background: rgba(239, 68, 68, 0.2);
border: 1px solid rgba(239, 68, 68, 0.3);
border-radius: 0.5rem;
color: #ef4444;
font-weight: 600;
cursor: pointer;
transition: all 0.2s ease;
}

.close-position-btn:hover {
background: rgba(239, 68, 68, 0.3);
border-color: rgba(239, 68, 68, 0.5);
}

.broker-section {
margin-bottom: 2rem;
}

.broker-cards {
display: grid;
grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
gap: 1rem;
}

.broker-card {
padding: 1.5rem;
background: rgba(0, 0, 0, 0.3);
border: 1px solid rgba(255, 255, 255, 0.1);
border-radius: 0.75rem;
text-align: center;
}

.broker-icon {
font-size: 3rem;
margin-bottom: 0.75rem;
}

.broker-card h4 {
margin: 0 0 1rem 0;
font-size: 1.125rem;
font-weight: 700;
color: #f9fafb;
}

.connect-btn {
width: 100%;
padding: 0.75rem;
background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
border: none;
border-radius: 0.5rem;
color: #fff;
font-weight: 700;
cursor: pointer;
transition: all 0.3s ease;
}

.connect-btn:hover:not(.disabled) {
transform: translateY(-2px);
box-shadow: 0 10px 25px rgba(59, 130, 246, 0.4);
}

.connect-btn.disabled {
background: rgba(107, 114, 128, 0.5);
cursor: not-allowed;
}

.setting-item {
margin-bottom: 1.5rem;
}

.setting-item label {
display: block;
margin-bottom: 0.5rem;
font-size: 0.875rem;
font-weight: 600;
color: #e5e7eb;
}

.toggle-label {
display: flex;
align-items: center;
gap: 0.75rem;
cursor: pointer;
}

.toggle-label input[type='checkbox'] {
width: 1.25rem;
height: 1.25rem;
cursor: pointer;
accent-color: #ff6b35;
}

.warning {
display: block;
margin-top: 0.375rem;
color: #fbbf24;
font-size: 0.75rem;
}

.trade-log {
margin-top: 2rem;
}

.log-table {
background: rgba(0, 0, 0, 0.3);
border: 1px solid rgba(255, 255, 255, 0.1);
border-radius: 0.75rem;
overflow: hidden;
}

table {
width: 100%;
border-collapse: collapse;
}

thead {
background: rgba(0, 0, 0, 0.3);
}

th {
padding: 1rem;
text-align: left;
font-size: 0.875rem;
font-weight: 600;
color: #9ca3af;
text-transform: uppercase;
}

td {
padding: 1rem;
border-top: 1px solid rgba(255, 255, 255, 0.05);
color: #e5e7eb;
}

.no-data {
text-align: center;
color: #6b7280;
}

.chart-section {
margin-bottom: 2rem;
}

.chart-placeholder {
padding: 4rem;
background: rgba(0, 0, 0, 0.3);
border: 1px solid rgba(255, 255, 255, 0.1);
border-radius: 0.75rem;
text-align: center;
color: #6b7280;
}

.chart-placeholder small {
display: block;
margin-top: 0.5rem;
font-size: 0.875rem;
color: #4b5563;
}

.trades-section {
display: grid;
grid-template-columns: 1fr 1fr;
gap: 1.5rem;
}

.trades-column {
padding: 1.5rem;
background: rgba(0, 0, 0, 0.3);
border: 1px solid rgba(255, 255, 255, 0.1);
border-radius: 0.75rem;
}

.trade-list {
display: flex;
flex-direction: column;
gap: 0.75rem;
}

.trade-item {
display: flex;
justify-content: space-between;
padding: 0.875rem;
background: rgba(255, 255, 255, 0.03);
border-radius: 0.5rem;
}

.trade-symbol {
font-weight: 700;
color: #f9fafb;
}

.trade-return {
font-weight: 700;
}

.trade-item.profit .trade-return {
color: #10b981;
}

.trade-item.loss .trade-return {
color: #ef4444;
}

@media (max-width: 1024px) {
.scanner-layout {
grid-template-columns: 1fr;
}

.trades-section {
grid-template-columns: 1fr;
}
}

@media (max-width: 640px) {
.tab-nav {
flex-wrap: wrap;
}

.form-row,
.form-grid {
grid-template-columns: 1fr;
}

.positions-grid,
.paper-positions,
.broker-cards {
grid-template-columns: 1fr;
}
}
</style>
