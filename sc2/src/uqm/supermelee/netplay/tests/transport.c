/* Exercise the real Wasm byte bridge and packet assembler in isolation. */
#include <assert.h>
#include <stdio.h>
#include <string.h>
#include "../netrcv.c"

static unsigned handled;
static int lastError;

static int handlePacket(NetConnection *conn, const void *data) {
	(void) conn;
	(void) data;
	handled++;
	return 0;
}

PacketTypeData packetTypeData[PACKET_NUM] = {
	{sizeof(Packet_Init), handlePacket, "Init"},
};

void log_add(log_Level level, const char *format, ...) {
	(void) level;
	(void) format;
}

NetStatistics *NetConnection_getStatistics(NetConnection *conn) {
	return &conn->statistics;
}

bool NetConnection_isConnected(const NetConnection *conn) {
	return conn->stateFlags.connected;
}

void NetConnection_webDisconnect(NetConnection *conn, int err) {
	conn->stateFlags.connected = false;
	conn->stateFlags.disconnected = true;
	lastError = err;
}

static void reset(NetConnection *conn, uint8 *buffer) {
	memset(conn, 0, sizeof *conn);
	conn->readBuf = buffer;
	conn->readEnd = buffer;
	conn->stateFlags.connected = true;
	handled = 0;
	lastError = 0;
	MAIN_THREAD_EM_ASM({
		Module.webNet = ({
			state: 'ready', role: 0, incoming: [], incomingBytes: 0, stats: {},
			send: function(bytes) { this.sent = bytes; return true; },
			close: function() { this.state = 'closed'; }
		});
	});
}

static void enqueue(const void *data, unsigned length) {
	MAIN_THREAD_EM_ASM({
		Module.webNet.incoming.push(HEAPU8.slice($0, $0 + $1));
		Module.webNet.incomingBytes += $1;
	}, data, length);
}

int main(void) {
	NetConnection conn;
	uint8 buffer[NETPLAY_READBUFSIZE];
	const uint8 init[] = {0, 12, 0, 0, 0, 4, 0, 0, 0, 8, 0, 0};
	uint8 combined[36];
	unsigned i;
	const uint8 oversized[] = {8, 1, 0, 0};
	const uint8 badType[] = {0, 12, 255, 255};
	const uint8 shortPacket[] = {0, 4, 0, 0};

	reset(&conn, buffer);
	assert(WebTransport_status() == 1);
	MAIN_THREAD_EM_ASM({ Module.webNet.role = 1; });
	assert(WebTransport_status() == 2);
	MAIN_THREAD_EM_ASM({ Module.webNet.state = 'waiting'; });
	assert(WebTransport_status() == 0);
	WebTransport_close();
	assert(WebTransport_status() == -1);

	// Every possible boundary, including a fragmented four-byte header.
	for (i = 1; i < sizeof init; i++) {
		reset(&conn, buffer);
		enqueue(init, i);
		webDataReady(&conn);
		assert(handled == 0 && lastError == 0);
		enqueue(init + i, sizeof init - i);
		webDataReady(&conn);
		assert(handled == 1 && conn.readEnd == buffer && lastError == 0);
		assert(MAIN_THREAD_EM_ASM_INT({ return Module.webNet.incomingBytes; }) == 0);
	}

	reset(&conn, buffer);
	for (i = 0; i < 3; i++) memcpy(combined + i * sizeof init, init, sizeof init);
	enqueue(combined, 13);
	enqueue(combined + 13, sizeof combined - 13);
	webDataReady(&conn);
	assert(handled == 3 && conn.readEnd == buffer);

	reset(&conn, buffer);
	enqueue(oversized, sizeof oversized);
	webDataReady(&conn);
	assert(lastError == EBADMSG && !conn.stateFlags.connected);
	reset(&conn, buffer);
	enqueue(badType, sizeof badType);
	webDataReady(&conn);
	assert(lastError == EBADMSG && handled == 0);
	reset(&conn, buffer);
	enqueue(shortPacket, sizeof shortPacket);
	webDataReady(&conn);
	assert(lastError == EBADMSG && handled == 0);

	reset(&conn, buffer);
	assert(WebTransport_send(init, sizeof init) == 1);
	assert(MAIN_THREAD_EM_ASM_INT({ return Module.webNet.sent.length; }) == sizeof init);
	assert(MAIN_THREAD_EM_ASM_INT({ return Module.webNet.sent[5]; }) == 4);
	MAIN_THREAD_EM_ASM({ Module.webNet.send = function() { return false; }; });
	assert(WebTransport_send(init, sizeof init) == 0);
	MAIN_THREAD_EM_ASM({ Module.webNet.state = 'closed'; });
	assert(WebTransport_send(init, sizeof init) == -1);
	assert(WebTransport_receive(buffer, sizeof buffer) == -1);

	reset(&conn, buffer);
	WebTransport_engineState("inBattle", 1);
	WebTransport_checksum(42, 1);
	WebTransport_checksum(43, 0);
	assert(MAIN_THREAD_EM_ASM_INT({ return Module.webNet.stats.engineState === 'inBattle'; }));
	assert(MAIN_THREAD_EM_ASM_INT({ return Module.webNet.stats.lastVerifiedFrame; }) == 42);
	assert(MAIN_THREAD_EM_ASM_INT({ return Module.webNet.stats.checksumsVerified; }) == 1);
	assert(MAIN_THREAD_EM_ASM_INT({ return Module.webNet.stats.checksumMismatches; }) == 1);
	puts("Wasm transport: fragmentation, coalescing, malformed packets, backpressure, disconnect and diagnostics passed.");
	return 0;
}
