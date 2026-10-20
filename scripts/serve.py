#!/usr/bin/env python3
"""Serve the built UQM preview on loopback with required Wasm thread headers."""
import argparse
import errno
from functools import partial
import hashlib
from http import HTTPStatus
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
import os
from pathlib import Path
import stat
import threading
import webbrowser
from urllib.request import urlopen


class Digests:
    """SHA-256 ETags, as the Cloudflare Worker sends them, kept while a file's size and time stay."""

    def __init__(self):
        self.lock = threading.Lock()
        self.known = {}
        self.busy = {}

    def etag(self, path):
        try:
            info = os.stat(path)
        except OSError:
            return None
        if not stat.S_ISREG(info.st_mode):
            return None
        key = (path, info.st_size, info.st_mtime_ns)
        with self.lock:
            if key in self.known:
                return self.known[key]
            # One reader per file: a second request waits for the first one's hash.
            busy = self.busy.setdefault(key, threading.Lock())
        with busy:
            with self.lock:
                if key in self.known:
                    return self.known[key]
            digest = hashlib.sha256()
            with open(path, 'rb') as handle:
                for block in iter(lambda: handle.read(1 << 20), b''):
                    digest.update(block)
            etag = '"' + digest.hexdigest() + '"'
            with self.lock:
                self.known[key] = etag
                self.busy.pop(key, None)
            return etag


class Handler(SimpleHTTPRequestHandler):
    extensions_map = {**SimpleHTTPRequestHandler.extensions_map, '.wasm': 'application/wasm'}
    digests = Digests()

    def send_head(self):
        # The game page keeps its data between visits when the server names the version (ETag).
        path = self.translate_path(self.path)
        self.etag = self.digests.etag(path) if os.path.isfile(path) else None
        self.status = None
        if self.etag and self.etag in [tag.strip() for tag in self.headers.get('If-None-Match', '').split(',')]:
            self.send_response(HTTPStatus.NOT_MODIFIED)
            self.end_headers()
            return None
        return super().send_head()

    def send_response(self, code, message=None):
        self.status = code
        super().send_response(code, message)

    def end_headers(self):
        if getattr(self, 'etag', None) and getattr(self, 'status', None) in (HTTPStatus.OK, HTTPStatus.NOT_MODIFIED):
            self.send_header('ETag', self.etag)
        self.etag = None
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
    parser.add_argument('--directory', type=Path, default=Path(__file__).resolve().parents[1] / 'dist',
                        help='The built site to serve (default: dist)')
    args = parser.parse_args()
    directory = args.directory.resolve()
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
    # Hash the large data files now, so the page's first version check gets its answer quickly.
    threading.Thread(target=lambda: [Handler.digests.etag(str(path)) for path in sorted(directory.rglob('*.data'))],
                     daemon=True).start()
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
