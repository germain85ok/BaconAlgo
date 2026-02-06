<script lang="ts">
	import { onMount } from 'svelte';
	import { getAllCoupons, createCoupon, updateCoupon, deleteCoupon, type Coupon } from '$lib/services/subscriptions';
	import GlassCard from '$lib/components/ui/GlassCard.svelte';

	let coupons = $state<Coupon[]>([]);
	let loading = $state(true);
	let showCreateForm = $state(false);

	// Form fields
	let formData = $state({
		code: '',
		discount_type: 'percent' as 'percent' | 'amount',
		discount_percent: 20,
		discount_amount: 10,
		valid_until: '',
		max_uses: null as number | null,
		applicable_plans: ['INDICATEUR', 'SCANNER', 'STATION'] as string[]
	});

	const allPlans = ['INDICATEUR', 'SCANNER', 'STATION'];

	onMount(async () => {
		await loadCoupons();
	});

	async function loadCoupons() {
		loading = true;
		const { data, error } = await getAllCoupons();
		if (error) {
			console.error('Failed to load coupons:', error);
		} else {
			coupons = data || [];
		}
		loading = false;
	}

	async function handleCreateCoupon() {
		try {
			const payload: any = {
				code: formData.code.toUpperCase(),
				applicable_plans: formData.applicable_plans
			};

			if (formData.discount_type === 'percent') {
				payload.discount_percent = formData.discount_percent;
			} else {
				payload.discount_amount = formData.discount_amount;
			}

			if (formData.max_uses) {
				payload.max_uses = formData.max_uses;
			}

			if (formData.valid_until) {
				payload.valid_until = new Date(formData.valid_until).toISOString();
			}

			const { error } = await createCoupon(payload);

			if (error) throw error;

			alert('Coupon created successfully!');
			resetForm();
			showCreateForm = false;
			await loadCoupons();
		} catch (error: any) {
			console.error('Failed to create coupon:', error);
			alert(error.message || 'Failed to create coupon');
		}
	}

	async function toggleActive(id: string, isActive: boolean) {
		try {
			const { error } = await updateCoupon(id, { is_active: !isActive });
			if (error) throw error;
			await loadCoupons();
		} catch (error) {
			console.error('Failed to toggle active:', error);
			alert('Failed to update coupon');
		}
	}

	async function handleDeleteCoupon(id: string) {
		if (!confirm('Are you sure you want to delete this coupon?')) {
			return;
		}

		try {
			const { error } = await deleteCoupon(id);
			if (error) throw error;
			await loadCoupons();
			alert('Coupon deleted successfully!');
		} catch (error) {
			console.error('Failed to delete coupon:', error);
			alert('Failed to delete coupon');
		}
	}

	function resetForm() {
		formData = {
			code: '',
			discount_type: 'percent',
			discount_percent: 20,
			discount_amount: 10,
			valid_until: '',
			max_uses: null,
			applicable_plans: ['INDICATEUR', 'SCANNER', 'STATION']
		};
	}

	function formatDate(dateStr: string | null): string {
		if (!dateStr) return 'Never';
		return new Date(dateStr).toLocaleDateString();
	}

	function getDiscountLabel(coupon: Coupon): string {
		if (coupon.discount_percent !== null) {
			return `${coupon.discount_percent}% Off`;
		}
		if (coupon.discount_amount !== null) {
			return `$${coupon.discount_amount} Off`;
		}
		return 'Invalid';
	}

	function togglePlan(plan: string) {
		if (formData.applicable_plans.includes(plan)) {
			formData.applicable_plans = formData.applicable_plans.filter(p => p !== plan);
		} else {
			formData.applicable_plans = [...formData.applicable_plans, plan];
		}
	}
</script>

<svelte:head>
	<title>Coupons - Admin - BaconAlgo</title>
</svelte:head>

