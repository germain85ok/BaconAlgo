<script lang="ts">
	interface StreamDay {
		day: string;
		time: string;
		isToday: boolean;
	}

	const schedule: StreamDay[] = [
		{ day: 'Monday', time: '9:00 AM - 12:00 PM EST', isToday: false },
		{ day: 'Tuesday', time: '9:00 AM - 12:00 PM EST', isToday: false },
		{ day: 'Wednesday', time: '2:00 PM - 5:00 PM EST', isToday: false },
		{ day: 'Thursday', time: '9:00 AM - 12:00 PM EST', isToday: false },
		{ day: 'Friday', time: '9:00 AM - 1:00 PM EST', isToday: false },
		{ day: 'Saturday', time: 'No Stream', isToday: false },
		{ day: 'Sunday', time: 'No Stream', isToday: false }
	];

	const today = new Date().getDay();
	const dayNames = ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'];
	const todayName = dayNames[today];

	$: highlightedSchedule = schedule.map((day) => ({
		...day,
		isToday: day.day === todayName
	}));
</script>

<div class="stream-schedule">
	<div class="header">
		<h3>
			<svg
				width="16"
				height="16"
				viewBox="0 0 16 16"
				fill="none"
				xmlns="http://www.w3.org/2000/svg"
			>
				<rect x="2" y="3" width="12" height="11" rx="2" stroke="currentColor" stroke-width="1.5" />
				<path d="M2 6H14" stroke="currentColor" stroke-width="1.5" />
				<path d="M5 1V4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
				<path d="M11 1V4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
			</svg>
			Stream Schedule
		</h3>
		<div class="timezone">EST</div>
	</div>

	<div class="schedule-list">
		{#each highlightedSchedule as day (day.day)}
			<div class="schedule-item" class:today={day.isToday} class:no-stream={day.time === 'No Stream'}>
				<div class="day-name">
					{day.day}
					{#if day.isToday}
						<span class="today-badge">Today</span>
					{/if}
				</div>
				<div class="time">{day.time}</div>
			</div>
		{/each}
	</div>

	<div class="footer">
		<p>Subscribe for notifications! 🔔</p>
	</div>
</div>

<style>
	.stream-schedule {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		padding: 1.5rem;
		min-width: 350px;
	}

	.header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
		padding-bottom: 0.75rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	h3 {
		margin: 0;
		font-size: 1rem;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.9);
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	h3 svg {
		color: #f97316;
	}

	.timezone {
		font-size: 0.75rem;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.6);
		background: rgba(255, 255, 255, 0.05);
		padding: 0.25rem 0.5rem;
		border-radius: 4px;
	}

	.schedule-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.schedule-item {
		background: rgba(255, 255, 255, 0.03);
		border-left: 3px solid rgba(255, 255, 255, 0.2);
		border-radius: 6px;
		padding: 0.875rem;
		transition: all 0.3s ease;
	}

	.schedule-item:hover {
		background: rgba(255, 255, 255, 0.08);
		transform: translateX(4px);
	}

	.schedule-item.today {
		background: rgba(249, 115, 22, 0.1);
		border-left-color: #f97316;
		box-shadow: 0 0 20px rgba(249, 115, 22, 0.2);
	}

	.schedule-item.no-stream {
		opacity: 0.5;
	}

	.day-name {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.9375rem;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.9);
		margin-bottom: 0.25rem;
	}

	.today-badge {
		background: #f97316;
		color: white;
		font-size: 0.625rem;
		font-weight: 700;
		padding: 0.25rem 0.5rem;
		border-radius: 4px;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.time {
		font-size: 0.875rem;
		color: rgba(255, 255, 255, 0.7);
	}

	.footer {
		margin-top: 1rem;
		padding-top: 1rem;
		border-top: 1px solid rgba(255, 255, 255, 0.1);
		text-align: center;
	}

	.footer p {
		margin: 0;
		font-size: 0.875rem;
		color: rgba(255, 255, 255, 0.7);
	}
</style>
