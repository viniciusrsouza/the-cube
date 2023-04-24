<script lang="ts">
	import { onMount } from 'svelte';
	import store from '$store/socket';
	import { MessageType, type Message, TransformType } from '$socket/message';

	let x = 0;
	let y = 0;
	let z = 0;
	let delay = 300;
	let timer: NodeJS.Timeout;
	let msgs: Message[] = [];

	let rotation = [0, 0, 0];

	$: {
		if (timer) clearTimeout(timer);
		timer = setTimeout(() => {
			rotation = [x ?? 0, y ?? 0, z ?? 0];
		}, delay);
	}

	let loadbar: HTMLElement;
	const animation = [{ width: '0%' }, { width: '100%' }];
	const timing = {
		duration: delay,
		easing: 'linear'
	};

	$: {
		x, y, z;
		loadbar?.animate(animation, timing);
	}

	$: {
		const message: Message = {
			type: MessageType.TRANSFORM,
			transform: {
				type: TransformType.ROTATE,
				data: [x, y, z]
			}
		};
		store.conn.send(message);
	}

	onMount(() => {
		store.conn.connect();
		store.subscribe((messages) => {
			msgs = messages;
		});
	});
</script>

<div>
	<div class="w-full h-full flex items-center justify-center gap-4">
		<div class="flex flex-col items-center mt-8">
			<div>X</div>
			<input
				class="w-32 border-2 border-gray-300 rounded-lg px-4 py-2"
				type="number"
				bind:value={x}
			/>
		</div>
		<div class="flex flex-col items-center mt-8">
			<div>Y</div>
			<input
				class="w-32 border-2 border-gray-300 rounded-lg px-4 py-2"
				type="number"
				bind:value={y}
			/>
		</div>
		<div class="flex flex-col items-center mt-8">
			<div>Z</div>
			<input
				class="w-32 border-2 border-gray-300 rounded-lg px-4 py-2"
				type="number"
				bind:value={z}
			/>
		</div>
	</div>
	<div class="w-80 mx-auto mt-8">
		<div class="w-full h-2 bg-red-400 rounded-full" bind:this={loadbar} />
	</div>
	<div class="flex items-center justify-center mt-8">
		<div class="bg-slate-300 px-4 py-2 rounded-full">
			({rotation[0]}, {rotation[1]}, {rotation[2]})
		</div>
	</div>
</div>
