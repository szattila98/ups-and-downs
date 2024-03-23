<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import ViewHeader from '@/lib/layouts/ViewHeader.svelte';
	import ViewMain from '@/lib/layouts/ViewMain.svelte';
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

	$: submitDisabled =
		submitting || (kind === 'WORST' ? !model['WORST'].content : !model['BEST'].content);

	let submitting = false;
	const submit = () => {
		submitting = true;
		const highlight = model[kind];
		if (highlight.content) {
			dispatch('submit', { content: highlight.content, kind });
			model[kind].content = '';
		}
		submitting = false;
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
				<svg
					width="32"
					height="32"
					viewBox="0 0 24 24"
					fill="none"
					xmlns="http://www.w3.org/2000/svg"
				>
					<path
						d="M12 4C11.4477 4 11 4.44772 11 5V11H5C4.44772 11 4 11.4477 4 12C4 12.5523 4.44772 13 5 13H11V19C11 19.5523 11.4477 20 12 20C12.5523 20 13 19.5523 13 19V13H19C19.5523 13 20 12.5523 20 12C20 11.4477 19.5523 11 19 11H13V5C13 4.44772 12.5523 4 12 4Z"
						fill="currentColor"
					/>
				</svg>
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
				{#each highlights as highlight}
					<li
						style={`background-color: #${randomColor(kind === HighlightKind.WORST ? 'dark' : 'light')};
						 color: ${kind === HighlightKind.WORST ? 'var(--light)' : 'var(--very-dark)'}`}
					>
						{highlight?.content}
					</li>
				{/each}
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

			& button:disabled:hover {
				background: transparent;
				color: var(--light);
				cursor: default;
			}
		}

		& textarea {
			border-bottom-left-radius: 8px;
			border-bottom-right-radius: 8px;
			border-color: var(--dark);
			border-style: solid;
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
				padding: 8px;
			}

			& li:first-of-type {
				border-top-left-radius: 8px;
				border-top-right-radius: 8px;
			}

			& li:last-of-type {
				border-bottom-left-radius: 8px;
				border-bottom-right-radius: 8px;
			}

			& li::marker {
				color: var(--very-dark);
			}
		}
	}
</style>
