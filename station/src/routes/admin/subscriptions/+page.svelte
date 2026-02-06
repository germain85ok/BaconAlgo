<script lang="ts">
	import { onMount } from 'svelte';
	import { getAllSubscriptions, type Subscription } from '$lib/services/subscriptions';
	import GlassCard from '$lib/components/ui/GlassCard.svelte';

	let subscriptions = $state<any[]>([]);
	let loading = $state(true);
	let filterStatus = $state<string>('all');
	let filterPlan = $state<string>('all');

	const statuses = ['all', 'active', 'cancelled', 'expired', 'past_due'];
	const plans = ['all', 'FREE', 'INDICATEUR', 'SCANNER', 'STATION'];

	onMount(async () => {
		await loadSubscriptions();
	});

	async function loadSubscriptions() {
		loading = true;
		const { data, error } = await getAllSubscriptions();
		if (error) {
			console.error('Failed to load subscriptions:', error);
		} else {
			subscriptions = data || [];
		}
		loading = false;
	}

	function formatDate(dateStr: string | null): string {
		if (!dateStr) return '-';
		return new Date(dateStr).toLocaleDateString();
	}

	function formatCurrency(amount: number): string {
		return `$${amount.toFixed(2)}`;
	}

	$effect(() => {
		// Filtered subscriptions based on status and plan
		filteredSubscriptions;
	});

	const filteredSubscriptions = $derived(
		subscriptions.filter(sub => {
			if (filterStatus !== 'all' && sub.status !== filterStatus) return false;
			if (filterPlan !== 'all' && sub.plan !== filterPlan) return false;
			return true;
		})
	);

	const stats = $derived({
		total: subscriptions.length,
		active: subscriptions.filter(s => s.status === 'active').length,
		revenue: subscriptions
			.filter(s => s.status === 'active')
			.reduce((sum, s) => sum + (s.amount || 0), 0)
	});
</script>

<svelte:head>
	<title>Subscriptions - Admin - BaconAlgo</title>
</svelte:head>

<div class="subscriptions-page">
	<!-- Header -->
	<div class="mb-8">
		<h2 class="text-3xl font-bold bg-gradient-to-r from-bacon-orange to-bacon-red bg-clip-text text-transparent mb-4">
			Subscription Management
		</h2>
		
		<!-- Stats Cards -->
		<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
			<GlassCard>
				<div class="text-center">
					<div class="text-3xl font-bold text-bacon-orange">{stats.total}</div>
					<div class="text-sm text-text-secondary mt-1">Total Subscriptions</div>
				</div>
			</GlassCard>
			<GlassCard>
				<div class="text-center">
					<div class="text-3xl font-bold text-green-400">{stats.active}</div>
					<div class="text-sm text-text-secondary mt-1">Active Subscriptions</div>
				</div>
			</GlassCard>
			<GlassCard>
				<div class="text-center">
					<div class="text-3xl font-bold text-blue-400">{formatCurrency(stats.revenue)}</div>
					<div class="text-sm text-text-secondary mt-1">Monthly Revenue</div>
				</div>
			</GlassCard>
		</div>
	</div>

	<!-- Filters -->
	<GlassCard class="mb-6">
		<div class="flex flex-wrap gap-4">
			<div class="flex-1 min-w-[200px]">
				<label class="block text-sm font-medium mb-2">Status</label>
				<select
					bind:value={filterStatus}
					class="w-full bg-white/5 border border-white/10 rounded-lg px-4 py-2 text-white focus:outline-none focus:ring-2 focus:ring-bacon-orange/50"
				>
					{#each statuses as status}
						<option value={status}>{status.charAt(0).toUpperCase() + status.slice(1)}</option>
					{/each}
				</select>
			</div>
			<div class="flex-1 min-w-[200px]">
				<label class="block text-sm font-medium mb-2">Plan</label>
				<select
					bind:value={filterPlan}
					class="w-full bg-white/5 border border-white/10 rounded-lg px-4 py-2 text-white focus:outline-none focus:ring-2 focus:ring-bacon-orange/50"
				>
					{#each plans as plan}
						<option value={plan}>{plan.charAt(0).toUpperCase() + plan.slice(1)}</option>
					{/each}
				</select>
			</div>
			<div class="flex items-end">
				<button
					onclick={() => {
						filterStatus = 'all';
						filterPlan = 'all';
					}}
					class="px-4 py-2 bg-white/10 text-white rounded-lg hover:bg-white/20 transition-colors"
				>
					Clear Filters
				</button>
			</div>
		</div>
	</GlassCard>

	<!-- Subscriptions Table -->
	{#if loading}
		<div class="text-center py-12 text-text-secondary">Loading subscriptions...</div>
	{:else if filteredSubscriptions.length === 0}
		<GlassCard>
			<div class="text-center py-12 text-text-secondary">No subscriptions found</div>
		</GlassCard>
	{:else}
		<GlassCard>
			<div class="overflow-x-auto">
				<table class="w-full">
					<thead>
						<tr class="border-b border-white/10">
							<th class="text-left py-4 px-4 text-sm font-medium text-text-secondary">User</th>
							<th class="text-left py-4 px-4 text-sm font-medium text-text-secondary">Plan</th>
							<th class="text-left py-4 px-4 text-sm font-medium text-text-secondary">Billing</th>
							<th class="text-left py-4 px-4 text-sm font-medium text-text-secondary">Amount</th>
							<th class="text-left py-4 px-4 text-sm font-medium text-text-secondary">Status</th>
							<th class="text-left py-4 px-4 text-sm font-medium text-text-secondary">Coupon</th>
							<th class="text-left py-4 px-4 text-sm font-medium text-text-secondary">Period End</th>
							<th class="text-left py-4 px-4 text-sm font-medium text-text-secondary">Created</th>
						</tr>
					</thead>
					<tbody>
						{#each filteredSubscriptions as sub}
							<tr class="border-b border-white/5 hover:bg-white/5 transition-colors">
								<td class="py-4 px-4">
									<div class="font-medium">{sub.profiles?.email || 'Unknown'}</div>
									{#if sub.profiles?.username}
										<div class="text-xs text-text-secondary">@{sub.profiles.username}</div>
									{/if}
								</td>
								<td class="py-4 px-4">
									<span class="inline-block px-3 py-1 rounded-full text-xs font-semibold bg-bacon-orange/20 text-bacon-orange">
										{sub.plan}
									</span>
								</td>
								<td class="py-4 px-4 text-sm capitalize">{sub.billing_cycle || '-'}</td>
								<td class="py-4 px-4 font-medium">{formatCurrency(sub.amount)}</td>
								<td class="py-4 px-4">
									<span
										class="inline-block px-3 py-1 rounded-full text-xs font-semibold {
											sub.status === 'active' ? 'bg-green-500/20 text-green-400' :
											sub.status === 'cancelled' ? 'bg-gray-500/20 text-gray-400' :
											sub.status === 'expired' ? 'bg-red-500/20 text-red-400' :
											'bg-yellow-500/20 text-yellow-400'
										}"
									>
										{sub.status}
									</span>
								</td>
								<td class="py-4 px-4 text-sm">
									{#if sub.coupon_used}
										<code class="bg-white/10 px-2 py-1 rounded text-xs">{sub.coupon_used}</code>
									{:else}
										-
									{/if}
								</td>
								<td class="py-4 px-4 text-sm">{formatDate(sub.current_period_end)}</td>
								<td class="py-4 px-4 text-sm">{formatDate(sub.created_at)}</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</GlassCard>
	{/if}
</div>
