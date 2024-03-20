export interface DailyHighlight {
	best: string | null;
	worst: string | null;
}

export type HtmlUnicodeEmoji = `&#${string};`;
export type ImageEmoji = `${string}.png`;
export type EmojiCode = HtmlUnicodeEmoji | ImageEmoji;
