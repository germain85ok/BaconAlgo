<script lang="ts">
	import { onMount } from 'svelte';
	import { getSupabaseClient } from '$lib/supabase/client';
	import type { PromoCode, PromoCodeType, UserPlan } from '$lib/supabase/types';

	const supabase = getSupabaseClient();

	let promos = $state<PromoCode[]>([]);
	let loading = $state(true);
	let showCreateForm = $state(false);

	// Form fields
	let formData = $state({
		code: '',
		type: 'full_access' as PromoCodeType,
		discount_percent: 0,
		trial_days: 0,
		plan_grant: 'station' as UserPlan,
		max_uses: null as number | null,
		expires_at: ''
	});

	onMount(async () => {
		await loadPromos();
	});

	async function loadPromos() {
		try {
			const { data, error } = await supabase.from('promo_codes').select('*').order('created_at', { ascending: false });

			if (error) throw error;
			promos = data || [];
			loading = false;
		} catch (error) {
			console.error('Failed to load promo codes:', error);
			loading = false;
		}
	}

	async function createPromo() {
		try {
			const payload: any = {
				code: formData.code.toUpperCase(),
				type: formData.type,
				is_active: true
			};

			if (formData.type === 'full_access') {
				payload.plan_grant = formData.plan_grant;
			} else if (formData.type === 'trial') {
				payload.trial_days = formData.trial_days;
			} else if (formData.type === 'discount') {
				payload.discount_percent = formData.discount_percent;
			}

			if (formData.max_uses) {
				payload.max_uses = formData.max_uses;
			}

			if (formData.expires_at) {
				payload.expires_at = new Date(formData.expires_at).toISOString();
			}

			const { error } = await supabase.from('promo_codes').insert(payload);

			if (error) throw error;

			alert('Promo code created successfully!');
			resetForm();
			showCreateForm = false;
			await loadPromos();
		} catch (error: any) {
			console.error('Failed to create promo:', error);
			alert(error.message || 'Failed to create promo code');
		}
	}

	async function toggleActive(id: string, isActive: boolean) {
		try {
			const { error } = await supabase.from('promo_codes').update({ is_active: !isActive }).eq('id', id);

			if (error) throw error;
			await loadPromos();
		} catch (error) {
			console.error('Failed to toggle active:', error);
			alert('Failed to update promo code');
		}
	}

	async function deletePromo(id: string) {
		if (!confirm('Are you sure you want to delete this promo code?')) {
			return;
		}

		try {
			const { error } = await supabase.from('promo_codes').delete().eq('id', id);

			if (error) throw error;
			await loadPromos();
			alert('Promo code deleted successfully!');
		} catch (error) {
			console.error('Failed to delete promo:', error);
			alert('Failed to delete promo code');
		}
	}

	function resetForm() {
		formData = {
			code: '',
			type: 'full_access',
			discount_percent: 0,
			trial_days: 0,
			plan_grant: 'station',
			max_uses: null,
			expires_at: ''
		};
	}

	function formatDate(dateStr: string | null): string {
		if (!dateStr) return 'Never';
		return new Date(dateStr).toLocaleDateString();
	}

	function getTypeLabel(promo: PromoCode): string {
		if (promo.type === 'full_access') return `Full Access (${promo.plan_grant})`;
		if (promo.type === 'trial') return `Trial (${promo.trial_days} days)`;
		if (promo.type === 'discount') return `${promo.discount_percent}% Off`;
		return promo.type;
	}
</script>

