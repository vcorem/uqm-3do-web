# Upstream reference files

## Test runner compatibility

`upstream-tests/src/game/SelectionScene.test.ts` and
`upstream-tests/src/game/ThrustEffect.test.ts` preserve the exact imported tests.
Their active copies use ordinary functions instead of arrow functions for
constructor mocks, as required by Vitest 4; their assertions are unchanged.
The provenance verifier checks the preserved copies against the original,
unchanged SHA-256 manifest. Reference copies are excluded from test discovery.
Gameplay and AI reference implementation files remain unchanged.
