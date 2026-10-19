import { spawn } from 'node:child_process';
import { createInterface } from 'node:readline';
import { fileURLToPath } from 'node:url';
import path from 'node:path';

// Node tests remain real, independent WebSocket clients of the Rust process.
// No relay behavior is implemented or simulated in this adapter.
export function createRustRelay(options = {}) {
  const binary = process.env.UQM_RUST_RELAY_BIN || (process.env.CARGO_TARGET_DIR
    ? path.resolve(process.env.CARGO_TARGET_DIR, 'release/uqm-relay')
    : fileURLToPath(new URL('../../rust/target/release/uqm-relay', import.meta.url)));
  let child;
  let exited;
  let startupFailed = false;
  let lastStats = { rooms: 0, connections: 0, usedRoomIds: 0, peakQueuedBytes: 0 };
  const statsWaiters = new Set();
  let stderr = '';
  return {
    get stats() { return lastStats; },
    waitForStats(predicate, timeoutMs = 2000) {
      if (predicate(lastStats)) return Promise.resolve(lastStats);
      return new Promise((resolve, reject) => {
        const waiter = { predicate, resolve, timer: setTimeout(() => {
          statsWaiters.delete(waiter);
          reject(new Error(`Rust stats timeout: ${JSON.stringify(lastStats)}`));
        }, timeoutMs) };
        statsWaiters.add(waiter);
      });
    },
    async listen(port = 9999, host = '127.0.0.1') {
      if (child) throw new Error('Rust relay already started');
      const { distDir, allowedOrigins, ...testOptions } = options;
      const args = ['--host', host, '--port', String(port), '--test-events'];
      if (distDir === false) testOptions.distDir = false;
      else if (distDir) args.push('--dist', distDir);
      for (const origin of allowedOrigins || []) args.push('--origin', origin);
      args.push('--test-options', JSON.stringify(testOptions));
      child = spawn(binary, args, { stdio: ['ignore', 'pipe', 'pipe'] });
      child.stderr.on('data', bytes => { stderr = (stderr + bytes).slice(-8192); });
      exited = new Promise(resolve => {
        child.once('exit', (code, signal) => resolve({ code, signal }));
        child.once('error', error => resolve({ code: null, signal: null, error: error.message }));
      });
      return new Promise((resolve, reject) => {
        let started = false;
        const fail = error => {
          clearTimeout(timer);
          if (!started) { startupFailed = true; child.kill('SIGKILL'); }
          reject(error);
        };
        const timer = setTimeout(() => fail(new Error(`Rust relay startup timeout: ${stderr}`)), 15000);
        child.once('error', fail);
        child.once('exit', (code, signal) => fail(new Error(`Rust relay exited: ${code ?? signal}; ${stderr}`)));
        const lines = createInterface({ input: child.stdout });
        lines.on('line', line => {
          let event;
          try { event = JSON.parse(line); } catch { fail(new Error(`Unexpected Rust relay output: ${line.slice(0, 1024)}`)); return; }
          if (event.event === 'listening') { started = true; clearTimeout(timer); resolve({ address: event.address, port: event.port }); }
          if (event.event === 'stats') {
            lastStats = { rooms: event.rooms, connections: event.connections,
              usedRoomIds: event.usedRoomIds, peakQueuedBytes: event.peakQueuedBytes };
            for (const waiter of statsWaiters) if (waiter.predicate(lastStats)) {
              clearTimeout(waiter.timer); statsWaiters.delete(waiter); waiter.resolve(lastStats);
            }
          }
        });
      });
    },
    async close() {
      if (!child || !child.pid) return;
      if (child.exitCode !== null || child.signalCode !== null) return;
      child.kill('SIGTERM');
      const timer = setTimeout(() => child.kill('SIGKILL'), 4000);
      try {
        const result = await exited;
        if (result.code !== 0 && !startupFailed) throw new Error(`Rust relay shutdown failed: ${JSON.stringify(result)}; ${stderr}`);
      } finally { clearTimeout(timer); }
    },
  };
}
