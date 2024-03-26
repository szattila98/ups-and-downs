import './lib/css/styles.css';
import App from './App.svelte';

import localizedFormat from 'dayjs/plugin/localizedFormat';
import dayjs from 'dayjs';

dayjs.extend(localizedFormat);

const app = new App({
	target: document.getElementById('app')!
});

export default app;
