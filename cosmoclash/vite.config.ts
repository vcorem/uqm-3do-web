import { defineConfig } from 'vite';
import path from 'path';

export default defineConfig({
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src'),
    },
  },
  server: {
    host: '127.0.0.1',
    proxy: { '/netplay/': { target: 'http://127.0.0.1:10000', ws: true } },
  },
  test: {
    globals: true,
    environment: 'jsdom',
  },
});
