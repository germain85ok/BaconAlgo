<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { getSupabaseClient } from '$lib/supabase/client';
	import { Chart, registerables } from 'chart.js';
	import { t, locale } from '$lib/i18n/i18n';

	Chart.register(...registerables);

	const supabase = getSupabaseClient();

	// Correct tier prices in CAD
	const PLAN_PRICES = {
		STATION: 50,
		SCANNER: 30,
		INDICATEUR: 20,
		FREE: 0
	} as const;

	let stats = $state({
		totalUsers: 0,
		activeSubscriptions: 0,
		mrr: 0,
		churnRate: 0,
		todaySignups: 0,
		todayRevenue: 0
	});

	let recentUsers = $state<any[]>([]);
	let recentPromos = $state<any[]>([]);
	let loading = $state(true);

	let signupsChart: Chart | null = null;
	let revenueChart: Chart | null = null;

	onMount(async () => {
		await loadDashboardData();
		initCharts();
	});

	onDestroy(() => {
		signupsChart?.destroy();
		revenueChart?.destroy();
	});

	async function loadDashboardData() {
		try {
			// Load stats
			const { data: users } = await supabase.from('profiles').select('*');
			stats.totalUsers = users?.length || 0;

			const activeUsers = users?.filter(
				(u) => {
					const planUpper = (u.plan || '').toUpperCase();
					return planUpper !== 'FREE' && (!u.plan_expires_at || new Date(u.plan_expires_at) > new Date());
				}
			);
			stats.activeSubscriptions = activeUsers?.length || 0;

			// MRR calculation with correct tier names
			stats.mrr =
				(activeUsers?.reduce((sum, u) => {
					const planUpper = (u.plan || '').toUpperCase();
					return sum + (PLAN_PRICES[planUpper as keyof typeof PLAN_PRICES] || 0);
				}, 0) || 0);

			// Recent users
			const { data: recent } = await supabase
				.from('profiles')
				.select('*')
				.order('created_at', { ascending: false })
				.limit(10);
			recentUsers = recent || [];

			// Recent promo uses (mock data - in real app query promo_uses table)
			const { data: promos } = await supabase
				.from('promo_codes')
				.select('*')
				.order('created_at', { ascending: false })
				.limit(5);
			recentPromos = promos || [];

			loading = false;
		} catch (error) {
			console.error('Failed to load dashboard data:', error);
			loading = false;
		}
	}

	function initCharts() {
		// Signups chart
		const signupsCtx = document.getElementById('signupsChart') as HTMLCanvasElement;
		if (signupsCtx) {
			signupsChart = new Chart(signupsCtx, {
				type: 'line',
				data: {
					labels: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'],
					datasets: [
						{
							label: 'Signups',
							data: [12, 19, 15, 22, 18, 25, 20],
							borderColor: '#f093fb',
							backgroundColor: 'rgba(240, 147, 251, 0.1)',
							tension: 0.4,
							fill: true
						}
					]
				},
				options: {
					responsive: true,
					maintainAspectRatio: false,
					plugins: {
						legend: { display: false }
					},
					scales: {
						y: { beginAtZero: true, grid: { color: 'rgba(255, 255, 255, 0.1)' }, ticks: { color: '#fff' } },
						x: { grid: { display: false }, ticks: { color: '#fff' } }
					}
				}
			});
		}

		// Revenue chart
		const revenueCtx = document.getElementById('revenueChart') as HTMLCanvasElement;
		if (revenueCtx) {
			revenueChart = new Chart(revenueCtx, {
				type: 'bar',
				data: {
					labels: Array.from({ length: 30 }, (_, i) => `Day ${i + 1}`),
					datasets: [
						{
							label: 'Revenue',
							data: Array.from({ length: 30 }, () => Math.floor(Math.random() * 500) + 100),
							backgroundColor: 'rgba(245, 87, 108, 0.8)',
							borderColor: '#f5576c',
							borderWidth: 1
						}
					]
				},
				options: {
					responsive: true,
					maintainAspectRatio: false,
					plugins: {
						legend: { display: false }
					},
					scales: {
						y: { beginAtZero: true, grid: { color: 'rgba(255, 255, 255, 0.1)' }, ticks: { color: '#fff' } },
						x: { grid: { display: false }, ticks: { color: '#fff', maxTicksLimit: 10 } }
					}
				}
			});
		}
	}

	function formatCurrency(amount: number): string {
		return `$${amount.toLocaleString()} CAD`;
	}

	function formatDate(dateStr: string): string {
		return new Date(dateStr).toLocaleDateString($locale === 'fr' ? 'fr-CA' : 'en-CA');
	}

	function getPlanBadgeClass(plan: string): string {
		const planUpper = (plan || '').toUpperCase();
		switch (planUpper) {
			case 'STATION':
				return 'badge-station';
			case 'SCANNER':
				return 'badge-scanner';
			case 'INDICATEUR':
			case 'INDICATOR':
				return 'badge-indicator';
			default:
				return 'badge-free';
		}
	}

	function getPlanDisplayName(plan: string): string {
		const planUpper = (plan || '').toUpperCase();
		if ($locale === 'fr') {
			switch (planUpper) {
				case 'STATION': return 'Station';
				case 'SCANNER': return 'Scanner';
				case 'INDICATEUR':
				case 'INDICATOR': return 'Indicateur';
				default: return 'Gratuit';
			}
		} else {
			switch (planUpper) {
				case 'STATION': return 'Station';
				case 'SCANNER': return 'Scanner';
				case 'INDICATEUR':
				case 'INDICATOR': return 'Indicator';
				default: return 'Free';
			}
		}
	}
