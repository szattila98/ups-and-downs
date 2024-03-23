<script lang="ts">
	import { commands } from '../bindings';
	import dayjs from 'dayjs';
	import RandomEmoji from '../lib/components/RandomEmoji.svelte';
	import FaRegCalendarAlt from 'svelte-icons/fa/FaRegCalendarAlt.svelte';
	import { happyEmojis, sadEmojis } from '../lib/constants/emojis';
	import ViewHeader from '@/lib/layouts/ViewHeader.svelte';
	import ViewMain from '@/lib/layouts/ViewMain.svelte';
	import { randomColor } from '@/lib/utils/color';
</script>

<ViewHeader on:toMenu>
	<h2 slot="middle">Highlights</h2>
	<button slot="right" class="calendar-btn"><FaRegCalendarAlt /></button>
</ViewHeader>
<ViewMain>
	{#await commands.listHighlights()}
		<p>Loading</p>
	{:then highlights}
		{#each highlights as highlight}
			<div class="highlight">
				<span class="highlight-date" title={dayjs(highlight.date).format('LLL')}>
					{dayjs(highlight.date).fromNow()}
				</span>
				<ul class="highlight-container">
					{#each highlight.best as best, index}
						<li
							class="highlight-best"
							style={`background-color: #${randomColor('light')};`}
							data-last={highlight.best.length === index + 1 &&
								!!highlight.worst.length}
						>
							<RandomEmoji emojis={happyEmojis} />
							<span>{best?.content}</span>
						</li>
					{/each}
					{#each highlight.worst as worst}
						<li
							class="highlight-worst"
							style={`background-color: #${randomColor('dark')};`}
						>
							<RandomEmoji emojis={sadEmojis} />
							<span>{worst?.content}</span>
						</li>
					{/each}
				</ul>
			</div>
		{:else}
			<p>No highlights to list yet!</p>
		{/each}
	{/await}
</ViewMain>

<style scoped>
	.highlight {
		display: flex;
		flex-direction: column;
		width: 100%;

		& .highlight-date {
			background: var(--dark);
			border-start-end-radius: 8px;
			border-start-start-radius: 8px;
			color: var(--light);
			margin: 0;
			padding: 4px 8px;
			width: fit-content;
		}

		& .highlight-container {
			font-size: small;
			margin: 0;
			padding-left: 0;

			& li {
				align-items: center;
				display: flex;
				gap: 4px;

				& * {
					margin: 4px;
				}
			}

			& li:first-of-type {
				border-top-right-radius: 8px;
			}

			& li:last-of-type {
				border-bottom-left-radius: 8px;
				border-bottom-right-radius: 8px;
			}

			& li[data-last='true'] {
				border-bottom: 2px solid var(--highlight);
			}

			& .highlight-best {
				color: var(--very-dark);
			}

			& .highlight-worst {
				color: var(--light);
			}
		}
	}

	.calendar-btn {
		align-content: center;
		background: transparent;
		border: none;
		border-radius: 12px;
		display: flex;
		padding: 8px;

		& svg {
			width: 32px;
		}

		&:hover {
			background: var(--highlight);
			cursor: pointer;
		}
	}
</style>
