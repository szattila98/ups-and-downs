<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher<{ toMenu: null }>();
</script>

<header>
	<slot name="left">
		<button on:click={() => dispatch('toMenu')}>
			<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"
				><path
					d="M20.3284 11.0001V13.0001L7.50011 13.0001L10.7426 16.2426L9.32842 17.6568L3.67157 12L9.32842 6.34314L10.7426 7.75735L7.49988 11.0001L20.3284 11.0001Z"
					fill="currentColor"
				/></svg
			>
		</button>
	</slot>
	<slot name="middle" />
	<slot name="right" />
</header>

<style scoped>
	header {
		align-items: center;
		background: var(--secondary);
		border-radius: 24px;
		display: flex;
		justify-content: space-between;
		margin: 8px;
		margin-bottom: 16px;
		padding-left: 8px;
		padding-right: 8px;

		& button {
			align-content: center;
			background: transparent;
			border: none;
			border-radius: 12px;
			display: flex;

			& svg {
				width: 40px;
			}

			&:hover {
				background: var(--highlight);
				cursor: pointer;
			}

			&:disabled {
				background: transparent;
				cursor: auto;
			}
		}
	}
</style>
