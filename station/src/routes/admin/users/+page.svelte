<script lang="ts">
	import { onMount } from 'svelte';
	import { getSupabaseClient } from '$lib/supabase/client';
	import type { Profile } from '$lib/supabase/types';

	const supabase = getSupabaseClient();

	let users = $state<Profile[]>([]);
	let filteredUsers = $state<Profile[]>([]);
	let loading = $state(true);
	let searchQuery = $state('');
	let planFilter = $state('all');
	let statusFilter = $state('all');
	let adminFilter = $state('all');
	let currentPage = $state(1);
	let pageSize = 20;

	let selectedUser = $state<Profile | null>(null);
	let showUserModal = $state(false);

	onMount(async () => {
		await loadUsers();
	});

	async function loadUsers() {
		try {
			const { data, error } = await supabase.from('profiles').select('*').order('created_at', { ascending: false });

			if (error) throw error;
			users = data || [];
			loading = false;
		} catch (error) {
			console.error('Failed to load users:', error);
			loading = false;
		}
	}

	$effect(() => {
		let result = users;

		// Search filter
		if (searchQuery) {
			const query = searchQuery.toLowerCase();
			result = result.filter(
				(u) =>
					u.email?.toLowerCase().includes(query) ||
					u.username?.toLowerCase().includes(query)
			);
		}

		// Plan filter
		if (planFilter !== 'all') {
			result = result.filter((u) => u.plan === planFilter);
		}

		// Status filter
		if (statusFilter === 'active') {
			result = result.filter((u) => !u.is_banned);
		} else if (statusFilter === 'banned') {
			result = result.filter((u) => u.is_banned);
		}

		// Admin filter
		if (adminFilter === 'yes') {
			result = result.filter((u) => u.is_admin);
		} else if (adminFilter === 'no') {
			result = result.filter((u) => !u.is_admin);
		}

		filteredUsers = result;
		currentPage = 1;
	});

	const paginatedUsers = $derived.by(() => {
		const start = (currentPage - 1) * pageSize;
		return filteredUsers.slice(start, start + pageSize);
	});

	const totalPages = $derived(Math.ceil(filteredUsers.length / pageSize));

	async function changePlan(userId: string, newPlan: string) {
		try {
			const { error } = await supabase.from('profiles').update({ plan: newPlan }).eq('id', userId);

			if (error) throw error;
			await loadUsers();
			alert('Plan updated successfully!');
		} catch (error) {
			console.error('Failed to update plan:', error);
			alert('Failed to update plan');
		}
	}

	async function toggleBan(userId: string, currentlyBanned: boolean) {
		try {
			const { error } = await supabase
				.from('profiles')
				.update({ is_banned: !currentlyBanned })
				.eq('id', userId);

			if (error) throw error;
			await loadUsers();
			alert(`User ${currentlyBanned ? 'unbanned' : 'banned'} successfully!`);
		} catch (error) {
			console.error('Failed to toggle ban:', error);
			alert('Failed to update ban status');
		}
	}

	async function deleteUser(userId: string) {
		if (!confirm('Are you sure you want to delete this user? This action cannot be undone.')) {
			return;
		}

		try {
			const { error } = await supabase.from('profiles').delete().eq('id', userId);

			if (error) throw error;
			await loadUsers();
			alert('User deleted successfully!');
		} catch (error) {
			console.error('Failed to delete user:', error);
			alert('Failed to delete user');
		}
	}

	function viewUserDetails(user: Profile) {
		selectedUser = user;
		showUserModal = true;
	}

	function exportToCSV() {
		const headers = ['Email', 'Username', 'Plan', 'Created', 'Last Active', 'Status', 'Admin'];
		const rows = filteredUsers.map((u) => [
			u.email || '',
			u.username || '',
			u.plan,
			new Date(u.created_at).toLocaleDateString(),
			u.last_active_at ? new Date(u.last_active_at).toLocaleDateString() : 'Never',
			u.is_banned ? 'Banned' : 'Active',
			u.is_admin ? 'Yes' : 'No'
		]);

		const csv = [headers, ...rows].map((row) => row.join(',')).join('\n');
		const blob = new Blob([csv], { type: 'text/csv' });
		const url = URL.createObjectURL(blob);
		const a = document.createElement('a');
		a.href = url;
		a.download = `users-${new Date().toISOString()}.csv`;
		a.click();
	}

	function formatDate(dateStr: string | null): string {
		if (!dateStr) return 'Never';
		return new Date(dateStr).toLocaleDateString();
	}

	function getPlanBadgeClass(plan: string): string {
		switch (plan) {
			case 'station':
				return 'badge-station';
			case 'scanner':
				return 'badge-scanner';
			case 'indicator':
				return 'badge-indicator';
			default:
				return 'badge-free';
		}
	}
