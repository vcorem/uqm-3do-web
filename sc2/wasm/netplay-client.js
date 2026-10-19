/* Browser transport only. The engine polls queues; socket callbacks never call C. */
(function (root, factory) {
    const api = factory(root);
    if (typeof module === 'object' && module.exports) module.exports = api;
    else root.UQMNetplay = api;
})(typeof globalThis !== 'undefined' ? globalThis : this, function (root) {
    'use strict';
    const TOKEN = /^[a-f0-9]{32}$/i;
    const MAX_FRAME_BYTES = 64 * 1024;
    const MAX_QUEUE_BYTES = 1024 * 1024;
    const MAX_QUEUE_FRAMES = 4096;

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
    function relayURL(token, role, location, edition) {
        if (role !== 'host' && role !== 'guest') throw new Error('Invalid room role.');
        if (edition !== undefined && edition !== 'original' && edition !== 'hd') throw new Error('Invalid edition.');
        const url = pageURL(location || root.location);
        url.protocol = url.protocol === 'https:' ? 'wss:' : 'ws:';
        // HD rooms have their own path: the two engines cannot play each other.
        url.pathname = '/netplay/' + (edition === 'hd' ? 'hd/' : '') + checkedToken(token);
        url.search = '?role=' + role;
        url.hash = '';
        return url.href;
    }

    function connect(options) {
        const role = options.role;
        const url = relayURL(options.token, role, options.location || root.location, options.edition);
        const Socket = options.WebSocket || root.WebSocket;
        const schedule = options.setTimeout || root.setTimeout.bind(root);
        const cancel = options.clearTimeout || root.clearTimeout.bind(root);
        let socket;
        let timeout;
        let ended = false;
        const client = {
            role: role === 'host' ? 0 : 1,
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
                    end('error', 'The connection was interrupted. Reload to start a new match.');
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
        function end(state, message) {
            if (ended) return;
            ended = true;
            if (timeout !== undefined) cancel(timeout);
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
            timeout = schedule(() => end('error', 'Could not connect to online play on this server. Please try the online preview link.'), 12000);
            socket.onopen = () => {
                if (ended) return;
                cancel(timeout);
                status('waiting', role === 'host' ? 'Room created. Share the invite with your opponent.' : 'Joining your opponent’s room…');
            };
            socket.onmessage = event => {
                if (ended) return;
                if (typeof event.data === 'string') {
                    if (event.data.length > 2048) { end('error', 'The online server sent an oversized message.'); return; }
                    let message;
                    try { message = JSON.parse(event.data); }
                    catch (_) { end('error', 'The online server sent an invalid message.'); return; }
                    if (!message || typeof message !== 'object') {
                        end('error', 'The online server sent an invalid message.'); return;
                    }
                    if (message.type === 'waiting') {
                        if (client.state !== 'ready') status('waiting', 'Waiting for your opponent to join…');
                    } else if (message.type === 'ready') {
                        if (message.role !== role) { end('error', 'The room assigned an unexpected player role.'); return; }
                        status('ready', 'Opponent connected. Each player can now launch online Melee.');
                    } else if (message.type === 'peer-left') {
                        end('closed', 'Your opponent left the room. Reload to start a new match.');
                    } else if (message.type === 'error') {
                        const detail = typeof message.message === 'string' ? message.message.slice(0, 240) : 'Please create a new room.';
                        end('error', 'Could not join this room. ' + detail);
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
                    end('error', 'Too much game data arrived to continue safely. Reload to start a new match.'); return;
                }
                client.incoming.push(bytes);
                client.incomingBytes += bytes.byteLength;
                client.stats.receivedBytes += bytes.byteLength;
                client.stats.receivedFrames++;
                client.stats.peakIncomingBytes = Math.max(client.stats.peakIncomingBytes, client.incomingBytes);
            };
            socket.onerror = () => end('error', 'Could not connect to online play on this server. Please try the online preview link.');
            socket.onclose = () => end('closed', 'The connection closed. Reload to start a new match.');
        } catch (_) {
            end('error', 'Online play is unavailable in this browser or on this server.');
        }
        return client;
    }
    return { createToken, parseRoom, inviteURL, relayURL, connect,
        limits: { frameBytes: MAX_FRAME_BYTES, queueBytes: MAX_QUEUE_BYTES, queueFrames: MAX_QUEUE_FRAMES } };
});
