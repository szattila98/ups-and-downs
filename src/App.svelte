<script lang="ts">
	import { onMount } from 'svelte';
	import { commands } from './bindings';
	import type { DailyHighlight } from './types';
	import Record from './view/Record.svelte';
	import Menu from './view/Menu.svelte';
	import List from './view/List.svelte';
	import { hasUserRecordedToday } from './store';
	import { exit } from '@tauri-apps/api/process';

	enum AppState {
		Menu,
		Record,
		List
	}

	let state: AppState = AppState.Menu;

	const init = async () => {
		const result = await commands.hasRecordedToday();
		hasUserRecordedToday.set(result);
	};

	onMount(async () => {
		await init();
		if ($hasUserRecordedToday) {
			state = AppState.Menu;
		} else {
			state = AppState.Record;
		}
	});

	const newHighlight = async ({ detail: dailyHighlight }: CustomEvent<DailyHighlight>) => {
		const promises: ReturnType<typeof commands.recordHighlight>[] = [];
		if (dailyHighlight.best) {
			promises.push(commands.recordHighlight({ content: dailyHighlight.best, kind: 'BEST' }));
		}
		if (dailyHighlight.worst) {
			promises.push(
				commands.recordHighlight({ content: dailyHighlight.worst, kind: 'WORST' })
			);
		}
		await Promise.all(promises);
		await init();
		state = AppState.Menu;
	};

	const toMenu = () => (state = AppState.Menu);
</script>

{#if state === AppState.Menu}
	<Menu
		on:toNew={() => (state = AppState.Record)}
		on:toList={() => (state = AppState.List)}
		on:exit={async () => await exit(0)}
	/>
{:else if state === AppState.Record}
	<Record on:submit={newHighlight} on:toMenu={toMenu} />
{:else if state === AppState.List}
	<List on:toMenu={toMenu} />
{/if}
