<script lang="ts">
	import { onMount } from 'svelte';
	import { commands, type CreateHighlightRequest } from './bindings';
	import Record from './view/Record.svelte';
	import Menu from './view/Menu.svelte';
	import List from './view/List.svelte';
	import { todaysHighlight } from './store';
	import { exit } from '@tauri-apps/api/process';
	import Spinner from './lib/components/Spinner.svelte';
	import { fade } from 'svelte/transition';

	enum AppState {
		Loading,
		Menu,
		Record,
		List
	}

	let state: AppState = AppState.Loading;

	const init = async () => {
		const result = await commands.getTodaysHighlight();
		$todaysHighlight = result;
	};

	onMount(async () => {
		await init();
		if ($todaysHighlight) {
			state = AppState.Menu;
		} else {
			state = AppState.Record;
		}
	});

	const newHighlight = async ({ detail: highlight }: CustomEvent<CreateHighlightRequest>) => {
		$todaysHighlight = await commands.recordHighlight(highlight);
	};

	const toMenu = () => (state = AppState.Menu);
</script>

{#if state === AppState.Loading}
	<div class="loader">
		<Spinner />
	</div>
{:else if state === AppState.Menu}
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

<style scoped>
	.loader {
		align-items: center;
		display: flex;
		height: 90vh;
		justify-content: center;
	}
</style>
