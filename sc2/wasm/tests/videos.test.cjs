const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");
const vm = require("node:vm");

const source = fs.readFileSync(path.join(__dirname, "..", "pre.js"), "utf8");
const root = "/content/addons/3dovideo/";
const rmp = fs.readFileSync(path.join(__dirname, "../../content/addons/3dovideo/3dovideo.rmp"), "utf8");
// Derive the fixture from the engine's resource map, independently of detection.
const files = new Map();
for (const match of rmp.matchAll(/addons\/3dovideo\/[^:\s]+\.(duk|aif)/g)) {
    const name = "/content/" + match[0];
    files.set(name, { mode: "file", size: 100 });
    if (match[1] === "duk") {
        for (const extension of ["frm", "hdr", "tbl"]) {
            files.set(name.replace(/duk$/, extension), { mode: "file", size: 100 });
        }
    }
}
function detect(input) {
    const context = { Module: {}, FS: {
        stat(name) {
            if (!input.has(name)) throw new Error("ENOENT");
            return input.get(name);
        },
        isFile(mode) { return mode === "file"; },
    } };
    vm.runInNewContext(source, context);
    return context.Module.detect3DOVideos();
}
test("metadata-only checkout does not activate missing movies", () => {
    const result = detect(new Map([[root + "3dovideo.rmp", { mode: "file", size: 100 }]]));
    assert.equal(result.complete, false);
    assert.equal(result.total, 134);
    assert.equal(result.missing.length, 134);
});
test("all files referenced by the engine and their decoder companions activate videos", () => {
    assert.equal(files.size, 134);
    assert.equal(detect(files).complete, true);
});
test("every missing companion or audio track prevents activation", () => {
    for (const name of files.keys()) {
        const incomplete = new Map(files);
        incomplete.delete(name);
        const result = detect(incomplete);
        assert.equal(result.complete, false, name);
        assert.equal(result.missing.length, 1, name);
    }
});
test("an empty file or directory cannot satisfy a required video file", () => {
    for (const stat of [{ mode: "file", size: 0 }, { mode: "directory", size: 100 }]) {
        const invalid = new Map(files);
        invalid.set(root + "intro/intro.duk", stat);
        assert.equal(detect(invalid).complete, false);
    }
});
