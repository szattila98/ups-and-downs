<script lang="ts">
	import { commands } from '../bindings';
	import dayjs from 'dayjs';
	import RandomEmoji from '../lib/components/RandomEmoji.svelte';
	import { happyEmojis, sadEmojis } from '../lib/constants/emojis';
	import ViewHeader from '@/lib/layouts/ViewHeader.svelte';
	import ViewMain from '@/lib/layouts/ViewMain.svelte';
	import { randomColor } from '@/lib/utils/color';
	import Tooltip from '@/lib/components/Tooltip.svelte';
</script>

<ViewHeader on:toMenu>
	<h2 slot="middle">Highlights</h2>
	<button slot="right" class="calendar-btn">
		<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"
			><path
				d="M8 9C7.44772 9 7 9.44771 7 10C7 10.5523 7.44772 11 8 11H16C16.5523 11 17 10.5523 17 10C17 9.44771 16.5523 9 16 9H8Z"
				fill="currentColor"
			/><path
				fill-rule="evenodd"
				clip-rule="evenodd"
				d="M6 3C4.34315 3 3 4.34315 3 6V18C3 19.6569 4.34315 21 6 21H18C19.6569 21 21 19.6569 21 18V6C21 4.34315 19.6569 3 18 3H6ZM5 18V7H19V18C19 18.5523 18.5523 19 18 19H6C5.44772 19 5 18.5523 5 18Z"
				fill="currentColor"
			/></svg
		>
	</button>
</ViewHeader>
<ViewMain>
	{#await commands.listHighlights()}
		<p>Loading</p>
	{:then highlights}
		{#each highlights as highlight}
			<div class="highlight">
				<Tooltip placement="right" offset={7}>
					<span slot="host" class="highlight-date">
						{dayjs(highlight.date).fromNow()}
					</span>
					<span class="tooltip-content" slot="content"
						>{dayjs(highlight.date).format('LL')}</span
					>
				</Tooltip>
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
			width: 40px;
		}

		&:hover {
			background: var(--highlight);
			cursor: pointer;
		}
	}

	.tooltip-content {
		font-size: smaller;
	}
</style>
