#!/usr/bin/env bash
# Kept for older notes: npm start runs scripts/serve.mjs.
set -euo pipefail
exec node "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/serve.mjs" "$@"
