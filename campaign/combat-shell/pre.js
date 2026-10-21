// Independent temporary settings. Original/HD IndexedDB saves are never mounted.
Module.preRun ||= [];
Module.preRun.push(function () {
    ENV.SDL_EMSCRIPTEN_KEYBOARD_ELEMENT = '#canvas';
    FS.mkdirTree('/tmp/afterlight-uqm');
});
// UQM's platform save callback is intentionally a no-op for temporary Melee settings.
if (typeof window !== 'undefined') window.wasm_syncfs = function () {};
