import { defineConfig } from 'astro/config'
import vue from '@astrojs/vue'

export default defineConfig({
	output: 'server',
	integrations: [vue()],
});
