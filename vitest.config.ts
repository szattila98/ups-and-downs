/// <reference types="vitest" />
import { fileURLToPath } from 'node:url';
import { mergeConfig, defineConfig, configDefaults } from 'vitest/config';
import viteConfig from './vite.config';

export default defineConfig((configEnv) =>
	mergeConfig(
		viteConfig(configEnv),
		defineConfig({
			test: {
				environment: 'jsdom',
				exclude: [...configDefaults.exclude, 'tests/*'],
				root: fileURLToPath(new URL('./', import.meta.url)),
				typecheck: {
					tsconfig: 'tsconfig.vitest.json',
					checker: 'vue-tsc'
				},
				coverage: {
					enabled: true,
					reporter: ['html', 'lcov', 'text', 'text-summary'],
					include: ['src/*'],
					exclude: ['src/types.ts', 'src/main.ts']
					/* thresholds: {
						branches: 80,
						functions: 80,
						lines: 80,
						statements: 80
					} */
				}
			}
		})
	)
);
