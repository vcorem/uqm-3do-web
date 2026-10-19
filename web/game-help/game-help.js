/* Page help shared by both editions: download progress, the keys panel, the
   Keys button with the ? key, and a first-visit "How to start" card. */
(function (root, factory) {
    'use strict';
    const api = factory();
    if (typeof module === 'object' && module.exports) module.exports = api;
    else root.UQMHelp = api;
})(typeof globalThis !== 'undefined' ? globalThis : this, function () {
    'use strict';
    const STORAGE_KEY = 'uqm-help-start-closed';
    const k = text => '<kbd>' + text + '</kbd>';
    // Keys come from sc2/content/menu.key and uqm.key; the flight and pause
    // behaviour was checked in the Original edition.
    const KEYS = [
        ['Menus', [k('↑ ↓ ← →') + ' move · ' + k('Enter') + ' select', k('Esc') + ' or ' + k('Space') + ' back']],
        ['Flying', ['Hold ' + k('↑') + ' to fly · ' + k('← →') + ' turn', k('Esc') + ' ship menu (Starmap, Devices, Cargo…)', 'Choose <b>Navigate</b> to fly again']],
        ['Starmap', [k('Arrows') + ' move · ' + k('Enter') + ' set course', k('Page Up') + ' / ' + k('Page Down') + ' zoom']],
        ['Talking', [k('↑ ↓') + ' choose a reply · ' + k('Enter') + ' say it']],
        ['Combat · player 1', [k('↑') + ' thrust · ' + k('← →') + ' turn', k('Right Ctrl') + ' or ' + k('Enter') + ' fire', k('Right Shift') + ' special']],
        ['Combat · player 2 (WASD)', [k('W') + ' thrust · ' + k('A D') + ' turn', k('V') + ' fire · ' + k('B') + ' special', 'Select WASD in the in-game controls']],
        ['Game', [k('F1') + ' pause · ' + k('?') + ' show these keys', k('Shift + Tab') + ', then release Shift, for page controls']],
    ];
    const START_STEPS = [
        'Choose <b>New Game</b> with ' + k('↑ ↓') + ' and ' + k('Enter') + '. ' + k('Esc') + ' skips the intro.',
        'The ship menu opens. Choose <b>Navigate</b> and press ' + k('Enter') + ' to fly.',
        'Hold ' + k('↑') + ' to fly and ' + k('← →') + ' to turn. ' + k('Esc') + ' opens the ship menu again.',
        'First stop: the starbase at Earth, the third planet from the Sun.',
    ];
    const STYLE = [
        '.stage { display: grid; gap: 20px; align-items: start; }',
        '@media (min-width: 1100px) { .stage { grid-template-columns: minmax(0, 1fr) 250px; } }',
        '.keys-panel { border: 1px solid #273142; border-radius: 5px; padding: 14px 16px; font-size: 13px; color: #a4adbc; }',
        '.keys-panel h2 { font-size: 14px; color: #e1e6ee; margin: 0 0 10px; }',
        '.keys-panel h3, .keys-pop h3 { font-size: 12px; color: #dfbd75; margin: 12px 0 4px; font-weight: 650; }',
        '.keys-panel p, .keys-pop p { margin: 0; line-height: 1.55; }',
        '@media (max-width: 1099px) { .keys-panel { display: grid; grid-template-columns: repeat(auto-fill, minmax(220px, 1fr)); column-gap: 24px; } .keys-panel h2 { grid-column: 1 / -1; } }',
        '.keys-pop { position: absolute; top: 10px; right: 10px; z-index: 2; width: min(330px, calc(100% - 20px)); max-height: calc(100% - 20px); overflow: auto;',
        '  background: #080b12f2; border: 1px solid #42516a; border-radius: 5px; padding: 12px 16px 14px; font-size: 13px; color: #a4adbc; text-align: left; }',
        '.keys-pop header { display: flex; justify-content: space-between; align-items: center; margin: 0; }',
        '.keys-pop header strong { color: #e1e6ee; font-size: 14px; }',
        '.keys-pop button, .start-card button.link { background: none; color: #dfbd75; padding: 4px 6px; font-size: 13px; font-weight: 600; }',
        '.overlay { overflow: auto; }',
        '#keys-button { margin-left: auto; }',
        '.start-card { text-align: left; border: 1px solid #42516a; border-radius: 5px; padding: 12px 16px; margin: 0 auto 18px; width: min(520px, 100%); font-size: 13px; color: #a4adbc; }',
        '.start-card summary { display: flex; justify-content: space-between; align-items: center; gap: 12px; cursor: pointer; list-style: none; }',
        '.start-card summary::-webkit-details-marker { display: none; }',
        '.start-card summary::after { content: "Show"; color: #dfbd75; font-weight: 600; }',
        '.start-card[open] summary::after { content: ""; }',
        '.start-card strong { color: #e1e6ee; font-size: 14px; }',
        '.start-card footer { display: flex; justify-content: space-between; align-items: center; gap: 12px; }',
        '.start-card ol { margin: 8px 0; padding-left: 20px; line-height: 1.55; }',
        '.start-card p { margin: 0; line-height: 1.5; }',
        '@media (max-width: 640px) { .start-card { padding: 10px 12px; font-size: 12px; } }',
    ].join('\n');

    function formatProgress(text) {
        const match = /\((\d+)\/(\d+)\)\s*$/.exec(String(text || ''));
        if (!match) return null;
        const done = Number(match[1]), total = Number(match[2]);
        if (!(total > 0) || done > total) return null;
        const mb = bytes => Math.round(bytes / 1e6);
        return { done, total, label: 'Downloading game data: ' + mb(done) + ' / ' + mb(total) + ' MB (' + Math.floor(done * 100 / total) + '%)' };
    }

    function keysHtml(heading) {
        return heading + KEYS.map(([title, lines]) => '<div><h3>' + title + '</h3><p>' + lines.join('<br>') + '</p></div>').join('');
    }

    function readClosed(getStorage) {
        try { return getStorage().getItem(STORAGE_KEY) === '1'; } catch (_) { return false; }
    }

    function init({ document, getStorage, isStarted, eventTarget, isWide = () => true }) {
        const byId = id => document.getElementById(id);
        const style = document.createElement('style');
        style.textContent = STYLE;
        document.head.appendChild(style);

        const panel = byId('keys-panel');
        if (panel) panel.innerHTML = keysHtml('<h2>Keys</h2>');

        const screen = byId('screen');
        const pop = document.createElement('div');
        pop.className = 'keys-pop';
        pop.id = 'keys-pop';
        pop.hidden = true;
        pop.setAttribute('role', 'dialog');
        pop.setAttribute('aria-label', 'Keys');
        pop.innerHTML = keysHtml('<header><strong>Keys</strong><button type="button" id="keys-close">Close</button></header>');
        screen.appendChild(pop);

        const button = document.createElement('button');
        button.type = 'button';
        button.id = 'keys-button';
        button.textContent = 'Keys (?)';
        button.setAttribute('aria-controls', 'keys-pop');
        button.setAttribute('aria-expanded', 'false');
        const fullscreen = byId('fullscreen');
        fullscreen.parentNode.insertBefore(button, fullscreen);

        const canvas = byId('canvas');
        function toggle(show) {
            pop.hidden = !(show === undefined ? pop.hidden : show);
            button.setAttribute('aria-expanded', String(!pop.hidden));
            // Give the keyboard back to the game after the panel closes.
            if (pop.hidden && isStarted()) canvas.focus();
        }
        button.addEventListener('click', () => toggle());
        byId('keys-close').addEventListener('click', () => toggle(false));

        // Capture phase: SDL listens on the canvas, so it does not see this key.
        eventTarget.addEventListener('keydown', event => {
            const typing = /^(INPUT|TEXTAREA|SELECT)$/.test((event.target && event.target.tagName) || '');
            if (event.key === '?' && !typing && !event.ctrlKey && !event.altKey && !event.metaKey) {
                event.preventDefault();
                event.stopPropagation();
                toggle();
            } else if (event.key === 'F1' && isStarted()) {
                event.preventDefault(); // The game pauses; the browser must not open its help page.
            }
        }, true);

        if (!readClosed(getStorage)) {
            // A small game area keeps the card folded, so the Launch button stays visible.
            const card = document.createElement('details');
            card.className = 'start-card';
            card.id = 'start-card';
            card.open = isWide();
            card.innerHTML = '<summary><strong>How to start</strong></summary>' +
                '<ol>' + START_STEPS.map(step => '<li>' + step + '</li>').join('') + '</ol>' +
                '<footer><p>Press ' + k('?') + ' at any time to see all keys.</p><button type="button" class="link" id="start-card-close">Got it</button></footer>';
            const start = byId('start');
            start.parentNode.insertBefore(card, start);
            byId('start-card-close').addEventListener('click', () => {
                card.hidden = true;
                try { getStorage().setItem(STORAGE_KEY, '1'); } catch (_) { /* Private mode: close for this visit only. */ }
            });
        }
        return { toggle };
    }

    return { STORAGE_KEY, KEYS, START_STEPS, formatProgress, init };
});
