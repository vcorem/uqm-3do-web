import { defineConfig } from 'vite';
import path from 'path';

// The site serves the game at /cosmoclash/ (urquanmasters.org/cosmoclash/).
// Unit tests keep '/', so the upstream tests see the original root paths.
export default defineConfig(({ mode }) => ({
  base: mode === 'test' ? '/' : '/cosmoclash/',
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src'),
    },
  },
  server: {
    host: '127.0.0.1',
    // npm start (port 10000) has the relay and the room list.
    proxy: { '/netplay/': { target: 'http://127.0.0.1:10000', ws: true } },
  },
  preview: {
    host: '127.0.0.1',
    proxy: { '/netplay/': { target: 'http://127.0.0.1:10000', ws: true } },
  },
  test: {
    globals: true,
    exclude: ['**/node_modules/**', '**/.git/**', 'reference/**'],
    environment: 'jsdom',
  },
}));