</script>

<div class="dashboard">
	{#if loading}
		<div class="loading">{$t.common.loading}</div>
	{:else}
		<!-- Stats Cards -->
		<div class="stats-grid">
			<div class="stat-card">
				<div class="stat-icon">👥</div>
				<div class="stat-content">
					<div class="stat-label">{$t.admin.totalUsers}</div>
					<div class="stat-value">{stats.totalUsers}</div>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon">✅</div>
				<div class="stat-content">
					<div class="stat-label">{$t.admin.activeSubscriptions}</div>
					<div class="stat-value">{stats.activeSubscriptions}</div>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon">💰</div>
				<div class="stat-content">
					<div class="stat-label">{$t.admin.mrr}</div>
					<div class="stat-value">{formatCurrency(stats.mrr)}</div>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon">📉</div>
				<div class="stat-content">
					<div class="stat-label">{$t.admin.churnRate}</div>
					<div class="stat-value">{stats.churnRate.toFixed(1)}%</div>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon">🆕</div>
				<div class="stat-content">
					<div class="stat-label">{$t.admin.todaySignups}</div>
					<div class="stat-value">{stats.todaySignups}</div>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon">💵</div>
				<div class="stat-content">
					<div class="stat-label">{$t.admin.todayRevenue}</div>
					<div class="stat-value">{formatCurrency(stats.todayRevenue)}</div>
				</div>
			</div>
		</div>

		<!-- Charts -->
		<div class="charts-grid">
			<div class="chart-card">
				<h3>{$t.admin.signupsChart}</h3>
				<div class="chart-container">
					<canvas id="signupsChart"></canvas>
				</div>
			</div>

			<div class="chart-card">
				<h3>{$t.admin.revenueChart}</h3>
				<div class="chart-container">
					<canvas id="revenueChart"></canvas>
				</div>
			</div>
		</div>

		<!-- Recent Users -->
		<div class="section-card">
			<div class="section-header">
				<h3>{$t.admin.recentUsers}</h3>
				<a href="/admin/users" class="view-all">{$t.admin.viewAll}</a>
			</div>
			<div class="table-container">
				<table>
					<thead>
						<tr>
							<th>{$t.admin.email}</th>
							<th>{$t.admin.username}</th>
							<th>{$t.admin.plan}</th>
							<th>{$t.admin.joined}</th>
							<th>{$t.admin.status}</th>
						</tr>
					</thead>
					<tbody>
						{#each recentUsers as user}
							<tr>
								<td>{user.email}</td>
								<td>{user.username || '-'}</td>
								<td><span class="plan-badge {getPlanBadgeClass(user.plan)}">{getPlanDisplayName(user.plan)}</span></td>
								<td>{formatDate(user.created_at)}</td>
								<td>
									<span class="status-badge {user.is_banned ? 'status-banned' : 'status-active'}">
										{user.is_banned ? $t.admin.banned : $t.admin.active}
									</span>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</div>

		<!-- Quick Actions -->
		<div class="quick-actions">
			<h3>{$t.admin.quickActions}</h3>
			<div class="actions-grid">
				<a href="/admin/promos" class="action-btn">
					<span>🎟️</span>
					<span>{$t.admin.createPromoAction}</span>
				</a>
				<a href="/admin/discord" class="action-btn">
					<span>💬</span>
					<span>{$t.admin.sendAlert}</span>
				</a>
				<a href="/admin/signals" class="action-btn">
					<span>📡</span>
					<span>{$t.admin.pushSignal}</span>
				</a>
			</div>
		</div>
	{/if}
</div>

<style>
	.dashboard {
		display: flex;
		flex-direction: column;
		gap: 2rem;
	}

	.loading {
		text-align: center;
		padding: 4rem;
		font-size: 1.125rem;
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

	.charts-grid {
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

	.chart-container {
		height: 300px;
	}

	.section-card {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 16px;
		padding: 1.5rem;
	}

	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.section-header h3 {
		margin: 0;
		font-size: 1.25rem;
	}

	.view-all {
		color: #f093fb;
		text-decoration: none;
		font-size: 0.875rem;
		transition: all 0.2s ease;
	}

	.view-all:hover {
		color: #f5576c;
	}

	.table-container {
		overflow-x: auto;
	}

	table {
		width: 100%;
		border-collapse: collapse;
	}

	th {
		text-align: left;
		padding: 0.75rem;
		font-size: 0.875rem;
		color: rgba(255, 255, 255, 0.6);
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	td {
		padding: 1rem 0.75rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.05);
	}

	.plan-badge {
		padding: 0.25rem 0.75rem;
		border-radius: 12px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
	}

	.badge-station {
		background: rgba(147, 51, 234, 0.2);
		color: #a78bfa;
	}

	.badge-scanner {
		background: rgba(249, 115, 22, 0.2);
		color: #fb923c;
	}

	.badge-indicator {
		background: rgba(59, 130, 246, 0.2);
		color: #60a5fa;
	}

	.badge-free {
		background: rgba(107, 114, 128, 0.2);
		color: #9ca3af;
	}

	.status-badge {
		padding: 0.25rem 0.75rem;
		border-radius: 12px;
		font-size: 0.75rem;
		font-weight: 600;
	}

	.status-active {
		background: rgba(34, 197, 94, 0.2);
		color: #4ade80;
	}

	.status-banned {
		background: rgba(239, 68, 68, 0.2);
		color: #f87171;
	}

	.quick-actions h3 {
		margin: 0 0 1rem 0;
		font-size: 1.25rem;
	}

	.actions-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1rem;
	}

	.action-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		padding: 1.25rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		color: #fff;
		text-decoration: none;
		font-weight: 600;
		transition: all 0.3s ease;
	}

	.action-btn:hover {
		background: linear-gradient(135deg, rgba(240, 147, 251, 0.2) 0%, rgba(245, 87, 108, 0.2) 100%);
		border-color: rgba(240, 147, 251, 0.5);
		transform: translateY(-2px);
	}

	.action-btn span:first-child {
		font-size: 1.5rem;
	}

	@media (max-width: 768px) {
		.stats-grid {
			grid-template-columns: 1fr;
		}

		.charts-grid {
			grid-template-columns: 1fr;
		}

		.actions-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
