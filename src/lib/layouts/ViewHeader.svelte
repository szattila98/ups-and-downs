<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import FaArrowLeft from 'svelte-icons/fa/FaArrowLeft.svelte';

	const dispatch = createEventDispatcher<{ toMenu: null }>();
</script>

<header>
	<button on:click={() => dispatch('toMenu')}>
		<FaArrowLeft />
	</button>
	<slot name="middle" />
	<slot name="right" />
</header>

<style scoped>
	header {
		align-items: center;
		background: var(--secondary);
		border-radius: 24px;
		display: flex;
		justify-content: space-evenly;
		margin: 8px;

		& button {
			align-content: center;
			background: transparent;
			border: none;
			border-radius: 12px;
			display: flex;

			& svg {
				width: 32px;
			}

			&:hover {
				background: var(--highlight);
				cursor: pointer;
			}
		}
	}
</style>
