<script lang="ts">
	import { run } from 'cube-renderer';
	import { onMount } from 'svelte';
	import store from '$store/socket';
	import type { Message } from '$socket/message';

	let height = 0;
	let width = 0;
	let messages: Message[] = [];

	onMount(() => {
		store.conn.connect();
		store.subscribe((messages) => {
			messages = messages;
		});
		run();
	});
</script>

<svelte:window bind:innerHeight={height} bind:innerWidth={width} />

<div class="relative w-full h-full flex items-center justify-center">
	<div class="absolute bg-gray-500 px-4 py-2 m-2 rounded-sm top-0 z-10 opacity-40">
		{height}px x {width}px
	</div>
	<canvas id="canvas" class="w-full h-full" {width} {height} />
</div>
