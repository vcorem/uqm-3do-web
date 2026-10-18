const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");
const vm = require("node:vm");

const source = fs.readFileSync(path.join(__dirname, "..", "pre.js"), "utf8");
const savePath = "/home/web_user/.uqm";

function harness({ initialThrow, onStatus } = {}) {
    const requests = [];
    const statuses = [];
    const mounts = [];
    const unmounts = [];
    const warnings = [];
    let dependencies = 0;
    let active = 0;
    let peakActive = 0;
    let persistRequests = 0;
    const context = {
        Module: { onStorageStatus: onStatus || (status => statuses.push(status)) },
        window: {},
        navigator: { storage: { persist: () => {
            persistRequests++;
            return Promise.resolve(true);
        } } },
        console: { log() {}, warn: (...args) => warnings.push(args) },
        IDBFS: {},
        addRunDependency(name) {
            assert.equal(name, "syncfs");
            dependencies++;
        },
        removeRunDependency(name) {
            assert.equal(name, "syncfs");
            dependencies--;
        },
        FS: {
            mkdirTree(directory) { assert.equal(directory, savePath); },
            mount(type, options, directory) { mounts.push(directory); },
            unmount(directory) { unmounts.push(directory); },
            syncfs(populate, callback) {
                if (initialThrow && populate) throw initialThrow;
                active++;
                peakActive = Math.max(peakActive, active);
                requests.push({ populate, callback });
            },
        },
    };
    vm.runInNewContext(source, context, { filename: "pre.js" });
    context.Module.preRun[0]();
    return {
        context, requests, statuses, mounts, unmounts, warnings,
        get dependencies() { return dependencies; },
        get peakActive() { return peakActive; },
        get persistRequests() { return persistRequests; },
        finish(error = null) {
            const request = requests.shift();
            assert.ok(request, "a storage request must be outstanding");
            active--;
            request.callback(error);
        },
        write() { context.window.wasm_syncfs(); },
    };
}

test("loads existing saves before starting and keeps their storage mount", () => {
    const h = harness();
    assert.equal(h.dependencies, 1);
    assert.deepEqual(h.mounts, [savePath]);
    assert.equal(h.requests[0].populate, true);
    h.finish();
    assert.equal(h.dependencies, 0);
    assert.equal(h.statuses.at(-1).state, "ready");
    assert.equal(h.requests.length, 0, "loading must not write over existing saves");
    assert.equal(h.unmounts.length, 0);
});

test("failed initial load starts in memory without writing to existing saves", () => {
    const h = harness();
    h.finish(new Error("IndexedDB denied"));
    assert.equal(h.dependencies, 0);
    assert.deepEqual(h.unmounts, [savePath]);
    assert.equal(h.statuses.at(-1).state, "memory");
    for (let i = 0; i < 20; i++) h.write();
    assert.equal(h.requests.length, 0);
    assert.equal(h.statuses.filter(s => s.state === "memory").length, 1);
});

test("a synchronous initialization error also releases startup", () => {
    const h = harness({ initialThrow: new Error("IndexedDB unavailable") });
    assert.equal(h.dependencies, 0);
    assert.deepEqual(h.unmounts, [savePath]);
    assert.equal(h.statuses.at(-1).state, "memory");
});

test("overlapping writes serialize and coalesce into one follow-up flush", async () => {
    const h = harness();
    h.finish();
    h.write();
    for (let i = 0; i < 20; i++) h.write();
    assert.equal(h.requests.length, 1);
    assert.equal(h.requests[0].populate, false);
    h.finish();
    assert.equal(h.requests.length, 1, "one flush covers all queued changes");
    h.finish();
    assert.equal(h.requests.length, 0);
    assert.equal(h.peakActive, 1);
    assert.equal(h.statuses.at(-1).state, "saved");
    await new Promise(resolve => setImmediate(resolve));
    assert.equal(h.persistRequests, 1);
});

test("repeated write failures update the page, and a later write can recover", () => {
    const h = harness();
    h.finish();
    for (let i = 0; i < 3; i++) {
        h.write();
        h.finish(new Error("Quota exceeded"));
        assert.equal(h.statuses.at(-1).state, "warning");
    }
    assert.equal(h.statuses.filter(s => s.state === "warning").length, 3);
    h.write();
    h.finish();
    assert.equal(h.statuses.at(-1).state, "saved");
});

test("repeated write failures only warn once in the console fallback", () => {
    const h = harness();
    h.finish();
    delete h.context.Module.onStorageStatus;
    for (let i = 0; i < 3; i++) {
        h.write();
        h.finish(new Error("Quota exceeded"));
    }
    assert.equal(h.warnings.length, 1);
});

test("an optional page callback cannot prevent startup", () => {
    const h = harness({ onStatus: () => { throw new Error("Page UI failed"); } });
    h.finish();
    assert.equal(h.dependencies, 0);
    assert.ok(h.warnings.length > 0);
});
