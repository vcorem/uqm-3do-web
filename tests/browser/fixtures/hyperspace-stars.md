# Ambient-star regression screenshots

These are unchanged canvas captures from the normal HD campaign flight test,
using the same `hd-new-game.uqm` save and official HD assets. Both were captured
in Chromium 155.0.8059.84 at 1310 × 983 CSS pixels on 2026-10-19.

- `hyperspace-stars.png`: prior hybrid C/Rust movement build, from
  `test-results/rust-hd-movement-campaign/hyperspace.png`.
  Wasm SHA256: `9f391ba7d55d151d364a6fd7ae45336023c1fa8bb558e37e6dfec001092a5773`.
- `hyperspace-no-stars.png`: full Rust build with the faulty C2Rust macro
  reconstruction, from `test-results/rust-full-hd-campaign/hyperspace.png`.
  Wasm SHA256: `85042eadb02c677af4e813e02adf846b34ce2dc30f742b74290153bf94a421cc`.

The latter still has the ship, nearby system, red field, title and HUD, so it
passes the former nonblank/color checks. It has no ambient detail outside the
excluded central ship/system region. The valid image has 3,501 such pixels in
13 spatial cells. `scene-pixels.test.mjs` preserves this real regression pair;
`hd-campaign.mjs` applies the same metric to freshly generated engine output,
both after entering Hyperspace and while flying.

The screenshot art retains its original content licenses; these files are test
captures, not new replacement game assets. The measurement does not compare an
exact star pattern, which changes during normal animation and travel.
