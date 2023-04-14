<script>
	import init, { greet } from 'cube-renderer';
	import { onMount } from 'svelte';

	import '../app.css';
	import { sleep } from '$utils/debug';

	let loading = true;

	onMount(async () => {
		await init();
		await sleep(1000);
		greet();
		loading = false;
	});
</script>

<div class="h-screen w-screen bg-teal-600">
	{#if loading}
		<div class="flex justify-center items-center h-full">
			<div class="animate-spin rounded-full h-32 w-32 border-t-2 border-b-2 border-gray-900" />
		</div>
	{:else}
		<slot />
	{/if}
</div>
