<script lang="ts">
	import type { EmojiCode } from '../../types';

	export let emojis: EmojiCode[];

	$: emoji = emojis[Math.floor(Math.random() * emojis.length)];
</script>

{#if emoji.startsWith('&#')}
	<!-- eslint-disable svelte/no-at-html-tags -->
	<span class="text-emoji">{@html emoji}</span>
{:else}
	<img class="img-emoji" src={`/img/emojis/${emoji}`} alt={emoji.split('.')[0]} />
{/if}

<style scoped>
	.text-emoji {
		font-size: var(--emoji-height, 24px);
	}

	.img-emoji {
		height: var(--emoji-height, 24px);
	}
</style>