<div class="promos-page">
	<!-- Header -->
	<div class="page-header">
		<h2>Promo Codes</h2>
		<button onclick={() => (showCreateForm = !showCreateForm)} class="create-btn">
			{showCreateForm ? 'Cancel' : '+ Create New Promo'}
		</button>
	</div>

	<!-- Create Form -->
	{#if showCreateForm}
		<div class="create-form">
			<h3>Create New Promo Code</h3>

			<div class="form-grid">
				<!-- Code Name -->
				<div class="form-field">
					<label>Code Name</label>
					<input type="text" bind:value={formData.code} placeholder="SUMMER2024" class="input" />
				</div>

				<!-- Type -->
				<div class="form-field">
					<label>Type</label>
					<div class="radio-group">
						<label class="radio-label">
							<input type="radio" bind:group={formData.type} value="full_access" />
							<span>Full Access</span>
						</label>
						<label class="radio-label">
							<input type="radio" bind:group={formData.type} value="trial" />
							<span>Trial Extension</span>
						</label>
						<label class="radio-label">
							<input type="radio" bind:group={formData.type} value="discount" />
							<span>Discount %</span>
						</label>
					</div>
				</div>

				<!-- Conditional Fields -->
				{#if formData.type === 'full_access'}
					<div class="form-field">
						<label>Plan Grant</label>
						<select bind:value={formData.plan_grant} class="select">
							<option value="indicator">Indicator</option>
							<option value="scanner">Scanner</option>
							<option value="station">Station</option>
						</select>
					</div>
				{/if}

				{#if formData.type === 'trial'}
					<div class="form-field">
						<label>Trial Days</label>
						<input type="number" bind:value={formData.trial_days} min="1" max="365" class="input" />
					</div>
				{/if}

				{#if formData.type === 'discount'}
					<div class="form-field">
						<label>Discount % ({formData.discount_percent}%)</label>
						<input type="range" bind:value={formData.discount_percent} min="5" max="100" step="5" class="slider" />
					</div>
				{/if}

				<!-- Max Uses -->
				<div class="form-field">
					<label>Max Uses (optional)</label>
					<input type="number" bind:value={formData.max_uses} placeholder="Unlimited" min="1" class="input" />
				</div>

				<!-- Expiry Date -->
				<div class="form-field">
					<label>Expiry Date (optional)</label>
					<input type="date" bind:value={formData.expires_at} class="input" />
				</div>
			</div>

			<button onclick={createPromo} class="submit-btn">Create Promo Code</button>
		</div>
	{/if}

	<!-- Promo Codes Table -->
	{#if loading}
		<div class="loading">Loading promo codes...</div>
	{:else}
		<div class="table-card">
			<div class="table-container">
				<table>
					<thead>
						<tr>
							<th>Code</th>
							<th>Type</th>
							<th>Uses</th>
							<th>Expires</th>
							<th>Status</th>
							<th>Created</th>
							<th>Actions</th>
						</tr>
					</thead>
					<tbody>
						{#each promos as promo}
							<tr>
								<td><code class="code">{promo.code}</code></td>
								<td>{getTypeLabel(promo)}</td>
								<td>
									{promo.current_uses} / {promo.max_uses || '∞'}
								</td>
								<td>{formatDate(promo.expires_at)}</td>
								<td>
									<span class="status-badge {promo.is_active ? 'status-active' : 'status-inactive'}">
										{promo.is_active ? 'Active' : 'Inactive'}
									</span>
								</td>
								<td>{formatDate(promo.created_at)}</td>
								<td>
									<div class="action-btns">
										<button onclick={() => toggleActive(promo.id, promo.is_active)} class="btn-toggle">
											{promo.is_active ? 'Disable' : 'Enable'}
										</button>
										<button onclick={() => deletePromo(promo.id)} class="btn-delete">Delete</button>
									</div>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</div>
	{/if}
</div>

<style>
	.promos-page {
		display: flex;
		flex-direction: column;
		gap: 2rem;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.page-header h2 {
		margin: 0;
		font-size: 1.5rem;
	}

	.create-btn {
		padding: 0.75rem 1.5rem;
		background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
		border: none;
		border-radius: 12px;
		color: #fff;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.create-btn:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 30px rgba(240, 147, 251, 0.3);
	}

	.create-form {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 16px;
		padding: 2rem;
	}

	.create-form h3 {
		margin: 0 0 1.5rem 0;
		font-size: 1.25rem;
	}

	.form-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		gap: 1.5rem;
		margin-bottom: 1.5rem;
	}

	.form-field {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.form-field label {
		font-size: 0.875rem;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.8);
	}

	.input,
	.select {
		padding: 0.75rem 1rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		color: #fff;
		font-size: 0.9375rem;
	}

	.input::placeholder {
		color: rgba(255, 255, 255, 0.4);
	}

	.radio-group {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.radio-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
	}

	.radio-label input[type='radio'] {
		cursor: pointer;
	}

	.slider {
		width: 100%;
		height: 6px;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 3px;
		outline: none;
		cursor: pointer;
	}

	.slider::-webkit-slider-thumb {
		appearance: none;
		width: 20px;
		height: 20px;
		background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
		border-radius: 50%;
		cursor: pointer;
	}

	.submit-btn {
		padding: 0.875rem 2rem;
		background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
		border: none;
		border-radius: 12px;
		color: #fff;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
		width: 100%;
		font-size: 1rem;
	}

	.submit-btn:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 30px rgba(240, 147, 251, 0.3);
	}

	.loading {
		text-align: center;
		padding: 4rem;
		color: rgba(255, 255, 255, 0.6);
	}

	.table-card {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 16px;
		overflow: hidden;
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
		padding: 1rem;
		font-size: 0.875rem;
		color: rgba(255, 255, 255, 0.6);
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		background: rgba(255, 255, 255, 0.02);
	}

	td {
		padding: 1rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.05);
	}

	.code {
		background: rgba(255, 255, 255, 0.1);
		padding: 0.25rem 0.75rem;
		border-radius: 6px;
		font-family: monospace;
		font-weight: 600;
	}

	.status-badge {
		display: inline-block;
		padding: 0.25rem 0.75rem;
		border-radius: 12px;
		font-size: 0.75rem;
		font-weight: 600;
	}

	.status-active {
		background: rgba(34, 197, 94, 0.2);
		color: #4ade80;
	}

	.status-inactive {
		background: rgba(107, 114, 128, 0.2);
		color: #9ca3af;
	}

	.action-btns {
		display: flex;
		gap: 0.5rem;
	}

	.action-btns button {
		padding: 0.375rem 0.75rem;
		border: none;
		border-radius: 8px;
		font-size: 0.75rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.btn-toggle {
		background: rgba(59, 130, 246, 0.2);
		color: #60a5fa;
	}

	.btn-toggle:hover {
		background: rgba(59, 130, 246, 0.3);
	}

	.btn-delete {
		background: rgba(239, 68, 68, 0.2);
		color: #f87171;
	}

	.btn-delete:hover {
		background: rgba(239, 68, 68, 0.3);
	}

	@media (max-width: 768px) {
		.page-header {
			flex-direction: column;
			gap: 1rem;
			align-items: flex-start;
		}

		.create-btn {
			width: 100%;
		}

		.form-grid {
			grid-template-columns: 1fr;
		}

		.table-container {
			overflow-x: scroll;
		}
	}
</style>
