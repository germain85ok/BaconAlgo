<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { getSupabaseClient } from '$lib/supabase/client';
	import { Chart, registerables } from 'chart.js';

	Chart.register(...registerables);

	const supabase = getSupabaseClient();

	let dateRange = $state('30');
	let loading = $state(true);

	let revenueChart: Chart | null = null;
	let signupsChart: Chart | null = null;
	let activeUsersChart: Chart | null = null;
	let churnChart: Chart | null = null;
	let tierChart: Chart | null = null;

	let analyticsData = $state({
		totalRevenue: 0,
		totalSignups: 0,
		activeUsers: 0,
		churnRate: 0,
		conversionRate: 0
	});

	let topReferrers = $state<any[]>([]);

	onMount(async () => {
		await loadAnalytics();
		initCharts();
	});

	onDestroy(() => {
		revenueChart?.destroy();
		signupsChart?.destroy();
		activeUsersChart?.destroy();
		churnChart?.destroy();
		tierChart?.destroy();
	});

	async function loadAnalytics() {
		try {
			// Mock data - in real app, query analytics tables
			analyticsData = {
				totalRevenue: 15847,
				totalSignups: 342,
				activeUsers: 287,
				churnRate: 3.2,
				conversionRate: 24.5
			};

			topReferrers = [
				{ source: 'Twitter', signups: 45, revenue: 2250 },
				{ source: 'YouTube', signups: 38, revenue: 1900 },
				{ source: 'Discord', signups: 29, revenue: 1450 },
				{ source: 'Reddit', signups: 22, revenue: 1100 },
				{ source: 'Direct', signups: 18, revenue: 900 }
			];

			loading = false;
		} catch (error) {
			console.error('Failed to load analytics:', error);
			loading = false;
		}
	}

	function initCharts() {
		const days = parseInt(dateRange);

		// Revenue Chart
		const revenueCtx = document.getElementById('revenueChart') as HTMLCanvasElement;
		if (revenueCtx) {
			revenueChart = new Chart(revenueCtx, {
				type: 'line',
				data: {
					labels: Array.from({ length: days }, (_, i) => `Day ${i + 1}`),
					datasets: [
						{
							label: 'Revenue',
							data: Array.from({ length: days }, () => Math.floor(Math.random() * 800) + 200),
							borderColor: '#4ade80',
							backgroundColor: 'rgba(74, 222, 128, 0.1)',
							tension: 0.4,
							fill: true
						}
					]
				},
				options: getChartOptions('$')
			});
		}

		// Signups Chart
		const signupsCtx = document.getElementById('signupsChart') as HTMLCanvasElement;
		if (signupsCtx) {
			signupsChart = new Chart(signupsCtx, {
				type: 'bar',
				data: {
					labels: Array.from({ length: days }, (_, i) => `Day ${i + 1}`),
					datasets: [
						{
							label: 'Signups',
							data: Array.from({ length: days }, () => Math.floor(Math.random() * 15) + 5),
							backgroundColor: 'rgba(240, 147, 251, 0.8)',
							borderColor: '#f093fb',
							borderWidth: 1
						}
					]
				},
				options: getChartOptions()
			});
		}

		// Active Users Chart
		const activeCtx = document.getElementById('activeUsersChart') as HTMLCanvasElement;
		if (activeCtx) {
			activeUsersChart = new Chart(activeCtx, {
				type: 'line',
				data: {
					labels: Array.from({ length: days }, (_, i) => `Day ${i + 1}`),
					datasets: [
						{
							label: 'Active Users',
							data: Array.from({ length: days }, () => Math.floor(Math.random() * 50) + 200),
							borderColor: '#60a5fa',
							backgroundColor: 'rgba(96, 165, 250, 0.1)',
							tension: 0.4,
							fill: true
						}
					]
				},
				options: getChartOptions()
			});
		}

		// Churn Chart
		const churnCtx = document.getElementById('churnChart') as HTMLCanvasElement;
		if (churnCtx) {
			churnChart = new Chart(churnCtx, {
				type: 'line',
				data: {
					labels: Array.from({ length: days }, (_, i) => `Day ${i + 1}`),
					datasets: [
						{
							label: 'Churn Rate',
							data: Array.from({ length: days }, () => Math.random() * 5 + 2),
							borderColor: '#f87171',
							backgroundColor: 'rgba(248, 113, 113, 0.1)',
							tension: 0.4,
							fill: true
						}
					]
				},
				options: getChartOptions('%')
			});
		}

		// Tier Breakdown Chart
		const tierCtx = document.getElementById('tierChart') as HTMLCanvasElement;
		if (tierCtx) {
			tierChart = new Chart(tierCtx, {
				type: 'pie',
				data: {
					labels: ['Free', 'Indicator', 'Scanner', 'Station'],
					datasets: [
						{
							data: [120, 80, 60, 27],
							backgroundColor: ['#9ca3af', '#60a5fa', '#fb923c', '#a78bfa'],
							borderColor: '#1a1a2e',
							borderWidth: 2
						}
					]
				},
				options: {
					responsive: true,
					maintainAspectRatio: false,
					plugins: {
						legend: {
							position: 'bottom',
							labels: { color: '#fff', padding: 15 }
						}
					}
				}
			});
		}
	}

	function getChartOptions(prefix: string = ''): any {
		return {
			responsive: true,
			maintainAspectRatio: false,
			plugins: {
				legend: { display: false }
			},
			scales: {
				y: {
					beginAtZero: true,
					grid: { color: 'rgba(255, 255, 255, 0.1)' },
					ticks: {
						color: '#fff',
						callback: function (value: any) {
							return prefix + value;
						}
					}
				},
				x: {
					grid: { display: false },
					ticks: {
						color: '#fff',
						maxTicksLimit: 10
					}
				}
			}
		};
	}

	async function handleDateRangeChange() {
		revenueChart?.destroy();
		signupsChart?.destroy();
		activeUsersChart?.destroy();
		churnChart?.destroy();
		await loadAnalytics();
		initCharts();
	}

	function exportToCSV() {
		const headers = ['Metric', 'Value'];
		const rows = [
			['Total Revenue', `$${analyticsData.totalRevenue}`],
			['Total Signups', analyticsData.totalSignups],
			['Active Users', analyticsData.activeUsers],
			['Churn Rate', `${analyticsData.churnRate}%`],
			['Conversion Rate', `${analyticsData.conversionRate}%`]
		];

		const csv = [headers, ...rows].map((row) => row.join(',')).join('\n');
		const blob = new Blob([csv], { type: 'text/csv' });
		const url = URL.createObjectURL(blob);
		const a = document.createElement('a');
		a.href = url;
		a.download = `analytics-${new Date().toISOString()}.csv`;
		a.click();
	}
