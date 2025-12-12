import { resolve } from 'node:path';

import { defineConfig } from 'vite';
import tsconfigPaths from 'vite-tsconfig-paths';
import react from '@vitejs/plugin-react';

// https://vite.dev/config/
export default defineConfig({
    plugins: [tsconfigPaths(), react()],
    build: {
        rollupOptions: {
            input: {
                main: resolve(__dirname, 'index.html'),
                day08: resolve(__dirname, 'day08.html'),
            },
            output: {
                dir: resolve(__dirname, '../../advent-of-code-data/2025/browser'),
            },
        },
    },
});
