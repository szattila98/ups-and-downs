<script lang="ts">
	import type { EmojiCode } from '../../types';

	export let emojis: EmojiCode[];

	$: emoji = emojis[Math.floor(Math.random() * emojis.length)];
</script>

<div class="emoji-container">
	{#if emoji.startsWith('&#')}
		<!-- eslint-disable svelte/no-at-html-tags -->
		<span class="text-emoji">{@html emoji}</span>
	{:else}
		<img class="img-emoji" src={`/img/emojis/${emoji}`} alt={emoji.split('.')[0]} />
	{/if}
</div>

<style scoped>
	.emoji-container {
		align-items: center;
		display: flex;
		height: var(--emoji-container-height, 32px);
		justify-content: center;
		width: var(--emoji-container-width, 32px);
	}

	.text-emoji {
		font-size: var(--emoji-font-size, 24px);
	}

	.img-emoji {
		height: var(--emoji-img-height, 24px);
	}
</style>
