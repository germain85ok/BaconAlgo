<script lang="ts">
	import { onMount } from 'svelte';
	import { getSupabaseClient } from '$lib/supabase/client';

	const supabase = getSupabaseClient();

	let channel = $state('general');
	let messageType = $state('custom');
	let customMessage = $state('');
	let embedTitle = $state('');
	let embedDescription = $state('');
	let embedColor = $state('#f093fb');
	let embedFields = $state<{ name: string; value: string }[]>([]);
	let scheduledDate = $state('');
	let sentMessages = $state<any[]>([]);
	let loading = $state(false);

	onMount(async () => {
		// In real app, load sent messages from database
	});

	function addField() {
		embedFields = [...embedFields, { name: '', value: '' }];
	}

	function removeField(index: number) {
		embedFields = embedFields.filter((_, i) => i !== index);
	}

	async function sendMessage() {
		if (!customMessage && !embedTitle) {
			alert('Please enter a message or embed title');
			return;
		}

		loading = true;

		try {
			// In real app, call Discord webhook API
			const payload = {
				channel,
				type: messageType,
				content: customMessage || null,
				embed: embedTitle
					? {
							title: embedTitle,
							description: embedDescription,
							color: parseInt(embedColor.replace('#', ''), 16),
							fields: embedFields.filter((f) => f.name && f.value)
						}
					: null,
				scheduled_at: scheduledDate || null
			};

			console.log('Sending Discord message:', payload);

			// Mock success
			alert(scheduledDate ? 'Message scheduled successfully!' : 'Message sent successfully!');

			// Reset form
			customMessage = '';
			embedTitle = '';
			embedDescription = '';
			embedFields = [];
			scheduledDate = '';
		} catch (error) {
			console.error('Failed to send message:', error);
			alert('Failed to send message');
		} finally {
			loading = false;
		}
	}

	function hexToRgb(hex: string): { r: number; g: number; b: number } {
		const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
		return result
			? {
					r: parseInt(result[1], 16),
					g: parseInt(result[2], 16),
					b: parseInt(result[3], 16)
				}
			: { r: 0, g: 0, b: 0 };
	}
</script>

