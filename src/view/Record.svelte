<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import ViewHeader from '@/lib/layouts/ViewHeader.svelte';
	import ViewMain from '@/lib/layouts/ViewMain.svelte';
	import FaPlus from 'svelte-icons/fa/FaPlus.svelte';
	import type { CreateHighlightRequest } from '@/bindings';
	import type { Nullable } from 'ts-toolbelt/out/Object/Nullable';
	import RandomEmoji from '@/lib/components/RandomEmoji.svelte';
	import { happyEmojis, sadEmojis } from '@/lib/constants/emojis';
	import { todaysHighlight } from '@/store';
	import { randomColor } from '@/lib/utils/color';

	const TEXTAREA_ROWS = 5;

	const dispatch = createEventDispatcher<{ submit: CreateHighlightRequest }>();

	enum HighlightKind {
		WORST = 'WORST',
		BEST = 'BEST'
	}

	let kind = HighlightKind.WORST;

	type Model = {
		[HighlightKind.WORST]: Nullable<CreateHighlightRequest, 'content'>;
		[HighlightKind.BEST]: Nullable<CreateHighlightRequest, 'content'>;
	};

	const model: Model = {
		[HighlightKind.WORST]: { content: null, kind: HighlightKind.WORST },
		[HighlightKind.BEST]: { content: null, kind: HighlightKind.BEST }
	};

	$: submitDisabled = kind === 'WORST' ? !model['WORST'].content : !model['BEST'].content;

	// Középen legyen egy átkattinható BEST|WORST radio gomb ami vált boldog és szomorú textarea között
	// alatta submit gomb, enterre is megy
	// alatta pedig random színes listában amit felvittél, lehet prop
	// random emoji a válaszott rádiógomb mellett

	const submit = () => {
		const highlight = model[kind];
		if (highlight.content) {
			dispatch('submit', { content: highlight.content, kind });
			model[kind].content = '';
		}
	};
</script>

<ViewHeader on:toMenu>
	<h2 slot="middle">
		{#if $todaysHighlight}
			Edit highlight
		{:else}
			New highlights
		{/if}
	</h2>
</ViewHeader>
<ViewMain>
	<form on:submit|preventDefault={submit}>
		<div class="highlight-header">
			<div class="highlight-kind-toggle">
				<input
					id="worst-kind"
					type="radio"
					name="highlight-kind"
					value={HighlightKind.WORST}
					bind:group={kind}
				/>
				<label for="worst-kind">
					{#if kind === HighlightKind.WORST}
						<RandomEmoji emojis={sadEmojis} />
					{/if}
					Worst
				</label>
				<input
					id="best-kind"
					type="radio"
					name="highlight-kind"
					value={HighlightKind.BEST}
					bind:group={kind}
				/>
				<label for="best-kind">
					{#if kind === HighlightKind.BEST}
						<RandomEmoji emojis={happyEmojis} --emoji-img-height="32px" />
					{/if}
					Best
				</label>
			</div>
			<button type="submit" disabled={submitDisabled}>
				<FaPlus />
			</button>
		</div>
		{#if kind === HighlightKind.WORST}
			<textarea bind:value={model['WORST'].content} rows={TEXTAREA_ROWS} maxlength="2000" />
		{:else}
			<textarea bind:value={model['BEST'].content} rows={TEXTAREA_ROWS} maxlength="2000" />
		{/if}
	</form>
	{#if $todaysHighlight}
		{@const highlights =
			kind === HighlightKind.WORST ? $todaysHighlight.worst : $todaysHighlight.best}
		<div class="todays-highlights">
			<ul>
				<li
					style={`background-color: #${randomColor(kind === HighlightKind.WORST ? 'dark' : 'light')};
					color: ${kind === HighlightKind.WORST ? 'var(--light)' : 'var(--very-dark)'}`}
				>
					{highlights?.content}
				</li>
			</ul>
		</div>
	{/if}
</ViewMain>

<style scoped>
	form {
		display: flex;
		flex-direction: column;
		width: 100%;

		& .highlight-header {
			display: flex;
			flex-direction: row;
			justify-content: space-between;

			& .highlight-kind-toggle {
				display: flex;
				justify-content: left;

				& input[type='radio'] {
					appearance: none;
					margin: 0;
				}

				& label {
					align-items: center;
					background: var(--dark);
					color: var(--light);
					cursor: pointer;
					display: flex;
					gap: 4px;
					padding: 8px;
					padding-bottom: 4px;
					padding-top: 4px;
					transition: background-color 0.5s ease;
					width: 48px;
				}

				& label:first-of-type {
					border-top-left-radius: 8px;
				}

				& label:last-of-type {
					border-top-right-radius: 8px;
				}

				& input[type='radio']:checked + label {
					background: var(--secondary);
					color: var(--very-dark);
					cursor: default;
					width: 80px;
				}
			}

			& button {
				align-items: center;
				background: transparent;
				border: none;
				color: var(--very-dark);
				display: flex;
				transition: color 0.2s ease;
				width: 40px;
			}

			& button:hover {
				background: var(--highlight);
				border-top-left-radius: 8px;
				border-top-right-radius: 8px;
				cursor: pointer;
			}

			& button:disabled {
				color: var(--light);
				cursor: default;
			}
		}

		& textarea {
			border-color: var(--dark);
			border-radius: 8px;
			border-style: solid;
			border-top-left-radius: 0;
			border-width: 4px;
			font-size: small;
			padding: 0;
			resize: none;
			width: 97%;
		}
	}

	.todays-highlights {
		display: flex;
		flex-direction: column;
		width: 100%;

		& ul {
			list-style-type: decimal;
			margin: 0;
			padding-left: 24px;

			& li {
				border-radius: 8px;
				padding: 8px;
			}

			& li::marker {
				color: var(--very-dark);
			}
		}
	}
</style>
