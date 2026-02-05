<script lang="ts">
	import { onMount } from 'svelte';
	import { getSupabaseClient } from '$lib/supabase/client';

	const supabase = getSupabaseClient();

	let formData = $state({
		symbol: '',
		direction: 'LONG' as 'LONG' | 'SHORT',
		horizon: 'day' as 'scalp' | 'day' | 'swing' | 'long',
		score: 75,
		grade: 'A' as 'S' | 'A' | 'B' | 'C',
		entry_price: 0,
		stop_loss: 0,
		tp1: 0,
		tp2: 0,
		tp3: 0,
		smc_tags: [] as string[],
		notes: ''
	});

	let pushToDiscord = $state(true);
	let recentSignals = $state<any[]>([]);
	let loading = $state(false);

	const smcTagOptions = ['FVG', 'OB', 'BOS', 'CHoCH', 'Liquidity Sweep', 'Premium/Discount', 'Imbalance'];

	onMount(async () => {
		await loadRecentSignals();
	});

	async function loadRecentSignals() {
		try {
			// In real app, query manual_signals table
			const { data, error } = await supabase
				.from('signals')
				.select('*')
				.order('created_at', { ascending: false })
				.limit(20);

			if (error) throw error;
			recentSignals = data || [];
		} catch (error) {
			console.error('Failed to load recent signals:', error);
		}
	}

	function toggleSmcTag(tag: string) {
		if (formData.smc_tags.includes(tag)) {
			formData.smc_tags = formData.smc_tags.filter((t) => t !== tag);
		} else {
			formData.smc_tags = [...formData.smc_tags, tag];
		}
	}

	async function pushSignal() {
		if (!formData.symbol) {
			alert('Please enter a symbol');
			return;
		}

		loading = true;

		try {
			const payload = {
				ticker: formData.symbol.toUpperCase(),
				direction: formData.direction === 'LONG' ? 'BUY' : 'SELL',
				grade: formData.grade,
				score: formData.score,
				indicators: {
					horizon: formData.horizon,
					entry_price: formData.entry_price || null,
					stop_loss: formData.stop_loss || null,
					take_profit_1: formData.tp1 || null,
					take_profit_2: formData.tp2 || null,
					take_profit_3: formData.tp3 || null,
					smc_tags: formData.smc_tags,
					notes: formData.notes,
					manual_push: true
				}
			};

			const { error } = await supabase.from('signals').insert(payload);

			if (error) throw error;

			alert('Signal pushed successfully!');

			// Reset form
			formData = {
				symbol: '',
				direction: 'LONG',
				horizon: 'day',
				score: 75,
				grade: 'A',
				entry_price: 0,
				stop_loss: 0,
				tp1: 0,
				tp2: 0,
				tp3: 0,
				smc_tags: [],
				notes: ''
			};

			await loadRecentSignals();
		} catch (error) {
			console.error('Failed to push signal:', error);
			alert('Failed to push signal');
		} finally {
			loading = false;
		}
	}

	function formatDate(dateStr: string): string {
		return new Date(dateStr).toLocaleString();
	}

	function getGradeColor(grade: string): string {
		switch (grade) {
			case 'S':
				return '#ffd700';
			case 'A':
				return '#4ade80';
			case 'B':
				return '#60a5fa';
			case 'C':
				return '#fb923c';
			default:
				return '#9ca3af';
		}
	}
</script>

