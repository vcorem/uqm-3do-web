Module.preRun ||= [];

Module.preRun.push(function () {
    const savePath = "/home/web_user/.uqm";
    const reportedWarnings = new Set();
    let mounted = false;
    let storageReady = false;
    let persistRequested = false;
    let syncing = false;
    let pendingWrite = false;

    // The page may present these messages without interrupting gameplay.
    function report(state, message, error, warningKey) {
        const status = { state, message };
        if (error) status.error = error.message || String(error);
        if (typeof Module.onStorageStatus === "function") {
            try {
                Module.onStorageStatus(status);
                return;
            } catch (callbackError) {
                console.warn("Storage status callback failed:", callbackError);
            }
        }
        // The page must receive every state transition, including repeated
        // failures after "saving". Only console fallback warnings are deduped.
        if (warningKey) {
            if (reportedWarnings.has(warningKey)) return;
            reportedWarnings.add(warningKey);
        }
        const log = state === "warning" || state === "memory"
            ? console.warn : console.log;
        log.call(console, message, status.error || "");
    }

    function requestPersistentStorage() {
        if (persistRequested) return;
        persistRequested = true;
        Promise.resolve().then(() => {
            if (typeof navigator === "undefined" || !navigator.storage ||
                    !navigator.storage.persist) {
                throw new Error("Persistent storage is unavailable in this browser.");
            }
            return navigator.storage.persist();
        }).then(isPersisted => {
            if (!isPersisted) {
                throw new Error("The browser did not grant persistent storage.");
            }
        }).catch(error => {
            report("warning",
                "Saves work, but the browser may remove them after inactivity.",
                error, "persistence");
        });
    }

    function flushWrites() {
        if (syncing || !pendingWrite || !storageReady) return;
        pendingWrite = false;
        syncing = true;
        report("saving", "Saving games and preferences to this browser.");
        function finish(error) {
            syncing = false;
            if (error) {
                report("warning",
                    "Saving to browser storage failed. Keep this tab open to retain the current session.",
                    error, "save-failed");
            } else if (!pendingWrite) {
                report("saved", "Games and preferences saved to this browser.");
            }
            // Changes made while a flush was running share one follow-up flush.
            flushWrites();
        }
        try {
            FS.syncfs(/* populate= */ false, finish);
        } catch (error) {
            finish(error);
        }
    }

    // Called from C code: uio_fclose(). Never overlap IndexedDB transactions.
    window.wasm_syncfs = () => {
        if (!storageReady) return;
        pendingWrite = true;
        requestPersistentStorage();
        flushWrites();
    };

    addRunDependency("syncfs");
    report("loading", "Loading saved games and preferences.");
    function finishLoading(error) {
        try {
            if (error) {
                // Do not sync this session back into a database we could not read.
                // The original IndexedDB saves remain untouched for a later visit.
                if (mounted) {
                    try {
                        FS.unmount(savePath);
                    } catch (unmountError) {
                        console.warn("Could not detach browser storage:", unmountError);
                    }
                }
                report("memory",
                    "Browser storage is unavailable. Games and preferences will last only while this tab stays open.",
                    error, "load-failed");
            } else {
                storageReady = true;
                report("ready", "Saved games and preferences loaded.");
            }
        } finally {
            // A blocked or corrupt IndexedDB must never prevent the game starting.
            removeRunDependency("syncfs");
        }
    }
    try {
        FS.mkdirTree(savePath);
        FS.mount(IDBFS, {}, savePath);
        mounted = true;
        FS.syncfs(/* populate= */ true, finishLoading);
    } catch (error) {
        finishLoading(error);
    }
});

// Run after preload dependencies finish and before callMain loads resource maps.
// Metadata alone is shipped upstream; it must not activate absent movies.
Module.detect3DOVideos = function () {
    const root = "/content/addons/3dovideo/";
    const required = [];
    const clips = ["intro/intro", "ending/victory"];
    for (let ship = 0; ship < 25; ship++) {
        const name = "spins/ship" + String(ship).padStart(2, "0");
        clips.push(name);
        required.push(name + ".aif");
    }
    required.push("spins/spin.aif");
    for (const clip of clips) {
        for (const extension of ["duk", "frm", "hdr", "tbl"]) {
            required.push(clip + "." + extension);
        }
    }
    const missing = required.filter(name => {
        try {
            const stat = FS.stat(root + name);
            return !FS.isFile(stat.mode) || stat.size === 0;
        } catch (_) {
            return true;
        }
    });
    return { complete: missing.length === 0, missing, total: required.length };
};
