<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { hasUserRecordedToday } from '../store';

	const dispatch = createEventDispatcher<{
		toNew: null;
		toList: null;
		exit: null;
	}>();
</script>

<button on:click={() => dispatch('toNew')} disabled={$hasUserRecordedToday}>New highlight</button>
{#if $hasUserRecordedToday}
	<button>Edit highlight</button>
{/if}
<button on:click={() => dispatch('toList')}>List</button>
<button on:click={() => dispatch('exit')}>Exit</button>
