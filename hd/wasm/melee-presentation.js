/* Presentation preferences only. The native engine owns every battle, sound,
   collision and network packet; this file never advances the simulation. */
(function (root, factory) {
    'use strict';
    const api = factory();
    if (typeof module === 'object' && module.exports) module.exports = api;
    else root.UQMMeleePresentation = api;
})(typeof globalThis !== 'undefined' ? globalThis : this, function () {
    'use strict';
    const STORAGE_KEY = 'uqm.hd.melee.presentation.v1';
    const DEFAULTS = Object.freeze({ style: 'remastered', soundtrack: 'remastered', musicVolume: 72 });
    function normalize(value) {
        value = value && typeof value === 'object' ? value : {};
        const volume = Number(value.musicVolume);
        return {
            style: value.style === 'classic' ? 'classic' : DEFAULTS.style,
            soundtrack: value.soundtrack === 'classic' ? 'classic' : DEFAULTS.soundtrack,
            musicVolume: value.musicVolume !== undefined && value.musicVolume !== null && Number.isFinite(volume)
                ? Math.round(Math.max(0, Math.min(100, volume))) : DEFAULTS.musicVolume,
        };
    }
    function read(getStorage) {
        try { return normalize(JSON.parse(getStorage().getItem(STORAGE_KEY))); }
        catch (_) { return normalize(null); }
    }
    function write(getStorage, settings) {
        try { getStorage().setItem(STORAGE_KEY, JSON.stringify(normalize(settings))); return true; }
        catch (_) { return false; }
    }
    function launchArguments(args, settings) {
        const selected = normalize(settings);
        const next = args.filter(arg => arg !== '--addon=remastered' && !arg.startsWith('--musicvol='));
        if (selected.soundtrack === 'remastered') next.push('--addon=remastered');
        next.push('--musicvol=' + selected.musicVolume);
        return next;
    }
    function init({ document, module, getStorage, isStarted, canvas }) {
        let settings = read(getStorage);
        let ready = false;
        let launched = null;
        const elements = Object.fromEntries(['presentation-panel', 'presentation-toggle', 'presentation-close',
            'melee-style', 'melee-soundtrack', 'melee-music-volume', 'melee-volume-value', 'presentation-status']
            .map(id => [id, document.getElementById(id)]));
        const panel = elements['presentation-panel'];
        const status = elements['presentation-status'];
        function audioPending() {
            return !!launched && (settings.soundtrack !== launched.soundtrack || settings.musicVolume !== launched.musicVolume);
        }
        function render() {
            elements['melee-style'].value = settings.style;
            elements['melee-soundtrack'].value = settings.soundtrack;
            elements['melee-music-volume'].value = settings.musicVolume;
            elements['melee-volume-value'].textContent = settings.musicVolume + '%';
            const screen = document.getElementById('screen');
            if (screen) screen.setAttribute('data-presentation', settings.style);
            status.textContent = audioPending()
                ? 'Sound preferences saved for your next launch. During play, adjust volume in Setup → Sound.'
                : 'Ship artwork switches instantly. Sound preferences apply when the game starts.';
        }
        function applyStyle() {
            if (!ready) return;
            if (typeof module._UQM_SetMeleeRemastered === 'function') {
                module._UQM_SetMeleeRemastered(settings.style === 'remastered' ? 1 : 0);
            } else {
                status.textContent = 'This engine does not include the new Melee artwork. Rebuild the HD edition to use it.';
            }
        }
        function update(key, value) {
            settings = normalize({ ...settings, [key]: value });
            const saved = write(getStorage, settings);
            render();
            if (key === 'style') applyStyle();
            if (!saved) status.textContent += ' Preferences last for this tab only.';
        }
        function show(open) {
            panel.hidden = !open;
            elements['presentation-toggle'].setAttribute('aria-expanded', String(open));
            if (open) elements['melee-style'].focus();
            else if (isStarted()) canvas.focus();
            else elements['presentation-toggle'].focus();
        }
        elements['presentation-toggle'].addEventListener('click', () => show(panel.hidden));
        elements['presentation-close'].addEventListener('click', () => show(false));
        panel.addEventListener('keydown', event => {
            // A dialog keystroke must not also fire a weapon or advance a menu.
            event.stopPropagation();
            if (event.key === 'Escape') { event.preventDefault(); show(false); }
        });
        panel.addEventListener('keyup', event => event.stopPropagation());
        elements['melee-style'].addEventListener('change', event => update('style', event.target.value));
        elements['melee-soundtrack'].addEventListener('change', event => update('soundtrack', event.target.value));
        elements['melee-music-volume'].addEventListener('input', event => update('musicVolume', event.target.value));
        render();
        return {
            get settings() { return { ...settings }; },
            runtimeReady() { ready = true; applyStyle(); },
            prepareLaunch(args) {
                launched = { ...settings };
                render();
                applyStyle();
                return launchArguments(args, settings);
            },
        };
    }
    return { STORAGE_KEY, DEFAULTS, normalize, read, write, launchArguments, init };
});
