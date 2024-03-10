<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { DailyHighlight } from '../types';

	const model: DailyHighlight = {
		best: null,
		worst: null
	};

	const dispatch = createEventDispatcher<{ submit: DailyHighlight; toMenu: null }>();

	$: submitDisabled = !model.best && !model.worst;
</script>

<button on:click={() => dispatch('toMenu')}>Back to menu</button>
<form on:submit|preventDefault={() => dispatch('submit', model)}>
	<h1>Best</h1>
	<input type="text" bind:value={model.best} maxlength="2000" />
	<h1>Worst</h1>
	<input type="text" bind:value={model.worst} maxlength="2000" />
	<button type="submit" disabled={submitDisabled}>Submit</button>
</form>
