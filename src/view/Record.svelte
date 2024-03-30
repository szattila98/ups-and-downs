<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import ViewHeader from '@/lib/layouts/ViewHeader.svelte';
	import ViewMain from '@/lib/layouts/ViewMain.svelte';
	import type { CreateHighlightRequest, EditHighlightRequest, Highlight } from '@/bindings';
	import type { Nullable as NullableField } from 'ts-toolbelt/out/Object/Nullable';
	import type { Nullable } from 'ts-toolbelt/out/Union/Nullable';
	import RandomEmoji from '@/lib/components/RandomEmoji.svelte';
	import { happyEmojis, sadEmojis } from '@/lib/constants/emojis';
	import { todaysHighlight } from '@/store';
	import { randomColor } from '@/lib/utils/color';
	import { fade } from 'svelte/transition';

	const fadeInDurationMs = 200;

	const dispatch = createEventDispatcher<{
		submit: CreateHighlightRequest;
		delete: number[];
		edit: EditHighlightRequest;
	}>();

	enum HighlightKind {
		WORST = 'WORST',
		BEST = 'BEST'
	}

	let kind = HighlightKind.WORST;

	type Model = {
		[HighlightKind.WORST]: NullableField<CreateHighlightRequest, 'content'>;
		[HighlightKind.BEST]: NullableField<CreateHighlightRequest, 'content'>;
	};

	const model: Model = {
		[HighlightKind.WORST]: { content: null, kind: HighlightKind.WORST },
		[HighlightKind.BEST]: { content: null, kind: HighlightKind.BEST }
	};
	let submitting = false;
	let updateId: Nullable<number> = null;
	let previousContent: Nullable<string> = null;
	let deleteIds: number[] = [];

	$: shownHighlights = $todaysHighlight.filter((highlight) => highlight.kind === kind);
	$: submitDisabled = (() => {
		const content = model[kind].content;
		return submitting || !content || content.length <= 1;
	})();
	$: showSubmit = !submitDisabled && !updateId && !deleteIds.length;
	$: showDeleteConfirm = !updateId && deleteIds.length;
	$: showUpdate = updateId && !deleteIds.length;

	const submit = () => {
		submitting = true;

		if (!updateId) {
			const highlight = model[kind];
			if (highlight.content && highlight.content.length > 1) {
				dispatch('submit', { content: highlight.content, kind });
				model[kind].content = '';
			}
		} else {
			const content = model[kind].content;
			if (content) {
				dispatch('edit', { id: updateId, content });
				revertPreviousContent();
			}
		}

		submitting = false;
	};

	const confirmDeleteAll = async () => {
		const isOk = await confirm("Delete all of today's highlights?");
		if (isOk) {
			const ids = $todaysHighlight.map((highlight) => highlight.id);
			dispatch('delete', ids);
		}
	};

	const loadContentToInput = (highlight: Highlight) => {
		updateId = highlight.id;
		previousContent = model[kind].content;
		model[kind].content = highlight.content;
	};
	const revertPreviousContent = () => {
		model[kind].content = previousContent;
		previousContent = null;
		updateId = null;
	};

	const markForDeletion = (highlight: Highlight) => {
		deleteIds = [...deleteIds, highlight.id];
	};
	$: isMarkedForDeletion = (highlight: Highlight): boolean => deleteIds.includes(highlight.id);
	const undoMarkingForDeletion = (highlight: Highlight) => {
		deleteIds = deleteIds.filter((id) => id !== highlight.id);
	};
	const confirmDeletion = async () => {
		const isOk = await confirm(
			`Delete ${deleteIds.length} highlight${deleteIds.length > 1 ? 's' : ''}?`
		);
		if (isOk) {
			dispatch('delete', deleteIds);
			deleteIds = [];
		}
	};
</script>

