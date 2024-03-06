<script lang="ts">
	import { commands } from '../bindings';

	interface DailyRecord {
		best: string | null;
		worst: string | null;
	}

	const model: DailyRecord = {
		best: null,
		worst: null
	};

	const submit = async () => {
		const promises: ReturnType<typeof commands.recordHighlight>[] = [];
		if (model.best) {
			promises.push(commands.recordHighlight({ content: model.best, kind: 'BEST' }));
		}
		if (model.worst) {
			promises.push(commands.recordHighlight({ content: model.worst, kind: 'WORST' }));
		}
		await Promise.all(promises);
	};

	$: disabled = !model.best && !model.worst;
</script>

<form on:submit={submit}>
	<h1>Best</h1>
	<input type="text" bind:value={model.best} maxlength="2000" />
	<h1>Worst</h1>
	<input type="text" bind:value={model.worst} maxlength="2000" />
	<button type="submit" {disabled}>Submit</button>
</form>
