<script lang="ts">
	interface Props {
		padding?: 'sm' | 'md' | 'lg' | 'xl';
		hover?: boolean;
		class?: string;
		children?: any;
	}

	let { padding = 'md', hover = false, class: className = '', children }: Props = $props();

	const paddingClasses = {
		sm: 'p-4',
		md: 'p-6',
		lg: 'p-8',
		xl: 'p-10'
	};
</script>

<div
	class="bg-white/5 backdrop-blur-md border border-white/10 rounded-2xl {paddingClasses[
		padding
	]} {hover
		? 'transition-all duration-300 hover:bg-white/10 hover:border-white/20 hover:shadow-lg hover:shadow-bacon-orange/20 hover:-translate-y-1'
		: ''} {className}"
>
	{@render children?.()}
</div>