<div class="discord-page">
	<div class="content-grid">
		<!-- Form Section -->
		<div class="form-section">
			<h2>Send Discord Alert</h2>

			<!-- Channel Selector -->
			<div class="form-field">
				<label>Channel</label>
				<select bind:value={channel} class="select">
					<option value="general">General</option>
					<option value="signals">Signals</option>
					<option value="announcements">Announcements</option>
					<option value="vip">VIP</option>
				</select>
			</div>

			<!-- Message Type -->
			<div class="form-field">
				<label>Message Type</label>
				<div class="radio-group">
					<label class="radio-label">
						<input type="radio" bind:group={messageType} value="signal" />
						<span>Signal Alert</span>
					</label>
					<label class="radio-label">
						<input type="radio" bind:group={messageType} value="announcement" />
						<span>Announcement</span>
					</label>
					<label class="radio-label">
						<input type="radio" bind:group={messageType} value="custom" />
						<span>Custom</span>
					</label>
				</div>
			</div>

			<!-- Custom Message -->
			<div class="form-field">
				<label>Message Content (optional)</label>
				<textarea bind:value={customMessage} placeholder="Enter your message..." class="textarea" rows="4"></textarea>
			</div>

			<!-- Embed Builder -->
			<div class="embed-section">
				<h3>Embed (optional)</h3>

				<div class="form-field">
					<label>Title</label>
					<input type="text" bind:value={embedTitle} placeholder="Embed title" class="input" />
				</div>

				<div class="form-field">
					<label>Description</label>
					<textarea
						bind:value={embedDescription}
						placeholder="Embed description"
						class="textarea"
						rows="3"
					></textarea>
				</div>

				<div class="form-field">
					<label>Color</label>
					<div class="color-picker">
						<input type="color" bind:value={embedColor} class="color-input" />
						<input type="text" bind:value={embedColor} placeholder="#f093fb" class="input color-hex" />
					</div>
				</div>

				<!-- Fields -->
				<div class="form-field">
					<label>Fields</label>
					{#each embedFields as field, i}
						<div class="field-row">
							<input type="text" bind:value={field.name} placeholder="Field name" class="input field-input" />
							<input type="text" bind:value={field.value} placeholder="Field value" class="input field-input" />
							<button onclick={() => removeField(i)} class="btn-remove">✕</button>
						</div>
					{/each}
					<button onclick={addField} class="btn-add">+ Add Field</button>
				</div>
			</div>

			<!-- Schedule -->
			<div class="form-field">
				<label>Schedule Send (optional)</label>
				<input type="datetime-local" bind:value={scheduledDate} class="input" />
			</div>

			<!-- Send Button -->
			<button onclick={sendMessage} class="send-btn" disabled={loading}>
				{loading ? 'Sending...' : scheduledDate ? 'Schedule Message' : 'Send Now'}
			</button>
		</div>

		<!-- Preview Section -->
		<div class="preview-section">
			<h3>Preview</h3>
			<div class="discord-preview">
				{#if customMessage}
					<div class="preview-message">{customMessage}</div>
				{/if}

				{#if embedTitle || embedDescription}
					<div class="preview-embed" style="border-left: 4px solid {embedColor}">
						{#if embedTitle}
							<div class="embed-title">{embedTitle}</div>
						{/if}
						{#if embedDescription}
							<div class="embed-description">{embedDescription}</div>
						{/if}
						{#if embedFields.length > 0}
							<div class="embed-fields">
								{#each embedFields.filter((f) => f.name && f.value) as field}
									<div class="embed-field">
										<div class="field-name">{field.name}</div>
										<div class="field-value">{field.value}</div>
									</div>
								{/each}
							</div>
						{/if}
					</div>
				{/if}

				{#if !customMessage && !embedTitle && !embedDescription}
					<div class="preview-placeholder">Your message preview will appear here...</div>
				{/if}
			</div>
		</div>
	</div>

	<!-- Sent Messages History -->
	<div class="history-section">
		<h3>Sent Messages</h3>
		<div class="table-card">
			<div class="table-container">
				<table>
					<thead>
						<tr>
							<th>Channel</th>
							<th>Type</th>
							<th>Content Preview</th>
							<th>Sent At</th>
							<th>Status</th>
						</tr>
					</thead>
					<tbody>
						<tr>
							<td colspan="5" class="no-data">No messages sent yet</td>
						</tr>
					</tbody>
				</table>
			</div>
		</div>
	</div>
</div>

<style>
	.discord-page {
		display: flex;
		flex-direction: column;
		gap: 2rem;
	}

	.content-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 2rem;
	}

	.form-section,
	.preview-section {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 16px;
		padding: 2rem;
	}

	.form-section h2,
	.preview-section h3 {
		margin: 0 0 1.5rem 0;
	}

	.form-field {
		margin-bottom: 1.5rem;
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

	.embed-section {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		padding: 1.5rem;
		margin-bottom: 1.5rem;
	}

	.embed-section h3 {
		margin: 0 0 1rem 0;
		font-size: 1.125rem;
	}

	.color-picker {
		display: flex;
		gap: 0.75rem;
		align-items: center;
	}

	.color-input {
		width: 60px;
		height: 40px;
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 8px;
		background: transparent;
		cursor: pointer;
	}

	.color-hex {
		flex: 1;
	}

	.field-row {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 0.5rem;
	}

	.field-input {
		flex: 1;
	}

	.btn-remove {
		padding: 0.75rem 1rem;
		background: rgba(239, 68, 68, 0.2);
		border: none;
		border-radius: 8px;
		color: #f87171;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.btn-remove:hover {
		background: rgba(239, 68, 68, 0.3);
	}

	.btn-add {
		padding: 0.5rem 1rem;
		background: rgba(255, 255, 255, 0.1);
		border: 1px solid rgba(255, 255, 255, 0.2);
		border-radius: 8px;
		color: #fff;
		cursor: pointer;
		transition: all 0.2s ease;
		font-size: 0.875rem;
	}

	.btn-add:hover {
		background: rgba(255, 255, 255, 0.15);
	}

	.send-btn {
		width: 100%;
		padding: 1rem;
		background: linear-gradient(135deg, #5865f2 0%, #7289da 100%);
		border: none;
		border-radius: 12px;
		color: #fff;
		font-weight: 600;
		font-size: 1rem;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.send-btn:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 10px 30px rgba(88, 101, 242, 0.4);
	}

	.send-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.discord-preview {
		background: #2b2d31;
		border-radius: 8px;
		padding: 1.5rem;
		min-height: 200px;
	}

	.preview-message {
		color: #fff;
		margin-bottom: 1rem;
		line-height: 1.5;
	}

	.preview-embed {
		background: #2b2d31;
		border-radius: 4px;
		padding: 1rem;
		margin-top: 0.5rem;
	}

	.embed-title {
		color: #fff;
		font-weight: 600;
		font-size: 1rem;
		margin-bottom: 0.5rem;
	}

	.embed-description {
		color: #b5bac1;
		font-size: 0.875rem;
		line-height: 1.4;
		margin-bottom: 0.75rem;
	}

	.embed-fields {
		display: grid;
		gap: 0.75rem;
		margin-top: 0.75rem;
	}

	.embed-field {
		margin-bottom: 0.5rem;
	}

	.field-name {
		color: #fff;
		font-weight: 600;
		font-size: 0.875rem;
		margin-bottom: 0.25rem;
	}

	.field-value {
		color: #b5bac1;
		font-size: 0.875rem;
	}

	.preview-placeholder {
		color: rgba(255, 255, 255, 0.4);
		text-align: center;
		padding: 3rem 1rem;
	}

	.history-section h3 {
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

	@media (max-width: 768px) {
		.content-grid {
			grid-template-columns: 1fr;
		}

		.field-row {
			flex-direction: column;
		}
	}
</style>
