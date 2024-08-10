import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import { resolve } from 'path'

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [svelte()],
	build: {
		// rollupOptions: {
		// 	input: {
		// 		home: resolve(__dirname, 'home/index.html'),
		// 		login: resolve(__dirname, 'login/index.html'),
		// 	},
		// },
	},
})
