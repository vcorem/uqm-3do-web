// Test-only: until the room client has the lobby calls (watchLobby, and name
// and engine for a host), add them here, as the lobby contract describes.
// Loaded with Playwright's addInitScript; a client that has them is unchanged.
(() => {
  'use strict';
  let api;
  function patch(base) {
    if (!base || typeof base.watchLobby === 'function') return base;
    const out = Object.assign({}, base);
    out.connect = options => {
      if (options.role !== 'host' || !options.name) return base.connect(options);
      const Socket = options.WebSocket || window.WebSocket;
      const extra = '&name=' + encodeURIComponent(options.name) + (options.engine ? '&engine=' + encodeURIComponent(options.engine) : '');
      return base.connect(Object.assign({}, options, { WebSocket: function (url) { return new Socket(url + extra); } }));
    };
    out.watchLobby = ({ edition, onRooms, onStatus }) => {
      const url = new URL('/netplay/lobby?edition=' + (edition === 'hd' ? 'hd' : 'original'), location.href);
      url.protocol = url.protocol === 'https:' ? 'wss:' : 'ws:';
      let ended = false;
      const status = (state, message) => { if (typeof onStatus === 'function') onStatus(state, message); };
      const ws = new WebSocket(url.href);
      status('connecting', 'Connecting to the lobby…');
      ws.onopen = () => { if (!ended) status('open', 'Lobby connected.'); };
      ws.onmessage = event => {
        if (ended) return;
        let message;
        try { message = JSON.parse(event.data); } catch (_) { return; }
        if (message && message.type === 'rooms' && Array.isArray(message.rooms)) onRooms(message.rooms);
      };
      ws.onerror = () => { if (!ended) { ended = true; status('error', 'The room list is not available right now.'); } };
      ws.onclose = () => { if (!ended) { ended = true; status('closed', 'The room list closed.'); } };
      return { close() { ended = true; try { ws.close(); } catch (_) { /* Closed. */ } } };
    };
    return out;
  }
  Object.defineProperty(window, 'UQMNetplay', {
    configurable: true,
    get() { return api; },
    set(value) { api = patch(value); },
  });
})();
