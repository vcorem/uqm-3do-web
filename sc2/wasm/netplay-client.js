/* Browser transport only. The engine polls queues; socket callbacks never call C. */
(function (root, factory) {
    const api = factory(root);
    if (typeof module === 'object' && module.exports) module.exports = api;
    else root.UQMNetplay = api;
})(typeof globalThis !== 'undefined' ? globalThis : this, function (root) {
    'use strict';
    const TOKEN = /^[a-f0-9]{32}$/i;
    const LISTED_TOKEN = /^[a-f0-9]{32}$/;
    const MAX_FRAME_BYTES = 64 * 1024;
    const MAX_QUEUE_BYTES = 1024 * 1024;
    const MAX_QUEUE_FRAMES = 4096;
    // Room names: the relay applies the same rule (deploy/netplay.js, server/netplay.mjs).
    // Refused: control and format characters, line separators, surrogates,
    // private-use and unassigned code points, blank fillers and more than 3
    // stacked nonspacing marks. A name needs a letter, digit, punctuation or symbol.
    const MAX_NAME_CHARS = 24;
    const NAME_FORBIDDEN = /[\p{Cc}\p{Cf}\p{Cs}\p{Co}\p{Cn}\p{Zl}\p{Zp}\u115f\u1160\u2800\u3164\uffa0]/u;
    // Counted in the decomposed form, where a precomposed letter shows its marks.
    const NAME_STACKED = /[\p{Mn}\p{Me}]{4}/u;
    const NAME_VISIBLE = /[\p{L}\p{N}\p{P}\p{S}]/u;
    const MAX_LISTED_ROOMS = 50;
    const MAX_LOBBY_TEXT = 64 * 1024;
    const CONNECT_MS = 12000;
    // Browsers cannot send ping frames. Quiet sockets send the text "ping", which
    // the relay answers with "pong", so that proxies do not close them.
    const KEEPALIVE_MS = 30000;
    const RETRY_MAX_MS = 15000;

    function pageURL(location) {
        const url = new URL(location.href || String(location));
        if (url.protocol !== 'http:' && url.protocol !== 'https:') {
            throw new Error('Open the game through its preview server to play online.');
        }
        return url;
    }
    function checkedToken(token) {
        if (!TOKEN.test(token || '')) throw new Error('Enter a valid room invite or room code.');
        return token.toLowerCase();
    }
    function createToken(crypto) {
        const bytes = new Uint8Array(16);
        (crypto || root.crypto).getRandomValues(bytes);
        return Array.from(bytes, byte => byte.toString(16).padStart(2, '0')).join('');
    }
    function parseRoom(input, location) {
        const value = String(input || '').trim();
        if (TOKEN.test(value)) return checkedToken(value);
        const base = pageURL(location || root.location);
        let invite;
        try { invite = new URL(value, base); }
        catch (_) { throw new Error('Enter a valid room invite or room code.'); }
        if (invite.origin !== base.origin) {
            throw new Error('Use an invite from this game server.');
        }
        return checkedToken(new URLSearchParams(invite.hash.slice(1)).get('room'));
    }
    function inviteURL(token, location) {
        const url = pageURL(location || root.location);
        url.search = '';
        url.hash = 'room=' + checkedToken(token);
        return url.href;
    }
    /**
     * The name as the room list shows it (NFC, inner spaces collapsed to one,
     * trimmed). Throws a message for the player when it cannot be listed.
     */
    function roomName(input) {
        const text = String(input == null ? '' : input).normalize('NFC');
        // Before collapsing spaces: line breaks and tabs are control characters.
        const refused = NAME_FORBIDDEN.test(text) || NAME_STACKED.test(text.normalize('NFD'));
        const name = refused ? '' : text.replace(/\s+/g, ' ').trim();
        const length = Array.from(name).length;
        if (!length || length > MAX_NAME_CHARS || !NAME_VISIBLE.test(name)) {
            throw new Error('Room names need 1 to ' + MAX_NAME_CHARS + ' visible characters.');
        }
        return name;
    }
    function checkedEdition(edition) {
        if (edition !== undefined && edition !== 'original' && edition !== 'hd') throw new Error('Invalid edition.');
        return edition === 'hd' ? 'hd' : 'original';
    }
    function socketURL(location) {
        const url = pageURL(location || root.location);
        url.protocol = url.protocol === 'https:' ? 'wss:' : 'ws:';
        url.hash = '';
        return url;
    }
    /**
     * listing ({ name, engine }) is for a host only: a name lists the room in
     * its edition's lobby, and engine ('c' or 'rust') is shown next to it.
     */
    function relayURL(token, role, location, edition, listing) {
        if (role !== 'host' && role !== 'guest') throw new Error('Invalid room role.');
        checkedEdition(edition);
        const url = socketURL(location);
        // HD rooms have their own path: the two engines cannot play each other.
        url.pathname = '/netplay/' + (edition === 'hd' ? 'hd/' : '') + checkedToken(token);
        let search = '?role=' + role;
        if (listing && listing.name != null) {
            if (role !== 'host') throw new Error('Only the host names a room.');
            search += '&name=' + encodeURIComponent(roomName(listing.name));
            if (listing.engine != null) {
                if (listing.engine !== 'c' && listing.engine !== 'rust') throw new Error('Invalid engine.');
                search += '&engine=' + listing.engine;
            }
        }
        url.search = search;
        return url.href;
    }
    function lobbyURL(edition, location) {
        const url = socketURL(location);
        url.pathname = '/netplay/lobby';
        url.search = '?edition=' + checkedEdition(edition);
        return url.href;
    }

    function connect(options) {
        const role = options.role;
        const listed = role === 'host' && options.name != null;
        const url = relayURL(options.token, role, options.location || root.location, options.edition,
            { name: options.name, engine: options.engine });
        const name = listed ? roomName(options.name) : null;
        const Socket = options.WebSocket || root.WebSocket;
        const schedule = options.setTimeout || root.setTimeout.bind(root);
        const cancel = options.clearTimeout || root.clearTimeout.bind(root);
        let socket;
        let timeout;
        let keepTimer;
        let answered = true;
        let ended = false;
        const client = {
            role: role === 'host' ? 0 : 1,
            name,
            state: 'waiting',
            incoming: [],
            incomingBytes: 0,
            stats: { receivedBytes: 0, receivedFrames: 0, sentBytes: 0,
                sentFrames: 0, peakIncomingBytes: 0 },
            send(bytes) {
                if (ended || client.state !== 'ready' || !socket || socket.readyState !== 1) return false;
                if (!(bytes instanceof Uint8Array) || !bytes.byteLength || bytes.byteLength > MAX_FRAME_BYTES) {
                    end('error', 'The game sent an invalid network packet.');
                    return false;
                }
                if (socket.bufferedAmount + bytes.byteLength > MAX_QUEUE_BYTES) {
                    end('error', 'The connection is too slow to continue this match.');
                    return false;
                }
                try {
                    socket.send(bytes);
                    client.stats.sentBytes += bytes.byteLength;
                    client.stats.sentFrames++;
                    return true;
                } catch (_) {
                    end('error', 'The connection was interrupted.');
                    return false;
                }
            },
            close() { end('closed', 'You left the room.'); }
        };
        function status(state, message) {
            client.state = state;
            if (typeof options.onStatus === 'function') {
                try { options.onStatus({ state, message, role }); }
                catch (error) { if (root.console) root.console.error('Room status could not be shown:', error); }
            }
        }
        function stopKeepalive() {
            if (keepTimer !== undefined) cancel(keepTimer);
            keepTimer = undefined;
        }
        // A named room can wait 10 minutes for a guest without any other traffic.
        // Only named rooms ping: every relay that lists rooms answers the ping.
        // The relay closes a listed room whose host stops pinging, and a ping
        // without an answer means that the connection is lost.
        function keepalive() {
            keepTimer = schedule(() => {
                keepTimer = undefined;
                if (ended || client.state !== 'waiting' || socket.readyState !== 1) return;
                if (!answered) { end('error', 'Lost the connection to online play. Create a new room.'); return; }
                answered = false;
                try { socket.send('ping'); } catch (_) { end('error', 'Lost the connection to online play. Create a new room.'); return; }
                keepalive();
            }, KEEPALIVE_MS);
        }
        function end(state, message) {
            if (ended) return;
            ended = true;
            if (timeout !== undefined) cancel(timeout);
            stopKeepalive();
            client.incoming.length = 0;
            client.incomingBytes = 0;
            status(state, message);
            if (socket && socket.readyState < 2) {
                try { socket.close(1000, 'Room closed'); } catch (_) { /* Already disconnected. */ }
            }
        }
        status('waiting', 'Connecting to online play…');
        try {
            socket = new Socket(url);
            socket.binaryType = 'arraybuffer';
            timeout = schedule(() => end('error', 'Could not connect to online play on this server. Please try the online preview link.'), CONNECT_MS);
            socket.onopen = () => {
                if (ended) return;
                cancel(timeout);
                if (listed) {
                    status('waiting', 'Opening room “' + name + '”…');
                    keepalive();
                } else {
                    status('waiting', role === 'host' ? 'Room created. Share the invite with your opponent.' : 'Joining your opponent’s room…');
                }
            };
            socket.onmessage = event => {
                if (ended) return;
                answered = true;
                if (event.data === 'pong') return;
                if (typeof event.data === 'string') {
                    if (event.data.length > 2048) { end('error', 'The online server sent an oversized message.'); return; }
                    let message;
                    try { message = JSON.parse(event.data); }
                    catch (_) { end('error', 'The online server sent an invalid message.'); return; }
                    if (!message || typeof message !== 'object') {
                        end('error', 'The online server sent an invalid message.'); return;
                    }
                    if (message.type === 'waiting') {
                        // For a named room, "waiting" also means that the room is on the list.
                        if (client.state !== 'ready') status('waiting', listed
                            ? 'Room “' + name + '” is open. Waiting for an opponent to pick it…'
                            : 'Waiting for your opponent to join…');
                    } else if (message.type === 'ready') {
                        if (message.role !== role) { end('error', 'The room assigned an unexpected player role.'); return; }
                        stopKeepalive();
                        status('ready', 'Opponent connected.');
                    } else if (message.type === 'peer-left') {
                        end('closed', 'Your opponent left the room.');
                    } else if (message.type === 'error') {
                        const detail = typeof message.message === 'string' ? message.message.slice(0, 240) : 'Please create a new room.';
                        // The relay's messages to a host stand alone ("The room list is full…").
                        end('error', role === 'host' ? detail : 'Could not join this room. ' + detail);
                    } else {
                        end('error', 'The online server sent an unknown message.');
                    }
                    return;
                }
                if (!(event.data instanceof ArrayBuffer) || client.state !== 'ready') {
                    end('error', 'The online server sent unexpected game data.'); return;
                }
                const bytes = new Uint8Array(event.data);
                if (!bytes.byteLength) return;
                if (bytes.byteLength > MAX_FRAME_BYTES || client.incomingBytes + bytes.byteLength > MAX_QUEUE_BYTES || client.incoming.length >= MAX_QUEUE_FRAMES) {
                    end('error', 'Too much game data arrived to continue safely.'); return;
                }
                client.incoming.push(bytes);
                client.incomingBytes += bytes.byteLength;
                client.stats.receivedBytes += bytes.byteLength;
                client.stats.receivedFrames++;
                client.stats.peakIncomingBytes = Math.max(client.stats.peakIncomingBytes, client.incomingBytes);
            };
            socket.onerror = () => end('error', 'Could not connect to online play on this server. Please try the online preview link.');
            socket.onclose = () => end('closed', 'The connection closed.');
        } catch (_) {
            end('error', 'Online play is unavailable in this browser or on this server.');
        }
        return client;
    }

    /** Checked copies of the relay's room entries; anything malformed is left out. */
    function lobbyRooms(list) {
        const rooms = [];
        for (const room of list.slice(0, MAX_LISTED_ROOMS)) {
            if (!room || typeof room !== 'object' || typeof room.id !== 'string' || !LISTED_TOKEN.test(room.id) ||
                typeof room.name !== 'string') continue;
            let name;
            try { name = roomName(room.name); } catch (_) { continue; }
            rooms.push({ id: room.id, name,
                engine: room.engine === 'c' || room.engine === 'rust' ? room.engine : null,
                created: Number.isSafeInteger(room.created) ? room.created : 0 });
        }
        return rooms;
    }

    /**
     * Live list of the named rooms that wait for a guest in one edition.
     * onRooms(rooms) gets every snapshot: [{ id, name, engine, created }],
     * newest first. onStatus(state, message) reports 'connecting', 'open' (the
     * list is live), 'error' (the connection failed or was lost; the watcher
     * tries again by itself) and 'closed' (after close(), or when this page
     * cannot use online play). Join a listed room with
     * connect({ token: room.id, role: 'guest', edition }).
     */
    function watchLobby(options) {
        options = options || {};
        const edition = checkedEdition(options.edition);
        const Socket = options.WebSocket || root.WebSocket;
        const schedule = options.setTimeout || root.setTimeout.bind(root);
        const cancel = options.clearTimeout || root.clearTimeout.bind(root);
        let socket = null;
        let timer;
        let keepTimer;
        let attempts = 0;
        let closed = false;
        let answered = false;
        const watcher = {
            state: 'connecting',
            rooms: [],
            close() {
                if (closed) return;
                closed = true;
                stopTimers();
                const current = socket;
                socket = null;
                closeSocket(current);
                status('closed', 'Left the room list.');
            }
        };
        function status(state, message) {
            watcher.state = state;
            if (typeof options.onStatus === 'function') {
                try { options.onStatus(state, message); }
                catch (error) { if (root.console) root.console.error('Room list status could not be shown:', error); }
            }
        }
        function stopTimers() {
            if (timer !== undefined) cancel(timer);
            if (keepTimer !== undefined) cancel(keepTimer);
            timer = keepTimer = undefined;
        }
        function closeSocket(current) {
            if (!current) return;
            current.onopen = current.onmessage = current.onerror = current.onclose = null;
            if (current.readyState < 2) {
                try { current.close(1000, 'Room list closed'); } catch (_) { /* Already disconnected. */ }
            }
        }
        // Lost or refused connections retry after 1, 2, 4, 8, then every 15 seconds.
        function drop(current, message) {
            if (closed || current !== socket) return;
            socket = null;
            stopTimers();
            closeSocket(current);
            const delay = Math.min(RETRY_MAX_MS, 1000 * Math.pow(2, Math.min(attempts, 4)));
            attempts++;
            status('error', message);
            if (!closed) timer = schedule(() => { timer = undefined; open(); }, delay);
        }
        function keepalive(current) {
            keepTimer = schedule(() => {
                keepTimer = undefined;
                if (current !== socket) return;
                if (!answered) { drop(current, 'The room list stopped answering. Reconnecting…'); return; }
                answered = false;
                try { current.send('ping'); }
                catch (_) { drop(current, 'Lost the connection to the room list. Reconnecting…'); return; }
                keepalive(current);
            }, KEEPALIVE_MS);
        }
        function receive(current, data) {
            answered = true;
            if (data === 'pong') return;
            let message = null;
            if (typeof data === 'string' && data.length <= MAX_LOBBY_TEXT) {
                try { message = JSON.parse(data); } catch (_) { message = null; }
            }
            if (message && message.type === 'error') {
                const detail = typeof message.message === 'string' ? message.message.slice(0, 240) : 'The room list is unavailable.';
                drop(current, detail + ' Trying again…');
                return;
            }
            if (!message || message.type !== 'rooms' || !Array.isArray(message.rooms)) {
                drop(current, 'The room list sent an unexpected message. Reconnecting…');
                return;
            }
            if (watcher.state !== 'open') {
                if (timer !== undefined) cancel(timer);
                timer = undefined;
                attempts = 0;
                status('open', 'Showing the open rooms.');
                keepalive(current);
            }
            watcher.rooms = lobbyRooms(message.rooms);
            if (typeof options.onRooms === 'function') {
                try { options.onRooms(watcher.rooms.slice()); }
                catch (error) { if (root.console) root.console.error('Room list could not be shown:', error); }
            }
        }
        function open() {
            if (closed) return;
            let url;
            try { url = lobbyURL(edition, options.location || root.location); }
            catch (error) { closed = true; status('closed', error.message); return; }
            status('connecting', attempts ? 'Reconnecting to the room list…' : 'Connecting to the room list…');
            let current;
            try { current = new Socket(url); }
            catch (_) { closed = true; status('closed', 'Online play is unavailable in this browser.'); return; }
            socket = current;
            answered = false;
            current.binaryType = 'arraybuffer';
            timer = schedule(() => { timer = undefined; drop(current, 'Could not reach the room list. Trying again…'); }, CONNECT_MS);
            current.onmessage = event => { if (!closed && current === socket) receive(current, event.data); };
            current.onerror = () => drop(current, 'Could not reach the room list. Trying again…');
            current.onclose = () => drop(current, 'Lost the connection to the room list. Reconnecting…');
        }
        open();
        return watcher;
    }

    return { createToken, parseRoom, inviteURL, relayURL, lobbyURL, roomName, connect, watchLobby,
        limits: { frameBytes: MAX_FRAME_BYTES, queueBytes: MAX_QUEUE_BYTES, queueFrames: MAX_QUEUE_FRAMES,
            roomNameChars: MAX_NAME_CHARS, listedRooms: MAX_LISTED_ROOMS } };
});
