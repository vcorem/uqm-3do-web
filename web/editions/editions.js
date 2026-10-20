/* This page selects a document. It never imports or initializes a game engine. */
(function (root, factory) {
    'use strict';
    const api = factory();
    if (typeof module === 'object' && module.exports) module.exports = api;
    else {
        root.UQMEditions = api;
        api.init({ document: root.document, location: root.location,
            getStorage: () => root.localStorage, eventTarget: root });
    }
})(typeof globalThis !== 'undefined' ? globalThis : this, function () {
    'use strict';
    const STORAGE_KEY = 'uqm.web.edition.v1';
    const validEdition = value => value === 'original' || value === 'hd';
    const normalize = value => validEdition(value) ? value : 'original';
    // The C engines are the reference; the Rust port is published next to them for testing.
    const ENGINE_KEY = 'uqm.web.engine.v1';
    const normalizeEngine = value => value === 'rust' ? 'rust' : 'c';

    function readPreference(getStorage) {
        try { return { edition: normalize(getStorage().getItem(STORAGE_KEY)), available: true }; }
        catch (_) { return { edition: 'original', available: false }; }
    }
    function writePreference(getStorage, edition) {
        try { getStorage().setItem(STORAGE_KEY, normalize(edition)); return true; }
        catch (_) { return false; }
    }
    function readEngine(getStorage) {
        try { return normalizeEngine(getStorage().getItem(ENGINE_KEY)); }
        catch (_) { return 'c'; }
    }
    function writeEngine(getStorage, engine) {
        try { getStorage().setItem(ENGINE_KEY, normalizeEngine(engine)); return true; }
        catch (_) { return false; }
    }
    function hasInvite(href) {
        // Current invitations use a fragment, keeping the private token out of
        // HTTP requests. The game validates it after the user chooses Join.
        return new URLSearchParams(new URL(href).hash.slice(1)).has('room');
    }
    function destination(href, edition, engine) {
        const url = new URL(href);
        if (url.protocol !== 'http:' && url.protocol !== 'https:') {
            throw new Error('Open this page through the included game server.');
        }
        const selected = hasInvite(href) ? 'original' : normalize(edition);
        // Never accept a URL from storage or query parameters as a destination.
        const folder = (selected === 'hd' ? 'hd' : 'original') + (normalizeEngine(engine) === 'rust' ? '-rust' : '');
        url.pathname = '/' + folder + '/index.html';
        return url.href;
    }
    function init({ document, location, getStorage, eventTarget }) {
        const form = document.getElementById('edition-form');
        const radios = ['original', 'hd'].map(value => document.getElementById('edition-' + value));
        const launch = document.getElementById('launch-edition');
        const label = document.getElementById('launch-label');
        const note = document.getElementById('selection-note');
        const preferenceNote = document.getElementById('preference-note');
        const inviteNote = document.getElementById('invite-note');
        const invite = hasInvite(location.href);
        const preference = readPreference(getStorage);
        let selected = invite ? 'original' : preference.edition;
        // The packager marks a package without Rust builds; then only C is offered.
        const engineRadios = ['c', 'rust'].map(value => document.getElementById('engine-' + value)).filter(Boolean);
        const engines = String((form.getAttribute && form.getAttribute('data-engines')) || 'c,rust').split(',');
        const rustAvailable = engineRadios.length === 2 && engines.includes('rust');
        const engineChoice = document.getElementById('engine-choice');
        if (engineChoice) engineChoice.hidden = !rustAvailable;
        let engine = rustAvailable ? readEngine(getStorage) : 'c';
        let leaving = false;

        function showStorage(available) {
            preferenceNote.textContent = available ? 'Your choice stays in this browser.'
                : 'This browser cannot remember the choice. You can still play either edition.';
        }
        function render() {
            radios.forEach(radio => { radio.checked = radio.value === selected; });
            engineRadios.forEach(radio => { radio.checked = radio.value === engine; });
            const rust = engine === 'rust';
            label.textContent = (selected === 'hd' ? 'Play Redrawn HD' : 'Play Original 3DO') + (rust ? ' · Rust' : '');
            note.textContent = (selected === 'hd' ? 'Redrawn artwork at 1280 × 960. Online Melee with HD players and separate saves.'
                : 'Original artwork. Campaign, local and online Melee.') +
                (rust ? ' Rust engine: a test version with its own saves.' : '');
        }
        function restoreControls() {
            leaving = false;
            launch.disabled = false;
            radios.forEach(radio => { radio.disabled = invite && radio.value === 'hd'; });
            engineRadios.forEach(radio => { radio.disabled = false; });
        }
        // Back/Forward can restore this document with its pre-navigation
        // disabled controls and closure state intact. Do not leave it locked.
        eventTarget?.addEventListener('pageshow', event => {
            if (!event.persisted) return;
            restoreControls();
            render();
        });
        showStorage(preference.available);
        inviteNote.hidden = !invite;
        if (invite) radios[1].disabled = true;
        render();
        engineRadios.forEach(radio => radio.addEventListener('change', function () {
            if (leaving || !rustAvailable) return;
            engine = normalizeEngine(this.value);
            showStorage(writeEngine(getStorage, engine));
            render();
        }));
        radios.forEach(radio => radio.addEventListener('change', function () {
            if (leaving) return;
            selected = invite ? 'original' : normalize(this.value);
            if (!invite) showStorage(writePreference(getStorage, selected));
            render();
        }));
        form.addEventListener('submit', event => {
            event.preventDefault();
            if (leaving) return;
            leaving = true;
            launch.disabled = true;
            radios.forEach(radio => { radio.disabled = true; });
            engineRadios.forEach(radio => { radio.disabled = true; });
            try {
                if (!invite) showStorage(writePreference(getStorage, selected));
                location.assign(destination(location.href, selected, engine));
            } catch (error) {
                restoreControls();
                note.textContent = error.message || 'The game could not open. Please try again.';
            }
        });
    }
    return { STORAGE_KEY, ENGINE_KEY, normalize, normalizeEngine, readPreference, writePreference, readEngine, writeEngine, hasInvite, destination, init };
});
