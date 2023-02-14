import { defineConfig } from 'vite'
// import { fileURLToPath, URL } from 'node:url'
import vue from '@vitejs/plugin-vue'

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [vue()],
    // define: { 'process.env': {} },
    build: {
        chunkSizeWarningLimit: 2000,
        rollupOptions: {
            output: {
                manualChunks: {
                    'openlayers': ['vue3-openlayers'],
                },
            },
        },
    },

    // root: "ui/",
    // resolve: {
    //     alias: {
    //         '@': fileURLToPath(new URL('./src', import.meta.url))
    //     },
    //     extensions: [
    //         '.js',
    //         '.json',
    //         '.jsx',
    //         '.mjs',
    //         '.ts',
    //         '.tsx',
    //         '.vue',
    //     ],
    // },
    // server: {
    //     port: 3000,
    //     open: true,
    //     cors: true,
    // },
})