<div class="coupons-page">
	<!-- Header -->
	<div class="flex justify-between items-center mb-8">
		<h2 class="text-3xl font-bold bg-gradient-to-r from-bacon-orange to-bacon-red bg-clip-text text-transparent">
			Coupon Management
		</h2>
		<button
			onclick={() => (showCreateForm = !showCreateForm)}
			class="px-6 py-3 bg-gradient-to-r from-bacon-orange to-bacon-red text-white rounded-lg font-semibold hover:shadow-lg hover:shadow-bacon-orange/30 transition-all"
		>
			{showCreateForm ? 'Cancel' : '+ Create Coupon'}
		</button>
	</div>

	<!-- Create Form -->
	{#if showCreateForm}
		<GlassCard class="mb-8">
			<h3 class="text-xl font-semibold mb-6">Create New Coupon</h3>

			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<!-- Code Name -->
				<div class="form-field">
					<label class="block text-sm font-medium mb-2">Code Name</label>
					<input
						type="text"
						bind:value={formData.code}
						placeholder="BACON20"
						class="w-full bg-white/5 border border-white/10 rounded-lg px-4 py-3 text-white placeholder-text-secondary focus:outline-none focus:ring-2 focus:ring-bacon-orange/50"
					/>
				</div>

				<!-- Discount Type -->
				<div class="form-field">
					<label class="block text-sm font-medium mb-2">Discount Type</label>
					<div class="flex gap-4">
						<label class="flex items-center gap-2 cursor-pointer">
							<input type="radio" bind:group={formData.discount_type} value="percent" class="cursor-pointer" />
							<span>Percentage</span>
						</label>
						<label class="flex items-center gap-2 cursor-pointer">
							<input type="radio" bind:group={formData.discount_type} value="amount" class="cursor-pointer" />
							<span>Fixed Amount</span>
						</label>
					</div>
				</div>

				<!-- Discount Value -->
				{#if formData.discount_type === 'percent'}
					<div class="form-field">
						<label class="block text-sm font-medium mb-2">Discount Percentage ({formData.discount_percent}%)</label>
						<input
							type="range"
							bind:value={formData.discount_percent}
							min="5"
							max="100"
							step="5"
							class="w-full h-2 bg-white/10 rounded-lg appearance-none cursor-pointer"
						/>
					</div>
				{:else}
					<div class="form-field">
						<label class="block text-sm font-medium mb-2">Discount Amount ($)</label>
						<input
							type="number"
							bind:value={formData.discount_amount}
							min="1"
							step="0.01"
							class="w-full bg-white/5 border border-white/10 rounded-lg px-4 py-3 text-white focus:outline-none focus:ring-2 focus:ring-bacon-orange/50"
						/>
					</div>
				{/if}

				<!-- Max Uses -->
				<div class="form-field">
					<label class="block text-sm font-medium mb-2">Max Uses (optional)</label>
					<input
						type="number"
						bind:value={formData.max_uses}
						placeholder="Unlimited"
						min="1"
						class="w-full bg-white/5 border border-white/10 rounded-lg px-4 py-3 text-white placeholder-text-secondary focus:outline-none focus:ring-2 focus:ring-bacon-orange/50"
					/>
				</div>

				<!-- Expiry Date -->
				<div class="form-field">
					<label class="block text-sm font-medium mb-2">Expiry Date (optional)</label>
					<input
						type="datetime-local"
						bind:value={formData.valid_until}
						class="w-full bg-white/5 border border-white/10 rounded-lg px-4 py-3 text-white focus:outline-none focus:ring-2 focus:ring-bacon-orange/50"
					/>
				</div>

				<!-- Applicable Plans -->
				<div class="form-field md:col-span-2">
					<label class="block text-sm font-medium mb-2">Applicable Plans</label>
					<div class="flex gap-4">
						{#each allPlans as plan}
							<label class="flex items-center gap-2 cursor-pointer">
								<input
									type="checkbox"
									checked={formData.applicable_plans.includes(plan)}
									onchange={() => togglePlan(plan)}
									class="cursor-pointer"
								/>
								<span>{plan}</span>
							</label>
						{/each}
					</div>
				</div>
			</div>

			<button
				onclick={handleCreateCoupon}
				class="mt-6 w-full py-3 bg-gradient-to-r from-bacon-orange to-bacon-red text-white rounded-lg font-semibold hover:shadow-lg hover:shadow-bacon-orange/30 transition-all"
			>
				Create Coupon
			</button>
		</GlassCard>
	{/if}

	<!-- Coupons Table -->
	{#if loading}
		<div class="text-center py-12 text-text-secondary">Loading coupons...</div>
	{:else if coupons.length === 0}
		<GlassCard>
			<div class="text-center py-12 text-text-secondary">No coupons created yet</div>
		</GlassCard>
	{:else}
		<GlassCard>
			<div class="overflow-x-auto">
				<table class="w-full">
					<thead>
						<tr class="border-b border-white/10">
							<th class="text-left py-4 px-4 text-sm font-medium text-text-secondary">Code</th>
							<th class="text-left py-4 px-4 text-sm font-medium text-text-secondary">Discount</th>
							<th class="text-left py-4 px-4 text-sm font-medium text-text-secondary">Uses</th>
							<th class="text-left py-4 px-4 text-sm font-medium text-text-secondary">Plans</th>
							<th class="text-left py-4 px-4 text-sm font-medium text-text-secondary">Expires</th>
							<th class="text-left py-4 px-4 text-sm font-medium text-text-secondary">Status</th>
							<th class="text-left py-4 px-4 text-sm font-medium text-text-secondary">Actions</th>
						</tr>
					</thead>
					<tbody>
						{#each coupons as coupon}
							<tr class="border-b border-white/5 hover:bg-white/5 transition-colors">
								<td class="py-4 px-4">
									<code class="bg-white/10 px-3 py-1 rounded text-bacon-orange font-mono font-semibold">
										{coupon.code}
									</code>
								</td>
								<td class="py-4 px-4">{getDiscountLabel(coupon)}</td>
								<td class="py-4 px-4">
									{coupon.current_uses} / {coupon.max_uses || '∞'}
								</td>
								<td class="py-4 px-4 text-sm">
									{coupon.applicable_plans.join(', ')}
								</td>
								<td class="py-4 px-4 text-sm">{formatDate(coupon.valid_until)}</td>
								<td class="py-4 px-4">
									<span
										class="inline-block px-3 py-1 rounded-full text-xs font-semibold {coupon.is_active
											? 'bg-green-500/20 text-green-400'
											: 'bg-gray-500/20 text-gray-400'}"
									>
										{coupon.is_active ? 'Active' : 'Inactive'}
									</span>
								</td>
								<td class="py-4 px-4">
									<div class="flex gap-2">
										<button
											onclick={() => toggleActive(coupon.id, coupon.is_active)}
											class="px-3 py-1 text-xs bg-blue-500/20 text-blue-400 rounded hover:bg-blue-500/30 transition-colors"
										>
											{coupon.is_active ? 'Disable' : 'Enable'}
										</button>
										<button
											onclick={() => handleDeleteCoupon(coupon.id)}
											class="px-3 py-1 text-xs bg-red-500/20 text-red-400 rounded hover:bg-red-500/30 transition-colors"
										>
											Delete
										</button>
									</div>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</GlassCard>
	{/if}
</div>

<style>
	/* Add custom range slider styling */
	input[type='range']::-webkit-slider-thumb {
		appearance: none;
		width: 20px;
		height: 20px;
		background: linear-gradient(135deg, #ff6b35 0%, #f7931e 100%);
		border-radius: 50%;
		cursor: pointer;
	}

	input[type='range']::-moz-range-thumb {
		width: 20px;
		height: 20px;
		background: linear-gradient(135deg, #ff6b35 0%, #f7931e 100%);
		border-radius: 50%;
		cursor: pointer;
		border: none;
	}
</style>
