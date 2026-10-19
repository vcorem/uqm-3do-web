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

    function readPreference(getStorage) {
        try { return { edition: normalize(getStorage().getItem(STORAGE_KEY)), available: true }; }
        catch (_) { return { edition: 'original', available: false }; }
    }
    function writePreference(getStorage, edition) {
        try { getStorage().setItem(STORAGE_KEY, normalize(edition)); return true; }
        catch (_) { return false; }
    }
    function hasInvite(href) {
        // Current invitations use a fragment, keeping the private token out of
        // HTTP requests. The game validates it after the user chooses Join.
        return new URLSearchParams(new URL(href).hash.slice(1)).has('room');
    }
    function destination(href, edition) {
        const url = new URL(href);
        if (url.protocol !== 'http:' && url.protocol !== 'https:') {
            throw new Error('Open this page through the included game server.');
        }
        const selected = hasInvite(href) ? 'original' : normalize(edition);
        // Never accept a URL from storage or query parameters as a destination.
        url.pathname = selected === 'hd' ? '/hd/index.html' : '/original/index.html';
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
        let leaving = false;

        function showStorage(available) {
            preferenceNote.textContent = available ? 'Your choice stays in this browser.'
                : 'This browser cannot remember the choice. You can still play either edition.';
        }
        function render() {
            radios.forEach(radio => { radio.checked = radio.value === selected; });
            label.textContent = selected === 'hd' ? 'Play Redrawn HD' : 'Play Original 3DO';
            note.textContent = selected === 'hd' ? 'Redrawn artwork at 1280 × 960. Local play with separate saves.'
                : 'Original artwork. Campaign, local and online Melee.';
        }
        function restoreControls() {
            leaving = false;
            launch.disabled = false;
            radios.forEach(radio => { radio.disabled = invite && radio.value === 'hd'; });
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
            try {
                if (!invite) showStorage(writePreference(getStorage, selected));
                location.assign(destination(location.href, selected));
            } catch (error) {
                restoreControls();
                note.textContent = error.message || 'The game could not open. Please try again.';
            }
        });
    }
    return { STORAGE_KEY, normalize, readPreference, writePreference, hasInvite, destination, init };
});
