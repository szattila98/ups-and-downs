<script lang="ts">
	import { commands } from '../bindings';

	let name = '';
	let greetMsg = '';

	async function greet() {
		// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
		greetMsg = await commands.greet(name);
	}
</script>

<div>
	<form class="row" on:submit|preventDefault={greet}>
		<input id="greet-input" placeholder="Enter a name..." bind:value={name} />
		<button type="submit">Greet</button>
	</form>
	<p>{greetMsg}</p>
</div>
