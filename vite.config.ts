/// <reference types="vitest" />

import legacy from '@vitejs/plugin-legacy'
import vue from '@vitejs/plugin-vue'
import path from 'path'
import {defineConfig} from 'vite'

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [
        vue(),
        legacy()
    ],
    resolve: {
        alias: {
            '@': path.resolve(__dirname, './src'),
        },
    },
    server: {
        port: 1420,
        watch: {
            ignored: [
                '**/src-tauri/**',
                '**/db/**',
                '**/logs/**',
            ],
        },
    },
    test: {
        globals: true,
        environment: 'jsdom'
    }
})