<ViewHeader on:toMenu>
	<h2 slot="middle">
		{#if $todaysHighlight.length}
			Edit
		{:else}
			Record
		{/if}
	</h2>
	<button
		slot="right"
		on:click={confirmDeleteAll}
		id="delete-todays"
		disabled={!$todaysHighlight.length || !!deleteIds.length}
	>
		<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
			<path
				fill-rule="evenodd"
				clip-rule="evenodd"
				d="M17 6V5C17 3.89543 16.1046 3 15 3H9C7.89543 3 7 3.89543 7 5V6H4C3.44772 6 3 6.44772 3 7C3 7.55228 3.44772 8 4 8H5V19C5 20.6569 6.34315 22 8 22H16C17.6569 22 19 20.6569 19 19V8H20C20.5523 8 21 7.55228 21 7C21 6.44772 20.5523 6 20 6H17ZM15 5H9V6H15V5ZM17 8H7V19C7 19.5523 7.44772 20 8 20H16C16.5523 20 17 19.5523 17 19V8Z"
				fill="currentColor"
			/>
		</svg>
	</button>
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
			{#if showSubmit}
				<button type="submit" in:fade={{ duration: fadeInDurationMs }}>
					<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
						<path
							d="M12 4C11.4477 4 11 4.44772 11 5V11H5C4.44772 11 4 11.4477 4 12C4 12.5523 4.44772 13 5 13H11V19C11 19.5523 11.4477 20 12 20C12.5523 20 13 19.5523 13 19V13H19C19.5523 13 20 12.5523 20 12C20 11.4477 19.5523 11 19 11H13V5C13 4.44772 12.5523 4 12 4Z"
							fill="currentColor"
						/>
					</svg>
				</button>
			{:else if showDeleteConfirm}
				<button
					type="button"
					id="delete-specific"
					on:click={confirmDeletion}
					in:fade={{ duration: fadeInDurationMs }}
				>
					<span>{deleteIds.length}</span>
					<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"
						><path
							fill-rule="evenodd"
							clip-rule="evenodd"
							d="M17 5V4C17 2.89543 16.1046 2 15 2H9C7.89543 2 7 2.89543 7 4V5H4C3.44772 5 3 5.44772 3 6C3 6.55228 3.44772 7 4 7H5V18C5 19.6569 6.34315 21 8 21H16C17.6569 21 19 19.6569 19 18V7H20C20.5523 7 21 6.55228 21 6C21 5.44772 20.5523 5 20 5H17ZM15 4H9V5H15V4ZM17 7H7V18C7 18.5523 7.44772 19 8 19H16C16.5523 19 17 18.5523 17 18V7Z"
							fill="currentColor"
						/><path d="M9 9H11V17H9V9Z" fill="currentColor" /><path
							d="M13 9H15V17H13V9Z"
							fill="currentColor"
						/></svg
					>
				</button>
			{:else if showUpdate}
				<div class="update-buttons" in:fade={{ duration: fadeInDurationMs }}>
					<button type="button" on:click={revertPreviousContent}>
						<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"
							><path
								d="M6.2253 4.81108C5.83477 4.42056 5.20161 4.42056 4.81108 4.81108C4.42056 5.20161 4.42056 5.83477 4.81108 6.2253L10.5858 12L4.81114 17.7747C4.42062 18.1652 4.42062 18.7984 4.81114 19.1889C5.20167 19.5794 5.83483 19.5794 6.22535 19.1889L12 13.4142L17.7747 19.1889C18.1652 19.5794 18.7984 19.5794 19.1889 19.1889C19.5794 18.7984 19.5794 18.1652 19.1889 17.7747L13.4142 12L19.189 6.2253C19.5795 5.83477 19.5795 5.20161 19.189 4.81108C18.7985 4.42056 18.1653 4.42056 17.7748 4.81108L12 10.5858L6.2253 4.81108Z"
								fill="currentColor"
							/></svg
						>
					</button>
					<button type="submit" disabled={submitDisabled}>
						<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"
							><path
								fill-rule="evenodd"
								clip-rule="evenodd"
								d="M12 24C18.6274 24 24 18.6274 24 12C24 5.37258 18.6274 0 12 0C5.37258 0 0 5.37258 0 12C0 18.6274 5.37258 24 12 24ZM18.5793 19.531C20.6758 17.698 22 15.0036 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 14.9616 3.28743 17.6225 5.33317 19.4535L6.99999 10.9738H9.17026L12 6.07251L14.8297 10.9738H17L18.5793 19.531ZM16.0919 21.1272L15.2056 12.9738H8.79438L7.90814 21.1272C9.15715 21.688 10.5421 22 12 22C13.4579 22 14.8428 21.688 16.0919 21.1272Z"
								fill="currentColor"
							/></svg
						>
					</button>
				</div>
			{/if}
		</div>
		<textarea
			on:keypress={(event) => {
				if (event.key === 'Enter') {
					event.preventDefault();
					submit();
				}
			}}
			bind:value={model[kind].content}
			rows={5}
			maxlength="2000"
		/>
	</form>

	{#if shownHighlights.length}
		<div class="todays-highlights">
			<ul>
				{#each shownHighlights as highlight}
					<li
						style:background={randomColor(
							kind === HighlightKind.WORST ? 'dark' : 'light'
						)}
						style:color={kind === HighlightKind.WORST
							? 'var(--light)'
							: 'var(--very-dark)'}
						data-marked={isMarkedForDeletion(highlight)}
					>
						{highlight.content}
						{#if !isMarkedForDeletion(highlight)}
							{#if !deleteIds.length && !updateId}
								<button
									class="edit-btn"
									on:click={() => loadContentToInput(highlight)}
								>
									<svg
										viewBox="0 0 24 24"
										fill="none"
										xmlns="http://www.w3.org/2000/svg"
										><path
											fill-rule="evenodd"
											clip-rule="evenodd"
											d="M21.2635 2.29289C20.873 1.90237 20.2398 1.90237 19.8493 2.29289L18.9769 3.16525C17.8618 2.63254 16.4857 2.82801 15.5621 3.75165L4.95549 14.3582L10.6123 20.0151L21.2189 9.4085C22.1426 8.48486 22.338 7.1088 21.8053 5.99367L22.6777 5.12132C23.0682 4.7308 23.0682 4.09763 22.6777 3.70711L21.2635 2.29289ZM16.9955 10.8035L10.6123 17.1867L7.78392 14.3582L14.1671 7.9751L16.9955 10.8035ZM18.8138 8.98525L19.8047 7.99429C20.1953 7.60376 20.1953 6.9706 19.8047 6.58007L18.3905 5.16586C18 4.77534 17.3668 4.77534 16.9763 5.16586L15.9853 6.15683L18.8138 8.98525Z"
											fill="currentColor"
										/><path
											d="M2 22.9502L4.12171 15.1717L9.77817 20.8289L2 22.9502Z"
											fill="currentColor"
										/></svg
									>
								</button>
							{/if}
							{#if !updateId}
								<button
									class="delete-btn"
									on:click={() => markForDeletion(highlight)}
								>
									<svg
										viewBox="0 0 24 24"
										fill="none"
										xmlns="http://www.w3.org/2000/svg"
										><path
											d="M6.2253 4.81108C5.83477 4.42056 5.20161 4.42056 4.81108 4.81108C4.42056 5.20161 4.42056 5.83477 4.81108 6.2253L10.5858 12L4.81114 17.7747C4.42062 18.1652 4.42062 18.7984 4.81114 19.1889C5.20167 19.5794 5.83483 19.5794 6.22535 19.1889L12 13.4142L17.7747 19.1889C18.1652 19.5794 18.7984 19.5794 19.1889 19.1889C19.5794 18.7984 19.5794 18.1652 19.1889 17.7747L13.4142 12L19.189 6.2253C19.5795 5.83477 19.5795 5.20161 19.189 4.81108C18.7985 4.42056 18.1653 4.42056 17.7748 4.81108L12 10.5858L6.2253 4.81108Z"
											fill="currentColor"
										/></svg
									>
								</button>
							{/if}
						{:else}
							<button
								class="delete-btn"
								on:click={() => undoMarkingForDeletion(highlight)}
								in:fade={{ duration: fadeInDurationMs }}>Undo</button
							>
						{/if}
					</li>
				{/each}
			</ul>
		</div>
	{/if}
</ViewMain>

<style scoped>
	#delete-todays svg {
		width: 32px;
	}

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
				justify-content: space-between;
				width: 40px;

				&[id='delete-specific'] {
					width: 68px;

					& span {
						background-color: var(--highlight);
						border-radius: 50%;
						color: var(--light);
						padding: 2px 6px 2px;
					}
				}

				&:hover {
					background: var(--highlight);
					border-top-left-radius: 8px;
					border-top-right-radius: 8px;
					cursor: pointer;
				}

				&:hover:disabled {
					background: transparent;
					cursor: auto;
				}

				& svg {
					width: 32px;
				}
			}

			& .update-buttons {
				display: flex;
			}
		}

		& textarea {
			border-bottom-left-radius: 8px;
			border-bottom-right-radius: 8px;
			border-color: var(--dark);
			border-style: solid;
			border-width: 4px;
			font-size: small;
			padding: 4px;
			resize: none;
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
				list-style-image: 'aa';
				padding: 8px;
				position: relative;

				&:hover button {
					display: block;
				}

				& button {
					background-color: var(--secondary);
					border: 2px solid var(--dark);
					border-radius: 8px;
					bottom: 2px;
					color: var(--very-dark);
					display: none;
					height: 32px;
					position: absolute;

					&:hover {
						background: var(--highlight);
						color: var(--light);
						cursor: pointer;
					}

					& svg {
						width: 20px;
					}
				}

				& .delete-btn {
					right: 2px;
				}

				& .edit-btn {
					right: 38px;
				}
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

			& li[data-marked='true'] {
				background: transparent !important;
				border-left: 2px solid var(--dark);
				border-right: 2px solid var(--dark);
				color: var(--very-dark) !important;
				padding-left: 6px;
				padding-right: 6px;
			}

			& li[data-marked='true']:first-of-type {
				border-top: 2px solid var(--dark);
				padding-top: 6px;
			}

			& li[data-marked='true']:last-of-type {
				border-bottom: 2px solid var(--dark);
				padding-bottom: 6px;
			}
		}
	}
</style>
