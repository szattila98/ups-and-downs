<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { todaysHighlight } from '../store';

	const dispatch = createEventDispatcher<{
		toNew: null;
		toList: null;
		exit: null;
	}>();
</script>

<header>
	<img src="/img/logo.png" alt="logo" />
</header>
<main>
	<button on:click={() => dispatch('toNew')}>
		{#if $todaysHighlight.length}
			Edit highlight
		{:else}
			New highlights
		{/if}
	</button>
	<button on:click={() => dispatch('toList')}>List</button>
	<button>Settings</button>
	<button on:click={() => dispatch('exit')}>Exit</button>
</main>
<footer>
	<a href="https://github.com/szattila98/ups-and-downs" target="_blank">
		<img src="/img/github-mark.png" alt="Github logo" />
	</a>
</footer>

<style scoped>
	header {
		display: flex;
		justify-content: center;
		margin-bottom: 12px;
		& img {
			margin-top: 16px;
			width: 160px;
		}
	}

	main {
		align-items: center;
		display: flex;
		flex-direction: column;

		& button {
			background: var(--secondary);
			border: none;
			border-radius: 12px;
			box-shadow: 3px 3px 2px var(--very-dark);
			color: var(--very-dark);
			font-size: medium;
			font-weight: bold;
			height: 32px;
			margin-top: 16px;
			width: 75%;

			&:hover {
				background-color: var(--highlight);
				color: var(--light);
				cursor: pointer;
			}

			&:disabled {
				background: var(--dark);
				color: var(--light);

				&:hover {
					background: var(--dark);
					cursor: auto;
				}
			}
		}
	}

	footer {
		align-items: center;
		display: flex;
		flex-direction: row;
		font-size: larger;
		justify-content: center;
		padding-top: 20px;

		& img {
			width: 32px;
		}
	}
</style>
