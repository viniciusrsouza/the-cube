import adapter from '@sveltejs/adapter-auto';
import { vitePreprocess } from '@sveltejs/kit/vite';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),
	kit: {
		adapter: adapter(),
		alias: {
			$components: './src/components',
			$atoms: './src/components/atoms',
			$molecules: './src/components/molecules',
			$organisms: './src/components/organisms',
			$utils: './src/utils',
			$store: './src/store',
			$socket: './src/socket'
		}
	}
};

export default config;