</script>

<div class="analytics-page">
	<!-- Controls -->
	<div class="controls">
		<select bind:value={dateRange} onchange={handleDateRangeChange} class="select">
			<option value="7">Last 7 Days</option>
			<option value="30">Last 30 Days</option>
			<option value="90">Last 90 Days</option>
		</select>

		<button onclick={exportToCSV} class="export-btn">Export CSV</button>
	</div>

	{#if loading}
		<div class="loading">Loading analytics...</div>
	{:else}
		<!-- Summary Stats -->
		<div class="stats-grid">
			<div class="stat-card">
				<div class="stat-icon">💰</div>
				<div class="stat-content">
					<div class="stat-label">Total Revenue</div>
					<div class="stat-value">${analyticsData.totalRevenue.toLocaleString()}</div>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon">👥</div>
				<div class="stat-content">
					<div class="stat-label">Total Signups</div>
					<div class="stat-value">{analyticsData.totalSignups}</div>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon">✅</div>
				<div class="stat-content">
					<div class="stat-label">Active Users</div>
					<div class="stat-value">{analyticsData.activeUsers}</div>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon">📉</div>
				<div class="stat-content">
					<div class="stat-label">Churn Rate</div>
					<div class="stat-value">{analyticsData.churnRate}%</div>
				</div>
			</div>
		</div>

		<!-- Charts -->
		<div class="charts-container">
			<div class="chart-row">
				<div class="chart-card">
					<h3>Revenue Over Time</h3>
					<div class="chart-wrapper">
						<canvas id="revenueChart"></canvas>
					</div>
				</div>

				<div class="chart-card">
					<h3>Signups Over Time</h3>
					<div class="chart-wrapper">
						<canvas id="signupsChart"></canvas>
					</div>
				</div>
			</div>

			<div class="chart-row">
				<div class="chart-card">
					<h3>Active Users Over Time</h3>
					<div class="chart-wrapper">
						<canvas id="activeUsersChart"></canvas>
					</div>
				</div>

				<div class="chart-card">
					<h3>Churn Rate Over Time</h3>
					<div class="chart-wrapper">
						<canvas id="churnChart"></canvas>
					</div>
				</div>
			</div>

			<div class="chart-row">
				<div class="chart-card">
					<h3>Tier Breakdown</h3>
					<div class="chart-wrapper">
						<canvas id="tierChart"></canvas>
					</div>
				</div>

				<!-- Top Referrers -->
				<div class="chart-card">
					<h3>Top Referrers</h3>
					<div class="referrers-table">
						<table>
							<thead>
								<tr>
									<th>Source</th>
									<th>Signups</th>
									<th>Revenue</th>
								</tr>
							</thead>
							<tbody>
								{#each topReferrers as referrer}
									<tr>
										<td><strong>{referrer.source}</strong></td>
										<td>{referrer.signups}</td>
										<td>${referrer.revenue.toLocaleString()}</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>

<style>
	.analytics-page {
		display: flex;
		flex-direction: column;
		gap: 2rem;
	}

	.controls {
		display: flex;
		gap: 1rem;
		justify-content: space-between;
		align-items: center;
	}

	.select {
		padding: 0.75rem 1rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		color: #fff;
		font-size: 0.875rem;
		cursor: pointer;
	}

	.export-btn {
		padding: 0.75rem 1.5rem;
		background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
		border: none;
		border-radius: 12px;
		color: #fff;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.export-btn:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 30px rgba(240, 147, 251, 0.3);
	}

	.loading {
		text-align: center;
		padding: 4rem;
		color: rgba(255, 255, 255, 0.6);
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		gap: 1.5rem;
	}

	.stat-card {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 16px;
		padding: 1.5rem;
		display: flex;
		align-items: center;
		gap: 1rem;
		transition: all 0.3s ease;
	}

	.stat-card:hover {
		background: rgba(255, 255, 255, 0.08);
		transform: translateY(-2px);
	}

	.stat-icon {
		font-size: 2.5rem;
	}

	.stat-content {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.stat-label {
		font-size: 0.875rem;
		color: rgba(255, 255, 255, 0.6);
	}

	.stat-value {
		font-size: 1.75rem;
		font-weight: 700;
	}

	.charts-container {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.chart-row {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
		gap: 1.5rem;
	}

	.chart-card {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 16px;
		padding: 1.5rem;
	}

	.chart-card h3 {
		margin: 0 0 1rem 0;
		font-size: 1.125rem;
	}

	.chart-wrapper {
		height: 300px;
		position: relative;
	}

	.referrers-table {
		overflow-x: auto;
	}

	.referrers-table table {
		width: 100%;
		border-collapse: collapse;
	}

	.referrers-table th {
		text-align: left;
		padding: 0.75rem;
		font-size: 0.875rem;
		color: rgba(255, 255, 255, 0.6);
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	.referrers-table td {
		padding: 0.75rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.05);
	}

	@media (max-width: 768px) {
		.stats-grid {
			grid-template-columns: 1fr;
		}

		.chart-row {
			grid-template-columns: 1fr;
		}

		.controls {
			flex-direction: column;
			align-items: stretch;
		}

		.export-btn {
			width: 100%;
		}
	}
</style>
