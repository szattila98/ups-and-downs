export const randomColor = (type?: 'light' | 'dark'): string => {
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