</script>

<div class="users-page">
	<!-- Filters -->
	<div class="filters-bar">
		<input type="text" bind:value={searchQuery} placeholder="Search by email or username..." class="search-input" />

		<select bind:value={planFilter} class="filter-select">
			<option value="all">All Plans</option>
			<option value="free">Free</option>
			<option value="indicator">Indicator</option>
			<option value="scanner">Scanner</option>
			<option value="station">Station</option>
		</select>

		<select bind:value={statusFilter} class="filter-select">
			<option value="all">All Status</option>
			<option value="active">Active</option>
			<option value="banned">Banned</option>
		</select>

		<select bind:value={adminFilter} class="filter-select">
			<option value="all">Admin: All</option>
			<option value="yes">Admin: Yes</option>
			<option value="no">Admin: No</option>
		</select>

		<button onclick={exportToCSV} class="export-btn">Export CSV</button>
	</div>

	<!-- Results Count -->
	<div class="results-info">
		Showing {paginatedUsers.length} of {filteredUsers.length} users
	</div>

	<!-- Users Table -->
	{#if loading}
		<div class="loading">Loading users...</div>
	{:else}
		<div class="table-card">
			<div class="table-container">
				<table>
					<thead>
						<tr>
							<th>Email</th>
							<th>Username</th>
							<th>Plan</th>
							<th>Status</th>
							<th>Joined</th>
							<th>Last Active</th>
							<th>Actions</th>
						</tr>
					</thead>
					<tbody>
						{#each paginatedUsers as user}
							<tr>
								<td>{user.email}</td>
								<td>{user.username || '-'}</td>
								<td>
									<select
										value={user.plan}
										onchange={(e) => changePlan(user.id, e.currentTarget.value)}
										class="plan-select"
									>
										<option value="free">Free</option>
										<option value="indicator">Indicator</option>
										<option value="scanner">Scanner</option>
										<option value="station">Station</option>
									</select>
								</td>
								<td>
									<span class="status-badge {user.is_banned ? 'status-banned' : 'status-active'}">
										{user.is_banned ? 'Banned' : 'Active'}
									</span>
									{#if user.is_admin}
										<span class="admin-badge">👑 Admin</span>
									{/if}
								</td>
								<td>{formatDate(user.created_at)}</td>
								<td>{formatDate(user.last_active_at)}</td>
								<td>
									<div class="action-btns">
										<button onclick={() => viewUserDetails(user)} class="btn-view">View</button>
										<button
											onclick={() => toggleBan(user.id, user.is_banned)}
											class="btn-ban"
										>
											{user.is_banned ? 'Unban' : 'Ban'}
										</button>
										<button onclick={() => deleteUser(user.id)} class="btn-delete">Delete</button>
									</div>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>

			<!-- Pagination -->
			{#if totalPages > 1}
				<div class="pagination">
					<button onclick={() => (currentPage = Math.max(1, currentPage - 1))} disabled={currentPage === 1}>
						Previous
					</button>
					<span>Page {currentPage} of {totalPages}</span>
					<button
						onclick={() => (currentPage = Math.min(totalPages, currentPage + 1))}
						disabled={currentPage === totalPages}
					>
						Next
					</button>
				</div>
			{/if}
		</div>
	{/if}

	<!-- User Detail Modal -->
	{#if showUserModal && selectedUser}
		<div class="modal-overlay" onclick={() => (showUserModal = false)}>
			<div class="modal-content" onclick={(e) => e.stopPropagation()}>
				<div class="modal-header">
					<h2>User Details</h2>
					<button onclick={() => (showUserModal = false)} class="close-btn">✕</button>
				</div>
				<div class="modal-body">
					<div class="detail-section">
						<h3>Profile</h3>
						<div class="detail-grid">
							<div class="detail-item">
								<span class="detail-label">Email:</span>
								<span>{selectedUser.email}</span>
							</div>
							<div class="detail-item">
								<span class="detail-label">Username:</span>
								<span>{selectedUser.username || '-'}</span>
							</div>
							<div class="detail-item">
								<span class="detail-label">Plan:</span>
								<span class="plan-badge {getPlanBadgeClass(selectedUser.plan)}">{selectedUser.plan}</span>
							</div>
							<div class="detail-item">
								<span class="detail-label">Bacon Points:</span>
								<span>{selectedUser.bacon_points}</span>
							</div>
							<div class="detail-item">
								<span class="detail-label">Streak:</span>
								<span>{selectedUser.streak_days} days</span>
							</div>
							<div class="detail-item">
								<span class="detail-label">Badges:</span>
								<span>{selectedUser.badges.join(', ') || 'None'}</span>
							</div>
							<div class="detail-item">
								<span class="detail-label">Joined:</span>
								<span>{formatDate(selectedUser.created_at)}</span>
							</div>
							<div class="detail-item">
								<span class="detail-label">Last Active:</span>
								<span>{formatDate(selectedUser.last_active_at)}</span>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>

<style>
	.users-page {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.filters-bar {
		display: flex;
		gap: 1rem;
		flex-wrap: wrap;
	}

	.search-input {
		flex: 1;
		min-width: 250px;
		padding: 0.75rem 1rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		color: #fff;
		font-size: 0.9375rem;
	}

	.search-input::placeholder {
		color: rgba(255, 255, 255, 0.4);
	}

	.filter-select {
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

	.results-info {
		color: rgba(255, 255, 255, 0.6);
		font-size: 0.875rem;
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

	.plan-select {
		padding: 0.375rem 0.75rem;
		background: rgba(255, 255, 255, 0.1);
		border: 1px solid rgba(255, 255, 255, 0.2);
		border-radius: 8px;
		color: #fff;
		font-size: 0.875rem;
		cursor: pointer;
	}

	.status-badge,
	.admin-badge {
		display: inline-block;
		padding: 0.25rem 0.75rem;
		border-radius: 12px;
		font-size: 0.75rem;
		font-weight: 600;
		margin-right: 0.5rem;
	}

	.status-active {
		background: rgba(34, 197, 94, 0.2);
		color: #4ade80;
	}

	.status-banned {
		background: rgba(239, 68, 68, 0.2);
		color: #f87171;
	}

	.admin-badge {
		background: rgba(255, 215, 0, 0.2);
		color: #ffd700;
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

	.btn-view {
		background: rgba(59, 130, 246, 0.2);
		color: #60a5fa;
	}

	.btn-view:hover {
		background: rgba(59, 130, 246, 0.3);
	}

	.btn-ban {
		background: rgba(249, 115, 22, 0.2);
		color: #fb923c;
	}

	.btn-ban:hover {
		background: rgba(249, 115, 22, 0.3);
	}

	.btn-delete {
		background: rgba(239, 68, 68, 0.2);
		color: #f87171;
	}

	.btn-delete:hover {
		background: rgba(239, 68, 68, 0.3);
	}

	.pagination {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 1rem;
		padding: 1.5rem;
		border-top: 1px solid rgba(255, 255, 255, 0.1);
	}

	.pagination button {
		padding: 0.5rem 1rem;
		background: rgba(255, 255, 255, 0.1);
		border: none;
		border-radius: 8px;
		color: #fff;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.pagination button:hover:not(:disabled) {
		background: rgba(255, 255, 255, 0.15);
	}

	.pagination button:disabled {
		opacity: 0.3;
		cursor: not-allowed;
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 1rem;
	}

	.modal-content {
		background: rgba(20, 20, 30, 0.95);
		backdrop-filter: blur(20px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 16px;
		max-width: 600px;
		width: 100%;
		max-height: 90vh;
		overflow-y: auto;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	.modal-header h2 {
		margin: 0;
		font-size: 1.5rem;
	}

	.close-btn {
		background: none;
		border: none;
		color: #fff;
		font-size: 1.5rem;
		cursor: pointer;
		padding: 0.5rem;
	}

	.modal-body {
		padding: 1.5rem;
	}

	.detail-section h3 {
		margin: 0 0 1rem 0;
		font-size: 1.125rem;
	}

	.detail-grid {
		display: grid;
		gap: 1rem;
	}

	.detail-item {
		display: flex;
		justify-content: space-between;
		padding: 0.75rem;
		background: rgba(255, 255, 255, 0.05);
		border-radius: 8px;
	}

	.detail-label {
		color: rgba(255, 255, 255, 0.6);
		font-weight: 600;
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

	@media (max-width: 768px) {
		.filters-bar {
			flex-direction: column;
		}

		.search-input {
			width: 100%;
		}

		.table-container {
			overflow-x: scroll;
		}

		.action-btns {
			flex-direction: column;
		}
	}
</style>
