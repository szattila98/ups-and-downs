<script lang="ts">
	const DISTANCE = 100;

	let scrollY = 0;

	$: display = scrollY > DISTANCE ? 'flex' : 'none';

	const scrollTop = () => window.scrollTo({ top: 0, behavior: 'smooth' });
</script>

<svelte:window bind:scrollY />

<button type="button" title="Go to top" style:display on:click={scrollTop}>
	<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"
		><path
			d="M17.6568 8.96219L16.2393 10.3731L12.9843 7.10285L12.9706 20.7079L10.9706 20.7059L10.9843 7.13806L7.75404 10.3532L6.34314 8.93572L12.0132 3.29211L17.6568 8.96219Z"
			fill="currentColor"
		/></svg
	>
</button>

<style scoped>
	button {
		align-items: center;
		background-color: var(--secondary);
		border: 2px solid var(--dark);
		border-radius: 8px;
		color: var(--very-dark);
		cursor: pointer;
		font-size: 18px;
		outline: none;
		padding: 4px;
		position: fixed;
		right: 12px;
		top: 12px;
		z-index: 99;

		& svg {
			height: 32px;
			width: 32px;
		}
	}

	button:hover {
		background-color: var(--highlight);
		color: var(--light);
	}
</style>
