// SPDX-License-Identifier: GPL-2.0-or-later
// The child accepts encounter configuration, not outcomes. Only Rust can report a result.
var canvas = document.getElementById('canvas');
var button = document.getElementById('start');
var statusText = document.getElementById('status');
var overlay = document.getElementById('overlay');
var retreatButton = document.getElementById('retreat');
var runtimeReady = false, launched = false, failed = false, current = null, queued = null;
var startedReported = false, logLines = [];
function emit(type, fields) {
    if (window.parent !== window) window.parent.postMessage({ type: 'uqm-campaign:' + type, ...fields }, location.origin);
}
function log(value) {
    logLines.push(String(value));
    if (logLines.length > 80) logLines.shift();
    console.log(value);
    if (/uncaught exception|Pthread .*sent an error|Aborted\(|RuntimeError/.test(String(value))) fail('Combat stopped: ' + value);
}
function fail(reason) {
    if (failed) return;
    failed = true;
    statusText.textContent = String(reason);
    overlay.hidden = false;
    button.disabled = true;
    button.textContent = 'Reload combat to retry';
    document.getElementById('progress').hidden = true;
    retreatButton.disabled = true;
    emit('error', { id: current?.id || queued?.id || null, message: String(reason) });
}
var Module = {
    canvas,
    noInitialRun: true,
    print: log, printErr: log,
    arguments: ['--configdir=/tmp/afterlight-uqm', '--menu=3do', '--font=3do', '--meleezoom=3do', '--intro=pc', '--subtitles', '--musicvol=0'],
    setStatus: function (text) { if (text && !launched) statusText.textContent = text; },
    monitorRunDependencies: function (remaining) {
        this.totalDependencies = Math.max(this.totalDependencies || 0, remaining);
        var progress = document.getElementById('progress');
        progress.max = this.totalDependencies || 1;
        progress.value = this.totalDependencies - remaining;
    },
    onRuntimeInitialized: function () {
        if (!window.crossOriginIsolated) return fail('Combat needs cross-origin isolation. Start the included game server.');
        if (Module._uqm_campaign_enable() !== 1) return fail('The campaign combat adapter is unavailable.');
        runtimeReady = true;
        document.getElementById('progress').hidden = true;
        statusText.textContent = 'Combat ready. Click Enter combat to activate sound and take the helm.';
        button.textContent = 'Enter combat';
        button.disabled = !queued;
        emit('ready', { version: 1 });
    },
    onAbort: function (reason) { fail('Combat stopped: ' + reason); },
    onExit: function (code) { fail('Combat engine exited (' + code + '). Reload combat to retry.'); }
};
function validRequest(request) {
    return request && typeof request.id === 'string' && request.id.length > 0 && request.id.length <= 128 &&
        request.player && Number.isInteger(request.player.ship) && request.player.ship >= 0 && request.player.ship < 25 &&
        Number.isInteger(request.player.crew) && request.player.crew > 0 && request.player.crew <= 42 &&
        Number.isInteger(request.seed) && request.seed >= 0 && request.seed <= 0xffffffff &&
        ['standard', 'good', 'awesome'].includes(request.difficulty || 'standard') &&
        Array.isArray(request.enemies) && request.enemies.length >= 1 && request.enemies.length <= 14 &&
        request.enemies.every(ship => Number.isInteger(ship) && ship >= 0 && ship < 25);
}
function begin(request) {
    if (!runtimeReady || failed) return;
    var enemies = request.enemies.concat(Array(14 - request.enemies.length).fill(255));
    var rating = ['standard', 'good', 'awesome'].indexOf(request.difficulty || 'standard');
    if (Module._uqm_campaign_begin(request.player.ship, request.player.crew, request.seed, rating, ...enemies) !== 1) {
        emit('error', { id: request.id, message: 'The combat engine rejected this encounter configuration.' });
        queued = null;
        return;
    }
    current = request;
    queued = null;
    startedReported = false;
    overlay.hidden = true;
    retreatButton.disabled = false;
    canvas.focus();
    if (!launched) {
        launched = true;
        try { Module.callMain(Module.arguments); } catch (error) { fail(error.message || error); }
    }
    setTimeout(function () {
        if (current?.id === request.id && !startedReported && !failed) fail('Combat did not start. Reload combat to retry.');
    }, 60000);
}
button.addEventListener('click', function () { if (queued) begin(queued); });
function retreat() {
    if (current && !failed) {
        Module._uqm_campaign_retreat();
        retreatButton.disabled = true;
        retreatButton.textContent = 'Withdrawing…';
        canvas.focus();
    }
}
retreatButton.addEventListener('click', retreat);
canvas.addEventListener('click', function () { canvas.focus(); });
window.addEventListener('error', function (event) { if (event.message) fail(event.message); });
window.addEventListener('message', function (event) {
    if (event.origin !== location.origin || event.source !== window.parent || !event.data || failed) return;
    if (event.data.type === 'uqm-campaign:start') {
        var request = event.data.request;
        if (!validRequest(request)) return emit('error', { id: request?.id || null, message: 'Invalid encounter request.' });
        if (current || queued) return emit('error', { id: request.id, message: 'A combat encounter is already active.' });
        queued = structuredClone(request);
        retreatButton.textContent = 'Retreat';
        if (launched) begin(queued);
        else button.disabled = !runtimeReady;
    } else if (event.data.type === 'uqm-campaign:retreat' && event.data.id === current?.id) retreat();
    else if (event.data.type === 'uqm-campaign:retreat' && event.data.id === queued?.id) {
        var id=queued.id; queued=null; button.disabled=true;
        emit('cancelled', {id});
    }
    else if (event.data.type === 'uqm-campaign:ping' && runtimeReady) emit('ready', { version: 1 });
});
setInterval(function () {
    if (!current || failed || !runtimeReady) return;
    var phase = Module._uqm_campaign_phase();
    if (phase === 3 && !startedReported) {
        startedReported = true;
        emit('started', { id: current.id });
    }
    if (phase === 4) {
        var outcome = ['invalid', 'victory', 'defeat', 'retreat', 'error'][Module._uqm_campaign_outcome()];
        if (!['victory', 'defeat', 'retreat'].includes(outcome)) return fail('Combat ended without a valid native result. Progress was not changed.');
        var result = { id: current.id, outcome, crew: Module._uqm_campaign_crew(),
            enemyRemaining: Module._uqm_campaign_remaining(), frames: Module._uqm_campaign_frames() };
        current = null;
        retreatButton.disabled = true;
        emit('result', result);
    }
}, 100);
// Read-only diagnostics; no victory/health mutation API exists.
Object.defineProperty(window, '__afterlightCombat', { get: function () {
    return { runtimeReady, launched, failed, requestId: current?.id || queued?.id || null,
        phase: runtimeReady ? Module._uqm_campaign_phase() : 0,
        crew: runtimeReady ? Module._uqm_campaign_crew() : null,
        frames: runtimeReady ? Module._uqm_campaign_frames() : 0, log: logLines.slice() };
} });
