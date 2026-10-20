/* The start screen and the online lobby, drawn inside the game frame with the
   game's own fonts and art (see build-art.py). Shared by both editions.

   createController: the state machine (no DOM; unit tested in Node).
   createRenderer:   draws one state on a canvas and returns the hit regions.
   init:             connects both to the page, the engine and UQMNetplay. */
(function (root, factory) {
    'use strict';
    const api = factory(root);
    if (typeof module === 'object' && module.exports) module.exports = api;
    else root.UQMOnline = api;
})(typeof globalThis !== 'undefined' ? globalThis : this, function (root) {
    'use strict';
    const NAME_KEY = 'uqm.online.roomName.v1';
    const NAME_MAX = 24;
    const COUNTDOWN_MS = 3000;
    const LOBBY_RETRY_MS = [2000, 5000, 10000, 20000];
    // A role queued for the engine's main menu that is not taken by then ends the session.
    const ROLE_TIMEOUT_MS = 15000;
    // Room names follow the relay's rule (UQMNetplay.roomName): NFC, inner spaces
    // collapsed, no control or format characters, line separators, surrogates,
    // private-use or unassigned code points, blank fillers or more than 3
    // stacked marks, and at least one letter, digit, punctuation or symbol.
    const NAME_FORBIDDEN = /[\p{Cc}\p{Cf}\p{Cs}\p{Co}\p{Cn}\p{Zl}\p{Zp}\u115f\u1160\u2800\u3164\uffa0]/u;
    const NAME_STACKED = /[\p{Mn}\p{Me}]{4}/u;
    const NAME_VISIBLE = /[\p{L}\p{N}\p{P}\p{S}]/u;
    const TOKEN = /^[a-f0-9]{32}$/;
    // Text drawn right to left when its first letter is from a right-to-left script.
    const RTL_FIRST = /^[^\p{L}]*[\p{Script=Hebrew}\p{Script=Arabic}\p{Script=Syriac}\p{Script=Thaana}\p{Script=Nko}]/u;

    // ------------------------------------------------------------ helpers

    // The name as the room list shows it, or the reason it cannot be used.
    function cleanName(value) {
        const text = String(value == null ? '' : value).normalize('NFC');
        if (NAME_FORBIDDEN.test(text) || NAME_STACKED.test(text.normalize('NFD'))) {
            return { ok: false, name: text.trim(), error: 'Use only visible letters, numbers and symbols.' };
        }
        const name = text.replace(/\s+/g, ' ').trim();
        const length = Array.from(name).length;
        if (!length) return { ok: false, name, error: 'Type a name for your room.' };
        if (length > NAME_MAX) return { ok: false, name, error: 'Use at most ' + NAME_MAX + ' characters.' };
        if (!NAME_VISIBLE.test(name)) return { ok: false, name, error: 'Use at least one letter or number.' };
        return { ok: true, name };
    }
    // The room client's own check (when it has one) is the final word.
    function checkName(value, netplay) {
        const checked = cleanName(value);
        if (!checked.ok || !netplay || typeof netplay.roomName !== 'function') return checked;
        try {
            return { ok: true, name: netplay.roomName(checked.name) };
        } catch (error) {
            return { ok: false, name: checked.name, error: String(error && error.message || 'This name cannot be used.') };
        }
    }
    function isRtl(text) { return RTL_FIRST.test(String(text)); }
    function defaultName(random) {
        return 'Captain ' + (10 + Math.floor((random || Math.random)() * 90));
    }
    function waitingTime(created, now) {
        const seconds = Math.max(0, Math.floor((now - created) / 1000));
        if (!(created > 0) || seconds < 60) return 'new';
        const minutes = Math.floor(seconds / 60);
        return minutes >= 60 ? Math.floor(minutes / 60) + ' h' : minutes + ' min';
    }
    function clock(ms) {
        const seconds = Math.max(0, Math.floor(ms / 1000));
        return Math.floor(seconds / 60) + ':' + String(seconds % 60).padStart(2, '0');
    }
    // The room client explains next steps for the old page ("Reload to…").
    // The lobby shows its own next steps.
    function cleanMessage(message) {
        return String(message || '').replace(/\s*(Reload|Please try)[^.]*\.\s*$/i, '').trim();
    }
    function readInvite(hash) {
        try {
            const token = new URLSearchParams(String(hash || '').replace(/^#/, '')).get('room');
            return token && TOKEN.test(token.toLowerCase()) ? token.toLowerCase() : null;
        } catch (_) { return null; }
    }

    // ------------------------------------------------------------ state machine

    const SCREENS = {
        start: () => ['play', 'online'],
        lobby: state => state.lobby.rooms.map(room => 'room:' + room.id).concat(['create', 'back']),
        name: () => ['name', 'open', 'cancel'],
        waiting: () => ['cancel', 'copy'],
        joining: () => ['cancel'],
        found: () => ['cancel'],
        invite: () => ['join', 'dismiss'],
        restart: () => ['reload', 'back'],
        message: state => state.message && state.message.reload === false ? ['back'] : ['reload'],
        hidden: () => [],
    };
    const ESCAPE = { lobby: 'back', name: 'cancel', waiting: 'cancel', joining: 'cancel', found: 'cancel',
        invite: 'dismiss', restart: 'back' };
    // The screens that can cover the game's main menu.
    const OVER_MENU = ['lobby', 'name', 'waiting', 'joining', 'found', 'restart'];

    /* env: { edition, engine, now(), setTimeout(fn, ms), clearTimeout(id), random(),
         storage: { get(key), set(key, value) }, netplay (UQMNetplay),
         game: { started(), ready(), screen(), hook(), start(role, client), queue(role, client) (role -1 withdraws) },
         fullscreen() -> Promise<boolean>, reload(online), copy(text) -> Promise, focusGame(), changed(state) } */
    function createController(env) {
        const now = () => env.now();
        const state = {
            screen: 'start', focus: 'play', items: ['play', 'online'],
            loading: { label: 'Loading the game…', fraction: 0 }, ready: false,
            pendingPlay: false, message: null,
            lobby: { status: 'idle', rooms: [], error: '', scroll: 0 },
            notice: null, name: '', nameError: '',
            room: null, countdownEnd: 0, waitingForEngine: false,
            session: null, invite: null, engineScreen: undefined, asked: false, touched: false,
            edition: env.edition, engine: env.engine,
        };
        let watch = null, retryTimer = null, retries = 0, countdownTimer = null;

        function changed() {
            state.items = (SCREENS[state.screen] || SCREENS.hidden)(state);
            if (!state.items.includes(state.focus)) state.focus = state.items[0] || null;
            if (env.changed) env.changed(state);
        }
        function show(screen, focus) {
            const wasLobby = state.screen === 'lobby' || state.screen === 'name';
            if (screen !== state.screen) state.touched = false;
            state.screen = screen;
            if (focus !== undefined) state.focus = focus;
            if (screen === 'lobby' || screen === 'name') watchLobby();
            else if (wasLobby) stopLobby();
            if (screen === 'hidden' && env.focusGame) env.focusGame();
            changed();
        }
        function notice(text, kind) {
            state.notice = text ? { text, kind: kind || 'info', at: now() } : null;
            if (text && kind === 'error') sound('failure');
        }
        // The game's menu sounds (menu.snd): move, success, failure, invoked.
        function sound(name) { if (env.sound) env.sound(name); }
        // The first choice is a user gesture: the frame goes full screen then.
        // A refused request (no user activation, as for Esc) does not count.
        function gesture() {
            if (state.asked || !env.fullscreen) return;
            state.asked = true;
            let request;
            try { request = env.fullscreen(); } catch (_) { request = false; }
            Promise.resolve(request).then(done => { if (done === false) state.asked = false; }, () => { state.asked = false; });
        }
        const isReady = () => state.ready || env.game.ready();

        // -------------------------------------------------- lobby list
        function watchLobby() {
            if (watch || retryTimer) return;
            const netplay = env.netplay;
            if (!netplay || typeof netplay.watchLobby !== 'function') {
                state.lobby.status = 'unsupported';
                return;
            }
            state.lobby.status = state.lobby.status === 'open' ? 'open' : 'connecting';
            let handle = null;
            const own = {
                edition: state.edition,
                onRooms(rooms) {
                    if (watch !== handle && handle) return;
                    retries = 0;
                    state.lobby.status = 'open';
                    setRooms(Array.isArray(rooms) ? rooms : []);
                },
                onStatus(status, message) {
                    if (watch !== handle && handle) return;
                    if (status === 'open') { state.lobby.status = 'open'; retries = 0; changed(); return; }
                    if (status === 'connecting') return;
                    if (status === 'closed' || status === 'error') {
                        state.lobby.status = 'error';
                        state.lobby.error = cleanMessage(message) || 'The room list is not available right now.';
                        watch = null;
                        // A client that reconnects by itself must not run twice.
                        try { if (handle && typeof handle.close === 'function') handle.close(); } catch (_) { /* Closed. */ }
                        if (state.screen === 'lobby' || state.screen === 'name') {
                            const delay = LOBBY_RETRY_MS[Math.min(retries++, LOBBY_RETRY_MS.length - 1)];
                            retryTimer = env.setTimeout(() => { retryTimer = null; if (state.screen === 'lobby' || state.screen === 'name') { watchLobby(); changed(); } }, delay);
                        }
                        changed();
                    }
                },
            };
            try {
                handle = netplay.watchLobby(own);
                watch = handle;
            } catch (error) {
                state.lobby.status = 'error';
                state.lobby.error = cleanMessage(error && error.message) || 'The room list is not available right now.';
            }
        }
        function stopLobby() {
            if (retryTimer) { env.clearTimeout(retryTimer); retryTimer = null; }
            const handle = watch;
            watch = null;
            if (handle && typeof handle.close === 'function') {
                try { handle.close(); } catch (_) { /* Already closed. */ }
            }
            if (state.lobby.status !== 'unsupported') state.lobby.status = 'idle';
        }
        function setRooms(rooms) {
            const mine = state.room && state.room.role === 'host' ? state.room.token : null;
            const before = state.focus, index = state.items.indexOf(before);
            state.lobby.rooms = rooms.filter(room => room && TOKEN.test(String(room.id)) && room.id !== mine)
                .map(room => ({ id: String(room.id), name: String(room.name || 'Room').slice(0, 64),
                    engine: room.engine === 'rust' || room.engine === 'c' ? room.engine : null,
                    created: Number(room.created) || 0 }))
                .slice(0, 50);
            state.items = SCREENS[state.screen] ? SCREENS[state.screen](state) : [];
            if (state.screen === 'lobby' && !state.touched && state.focus === 'create' && state.lobby.rooms.length) {
                // The first rooms arrived before the player moved: offer the newest.
                state.focus = 'room:' + state.lobby.rooms[0].id;
            } else if (state.screen === 'lobby' && !state.items.includes(before) && String(before).startsWith('room:')) {
                // The focused room went away: stay at the same place in the list.
                const rooms = state.items.filter(item => item.startsWith('room:'));
                state.focus = rooms[Math.min(index, rooms.length - 1)] || 'create';
            }
            changed();
        }

        // -------------------------------------------------- rooms
        function closeRoom() {
            const room = state.room;
            state.room = null;
            if (countdownTimer) { env.clearTimeout(countdownTimer); countdownTimer = null; }
            if (room && room.client && !['closed', 'error'].includes(room.client.state)) {
                room.closing = true;
                try { room.client.close(); } catch (_) { /* Already closed. */ }
            }
        }
        // name: the listed room's name for a host. A host without a name opens a
        // private room, which only its invite link joins (scripted tests).
        function connect(role, token, name) {
            closeRoom();
            const room = { token, role, name: name || '', listed: role === 'host' && !!name,
                status: 'loading', message: '', client: null, opened: now() };
            state.room = room;
            // Enter must not close a room just opened: the host's waiting screen
            // offers Copy link first, and Enter does nothing on the joining screen.
            show(role === 'host' ? 'waiting' : 'joining', role === 'host' ? 'copy' : 'cancel');
            // A room opens (and a guest joins) only when this page's game is ready,
            // so that no opponent waits for a download.
            if (isReady()) openRoom(room);
        }
        function openRoom(room) {
            room.status = 'waiting';
            room.opened = now();
            const options = { token: room.token, role: room.role, edition: state.edition,
                onStatus: event => roomStatus(room, event) };
            if (room.listed) { options.name = room.name; options.engine = state.engine; }
            let client;
            try {
                client = env.netplay.connect(options);
            } catch (error) {
                if (state.room !== room) return;
                state.room = null;
                notice(cleanMessage(error && error.message) || 'Could not open the room.', 'error');
                show('lobby');
                return;
            }
            // connect() may have reported an error at once; roomStatus has shown it.
            if (state.room !== room) return;
            room.client = client;
            // The name as the room list shows it.
            if (room.listed && client && typeof client.name === 'string' && client.name) room.name = client.name;
            changed();
        }
        function roomStatus(room, event) {
            if (!event) return;
            room.status = event.state;
            room.message = cleanMessage(event.message);
            if (state.session && state.session.room === room) {
                if (event.state === 'closed' || event.state === 'error') state.session.endMessage = room.message;
                return;
            }
            if (state.room !== room || room.closing) return;
            if (event.state === 'ready') {
                sound('invoked');
                state.countdownEnd = now() + COUNTDOWN_MS;
                show('found', 'cancel');
                scheduleCountdown();
            } else if (event.state === 'closed' || event.state === 'error') {
                state.room = null;
                if (countdownTimer) { env.clearTimeout(countdownTimer); countdownTimer = null; }
                const fallback = room.role === 'host' ? 'The room closed.' : 'Could not join this room.';
                notice(room.message || fallback, 'error');
                show('lobby');
            } else if (state.screen === 'waiting' || state.screen === 'joining') {
                changed();
            }
        }
        function scheduleCountdown() {
            if (countdownTimer) env.clearTimeout(countdownTimer);
            const left = state.countdownEnd - now();
            countdownTimer = env.setTimeout(() => {
                countdownTimer = null;
                if (state.screen !== 'found') return;
                if (now() < state.countdownEnd) { scheduleCountdown(); return; }
                begin();
            }, Math.max(0, Math.min(left, 250)));
            if (left > 0) changed();
        }
        function begin() {
            const room = state.room;
            if (!room || room.status !== 'ready') return;
            const game = env.game;
            if (!game.started()) {
                if (!game.ready() && !state.ready) { state.waitingForEngine = true; changed(); return; }
                state.waitingForEngine = false;
                state.session = { room, role: room.role, hook: false, sawMelee: false };
                show('hidden');
                game.start(room.role, room.client);
                return;
            }
            if (game.hook()) {
                // The engine takes the role at its main menu (at once, or when it is back there).
                const session = { room, role: room.role, hook: true, sawMelee: false, taken: false };
                state.session = session;
                show('hidden');
                game.queue(room.role === 'host' ? 0 : 1, room.client);
                // An engine that does not take the role (it left its main menu) must not
                // leave the player with neither a match nor the lobby.
                env.setTimeout(() => {
                    if (state.session !== session || session.taken || session.sawMelee) return;
                    game.queue(-1, null);
                    state.session = null;
                    if (state.room === room) closeRoom();
                    notice('The game did not start the match. Try again.', 'error');
                    show('lobby', 'create');
                }, ROLE_TIMEOUT_MS);
                return;
            }
            closeRoom();
            restart('Online Melee starts from a fresh game page.');
        }
        function endSession() {
            const session = state.session;
            state.session = null;
            const message = session.endMessage || (session.room.client && cleanMessage(session.room.client.message)) || 'The online match is over.';
            if (state.room === session.room) closeRoom();
            // The engine marks the main-menu hook when it reaches its main menu;
            // after a first session that may be a moment after the screen changes.
            const deadline = now() + 2000;
            const decide = () => {
                if (env.game.hook()) { notice(message, 'info'); show('lobby', 'create'); }
                else if (now() < deadline) env.setTimeout(decide, 100);
                else restart(message, true);
            };
            decide();
        }
        function restart(reason, again) {
            state.message = { title: 'ONLINE MELEE', text: reason,
                detail: (again ? 'To play online again, reload the game.' : 'Reload to open Online Melee.') + ' Saved games stay in this browser.' };
            show('restart', 'reload');
        }

        // -------------------------------------------------- choices
        // how.escape: chosen with Esc, which gives no user activation for full screen.
        function activate(id, how) {
            if (!id || !state.items.includes(id)) return;
            if (!(how && how.escape)) gesture();
            if (id !== 'open' && id !== 'name') sound('success');
            if (id === 'play') {
                if (state.ready || env.game.ready()) { state.pendingPlay = false; show('hidden'); env.game.start(null); }
                else { state.pendingPlay = true; changed(); }
            } else if (id === 'online') {
                state.pendingPlay = false;
                openLobby();
            } else if (id.startsWith('room:')) {
                const room = state.lobby.rooms.find(room => 'room:' + room.id === id);
                if (room) { notice(null); connect('guest', room.id, room.name); }
            } else if (id === 'create') {
                notice(null);
                state.name = (env.storage && env.storage.get(NAME_KEY)) || defaultName(env.random);
                state.nameError = '';
                show('name', 'name');
            } else if (id === 'open' || id === 'name') {
                const checked = checkName(state.name, env.netplay);
                if (!checked.ok) { sound('failure'); state.nameError = checked.error; state.focus = 'name'; changed(); return; }
                sound('success');
                // The name as the list shows it; remembered only once it is accepted.
                state.name = checked.name;
                if (env.storage) env.storage.set(NAME_KEY, checked.name);
                connect('host', env.netplay.createToken(), checked.name);
            } else if (id === 'cancel') {
                if (state.screen === 'name') { show('lobby', 'create'); return; }
                const wasHost = state.room && state.room.role === 'host';
                closeRoom();
                notice(wasHost ? 'Your room is closed.' : null);
                show('lobby', wasHost ? 'create' : undefined);
            } else if (id === 'copy') {
                const room = state.room;
                if (!room) return;
                let url;
                try { url = env.netplay.inviteURL(room.token); } catch (_) { return; }
                Promise.resolve(env.copy ? env.copy(url) : Promise.reject(new Error('no clipboard'))).then(
                    () => { if (state.room === room) { notice('Invite link copied. Send it to your opponent.'); changed(); } },
                    () => { if (state.room === room) { notice('Copy this link: ' + url, 'link'); changed(); } });
            } else if (id === 'join') {
                const token = state.invite;
                state.invite = null;
                if (env.clearInvite) env.clearInvite();
                connect('guest', token, '');
            } else if (id === 'dismiss') {
                state.invite = null;
                if (env.clearInvite) env.clearInvite();
                show('start', 'online');
            } else if (id === 'back') {
                if (state.screen === 'restart' || env.game.started()) show('hidden');
                else show('start', 'online');
            } else if (id === 'reload') {
                env.reload(state.screen === 'restart');
            }
        }
        function openLobby() {
            if (!env.netplay || typeof env.netplay.connect !== 'function') {
                state.message = { title: 'ONLINE MELEE', text: 'Online play is not available in this build.', reload: false };
                show('message');
                return;
            }
            const game = env.game;
            if (game.started() && !game.hook()) {
                restart('Online Melee starts from a fresh game page.');
                return;
            }
            show('lobby', state.lobby.rooms.length ? 'room:' + state.lobby.rooms[0].id : 'create');
        }
        function move(delta) {
            const items = state.items;
            if (!items.length) return;
            state.touched = true;
            const index = items.indexOf(state.focus);
            const next = items[(index + delta + items.length) % items.length];
            if (next !== state.focus) sound('move');
            state.focus = next;
            changed();
        }
        // Lobby: Left/Right jump between the room list and the buttons.
        function side(direction) {
            if (state.screen !== 'lobby') { move(direction); return; }
            const rooms = state.items.filter(item => item.startsWith('room:'));
            state.touched = true;
            if (direction > 0 && String(state.focus).startsWith('room:')) { state.lastRoom = state.focus; state.focus = 'create'; }
            else if (direction < 0 && !String(state.focus).startsWith('room:') && rooms.length) state.focus = rooms.includes(state.lastRoom) ? state.lastRoom : rooms[0];
            else return;
            sound('move');
            changed();
        }
        function key(name) {
            switch (name) {
            case 'ArrowUp': move(-1); return true;
            case 'ArrowDown': move(1); return true;
            case 'ArrowLeft': side(-1); return true;
            case 'ArrowRight': side(1); return true;
            case 'Home': if (state.items.length) { state.focus = state.items[0]; changed(); } return true;
            case 'End': if (state.items.length) { state.focus = state.items[state.items.length - 1]; changed(); } return true;
            case 'PageUp': move(-6); return true;
            case 'PageDown': move(6); return true;
            case 'Enter': case ' ':
                // Joining and Opponent found have only Cancel: a reflex Enter must not
                // undo the join or the match (Esc or a click cancels).
                if (state.screen === 'joining' || state.screen === 'found') return true;
                activate(state.focus);
                return true;
            case 'Escape':
                if (ESCAPE[state.screen]) { activate(ESCAPE[state.screen], { escape: true }); return true; }
                if (state.items.includes('back')) { activate('back', { escape: true }); return true; }
                if (state.screen === 'start' && state.pendingPlay) { state.pendingPlay = false; changed(); return true; }
                return false;
            default: return false;
            }
        }

        // -------------------------------------------------- page and engine events
        const api = {
            state,
            activate, key, move, openLobby,
            focus(id) { if (state.items.includes(id) && state.focus !== id) { state.focus = id; changed(); } },
            setName(value) { state.name = Array.from(String(value)).slice(0, 64).join(''); state.nameError = ''; changed(); },
            progress(loading) {
                const before = state.loading;
                state.loading = Object.assign({}, before, loading);
                // Download progress arrives very often; redraw only for a visible change.
                const moved = Math.abs((state.loading.fraction || 0) - (before.fraction || 0)) >= 0.004;
                if (!state.ready && (moved || state.loading.label !== before.label)) changed();
            },
            ready() {
                state.ready = true;
                state.loading = { label: 'Ready.', fraction: 1 };
                if (state.pendingPlay && state.screen === 'start') { state.pendingPlay = false; show('hidden'); env.game.start(null); return; }
                // A room chosen during the download opens now.
                if (state.room && state.room.status === 'loading') openRoom(state.room);
                if (state.screen === 'found' && state.waitingForEngine) { begin(); return; }
                changed();
            },
            fail(text) {
                closeRoom();
                stopLobby();
                state.pendingPlay = false;
                const value = String(text || 'The game could not start.');
                // After the game has started, a failure stops it.
                state.message = env.game.started()
                    ? { title: 'GAME STOPPED', text: value, say: 'Game stopped. ' + value, failed: true }
                    : { title: 'UNABLE TO START', text: value, say: 'Unable to start. ' + value, failed: true };
                show('message', 'reload');
            },
            exited(text) {
                closeRoom();
                state.session = null;
                const value = text || 'Reload this page to play again.';
                state.message = { title: 'GAME CLOSED', text: value, say: 'Game closed. ' + value, closed: true };
                show('message', 'reload');
            },
            // Called with Module.gameScreen whenever it changes.
            gameScreen(name) {
                if (name === state.engineScreen) return;
                const was = state.engineScreen;
                state.engineScreen = name;
                const session = state.session;
                if (session) {
                    if (name === 'melee') session.sawMelee = true;
                    else if (name === 'mainMenu' && session.sawMelee) { endSession(); return; }
                } else if (was === 'mainMenu' && env.game.started() && OVER_MENU.includes(state.screen)) {
                    // The game left its main menu under the menu (a key pressed while the
                    // main menu faded in): no lobby over another game screen.
                    closeRoom();
                    show('hidden');
                    return;
                }
                changed();
            },
            invite(token) {
                if (!token) return;
                state.invite = token;
                show('invite', 'join');
            },
            // Show the lobby from the game's main menu (corner button or the O key).
            fromMenu() {
                if (state.screen !== 'hidden' || state.session) return;
                gesture();
                notice(null);
                openLobby();
            },
            close() { closeRoom(); stopLobby(); },
            tick() { changed(); },
            // The engine took the role queued for its main menu.
            roleTaken() { if (state.session) state.session.taken = true; },
            // Scripted play (remote tests): a room with a known token. A host without
            // a name opens a private room (not in the list; its invite link joins it).
            host(name, token) {
                token = token || env.netplay.createToken();
                if (name == null) { connect('host', token, ''); return; }
                const checked = checkName(name, env.netplay);
                if (checked.ok) connect('host', token, checked.name);
            },
            join(token, name) { if (TOKEN.test(String(token))) connect('guest', String(token), name || ''); },
        };
        changed();
        return api;
    }

    // ------------------------------------------------------------ drawing

    // Colours of the Super-Melee screen (melee.c, util.c, colors.h).
    const RGB = {
        teamName: [123, 132, 222], teamNameHi: [189, 198, 239], editRect: [165, 0, 165], cursor: [255, 255, 255],
        status: [0, 165, 0], notice: [248, 224, 0], error: [255, 84, 64], hint: [116, 116, 116],
        boxTL: [0, 0, 74], boxBR: [0, 0, 115], boxFill: [0, 0, 99],
        boxTLHi: [57, 0, 99], boxBRHi: [99, 0, 165], boxFillHi: [82, 0, 140],
        btnTL: [58, 58, 58], btnBR: [116, 116, 116], btnFill: [87, 87, 87], btnText: [138, 138, 138], btnTextHi: [248, 224, 0],
        ctrlFill: [2, 4, 62], ctrlFillHi: [4, 8, 124], ctrlBevel: [116, 116, 116], ctrlText: [35, 140, 210], ctrlTextHi: [40, 160, 240],
        popup: [165, 165, 165], popupText: [16, 16, 16], popupDark: [66, 66, 66], popupMedium: [132, 132, 132],
        tag: [0, 165, 165], tagRust: [222, 115, 41],
    };
    const MENU = {
        original: { text: [150, 150, 150], hi: [214, 214, 214], glow: [255, 255, 255] },
        hd: { text: [0, 110, 0], hi: [0, 190, 0], glow: [90, 255, 90] },
    };
    const css = rgb => 'rgb(' + rgb.join(',') + ')';

    function createRenderer(canvas, art, images, options) {
        const edition = options.edition;
        const W = 320, H = 240;
        const k = canvas.width / W;              // device pixels per logical pixel
        const a = art.scale;                     // art pixels per logical pixel
        let ctx = canvas.getContext('2d');
        const tints = new Map();
        const atlas = images.atlas;
        ctx.imageSmoothingEnabled = a > 1;

        // One tinted copy per font and colour, cut to that font's part of the atlas.
        const bounds = {};
        function fontBounds(role) {
            if (!bounds[role]) {
                let x0 = Infinity, y0 = Infinity, x1 = 0, y1 = 0;
                for (const glyph of Object.values(art.fonts[role].glyphs)) {
                    x0 = Math.min(x0, glyph[0]); y0 = Math.min(y0, glyph[1]);
                    x1 = Math.max(x1, glyph[0] + glyph[2]); y1 = Math.max(y1, glyph[1] + glyph[3]);
                }
                bounds[role] = { x: x0, y: y0, w: Math.max(1, x1 - x0), h: Math.max(1, y1 - y0) };
            }
            return bounds[role];
        }
        function tinted(role, rgb) {
            const keyName = role + ':' + rgb.join(',');
            let surface = tints.get(keyName);
            if (!surface) {
                const b = fontBounds(role);
                surface = (options.createCanvas || (() => root.document.createElement('canvas')))();
                surface.width = b.w; surface.height = b.h;
                const t = surface.getContext('2d');
                t.fillStyle = css(rgb);
                t.fillRect(0, 0, b.w, b.h);
                t.globalCompositeOperation = 'destination-in';
                t.drawImage(atlas, b.x, b.y, b.w, b.h, 0, 0, b.w, b.h);
                surface.ox = b.x; surface.oy = b.y;
                tints.set(keyName, surface);
            }
            return surface;
        }
        const font = role => art.fonts[role];
        const lineHeight = role => font(role).height / a * ((options.zoom && options.zoom[role]) || 1);
        function glyphsFor(role, text) {
            const f = font(role);
            const chars = Array.from(String(text));
            const out = [];
            for (const ch of chars) {
                let glyph = f.glyphs[ch.codePointAt(0)];
                if (!glyph && ch !== ' ') glyph = f.glyphs[ch.toUpperCase().codePointAt(0)];
                if (!glyph && ch !== ' ') return null;
                out.push({ ch, glyph });
            }
            return out;
        }
        // Advance follows the engine (font.c): width + spacing - kerning.
        function layout(role, text) {
            const f = font(role);
            const glyphs = glyphsFor(role, text);
            if (!glyphs) return null;
            const space = (f.glyphs[32] ? f.glyphs[32][2] : (f.glyphs[105] || [0, 0, 4 * a])[2]) + f.charSpace;
            let x = 0;
            const placed = [];
            glyphs.forEach((item, i) => {
                if (!item.glyph) { x += space; return; }
                placed.push({ x, glyph: item.glyph });
                x += item.glyph[2] + f.charSpace;
                const next = glyphs[i + 1];
                if (next && f.kernAmount) {
                    const here = f.kern[item.ch.codePointAt(0)], there = f.kern[next.ch.codePointAt(0)];
                    const mine = here === undefined ? 255 : here, theirs = there === undefined ? 255 : there;
                    if (here !== undefined && !(mine & (theirs >> 2))) x -= f.kernAmount;
                }
            });
            return { placed, width: Math.max(0, x - f.charSpace) / a * zoom(role) };
        }
        // Text size factor per font: HD's main-menu font is drawn smaller to fit two items.
        const zoom = role => (options.zoom && options.zoom[role]) || 1;
        function fallbackFont(role) {
            return Math.round(lineHeight(role) * 0.78 * k) + 'px "Trebuchet MS", "Segoe UI", system-ui, sans-serif';
        }
        function measure(role, text) {
            const placed = layout(role, text);
            if (placed) return placed.width;
            ctx.font = fallbackFont(role);
            return ctx.measureText(text).width / k;
        }
        function fit(role, text, maxWidth) {
            if (!maxWidth || measure(role, text) <= maxWidth) return text;
            let chars = Array.from(text);
            while (chars.length > 1 && measure(role, chars.join('').trimEnd() + '...') > maxWidth) chars.pop();
            return chars.join('').trimEnd() + '...';
        }
        // Draws text with its line box top at y. Returns the width.
        function text(role, value, x, y, rgb, align, maxWidth) {
            value = fit(role, String(value), maxWidth);
            const placed = layout(role, value);
            const width = placed ? placed.width : measure(role, value);
            let left = x;
            if (align === 'center') left = x - width / 2;
            else if (align === 'right') left = x - width;
            if (!placed) {
                ctx.save();
                ctx.font = fallbackFont(role);
                ctx.fillStyle = css(rgb);
                ctx.textBaseline = 'middle';
                // Bidi base direction from the first letter; 'left' keeps x the left edge.
                ctx.direction = isRtl(value) ? 'rtl' : 'ltr';
                ctx.textAlign = 'left';
                ctx.fillText(value, Math.round(left * k), Math.round((y + lineHeight(role) * 0.55) * k));
                ctx.restore();
                return width;
            }
            const source = tinted(role, rgb);
            const s = k / a * zoom(role);
            const baseX = Math.round(left * k), baseY = Math.round(y * k);
            for (const item of placed.placed) {
                const [gx, gy, gw, gh, dy] = item.glyph;
                ctx.drawImage(source, gx - source.ox, gy - source.oy, gw, gh, baseX + Math.round(item.x * s), baseY + Math.round(dy * s),
                    Math.round(gw * s), Math.round(gh * s));
            }
            return width;
        }
        function rect(x, y, w, h, rgb, alpha) {
            ctx.fillStyle = alpha === undefined ? css(rgb) : 'rgba(' + rgb.join(',') + ',' + alpha + ')';
            const x0 = Math.round(x * k), y0 = Math.round(y * k);
            ctx.fillRect(x0, y0, Math.round((x + w) * k) - x0, Math.round((y + h) * k) - y0);
        }
        // DrawStarConBox (util.c): top/left lines, then bottom/right lines, then the inside.
        // HD lines are half a logical pixel (two screen pixels) wide.
        function starconBox(x, y, w, h, border, tl, br, fill) {
            const p = a > 1 ? 0.5 : 1;
            for (let i = 0; i < border; i++) {
                const o = i * p;
                rect(x + o, y + o, w - 2 * o, p, tl);
                rect(x + o, y + o, p, h - 2 * o, tl);
                rect(x + w - o - p, y + o + p, p, h - 2 * o - p, br);
                rect(x + o, y + h - o - p, w - 2 * o, p, br);
            }
            if (fill) rect(x + border * p, y + border * p, w - 2 * border * p, h - 2 * border * p, fill);
        }
        function shipBox(x, y, w, h, hi) {
            starconBox(x, y, w, h, a > 1 ? 2 : 1, hi ? RGB.boxTLHi : RGB.boxTL, hi ? RGB.boxBRHi : RGB.boxBR, hi ? RGB.boxFillHi : RGB.boxFill);
        }
        // Grey button of the Super-Melee screen (NET..., SAVE, LOAD, QUIT).
        function button(x, y, w, h, label, hi, disabled) {
            starconBox(x, y, w, h, a > 1 ? 2 : 1, RGB.btnTL, RGB.btnBR, RGB.btnFill);
            const color = disabled ? RGB.btnTL : hi ? RGB.btnTextHi : RGB.btnText;
            text('label', label.toUpperCase(), x + w / 2, y + (h - lineHeight('label')) / 2 + 0.5, color, 'center', w - 4);
        }
        // Control box (HUMAN CONTROL / NETWORK CONTROL): grey frame, navy inside.
        function controlBox(x, y, w, h, hi) {
            starconBox(x, y, w, h, a > 1 ? 2 : 1, RGB.btnTL, RGB.btnBR, RGB.btnFill);
            starconBox(x + 2, y + 2, w - 4, h - 4, 1, RGB.btnTL, RGB.ctrlBevel, hi ? RGB.ctrlFillHi : RGB.ctrlFill);
        }
        function popup(x, y, w, h) {
            starconBox(x, y, w, h, 2, RGB.popupMedium, RGB.popupDark, RGB.popup);
        }
        function image(name, x, y, w, h) {
            const img = images[name];
            if (img) ctx.drawImage(img, 0, 0, img.width, img.height, Math.round(x * k), Math.round(y * k), Math.round(w * k), Math.round(h * k));
        }
        function sprite(name, x, y, alpha) {
            const place = art.sprites[name];
            if (!place) return { w: 0, h: 0 };
            const [sx, sy, sw, sh] = place;
            if (alpha !== undefined) ctx.globalAlpha = alpha;
            ctx.drawImage(atlas, sx, sy, sw, sh, Math.round(x * k), Math.round(y * k), Math.round(sw * k / a), Math.round(sh * k / a));
            ctx.globalAlpha = 1;
            return { w: sw / a, h: sh / a };
        }
        function title(y) {
            const place = art.sprites.title;
            sprite('title', Math.round((W - place[2] / a) / 2), y);
        }
        function gradientShade(y0, y1, alpha) {
            const g = ctx.createLinearGradient(0, y0 * k, 0, y1 * k);
            g.addColorStop(0, 'rgba(0,0,0,0)');
            g.addColorStop(1, 'rgba(0,0,0,' + alpha + ')');
            ctx.fillStyle = g;
            ctx.fillRect(0, Math.round(y0 * k), canvas.width, canvas.height - Math.round(y0 * k));
        }
        function wrap(role, value, width) {
            const words = [];
            // A word wider than the line (a link) is cut into pieces that fit.
            for (const word of String(value).split(/\s+/).filter(Boolean)) {
                let rest = Array.from(word);
                while (rest.length && measure(role, rest.join('')) > width) {
                    let n = rest.length - 1;
                    while (n > 1 && measure(role, rest.slice(0, n).join('')) > width) n--;
                    words.push(rest.slice(0, n).join(''));
                    rest = rest.slice(n);
                }
                if (rest.length) words.push(rest.join(''));
            }
            const lines = [];
            let line = '';
            for (const word of words) {
                const next = line ? line + ' ' + word : word;
                if (line && measure(role, next) > width) { lines.push(line); line = word; }
                else line = next;
            }
            if (line) lines.push(line);
            return lines;
        }
        function paragraph(role, value, x, y, width, rgb, align, gap) {
            const lines = wrap(role, value, width);
            const step = lineHeight(role) + (gap || 0);
            lines.forEach((line, i) => text(role, line, align === 'center' ? x + width / 2 : x, y + i * step, rgb, align));
            return lines.length * step;
        }

        // -------------------------------------------------- screens
        const regions = [];
        function region(id, x, y, w, h, label, extra) { regions.push(Object.assign({ id, x, y, w, h, label }, extra || {})); }
        const pulse = t => 0.5 + 0.5 * Math.sin(t / 260);

        const ONLINE_TEXT = ['ONLINE MELEE: fight another player over the internet.',
            'Pick an open room, or create your own and wait.', 'The battle starts by itself when you are both ready.'];
        // The start screen's picture and shading do not change: draw them once.
        let startBackground = null;
        function drawStartBackground() {
            image('title', 0, 0, W, H);
            // HD: the panel starts lower, clear of the second menu item (it ends near 172).
            const panel = edition === 'hd' ? 176 : 172;
            gradientShade(edition === 'hd' ? 118 : 100, panel, 0.85);
            rect(0, panel, W, H - panel, [0, 0, 0], 0.85);
        }
        function drawStart(state, t) {
            if (!startBackground && root.document && images.title) {
                startBackground = root.document.createElement('canvas');
                startBackground.width = canvas.width; startBackground.height = canvas.height;
                const main = ctx;
                ctx = startBackground.getContext('2d');
                ctx.imageSmoothingEnabled = a > 1;
                drawStartBackground();
                ctx = main;
            }
            if (startBackground) ctx.drawImage(startBackground, 0, 0);
            else drawStartBackground();
            const menu = MENU[edition];
            const items = [['play', 'Play'], ['online', 'Online Melee']];
            const mh = lineHeight('menu');
            const step = edition === 'hd' ? mh - 1 : mh + 4;
            // Under the title logo (HD: under "HD MEGAMOD").
            const top = edition === 'hd' ? 135 : 128;
            items.forEach(([id, label], i) => {
                const y = top + i * step;
                const hi = state.focus === id;
                const width = text('menu', label, W / 2, y, hi ? menu.hi : menu.text, 'center');
                if (hi) {
                    // The selected item pulses like the game's menu flash.
                    ctx.globalAlpha = pulse(t);
                    text('menu', label, W / 2, y, menu.glow, 'center');
                    ctx.globalAlpha = 1;
                }
                region(id, W / 2 - width / 2 - 6, y, width + 12, mh, label, { button: id === 'play' ? 'start' : undefined });
            });
            const infoY = top + items.length * step + (edition === 'hd' ? 6 : 4);
            const info = state.focus === 'online' ? ONLINE_TEXT : (options.startSteps || ['The campaign and local Super-Melee.']);
            const sh = lineHeight('small') + 1;
            info.forEach((line, i) => text('small', line, W / 2, infoY + i * sh, i === 0 ? RGB.teamNameHi : RGB.teamName, 'center', W - 16));
            // Loading progress in the game's own palette.
            const barY = H - 12;
            if (!state.ready) {
                const fraction = Math.max(0, Math.min(1, state.loading.fraction || 0));
                starconBox(60, barY, 200, 6, 1, RGB.btnTL, RGB.btnBR, [0, 0, 0]);
                if (fraction > 0) rect(61, barY + 1, 198 * fraction, 4, RGB.status);
                const label = state.pendingPlay ? 'Starting as soon as the download is done...' : state.loading.label;
                text('small', label, W / 2, barY - lineHeight('small') - 2, state.pendingPlay ? RGB.notice : RGB.hint, 'center', W - 20);
            } else {
                text('small', 'ARROWS AND ENTER TO CHOOSE · ? FOR ALL KEYS', W / 2, H - lineHeight('small') - 4, RGB.hint, 'center');
            }
        }
        function lobbyFrame(state) {
            image('stars', 0, 0, W, H);
            title(4);
        }
        // An info notice (why a session ended, "link copied") gives way to the screen's
        // own line after a while; errors and a link to copy stay.
        const NOTICE_MS = 10000;
        function statusLine(state, fallback, kind) {
            const n = state.notice && (state.notice.kind !== 'info' || options.now() - state.notice.at < NOTICE_MS) ? state.notice : null;
            const value = n ? n.text : fallback;
            const color = (n ? n.kind : kind) === 'error' ? RGB.error : n ? RGB.notice : RGB.status;
            // Long messages (a link to copy) use the small font so they keep clear of the keys line.
            if (value) paragraph(wrap('body', value, 304).length > 2 ? 'small' : 'body', value, 8, 199, 304, color, 'left');
        }
        function hint(value) { text('small', value, W / 2, H - lineHeight('small') - 3, RGB.hint, 'center', W - 8); }

        function drawLobby(state, t, now, under) {
            lobbyFrame(state);
            const listX = 8, listY = 26, listW = 240, rowH = 21, rows = 7;
            const lobby = state.lobby;
            const rooms = lobby.rooms;
            text('label', 'OPEN ROOMS', listX + 2, listY, RGB.ctrlText);
            const top = listY + lineHeight('label') + 3;
            // Keep the focused room in view.
            const focusIndex = rooms.findIndex(room => 'room:' + room.id === state.focus);
            let scroll = lobby.scroll || 0;
            if (focusIndex >= 0) {
                if (focusIndex < scroll) scroll = focusIndex;
                if (focusIndex >= scroll + rows) scroll = focusIndex - rows + 1;
            }
            scroll = Math.max(0, Math.min(scroll, Math.max(0, rooms.length - rows)));
            lobby.scroll = scroll;
            for (let i = 0; i < (rooms.length ? rows : 0); i++) {
                const y = top + i * (rowH + 2);
                const room = rooms[scroll + i];
                const hi = room && state.focus === 'room:' + room.id;
                if (!room) { starconBox(listX, y, listW, rowH, 1, RGB.boxTL, RGB.boxBR, null); continue; }
                shipBox(listX, y, listW, rowH, hi);
                // Under the name dialog only the boxes show.
                if (under) continue;
                const time = waitingTime(room.created, now);
                const timeW = measure('body', time);
                text('body', time, listX + listW - 5, y + (rowH - lineHeight('body')) / 2 + 1, hi ? RGB.teamNameHi : RGB.teamName, 'right');
                let tagX = listX + listW - 9 - timeW;
                if (room.engine) {
                    // The word ENGINE: a lone "C" tag does not say what it is.
                    const label = room.engine === 'rust' ? 'RUST ENGINE' : 'C ENGINE';
                    const tagW = measure('small', label) + 5;
                    tagX -= tagW;
                    starconBox(tagX, y + 6, tagW, rowH - 12, 1, RGB.boxBR, RGB.boxTL, [0, 0, 40]);
                    text('small', label, tagX + tagW / 2, y + 6 + (rowH - 12 - lineHeight('small')) / 2 + 0.5, room.engine === 'rust' ? RGB.tagRust : RGB.tag, 'center');
                }
                text('body', room.name, listX + 6, y + (rowH - lineHeight('body')) / 2 + 1, hi ? RGB.teamNameHi : RGB.teamName, 'left', tagX - listX - 12);
                region('room:' + room.id, listX, y, listW, rowH, 'Join ' + room.name + ', ' + (room.engine ? room.engine.toUpperCase() + ' engine, ' : '') + 'waiting ' + time);
            }
            if (rooms.length > rows && !under) {
                text('small', (scroll + 1) + '-' + (scroll + rows) + ' OF ' + rooms.length, listX + listW, listY + 1, RGB.hint, 'right');
            }
            const lobbyError = lobby.error || 'The room list is not available right now.';
            if (!rooms.length) {
                const panelH = rows * (rowH + 2) - 2;
                shipBox(listX, top, listW, panelH, false);
            }
            if (!rooms.length && !under) {
                let main, sub;
                if (lobby.status === 'connecting' || lobby.status === 'idle') { main = 'Looking for open rooms' + '...'.slice(0, 1 + Math.floor(t / 400) % 3); sub = ''; }
                else if (lobby.status === 'unsupported') { main = 'The room list is not available on this server.'; sub = 'Create a room and send its invite link.'; }
                // The room client may already say that it tries again.
                else if (lobby.status === 'error') { main = lobbyError; sub = /trying again|reconnect/i.test(main) ? '' : 'Trying again...'; }
                else { main = 'No open rooms right now.'; sub = 'Create a room, or wait here: new rooms show up by themselves.'; }
                const art = art_battle();
                let cy = top + 22;
                if (art) { battle(listX + listW / 2 - art.w / 2, cy, false, 0.35); cy += art.h + 8; }
                cy += paragraph('body', main, listX + 12, cy, listW - 24, RGB.teamNameHi, 'center') + 4;
                if (sub) paragraph('body', sub, listX + 12, cy, listW - 24, RGB.teamName, 'center');
            }
            // Right column, like the Super-Melee control boxes.
            const colX = 256, colW = 58;
            const createHi = state.focus === 'create' || (under && state.screen === 'name');
            controlBox(colX, top, colW, 44, createHi);
            const lh = lineHeight('label');
            // Focused: the yellow of the other focused buttons (the game's own lighter blue is hard to see).
            const createText = createHi ? RGB.btnTextHi : RGB.ctrlText;
            text('label', 'CREATE', colX + colW / 2, top + 22 - lh - 1, createText, 'center');
            text('label', 'ROOM', colX + colW / 2, top + 22 + 1, createText, 'center');
            region('create', colX, top, colW, 44, 'Create room');
            const backY = top + rows * (rowH + 2) - 15;
            button(colX + 4, backY, colW - 8, 13, 'Back', state.focus === 'back');
            region('back', colX + 4, backY, colW - 8, 13, options.started() ? 'Back to the game' : 'Back');
            const count = rooms.length;
            if (under) return;
            // With rooms on screen, a lost list is reported below it (the rooms may be out of date).
            if (lobby.status === 'error' && count) {
                statusLine(state, lobbyError + (/trying again|reconnect/i.test(lobbyError) ? '' : ' Trying again...'), 'error');
            } else {
                statusLine(state, lobby.status === 'open' && count
                    ? count + (count === 1 ? ' room is' : ' rooms are') + ' open. Pick one to fight.' : '');
            }
            hint(rooms.length ? 'ARROWS CHOOSE · ENTER JOIN · ESC BACK' : 'ENTER CREATE ROOM · ESC BACK');
        }
        function dim() { rect(0, 0, W, H, [0, 0, 0], 0.55); }
        function drawName(state, t) {
            drawLobby(state, t, options.now(), true);
            regions.length = 0;
            dim();
            // Over the room list, beside CREATE ROOM (the choice that opened it).
            const x = 8, w = 240, lh = lineHeight('label'), bh = lineHeight('body');
            const message = state.nameError || 'Players see this name in the room list.';
            const lines = wrap('body', message, w - 24);
            const h = 9 + lh + 5 + (bh + 6) + 4 + Math.max(2, lines.length) * bh + 6 + 13 + 7;
            const listTop = 26 + lh + 3, listH = 7 * 23 - 2;
            const y = Math.round(listTop + (listH - h) / 2);
            controlBox(x, y, w, h, false);
            text('label', 'NAME YOUR ROOM', x + w / 2, y + 9, RGB.ctrlTextHi, 'center');
            const fx = x + 12, fy = y + 9 + lh + 5, fw = w - 24, fh = bh + 6;
            const editing = state.focus === 'name';
            starconBox(fx, fy, fw, fh, 1, RGB.boxTL, RGB.boxBR, editing ? RGB.editRect : RGB.boxFill);
            const value = state.name;
            const caret = options.caret ? options.caret() : { start: Array.from(value).length, end: Array.from(value).length };
            const shown = fit('body', value, fw - 8);
            const tx = fx + 4, ty = fy + 3;
            text('body', shown, tx, ty, editing ? RGB.teamNameHi : RGB.teamName, 'left');
            if (editing && Math.floor(t / 450) % 2 === 0) {
                const before = Array.from(value).slice(0, caret.start).join('');
                const cx = tx + (before ? measure('body', before) + 0.5 : 0);
                rect(Math.min(cx, fx + fw - 3), ty, a > 1 ? 0.75 : 1, lineHeight('body') - 1, RGB.cursor);
            }
            region('name', fx, fy, fw, fh, 'Room name', { input: true });
            lines.forEach((line, i) => text('body', line, x + 12, fy + fh + 4 + i * bh, state.nameError ? RGB.error : RGB.teamName, 'left'));
            const by = y + h - 20, bw = 86;
            button(x + 24, by, bw, 13, 'Open room', state.focus === 'open');
            region('open', x + 24, by, bw, 13, 'Open room');
            button(x + w - 24 - bw, by, bw, 13, 'Cancel', state.focus === 'cancel');
            region('cancel', x + w - 24 - bw, by, bw, 13, 'Cancel');
            hint('ENTER OPEN ROOM · ESC CANCEL');
        }
        function centerBox(y, h) {
            const x = 30, w = 260;
            controlBox(x, y, w, h, false);
            return { x, y, w, h };
        }
        function battle(x, y, hi, alpha) {
            const size = sprite(hi ? 'battleHi' : 'battle', x, y, alpha);
            // HD draws the BATTLE! label over the picture; Original has it in the picture.
            if (edition === 'hd' && size.w) {
                if (alpha !== undefined) ctx.globalAlpha = alpha;
                text('label', 'BATTLE!', x + size.w / 2, y + size.h - lineHeight('label') - 4, hi ? [240, 80, 240] : [190, 40, 190], 'center');
                ctx.globalAlpha = 1;
            }
            return size;
        }
        // This page's own download, while a chosen room waits for it.
        function loadingLine(state, y) {
            const fraction = Math.max(0, Math.min(1, state.loading.fraction || 0));
            text('body', 'Loading the game: ' + Math.floor(fraction * 100) + '%', W / 2, y, RGB.status, 'center');
            y += lineHeight('body') + 2;
            starconBox(W / 2 - 80, y, 160, 6, 1, RGB.btnTL, RGB.btnBR, [0, 0, 0]);
            if (fraction > 0) rect(W / 2 - 79, y + 1, 158 * fraction, 4, RGB.status);
            return lineHeight('body') + 2 + 6 + 4;
        }
        function drawWaiting(state, t, now) {
            lobbyFrame(state);
            const room = state.room || { name: '', opened: now, listed: true, status: 'waiting' };
            const loading = room.status === 'loading';
            const box = centerBox(34, 158);
            const art = art_battle();
            let y = box.y + 9;
            text('label', loading ? 'ALMOST READY' : room.listed ? 'ROOM OPEN' : 'PRIVATE ROOM OPEN', W / 2, y, RGB.ctrlTextHi, 'center');
            y += lineHeight('label') + 4;
            if (room.name) text('body', '"' + room.name + '"', W / 2, y, RGB.teamNameHi, 'center', box.w - 20);
            y += lineHeight('body') + 6;
            if (art) {
                battle(W / 2 - art.w / 2, y, !loading && Math.floor(t / 700) % 2 === 1, loading ? 0.45 : 0.85);
                y += art.h + 6;
            }
            if (loading) {
                loadingLine(state, y);
            } else {
                const dots = '...'.slice(0, 1 + Math.floor(t / 400) % 3);
                text('body', 'Waiting for an opponent' + dots, W / 2 - measure('body', 'Waiting for an opponent...') / 2, y, RGB.status, 'left');
                y += lineHeight('body') + 2;
                text('small', 'OPEN FOR ' + clock(now - room.opened) + ' · CLOSES AFTER ' + (room.listed ? '10' : '2') + ' MIN', W / 2, y, RGB.hint, 'center');
            }
            const by = box.y + box.h - 19, bw = 96;
            button(W / 2 - bw - 6, by, bw, 13, 'Cancel', state.focus === 'cancel');
            region('cancel', W / 2 - bw - 6, by, bw, 13, 'Cancel and close the room');
            button(W / 2 + 6, by, bw, 13, 'Copy link', state.focus === 'copy');
            region('copy', W / 2 + 6, by, bw, 13, 'Copy invite link');
            statusLine(state, loading ? 'Your room opens when the game has loaded. Then other players see it.'
                : room.listed ? 'Your room is in the list. The match starts when someone joins.'
                    : 'Send the invite link to your opponent. The match starts when they join.');
            hint('ESC CLOSES THE ROOM');
        }
        function art_battle() {
            const place = art.sprites.battle;
            return place ? { w: place[2] / a, h: place[3] / a } : null;
        }
        function drawJoining(state, t) {
            lobbyFrame(state);
            const room = state.room || { name: '', status: 'waiting' };
            const loading = room.status === 'loading';
            const box = loading ? centerBox(60, 112) : centerBox(70, 90);
            let y = box.y + 12;
            text('label', 'JOINING', W / 2, y, RGB.ctrlTextHi, 'center');
            y += lineHeight('label') + 6;
            if (room.name) { text('body', '"' + room.name + '"', W / 2, y, RGB.teamNameHi, 'center', box.w - 20); y += lineHeight('body') + 6; }
            if (loading) {
                y += loadingLine(state, y);
                text('small', 'YOU JOIN WHEN THE GAME HAS LOADED', W / 2, y, RGB.hint, 'center', box.w - 12);
            } else {
                const dots = '...'.slice(0, 1 + Math.floor(t / 400) % 3);
                text('body', 'Connecting to the room' + dots, W / 2 - measure('body', 'Connecting to the room...') / 2, y, RGB.status, 'left');
            }
            const by = box.y + box.h - 19, bw = 96;
            button(W / 2 - bw / 2, by, bw, 13, 'Cancel', state.focus === 'cancel');
            region('cancel', W / 2 - bw / 2, by, bw, 13, 'Cancel');
            statusLine(state, '');
            hint('ESC CANCEL');
        }
        function drawFound(state, t, now) {
            lobbyFrame(state);
            const room = state.room || { name: '', role: 'host' };
            const box = centerBox(34, 158);
            let y = box.y + 9;
            text('label', 'OPPONENT FOUND!', W / 2, y, Math.floor(t / 250) % 2 ? RGB.notice : RGB.ctrlTextHi, 'center');
            y += lineHeight('label') + 4;
            if (room.name) { text('body', '"' + room.name + '"', W / 2, y, RGB.teamNameHi, 'center', box.w - 20); }
            y += lineHeight('body') + 6;
            const art = art_battle();
            if (art) {
                battle(W / 2 - art.w / 2, y, Math.floor(t / 180) % 2 === 0);
                y += art.h + 6;
            }
            const left = Math.ceil((state.countdownEnd - now) / 1000);
            const line = state.waitingForEngine
                ? 'Loading the game: ' + Math.round((state.loading.fraction || 0) * 100) + '%'
                : left > 0 ? 'The battle begins in ' + left + '...' : 'Here we go!';
            text('body', line, W / 2, y, RGB.status, 'center');
            y += lineHeight('body') + 2;
            text('small', room.role === 'host' ? 'YOU COMMAND THE BOTTOM FLEET' : 'YOU COMMAND THE TOP FLEET', W / 2, y, RGB.hint, 'center');
            const by = box.y + box.h - 19, bw = 96;
            button(W / 2 - bw / 2, by, bw, 13, 'Cancel', state.focus === 'cancel');
            region('cancel', W / 2 - bw / 2, by, bw, 13, 'Cancel');
            statusLine(state, 'Pick your fleet and press BATTLE! together.');
            hint('ESC CANCEL');
        }
        function drawInvite(state) {
            lobbyFrame(state);
            const box = centerBox(70, 96);
            let y = box.y + 12;
            text('label', 'YOU ARE INVITED', W / 2, y, RGB.ctrlTextHi, 'center');
            y += lineHeight('label') + 6;
            y += paragraph('body', 'Another player invited you to an online Super-Melee.', box.x + 16, y, box.w - 32, RGB.teamNameHi, 'center');
            const by = box.y + box.h - 19, bw = 96;
            button(W / 2 - bw - 6, by, bw, 13, 'Join', state.focus === 'join');
            region('join', W / 2 - bw - 6, by, bw, 13, 'Join the room');
            button(W / 2 + 6, by, bw, 13, 'Not now', state.focus === 'dismiss');
            region('dismiss', W / 2 + 6, by, bw, 13, 'Not now');
            statusLine(state, '');
            hint('ENTER JOIN · ESC NOT NOW');
        }
        function drawMessage(state, buttons) {
            const message = state.message || { title: '', text: '' };
            if (state.screen === 'message' && !options.started()) image('title', 0, 0, W, H); else lobbyFrame(state);
            dim();
            const x = 40, w = 240;
            const lines = wrap('body', message.text, w - 32);
            const detail = message.detail ? wrap('body', message.detail, w - 32) : [];
            const h = 24 + lineHeight('label') + (lines.length + detail.length) * lineHeight('body') + (detail.length ? 6 : 0) + (buttons.length ? 26 : 8);
            const y = Math.round((H - h) / 2);
            popup(x, y, w, h);
            let ty = y + 10;
            text('label', message.title, W / 2, ty, RGB.popupText, 'center');
            ty += lineHeight('label') + 6;
            lines.forEach(line => { text('body', line, W / 2, ty, RGB.popupText, 'center'); ty += lineHeight('body'); });
            if (detail.length) ty += 6;
            detail.forEach(line => { text('body', line, W / 2, ty, [70, 70, 70], 'center'); ty += lineHeight('body'); });
            const by = y + h - 19;
            const bw = 96;
            buttons.forEach(([id, label, aria], i) => {
                const bx = buttons.length === 1 ? W / 2 - bw / 2 : (i === 0 ? W / 2 - bw - 6 : W / 2 + 6);
                button(bx, by, bw, 13, label, state.focus === id);
                region(id, bx, by, bw, 13, aria || label);
            });
        }

        return {
            draw(state, t) {
                regions.length = 0;
                const now = options.now();
                ctx.setTransform(1, 0, 0, 1, 0, 0);
                ctx.globalAlpha = 1;
                ctx.globalCompositeOperation = 'source-over';
                ctx.fillStyle = '#000';
                ctx.fillRect(0, 0, canvas.width, canvas.height);
                switch (state.screen) {
                case 'start': drawStart(state, t); break;
                case 'lobby': drawLobby(state, t, now); break;
                case 'name': drawName(state, t); break;
                case 'waiting': drawWaiting(state, t, now); break;
                case 'joining': drawJoining(state, t); break;
                case 'found': drawFound(state, t, now); break;
                case 'invite': drawInvite(state); break;
                case 'restart': drawMessage(state, [['reload', 'Reload', 'Reload and open Online Melee'], ['back', 'Back to game']]); break;
                case 'message': drawMessage(state, state.items.includes('reload') ? [['reload', 'Reload', 'Reload the page']] : [['back', 'Back']]); break;
                default: break;
                }
                return regions.slice();
            },
            measure, text, button, lineHeight,
            // The main-menu corner button (54x36): a Super-Melee control box.
            corner(hi) {
                controlBox(0, 0, 54, 36, hi);
                const color = hi ? RGB.ctrlTextHi : RGB.ctrlText;
                const lh = lineHeight('label'), sh = lineHeight('small');
                const y = (36 - (2 * lh + 1 + sh)) / 2 + 0.5;
                text('label', 'ONLINE', 27, y, color, 'center');
                text('label', 'MELEE', 27, y + lh, color, 'center');
                text('small', 'PRESS O', 27, y + 2 * lh + 1, RGB.btnText, 'center');
            },
        };
    }

    // ------------------------------------------------------------ page

    const STYLE = [
        '.uqm-online { position: absolute; inset: 0; z-index: 1; overflow: hidden; background: #000; }',
        '.uqm-online[hidden] { display: none !important; }',
        '.uqm-online canvas { display: block; width: 100%; height: 100%; }',
        '.uqm-online.pixel canvas { image-rendering: pixelated; }',
        '.uqm-online-hit { position: absolute; margin: 0; padding: 0; border: 0; border-radius: 0; background: transparent; color: transparent;',
        '  font: 1px/1 sans-serif; overflow: hidden; cursor: pointer; outline: none; -webkit-tap-highlight-color: transparent; }',
        '.uqm-online-hit:focus-visible { outline: none; }',
        '.uqm-online input.uqm-online-hit { cursor: text; caret-color: transparent; font-size: 16px; padding: 0 6px; }',
        '.uqm-online input.uqm-online-hit::selection { background: transparent; }',
        '.uqm-online-live { position: absolute; width: 1px; height: 1px; overflow: hidden; clip-path: inset(50%); white-space: nowrap; }',
        '.uqm-online-corner { position: absolute; z-index: 1; top: 1.667%; right: 1.25%; width: 16.875%; aspect-ratio: 54 / 36; padding: 0; margin: 0;',
        '  border: 0; border-radius: 0; background: transparent; cursor: pointer; opacity: .92; }',
        '.uqm-online-corner:hover, .uqm-online-corner:focus-visible { opacity: 1; outline: none; }',
        '.uqm-online-corner canvas { display: block; width: 100%; height: 100%; }',
        // Both looks of the corner button are drawn once; the pointer or focus picks one.
        '.uqm-online-corner canvas.hi, .uqm-online-corner:hover canvas.normal, .uqm-online-corner:focus-visible canvas.normal { display: none; }',
        '.uqm-online-corner:hover canvas.hi, .uqm-online-corner:focus-visible canvas.hi { display: block; }',
        '.uqm-online-corner.pixel canvas { image-rendering: pixelated; }',
        '.uqm-online-corner[hidden] { display: none !important; }',
        // Full screen: the frame letterboxes the canvas; the overlay covers only the canvas.
        '.screen:fullscreen .uqm-online { inset: auto; left: 50%; top: 50%; transform: translate(-50%, -50%); width: min(100vw, 133.333vh); height: min(100vh, 75vw); }',
        '.screen:fullscreen .uqm-online-corner { top: calc(50% - min(50vh, 37.5vw) + min(1.667vh, 1.25vw)); right: calc(50% - min(50vw, 66.667vh) + min(1.25vw, 1.667vh)); width: min(16.875vw, 22.5vh); }',
    ].join('\n');

    // The game's menu sounds, through Web Audio once the player has pressed a key or clicked.
    function createSounds(win) {
        let context = null, allowed = false, sources = {};
        const buffers = {};
        const allow = () => { allowed = true; };
        win.addEventListener('keydown', allow, true);
        win.addEventListener('pointerdown', allow, true);
        // The sounds are 8-bit PCM WAV files at 8363 Hz; read them here, as some
        // browsers do not decode such files.
        function wav(data) {
            const view = new DataView(data);
            let pos = 12, format = null, samples = null;
            while (pos + 8 <= view.byteLength) {
                const id = String.fromCharCode(view.getUint8(pos), view.getUint8(pos + 1), view.getUint8(pos + 2), view.getUint8(pos + 3));
                const size = view.getUint32(pos + 4, true);
                if (id === 'fmt ') format = { channels: view.getUint16(pos + 10, true), rate: view.getUint32(pos + 12, true), bits: view.getUint16(pos + 22, true) };
                else if (id === 'data') samples = { start: pos + 8, size: Math.min(size, view.byteLength - pos - 8) };
                pos += 8 + size + (size & 1);
            }
            if (!format || !samples || ![8, 16].includes(format.bits) || !format.channels) return null;
            const step = format.bits / 8 * format.channels;
            const frames = Math.floor(samples.size / step);
            const buffer = context.createBuffer(format.channels, frames, format.rate);
            for (let c = 0; c < format.channels; c++) {
                const out = buffer.getChannelData(c);
                for (let i = 0; i < frames; i++) {
                    const at = samples.start + i * step + c * format.bits / 8;
                    out[i] = format.bits === 8 ? (view.getUint8(at) - 128) / 128 : view.getInt16(at, true) / 32768;
                }
            }
            return buffer;
        }
        function decode(name) {
            if (!buffers[name]) {
                buffers[name] = win.fetch(sources[name]).then(response => response.arrayBuffer()).then(wav).catch(() => null);
            }
            return buffers[name];
        }
        return {
            load(map) { sources = map || {}; },
            play(name) {
                const Context = win.AudioContext || win.webkitAudioContext;
                if (!sources[name] || !Context || (!context && !allowed)) return;
                try {
                    if (!context) { context = new Context(); Object.keys(sources).forEach(decode); }
                    if (context.state === 'suspended') context.resume().catch(() => {});
                    decode(name).then(buffer => {
                        if (!buffer) return;
                        const source = context.createBufferSource();
                        const gain = context.createGain();
                        gain.gain.value = 0.45;
                        source.buffer = buffer;
                        source.connect(gain);
                        gain.connect(context.destination);
                        source.start();
                    });
                } catch (_) { /* No sound. */ }
            },
        };
    }
    function loadImage(src) {
        return new Promise((resolve, reject) => {
            const image = new root.Image();
            image.onload = () => resolve(image);
            image.onerror = () => reject(new Error('Could not load ' + src));
            image.src = src;
        });
    }
    function loadArt(base) {
        const url = name => new URL(name, base).href;
        return root.fetch(url('online-art.json')).then(response => {
            if (!response.ok) throw new Error('Could not load the online art');
            return response.json();
        }).then(art => Promise.all(['atlas', 'stars', 'title'].map(name => loadImage(url(art.images[name])).catch(() => null)))
            .then(([atlas, stars, title]) => {
                if (!atlas) throw new Error('Could not load the online art');
                return { art, images: { atlas, stars, title } };
            }));
    }

    /* options: { document, window, screen (element), canvas (game canvas), edition, engine,
         netplay, game: { started(), ready(), screen(), hook(), start(role, client), queue(role, client),
           hold(on) (Module.holdMainMenu), watchScreen(callback) (each Module.gameScreen change) },
         startSteps: [lines], storage, onLog } */
    function init(options) {
        const document = options.document;
        const win = options.window || root;
        const screen = options.screen;
        const gameCanvas = options.canvas;
        const edition = options.edition === 'hd' ? 'hd' : 'original';
        const pixel = edition === 'original';
        const style = document.createElement('style');
        style.textContent = STYLE;
        document.head.appendChild(style);

        const overlay = document.createElement('div');
        overlay.className = 'uqm-online' + (pixel ? ' pixel' : '');
        overlay.id = 'online';
        overlay.setAttribute('role', 'region');
        overlay.setAttribute('aria-label', 'Game menu');
        const canvas = document.createElement('canvas');
        canvas.width = gameCanvas.width; canvas.height = gameCanvas.height;
        canvas.setAttribute('aria-hidden', 'true');
        overlay.appendChild(canvas);
        const layer = document.createElement('div');
        overlay.appendChild(layer);
        const live = document.createElement('div');
        live.className = 'uqm-online-live';
        live.id = 'status';
        live.setAttribute('role', 'status');
        live.setAttribute('aria-live', 'polite');
        overlay.appendChild(live);
        screen.appendChild(overlay);

        const corner = document.createElement('button');
        corner.type = 'button';
        corner.className = 'uqm-online-corner' + (pixel ? ' pixel' : '');
        corner.id = 'online-corner';
        corner.hidden = true;
        corner.setAttribute('aria-label', 'Online Melee (O)');
        corner.title = 'Online Melee (O)';
        const cornerCanvases = [false, true].map(hi => {
            const surface = document.createElement('canvas');
            surface.className = hi ? 'hi' : 'normal';
            corner.appendChild(surface);
            return surface;
        });
        screen.appendChild(corner);

        const storage = {
            get(key) { try { return options.storage().getItem(key); } catch (_) { return null; } },
            set(key, value) { try { options.storage().setItem(key, value); } catch (_) { /* Private mode. */ } },
        };
        let renderer = null, frame = 0, hits = new Map(), hitSignature = '', input = null, dirty = true, lastDraw = -1e9;
        const sounds = createSounds(win);
        const startedAt = win.performance ? win.performance.now() : Date.now();

        // Resolves false when the browser refused (no user activation) or cannot.
        function fullscreen() {
            try {
                if (document.fullscreenElement) return Promise.resolve(true);
                if (!screen.requestFullscreen) return Promise.resolve(false);
                return Promise.resolve(screen.requestFullscreen({ navigationUI: 'hide' })).then(() => {
                    const keyboard = win.navigator && win.navigator.keyboard;
                    if (keyboard && keyboard.lock) keyboard.lock(['Escape']).catch(() => {});
                    return true;
                }, () => false);
            } catch (_) { return Promise.resolve(false); }
        }
        const controller = createController({
            edition, engine: options.engine === 'rust' ? 'rust' : 'c',
            now: () => Date.now(), random: Math.random,
            setTimeout: (fn, ms) => win.setTimeout(fn, ms), clearTimeout: id => win.clearTimeout(id),
            storage, netplay: options.netplay, game: options.game,
            fullscreen,
            reload(online) {
                const url = new URL(win.location.href);
                url.hash = online ? 'online' : '';
                win.location.replace(url.href);
                if (online) win.location.reload();
            },
            copy(text) {
                // A hidden field and the copy command work in more places (also over plain HTTP).
                try {
                    const field = document.createElement('textarea');
                    field.value = text;
                    field.setAttribute('readonly', '');
                    field.style.cssText = 'position:fixed;left:-9999px;top:0;opacity:0';
                    (document.fullscreenElement || document.body).appendChild(field);
                    field.select();
                    const copied = document.execCommand && document.execCommand('copy');
                    field.remove();
                    if (copied) return Promise.resolve();
                } catch (_) { /* Try the clipboard API. */ }
                const clipboard = win.navigator && win.navigator.clipboard;
                return clipboard && clipboard.writeText ? clipboard.writeText(text) : Promise.reject(new Error('No clipboard'));
            },
            clearInvite() {
                try {
                    const url = new URL(win.location.href);
                    url.hash = '';
                    win.history.replaceState(null, '', url.href);
                } catch (_) { /* Keep the address. */ }
            },
            focusGame() { win.setTimeout(() => { if (overlay.hidden) gameCanvas.focus(); }, 0); },
            sound: name => sounds.play(name),
            changed: current => {
                dirty = true;
                if (current.screen === 'hidden') showOverlay(false);
                else if (overlay.hidden) {
                    // Shown at once, drawn at once: a key pressed right after O (or a
                    // session end) belongs to the menu, not to the game under it.
                    if (frame) { win.cancelAnimationFrame(frame); frame = 0; }
                    render(win.performance ? win.performance.now() : Date.now(), true);
                    return;
                }
                schedule();
            },
        });
        const state = controller.state;

        function schedule() {
            if (!frame) frame = win.requestAnimationFrame(render);
        }
        function animating() {
            return ['start', 'lobby', 'name', 'waiting', 'joining', 'found'].includes(state.screen);
        }
        function announce() {
            const titles = { start: 'The Ur-Quan Masters. Play or Online Melee.', lobby: 'Online Melee lobby.', name: 'Name your room.',
                waiting: 'Room open. Waiting for an opponent.', joining: 'Joining the room.', found: 'Opponent found. The battle begins soon.',
                invite: 'You are invited to an online Super-Melee.', restart: 'Online Melee needs a reload.' };
            let textValue = titles[state.screen] || '';
            // A room chosen during the download (no percentages: they would be read out all the time).
            if (state.room && state.room.status === 'loading') {
                if (state.screen === 'waiting') textValue = 'Your room opens when the game has loaded.';
                else if (state.screen === 'joining') textValue = 'You join the room when the game has loaded.';
            }
            if (state.screen === 'start' && !state.ready && state.loading.label) textValue += ' ' + state.loading.label;
            if (state.message && (state.screen === 'message' || state.screen === 'restart')) textValue = state.message.say || state.message.text;
            if (state.notice) textValue += ' ' + state.notice.text;
            if (state.screen === 'lobby' && !state.notice) {
                const n = state.lobby.rooms.length;
                textValue += state.lobby.status === 'open' ? ' ' + n + (n === 1 ? ' open room.' : ' open rooms.') : '';
            }
            if (live.textContent !== textValue) live.textContent = textValue;
        }
        function syncRegions(regions) {
            const signature = regions.map(r => [r.id, r.x, r.y, r.w, r.h, r.label].join(':')).join('|');
            if (signature !== hitSignature) {
                hitSignature = signature;
                const keep = new Set();
                for (const r of regions) {
                    keep.add(r.id);
                    let element = hits.get(r.id);
                    if (!element) {
                        element = document.createElement(r.input ? 'input' : 'button');
                        element.className = 'uqm-online-hit';
                        element.dataset.id = r.id;
                        if (r.input) {
                            element.type = 'text';
                            element.autocomplete = 'off';
                            element.spellcheck = false;
                            element.setAttribute('enterkeyhint', 'go');
                            // At most NAME_MAX characters (code points, as the relay counts them).
                            element.addEventListener('input', () => {
                                const chars = Array.from(element.value);
                                if (chars.length > NAME_MAX) {
                                    const caret = element.selectionStart;
                                    element.value = chars.slice(0, NAME_MAX).join('');
                                    if (caret != null) element.setSelectionRange(Math.min(caret, element.value.length), Math.min(caret, element.value.length));
                                }
                                controller.setName(element.value);
                            });
                            ['select', 'keyup', 'click'].forEach(type => element.addEventListener(type, schedule));
                            input = element;
                        } else {
                            element.type = 'button';
                            element.addEventListener('click', event => { event.preventDefault(); controller.activate(r.id); });
                        }
                        element.addEventListener('pointerenter', event => { if (event.pointerType !== 'touch') controller.focus(element.dataset.id); });
                        element.addEventListener('focus', () => controller.focus(element.dataset.id));
                        hits.set(r.id, element);
                        layer.appendChild(element);
                    }
                    element.style.left = (r.x / 3.2) + '%';
                    element.style.top = (r.y / 2.4) + '%';
                    element.style.width = (r.w / 3.2) + '%';
                    element.style.height = (r.h / 2.4) + '%';
                    if (r.input) element.setAttribute('aria-label', r.label);
                    else element.textContent = r.label;
                    if (r.button === 'start') element.id = 'start';
                    else if (element.id === 'start') element.removeAttribute('id');
                }
                for (const [id, element] of hits) {
                    if (!keep.has(id)) {
                        if (element === input) input = null;
                        element.remove();
                        hits.delete(id);
                    }
                }
                // Keep the order of the DOM the same as the visual order.
                for (const r of regions) layer.appendChild(hits.get(r.id));
            }
            if (input && input.value !== state.name) input.value = state.name;
            // The keyboard follows the drawn focus (unless the player is using page controls).
            const focused = hits.get(state.focus);
            const active = document.activeElement;
            const ours = !active || active === document.body || active === gameCanvas || active === corner || overlay.contains(active);
            if (focused && !overlay.hidden && active !== focused && ours) focused.focus({ preventScroll: true });
        }
        // While the menu covers the game's main menu, that menu must not time out
        // into the intro (Module.holdMainMenu, read by the engine's main menu).
        function holdMenu() {
            const hold = !overlay.hidden && options.game.started();
            if (options.game.hold) options.game.hold(hold);
        }
        // The overlay goes away at once when a match or the game starts.
        function showOverlay(visible) {
            if (overlay.hidden !== visible) return;
            overlay.hidden = !visible;
            holdMenu();
            if (visible) return;
            // Give the keyboard back to the game.
            if (document.activeElement && overlay.contains(document.activeElement)) gameCanvas.focus();
            // No page text over a running match.
            const keys = document.getElementById('keys-pop');
            if (keys && state.session) keys.hidden = true;
        }
        function render(time, force) {
            frame = 0;
            const visible = state.screen !== 'hidden';
            showOverlay(visible);
            corner.hidden = visible || state.session !== null || state.engineScreen !== 'mainMenu' || !options.game.started();
            announce();
            if (!visible) return;
            if (!renderer) {
                drawFallback();
                return;
            }
            // Changes are drawn within a frame or two; the small animations (pulse,
            // cursor, dots) at about 20 frames a second. The game may be loading meanwhile.
            const now = time || 0;
            if (force || (dirty && now - lastDraw >= 30) || now - lastDraw >= 50) {
                dirty = false;
                lastDraw = now;
                syncRegions(renderer.draw(state, now - startedAt));
            }
            if (animating() || dirty) schedule();
        }
        function drawFallback() {
            const ctx = canvas.getContext('2d');
            ctx.fillStyle = '#000';
            ctx.fillRect(0, 0, canvas.width, canvas.height);
        }
        // The corner button: a Super-Melee control box at the game's scale, drawn once
        // in each look.
        function drawCorner(data) {
            const k = canvas.width / 320;
            const w = Math.round(54 * k), h = Math.round(36 * k);
            // Both looks side by side with one renderer (one set of tinted fonts), then cut apart.
            const sheet = document.createElement('canvas');
            sheet.width = 2 * w; sheet.height = h;
            const ctx = sheet.getContext('2d');
            const painter = createRenderer({ width: canvas.width, height: canvas.height, getContext: () => ctx },
                data.art, data.images, { edition, now: () => Date.now(), started: options.game.started });
            painter.corner(false);
            ctx.translate(w, 0);
            painter.corner(true);
            cornerCanvases.forEach((target, hi) => {
                target.width = w; target.height = h;
                target.getContext('2d').drawImage(sheet, hi * w, 0, w, h, 0, 0, w, h);
            });
        }
        loadArt(options.artBase || win.location.href).then(data => {
            sounds.load(data.art.sounds);
            renderer = createRenderer(canvas, data.art, data.images, {
                edition, now: () => Date.now(), started: options.game.started,
                startSteps: options.startSteps, zoom: edition === 'hd' ? { menu: 0.62 } : null,
                caret: () => {
                    if (!input) return { start: Array.from(state.name).length };
                    const value = input.value;
                    return { start: Array.from(value.slice(0, input.selectionStart == null ? value.length : input.selectionStart)).length };
                },
            });
            drawCorner(data);
            dirty = true;
            schedule();
        }).catch(error => {
            if (options.onLog) options.onLog(String(error && error.message || error));
            useHtmlFallback();
        });
        // Without the art (offline page copy, blocked request) the menu still works as plain buttons.
        function useHtmlFallback() {
            renderer = {
                draw(s) {
                    const ctx = canvas.getContext('2d');
                    ctx.fillStyle = '#05070d'; ctx.fillRect(0, 0, canvas.width, canvas.height);
                    ctx.fillStyle = '#e1e6ee'; ctx.font = Math.round(canvas.width / 30) + 'px system-ui, sans-serif'; ctx.textAlign = 'center';
                    const labels = { play: 'Play', online: 'Online Melee', create: 'Create room', back: 'Back', open: 'Open room', cancel: 'Cancel',
                        copy: 'Copy invite link', join: 'Join', dismiss: 'Not now', reload: 'Reload', name: 'Room name' };
                    const regions = [];
                    const head = s.message ? s.message.text : s.notice ? s.notice.text : s.screen === 'start' ? s.loading.label : 'Online Melee';
                    ctx.fillText(head || '', canvas.width / 2, canvas.height * 0.2);
                    s.items.forEach((id, i) => {
                        const room = id.startsWith('room:') ? s.lobby.rooms.find(r => 'room:' + r.id === id) : null;
                        const label = room ? room.name : labels[id] || id;
                        const y = 60 + i * 18;
                        ctx.fillStyle = s.focus === id ? '#dfbd75' : '#e1e6ee';
                        ctx.fillText(id === 'name' ? s.name : label, canvas.width / 2, (y + 10) / 240 * canvas.height);
                        regions.push({ id, x: 80, y, w: 160, h: 16, label, input: id === 'name', button: id === 'play' ? 'start' : undefined });
                    });
                    return regions;
                },
            };
            schedule();
        }

        // Keyboard: the menu takes arrows, Enter and Esc while it is shown.
        document.addEventListener('keydown', event => {
            if (event.ctrlKey || event.altKey || event.metaKey) return;
            if (!overlay.hidden) {
                const target = event.target;
                // Page controls outside the frame keep their keys.
                const active = document.activeElement;
                if (active && active !== document.body && active !== gameCanvas && active !== corner && !overlay.contains(active)) return;
                const typing = target === input;
                // A held key repeats: one press makes one choice (a held Enter must not
                // open a room and then close it).
                if (event.repeat && (event.key === 'Enter' || (event.key === ' ' && !typing))) {
                    event.preventDefault();
                    event.stopPropagation();
                    return;
                }
                if (typing) {
                    if (event.key === 'Enter') { event.preventDefault(); controller.activate('open'); return; }
                    if (event.key === 'Escape') { event.preventDefault(); controller.key('Escape'); return; }
                    if (event.key === 'ArrowDown' || event.key === 'Tab' && !event.shiftKey) { event.preventDefault(); controller.focus('open'); return; }
                    return;
                }
                if (event.key === 'Tab') return;
                if (event.key === '?') return;
                if (controller.key(event.key)) {
                    event.preventDefault();
                    event.stopPropagation();
                }
                return;
            }
            // From the game's main menu, O opens the lobby (no menu uses O).
            if ((event.key === 'o' || event.key === 'O') && !event.shiftKey && state.engineScreen === 'mainMenu' &&
                    options.game.started() && !state.session && document.activeElement === gameCanvas) {
                event.preventDefault();
                event.stopPropagation();
                controller.fromMenu();
            }
        }, true);
        // A key that the game saw go down must also reach it going up. The menu can
        // open while a game key is held (the Enter that closed the engine's message
        // returns it to the main menu, and the lobby opens there); the release then
        // goes to the menu's focused item, and the game would see the key held
        // (and repeat it: Enter in the main menu starts a game).
        const gameKeys = new Map();
        gameCanvas.addEventListener('keydown', event => {
            gameKeys.set(event.code || event.key, { key: event.key, code: event.code, keyCode: event.keyCode, location: event.location });
        });
        document.addEventListener('keyup', event => {
            const id = event.code || event.key;
            const held = gameKeys.get(id);
            if (!held) return;
            gameKeys.delete(id);
            if (event.target === gameCanvas) return;
            const release = new win.KeyboardEvent('keyup', { key: held.key, code: held.code, location: held.location, bubbles: true, cancelable: true });
            Object.defineProperty(release, 'keyCode', { get: () => held.keyCode });
            Object.defineProperty(release, 'which', { get: () => held.keyCode });
            gameCanvas.dispatchEvent(release);
        }, true);
        overlay.addEventListener('wheel', event => {
            if (state.screen !== 'lobby') return;
            event.preventDefault();
            controller.move(event.deltaY > 0 ? 1 : -1);
        }, { passive: false });
        corner.addEventListener('mousedown', event => event.preventDefault());
        corner.addEventListener('click', () => controller.fromMenu());

        // Engine: follow Module.gameScreen. The shell reports every change as the engine
        // sets it (a short Super-Melee between two looks must not be missed); the
        // poll is for pages without that report.
        let screenName;
        const engineScreen = name => { if (name !== screenName) { screenName = name; controller.gameScreen(name); } };
        if (options.game.watchScreen) options.game.watchScreen(engineScreen);
        win.setInterval(() => engineScreen(options.game.screen()), 200);
        if (state.screen === 'start') schedule();

        const invite = readInvite(win.location.hash);
        if (invite && options.netplay) controller.invite(invite);
        else if (String(win.location.hash) === '#online') {
            try { win.history.replaceState(null, '', win.location.pathname + win.location.search); } catch (_) { /* Keep. */ }
            controller.openLobby();
        }
        win.addEventListener('beforeunload', () => controller.close());
        return { controller, state, overlay, corner };
    }

    return { createController, createRenderer, init, cleanName, defaultName, waitingTime, clock, cleanMessage, readInvite,
        NAME_KEY, NAME_MAX, COUNTDOWN_MS, RGB };
});
