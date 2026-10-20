# Rust private Melee relay

This binary replaces `server/netplay.mjs` for static game delivery and the
two-player private-room WebSocket relay. It does not replace the UQM engine or
the browser transport. The existing browser protocol needs no changes.

From the repository root, with Rust 1.85.1:

```sh
cargo build --locked --release --manifest-path rust/Cargo.toml -p uqm-relay
rust/target/release/uqm-relay --host 127.0.0.1 --port 9999 --dist dist
```

For HTTPS behind an existing reverse proxy, add the exact public origin:

```sh
rust/target/release/uqm-relay --host 127.0.0.1 --port 10010 \
  --origin https://game.example --dist /srv/uqm/web
```

`--origin` can be repeated. Forwarded headers are intentionally not trusted;
connections behind one proxy share its source-IP limit. The default remains
16 connections per IP, 128 active rooms, and 10,000 distinct room IDs for the
process lifetime. Room state is memory-only. Restarting terminates all rooms.
These production constraints are unchanged from the Node implementation.

The root of `--dist` must contain the already-built game or edition picker.
A folder path with a slash serves the folder's `index.html` (`/cosmoclash/`,
the Cosmo Clash build of a packaged site); the same path without the slash
redirects (301) to it and keeps the query (an invitation). The server sends the
WASM isolation headers. Static delivery streams 64 KiB
chunks, allows progressing downloads of any duration, and closes connections
after 30 seconds of inactivity during an individual file read/write. No directory
listing, hidden path segments or escaping symlinks are served. HTTP parsing is
strict: ambiguous duplicate headers, transfer encoding, request bodies, and
non-HTTP/1.1 request lines are rejected; browsers use supported GET/HEAD requests.
Non-regular files are rejected before and after opening. Unix opens also use
`O_NONBLOCK` and `O_NOFOLLOW` to prevent a concurrent FIFO or leaf-symlink swap
from blocking a worker. Failed accepts back off for 100 ms under resource
exhaustion while shutdown remains responsive.

## Protocol and resource guarantees

The endpoints, JSON notifications, roles, binary payload limit, queue bounds,
message/control quotas, heartbeats, room retirement and timeouts match the
[Node relay contract](../../docs/netplay-server.md). In particular:

- Original uses `/netplay/<token>`, HD uses `/netplay/hd/<token>`, and
  Cosmo Clash uses `/netplay/cosmoclash/<token>`, each followed by the exact
  `?role=host` or `?role=guest` query. Tokens are 32 lowercase hexadecimal
  characters. Active and retired IDs include the game/edition prefix; identical
  tokens across games cannot pair, exchange data or disconnect each other.
  All namespaces share the same capacity limits.
- Static Cosmo Clash assets use `model/gltf-binary`, `audio/ogg` and `audio/wav`
  for GLB models and Ogg/WAV audio, alongside existing WASM/image types.
- There are no room lists (lobbies). The room-list URL
  `/netplay/lobby?edition=original|hd|cosmoclash` and a host URL with `name` or
  `engine` get `{"type":"error","message":"Invalid room URL or role."}` and
  close code 1008, before any room list. The browser clients of UQM and Cosmo
  Clash take this as "no room lists here", do not try again and offer a
  private room with an invite link. The Node relay and the Cloudflare relay
  have the room lists.
- Binary data retains order and bytes; it is never accepted before pairing.
- A room has at most one host and one guest. Admission and teardown share one
  registry lock; a duplicate participant cannot displace another.
- Pending and in-flight output share byte/frame accounting. There are no
  unbounded task queues. Delayed data is canceled when its room ends.
- Ping/pong frames use a bounded priority channel and the same memory budget;
  artificial test latency does not delay heartbeat responses.
- WebSocket masking, reserved bits/opcodes, canonical lengths, close codes,
  UTF-8 and fragmented messages are validated. A message is at most 64 KiB,
  and at most 128 fragments, before any complete message reaches the relay.
- The framing implementation performs no automatic pong writes. It explicitly
  applies the control-frame quota before enqueueing a reply. An in-flight frame
  is completed or its TCP connection dropped; a close frame never interrupts
  an incomplete payload.
- SIGINT/SIGTERM stop admission, notify participants and bound shutdown time.

## Verification

```sh
cargo test --locked --manifest-path rust/Cargo.toml -p uqm-relay
cargo clippy --locked --manifest-path rust/Cargo.toml -p uqm-relay \
  --all-targets --all-features -- -D warnings
node --test tests/server/rust-netplay.test.mjs
```

The socket tests run the reference Node contract cases against a separate
Rust process, plus raw malformed/fragmented frames, simultaneous joins,
IP/global exhaustion, forwarded-header spoofing, independent byte-rate limits,
delayed-data heartbeat behavior, close-code echo, shutdown and HTTP validation.
The paused-reader case measures the Rust application's real queue accounting
instead of monkeypatching Node socket internals. Process stats are asynchronous;
the adapter waits for the relevant snapshot before asserting them.

`tests/helpers/rust-relay.mjs` exports `createRustRelay(options)` for integration
and browser tests. Its default is `rust/target/release/uqm-relay`; override with
`UQM_RUST_RELAY_BIN` for another build. `listen(0)` returns `{address, port}`;
`close()` sends SIGTERM and waits for exit. `stats` and `waitForStats(predicate)`
expose aggregate counters without room IDs or payload contents.

For test harnesses only, `--test-events` emits JSON lines with `event` equal to
`listening` or `stats`. `--test-options` accepts a JSON object with the Node
constructor's numeric limits/timeouts, `latencyPatternMs`, or `distDir:false`.
These are process arguments, never URL options. All values have enforced
bounds. Production use should omit both flags.

The implementation follows [RFC 6455](https://www.rfc-editor.org/rfc/rfc6455).
Passing relay tests does not alone establish full-game compatibility; real
browser matches and the engine's synchronization checks are required too.
