#!/usr/bin/env python3
"""Serve the built UQM preview on loopback with required Wasm thread headers."""
import argparse
import errno
from functools import partial
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
import webbrowser
from urllib.request import urlopen


class Handler(SimpleHTTPRequestHandler):
    extensions_map = {**SimpleHTTPRequestHandler.extensions_map, '.wasm': 'application/wasm'}

    def end_headers(self):
        self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
        self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
        self.send_header('Cache-Control', 'no-cache')
        super().end_headers()

    def list_directory(self, path):
        self.send_error(404)
        return None


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--port', type=int, default=9999)
    parser.add_argument('--open', action='store_true', help='Open the game in your default browser')
    args = parser.parse_args()
    directory = Path(__file__).resolve().parents[1] / 'dist'
    if not (directory / 'index.html').is_file():
        parser.error('No build found. Run scripts/build-web.sh first.')
    url = f'http://127.0.0.1:{args.port}'
    try:
        server = ThreadingHTTPServer(('127.0.0.1', args.port), partial(Handler, directory=str(directory)))
    except OSError as error:
        if error.errno == errno.EADDRINUSE and args.open:
            try:
                with urlopen(url, timeout=2) as response:
                    existing_game = b'<title>The Ur-Quan Masters' in response.read(4096)
                if existing_game:
                    webbrowser.open(url)
                    raise SystemExit(0)
            except OSError:
                pass
        parser.error(f'Could not start on port {args.port}: {error}')
    print(f'Local game: {url}', flush=True)
    print('Keep this window open while playing. Press Ctrl-C to stop.', flush=True)
    if args.open:
        webbrowser.open(url)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()
