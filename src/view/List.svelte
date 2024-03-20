<script lang="ts">
	import { commands } from '../bindings';
	import dayjs from 'dayjs';
	import RandomEmoji from '../lib/components/RandomEmoji.svelte';
	import FaRegCalendarAlt from 'svelte-icons/fa/FaRegCalendarAlt.svelte';
	import { happyEmojis, sadEmojis } from '../lib/constants/emojis';
	import ViewHeader from '@/lib/components/ViewHeader.svelte';

	const randomColor = (type?: 'light' | 'dark'): string => {
		let base, range;

		switch (type) {
			case 'light':
				base = 127;
				range = 128;
				break;
			case 'dark':
				base = 0;
				range = 128;
				break;
			default:
				base = 0;
				range = 256;
		}

		const r = Math.floor(Math.random() * range) + base;
		const g = Math.floor(Math.random() * range) + base;
		const b = Math.floor(Math.random() * range) + base;

		return ((1 << 24) + (r << 16) + (g << 8) + b).toString(16).slice(1);
	};
</script>

<ViewHeader on:toMenu>
	<h2 slot="middle">Highlights</h2>
	<button slot="right" class="calendar-btn"><FaRegCalendarAlt /></button>
</ViewHeader>
<main>
	{#await commands.listHighlights()}
		<p>Loading</p>
	{:then highlights}
		{#each highlights as highlight}
			<div class="highlight">
				<span class="highlight-date" title={dayjs(highlight.date).format('LLL')}>
					{dayjs(highlight.date).fromNow()}
				</span>
				<div class="highlight-container">
					<div
						class="highlight-best"
						style={`background-color: #${randomColor('light')};`}
					>
						<ul>
							<li><RandomEmoji emojis={happyEmojis} />{highlight.best?.content}</li>
						</ul>
					</div>
					<div
						class="highlight-worst"
						style={`background-color: #${randomColor('dark')};`}
					>
						<ul>
							<li><RandomEmoji emojis={sadEmojis} />{highlight.worst?.content}</li>
						</ul>
					</div>
				</div>
			</div>
		{:else}
			<p>No highlights to list yet!</p>
		{/each}
	{/await}
</main>

<style scoped>
	main {
		align-items: center;
		display: flex;
		flex-direction: column;

		& .highlight {
			display: flex;
			flex-direction: column;
			width: 100%;

			& .highlight-date {
				background: var(--dark);
				border-start-end-radius: 8px;
				border-start-start-radius: 8px;
				color: var(--light);
				margin: 0;
				margin-top: 8px;
				padding: 4px 8px;
				width: fit-content;
			}

			& .highlight-container {
				display: flex;
				flex-direction: column;
				justify-content: center;
				width: 100%;

				& div[class^='highlight-'] {
					font-size: small;

					& ul {
						list-style-type: none;
						margin: 8px;
						padding-left: 4px;

						& li {
							align-items: center;
							display: flex;
							gap: 4px;
						}
					}
				}

				& .highlight-best {
					border-top-right-radius: 8px;
					color: var(--very-dark);
				}

				& .highlight-worst {
					border-bottom-left-radius: 8px;
					border-bottom-right-radius: 8px;
					color: var(--light);
				}
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
