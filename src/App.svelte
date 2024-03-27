<script lang="ts">
	import { onMount } from 'svelte';
	import { commands, type CreateHighlightRequest, type EditHighlightRequest } from './bindings';
	import Record from './view/Record.svelte';
	import Menu from './view/Menu.svelte';
	import List from './view/List.svelte';
	import { todaysHighlight } from './store';
	import { exit } from '@tauri-apps/api/process';
	import Spinner from './lib/components/Spinner.svelte';
	import JumpToTopButton from './lib/components/JumpToTopButton.svelte';
	import { getToday } from './lib/utils/date';

	enum AppState {
		Loading,
		Menu,
		Record,
		List
	}

	let state: AppState = AppState.Loading;

	const init = async () => {
		$todaysHighlight = await commands.getHighlightsByDate(getToday());
	};

	onMount(async () => {
		await init();
		/* 
		if ($todaysHighlight) {
			state = AppState.Menu;
		} else {
			state = AppState.Record;
		} 
		*/
		state = AppState.Menu;
	});

	const newHighlight = async ({ detail: request }: CustomEvent<CreateHighlightRequest>) => {
		$todaysHighlight = await commands.recordHighlight(request);
	};

	const toMenu = () => (state = AppState.Menu);

	const deleteHighlights = async ({ detail: ids }: CustomEvent<number[]>) => {
		await commands.deleteHighlight(ids);
		await init();
	};

	const editHighlight = async ({ detail: request }: CustomEvent<EditHighlightRequest>) => {
		$todaysHighlight = await commands.editHighlight(request);
	};

	const quit = async () => {
		await exit(0);
	};
</script>

<JumpToTopButton />
{#if state === AppState.Loading}
	<div class="loader">
		<Spinner />
	</div>
{:else if state === AppState.Menu}
	<Menu
		on:toNew={() => (state = AppState.Record)}
		on:toList={() => (state = AppState.List)}
		on:exit={quit}
	/>
{:else if state === AppState.Record}
	<Record
		on:submit={newHighlight}
		on:toMenu={toMenu}
		on:delete={deleteHighlights}
		on:edit={editHighlight}
	/>
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
