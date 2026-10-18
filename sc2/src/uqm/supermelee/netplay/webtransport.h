/* Browser transport for the unchanged UQM netplay protocol. */
#ifndef UQM_SUPERMELEE_NETPLAY_WEBTRANSPORT_H_
#define UQM_SUPERMELEE_NETPLAY_WEBTRANSPORT_H_

#ifdef __EMSCRIPTEN__
#include <emscripten.h>
#include <stddef.h>

/* All JavaScript access runs on the browser main runtime. The simulation
 * runs on a pthread; WebSocket callbacks only enqueue bytes, never call C. */
static inline int
WebTransport_status(void) {
	return MAIN_THREAD_EM_ASM_INT({
		var net = Module.webNet;
		if (!net || net.state === 'closed' || net.state === 'error') return -1;
		if (net.state !== 'ready') return 0;
		return net.role === 0 ? 1 : net.role === 1 ? 2 : -1;
	});
}

/* Copy at most capacity bytes, preserving ordered stream semantics even
 * when a WebSocket message contains partial or multiple UQM packets. */
static inline int
WebTransport_receive(void *dest, size_t capacity) {
	return MAIN_THREAD_EM_ASM_INT({
		var net = Module.webNet;
		if (!net || net.state !== 'ready') return -1;
		var copied = 0;
		while (net.incoming.length && copied < $1) {
			var chunk = net.incoming[0];
			if (!(chunk instanceof Uint8Array)) return -1;
			var length = Math.min(chunk.byteLength, $1 - copied);
			HEAPU8.set(chunk.subarray(0, length), $0 + copied);
			copied += length;
			net.incomingBytes -= length;
			if (length === chunk.byteLength) net.incoming.shift();
			else net.incoming[0] = chunk.subarray(length);
		}
		return copied;
	}, dest, capacity);
}

/* 1 sent, 0 backpressure (retain packet for retry), -1 disconnected. */
static inline int
WebTransport_send(const void *data, size_t size) {
	return MAIN_THREAD_EM_ASM_INT({
		var net = Module.webNet;
		if (!net || net.state !== 'ready') return -1;
		try {
			if (net.send(HEAPU8.slice($0, $0 + $1))) return 1;
			return net.state === 'ready' ? 0 : -1;
		} catch (error) {
			return -1;
		}
	}, data, size);
}

static inline void
WebTransport_close(void) {
	MAIN_THREAD_EM_ASM({
		if (Module.webNet) Module.webNet.close();
	});
}

static inline void
WebTransport_engineState(const char *state, int player) {
	MAIN_THREAD_EM_ASM({
		var net = Module.webNet;
		if (!net) return;
		net.stats.engineState = UTF8ToString($0);
		net.stats.remotePlayer = $1;
	}, state, player);
}

static inline void
WebTransport_checksum(unsigned frame, int matches) {
	MAIN_THREAD_EM_ASM({
		var net = Module.webNet;
		if (!net) return;
		if ($1) {
			net.stats.checksumsVerified = (net.stats.checksumsVerified || 0) + 1;
			net.stats.lastVerifiedFrame = $0 >>> 0;
		} else {
			net.stats.checksumMismatches = (net.stats.checksumMismatches || 0) + 1;
		}
	}, frame, matches);
}
#endif
#endif
