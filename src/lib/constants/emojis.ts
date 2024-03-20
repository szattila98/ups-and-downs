import type { EmojiCode, HtmlUnicodeEmoji, ImageEmoji } from '@/types';

const happyUnicodeEmojis: HtmlUnicodeEmoji[] = ['&#128522;'];
const happyImageEmojis: ImageEmoji[] = ['capybara.png'];

const sadUnicodeEmojis: HtmlUnicodeEmoji[] = ['&#128532;'];
const sadImageEmojis: ImageEmoji[] = [];

export const happyEmojis: EmojiCode[] = [...happyUnicodeEmojis, ...happyImageEmojis];
export const sadEmojis: EmojiCode[] = [...sadUnicodeEmojis, ...sadImageEmojis];