<div class="signals-page">
	<!-- Create Signal Form -->
	<div class="form-card">
		<h2>Push Manual Signal</h2>

		<div class="form-grid">
			<!-- Symbol -->
			<div class="form-field full-width">
				<label>Symbol</label>
				<input type="text" bind:value={formData.symbol} placeholder="BTC, ETH, AAPL, etc." class="input" />
			</div>

			<!-- Direction -->
			<div class="form-field">
				<label>Direction</label>
				<div class="radio-group horizontal">
					<label class="radio-label">
						<input type="radio" bind:group={formData.direction} value="LONG" />
						<span>🟢 LONG</span>
					</label>
					<label class="radio-label">
						<input type="radio" bind:group={formData.direction} value="SHORT" />
						<span>🔴 SHORT</span>
					</label>
				</div>
			</div>

			<!-- Horizon -->
			<div class="form-field">
				<label>Horizon</label>
				<select bind:value={formData.horizon} class="select">
					<option value="scalp">Scalp</option>
					<option value="day">Day</option>
					<option value="swing">Swing</option>
					<option value="long">Long</option>
				</select>
			</div>

			<!-- Score -->
			<div class="form-field">
				<label>Score ({formData.score}/100)</label>
				<input type="range" bind:value={formData.score} min="0" max="100" step="5" class="slider" />
			</div>

			<!-- Grade -->
			<div class="form-field">
				<label>Grade</label>
				<div class="radio-group horizontal">
					<label class="radio-label">
						<input type="radio" bind:group={formData.grade} value="S" />
						<span>S</span>
					</label>
					<label class="radio-label">
						<input type="radio" bind:group={formData.grade} value="A" />
						<span>A</span>
					</label>
					<label class="radio-label">
						<input type="radio" bind:group={formData.grade} value="B" />
						<span>B</span>
					</label>
					<label class="radio-label">
						<input type="radio" bind:group={formData.grade} value="C" />
						<span>C</span>
					</label>
				</div>
			</div>

			<!-- Price Levels -->
			<div class="form-field">
				<label>Entry Price</label>
				<input type="number" bind:value={formData.entry_price} step="0.01" class="input" placeholder="0.00" />
			</div>

			<div class="form-field">
				<label>Stop Loss</label>
				<input type="number" bind:value={formData.stop_loss} step="0.01" class="input" placeholder="0.00" />
			</div>

			<div class="form-field">
				<label>Take Profit 1</label>
				<input type="number" bind:value={formData.tp1} step="0.01" class="input" placeholder="0.00" />
			</div>

			<div class="form-field">
				<label>Take Profit 2</label>
				<input type="number" bind:value={formData.tp2} step="0.01" class="input" placeholder="0.00" />
			</div>

			<div class="form-field">
				<label>Take Profit 3</label>
				<input type="number" bind:value={formData.tp3} step="0.01" class="input" placeholder="0.00" />
			</div>

			<!-- SMC Tags -->
			<div class="form-field full-width">
				<label>SMC Tags</label>
				<div class="checkbox-group">
					{#each smcTagOptions as tag}
						<label class="checkbox-label">
							<input type="checkbox" checked={formData.smc_tags.includes(tag)} onchange={() => toggleSmcTag(tag)} />
							<span>{tag}</span>
						</label>
					{/each}
				</div>
			</div>

			<!-- Notes -->
			<div class="form-field full-width">
				<label>Notes</label>
				<textarea bind:value={formData.notes} class="textarea" rows="3" placeholder="Additional notes..."></textarea>
			</div>

			<!-- Push to Discord -->
			<div class="form-field full-width">
				<label class="checkbox-label">
					<input type="checkbox" bind:checked={pushToDiscord} />
					<span>Also push to Discord #signals channel</span>
				</label>
			</div>
		</div>

		<button onclick={pushSignal} class="push-btn" disabled={loading}>
			{loading ? 'Pushing...' : '📡 Push Signal to Scanner'}
		</button>
	</div>

	<!-- Recent Signals -->
	<div class="recent-signals">
		<h3>Recent Manual Signals</h3>
		<div class="table-card">
			<div class="table-container">
				<table>
					<thead>
						<tr>
							<th>Symbol</th>
							<th>Direction</th>
							<th>Grade</th>
							<th>Score</th>
							<th>Horizon</th>
							<th>Entry</th>
							<th>Pushed At</th>
						</tr>
					</thead>
					<tbody>
						{#if recentSignals.length === 0}
							<tr>
								<td colspan="7" class="no-data">No manual signals yet</td>
							</tr>
						{:else}
							{#each recentSignals as signal}
								<tr>
									<td><strong>{signal.ticker}</strong></td>
									<td>
										<span class="direction-badge {signal.direction === 'BUY' ? 'direction-long' : 'direction-short'}">
											{signal.direction === 'BUY' ? '🟢 LONG' : '🔴 SHORT'}
										</span>
									</td>
									<td>
										<span class="grade-badge" style="background: {getGradeColor(signal.grade)}">
											{signal.grade}
										</span>
									</td>
									<td>{signal.score}</td>
									<td>{signal.indicators?.horizon || '-'}</td>
									<td>{signal.indicators?.entry_price || '-'}</td>
									<td>{formatDate(signal.created_at)}</td>
								</tr>
							{/each}
						{/if}
					</tbody>
				</table>
			</div>
		</div>
	</div>
</div>

<style>
	.signals-page {
		display: flex;
		flex-direction: column;
		gap: 2rem;
	}

	.form-card {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 16px;
		padding: 2rem;
	}

	.form-card h2 {
		margin: 0 0 1.5rem 0;
		font-size: 1.5rem;
	}

	.form-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		gap: 1.5rem;
		margin-bottom: 1.5rem;
	}

	.full-width {
		grid-column: 1 / -1;
	}

	.form-field label {
		display: block;
		margin-bottom: 0.5rem;
		font-size: 0.875rem;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.8);
	}

	.input,
	.select,
	.textarea {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		color: #fff;
		font-size: 0.9375rem;
		font-family: inherit;
	}

	.input::placeholder,
	.textarea::placeholder {
		color: rgba(255, 255, 255, 0.4);
	}

	.textarea {
		resize: vertical;
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

	.radio-group {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.radio-group.horizontal {
		flex-direction: row;
		flex-wrap: wrap;
	}

	.radio-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
	}

	.radio-label input {
		cursor: pointer;
	}

	.checkbox-group {
		display: flex;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
	}

	.checkbox-label input {
		cursor: pointer;
	}

	.push-btn {
		width: 100%;
		padding: 1rem;
		background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
		border: none;
		border-radius: 12px;
		color: #fff;
		font-weight: 600;
		font-size: 1rem;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.push-btn:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 10px 30px rgba(240, 147, 251, 0.3);
	}

	.push-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.recent-signals h3 {
		margin: 0 0 1rem 0;
		font-size: 1.25rem;
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

	.no-data {
		text-align: center;
		color: rgba(255, 255, 255, 0.4);
		padding: 2rem;
	}

	.direction-badge {
		padding: 0.25rem 0.75rem;
		border-radius: 12px;
		font-size: 0.75rem;
		font-weight: 600;
		display: inline-block;
	}

	.direction-long {
		background: rgba(34, 197, 94, 0.2);
		color: #4ade80;
	}

	.direction-short {
		background: rgba(239, 68, 68, 0.2);
		color: #f87171;
	}

	.grade-badge {
		display: inline-block;
		width: 32px;
		height: 32px;
		line-height: 32px;
		text-align: center;
		border-radius: 50%;
		font-weight: 700;
		color: #000;
	}

	@media (max-width: 768px) {
		.form-grid {
			grid-template-columns: 1fr;
		}

		.radio-group.horizontal {
			flex-direction: column;
		}

		.table-container {
			overflow-x: scroll;
		}
	}
</style>
