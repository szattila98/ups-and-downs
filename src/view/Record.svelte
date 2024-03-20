<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { DailyHighlight } from '../types';
	import ViewHeader from '@/lib/components/ViewHeader.svelte';

	const model: DailyHighlight = {
		best: null,
		worst: null
	};

	const dispatch = createEventDispatcher<{ submit: DailyHighlight }>();

	$: submitDisabled = !model.best && !model.worst;
</script>

<ViewHeader on:toMenu>
	<h2 slot="middle">New highlights</h2>
</ViewHeader>
<main>
	<form on:submit|preventDefault={() => dispatch('submit', model)}>
		<h1>Best</h1>
		<input type="text" bind:value={model.best} maxlength="2000" />
		<h1>Worst</h1>
		<input type="text" bind:value={model.worst} maxlength="2000" />
		<button type="submit" disabled={submitDisabled}>Submit</button>
	</form>
</main>
