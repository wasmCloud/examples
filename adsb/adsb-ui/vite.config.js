import { defineConfig } from 'vite'
// import { fileURLToPath, URL } from 'node:url'
import vue from '@vitejs/plugin-vue'

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [vue()],
    build: {
        chunkSizeWarningLimit: 2000,
        rollupOptions: {
            output: {
                manualChunks: manualChunks,
            },
        },
    },
})

function manualChunks(id) {
    console.log(id);
    if (id.includes('vue3-openlayers')) {
        return 'openlayers';
    } else {
        return 'vendor';
    }
}
