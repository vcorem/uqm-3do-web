/* Compile the real blocking-wait and battle-transition functions. Only I/O
 * is stubbed, so disconnects are injected at the point the engine yields. */
#define NETCONNECTION_INTERNAL
#include <assert.h>
#include <stdio.h>
#include <string.h>
#include "../netmelee.c"
#include "../../../tactrans.c"

GLOBDATA GlobData;
BattleFrameCounter battleFrameCount;
static BattleStateData battleState;
static NetConnection *pending;
static unsigned inputCalls;
static unsigned failOnInput;
static bool resetWait;

bool NetConnection_isConnected(const NetConnection *conn) { return conn->stateFlags.connected; }
NetState NetConnection_getState(const NetConnection *conn) { return conn->state; }
void NetConnection_setState(NetConnection *conn, NetState state) { conn->state = state; }
NetConnectionStateData *NetConnection_getStateData(const NetConnection *conn) {
	(void) conn; return (NetConnectionStateData *) &battleState;
}
void NetConnection_setReadyCallback(NetConnection *conn, NetConnection_ReadyCallback cb, void *arg) {
	conn->readyCallback = cb; conn->readyCallbackArg = arg; pending = conn; resetWait = false;
}
NetConnection_ReadyCallback NetConnection_getReadyCallback(const NetConnection *conn) { return conn->readyCallback; }
void *NetConnection_getReadyCallbackArg(const NetConnection *conn) { return conn->readyCallbackArg; }
bool Netplay_localReady(NetConnection *conn, NetConnection_ReadyCallback cb, void *arg, bool notify) {
	(void) notify; NetConnection_setReadyCallback(conn, cb, arg); return false;
}
bool Netplay_isLocalReady(const NetConnection *conn) { return conn->stateFlags.ready.localReady; }
void Netplay_setResetCallback(NetConnection *conn, NetConnection_ResetCallback cb, void *arg) {
	conn->resetCallback = cb; conn->resetCallbackArg = arg; pending = conn; resetWait = true;
}
bool Netplay_isLocalReset(const NetConnection *conn) { (void) conn; return true; }
void Netplay_localReset(NetConnection *conn, NetplayResetReason reason) { (void) conn; (void) reason; }
int flushPacketQueue(NetConnection *conn) { assert(conn->stateFlags.connected); return 0; }
void Netplay_Notify_frameCount(NetConnection *conn, BattleFrameCounter frame) { (void) conn; (void) frame; }
void NetConnection_webPoll(NetConnection *conn) { (void) conn; }
void NetConnection_webDisconnect(NetConnection *conn, int error) {
	(void) error; conn->stateFlags.connected = false; conn->state = NetState_unconnected;
}
void Async_process(void) {}
uint32 Async_timeBeforeNextMs(void) { return 0; }
void Callback_process(void) {}

void DoInput(void *state, BOOLEAN reset) {
	(void) state; (void) reset;
	assert(pending != NULL);
	inputCalls++;
	if (inputCalls == failOnInput) {
		pending->stateFlags.connected = false;
		pending->state = NetState_unconnected;
	} else if (resetWait) {
		pending->resetCallback(pending, pending->resetCallbackArg);
	} else {
		NetConnection_ReadyCallback cb = pending->readyCallback;
		void *arg = pending->readyCallbackArg;
		pending->readyCallback = NULL;
		pending->readyCallbackArg = NULL;
		cb(pending, arg);
	}
}

static void resetFixture(NetConnection *conn, NetState state, unsigned failure) {
	memset(conn, 0, sizeof *conn);
	memset(&battleState, 0, sizeof battleState);
	memset(&GlobData, 0, sizeof GlobData);
	conn->stateFlags.connected = true;
	conn->state = state;
	netConnections[0] = NULL;
	netConnections[1] = conn;
	pending = conn;
	inputCalls = 0;
	failOnInput = failure;
	resetWait = false;
	battleFrameCount = 100;
	battleState.endFrameCount = 100;
}

int main(void) {
	NetConnection conn;
	resetFixture(&conn, NetState_preBattle, 1);
	assert(!negotiateReadyConnections(true, NetState_interBattle));
	assert(inputCalls == 1 && !conn.stateFlags.connected);
	resetFixture(&conn, NetState_preBattle, 0);
	assert(negotiateReadyConnections(true, NetState_interBattle));
	assert(conn.state == NetState_interBattle);
	resetFixture(&conn, NetState_preBattle, 0);
	conn.stateFlags.connected = false;
	assert(!negotiateReadyConnections(true, NetState_interBattle));
	assert(inputCalls == 0);

	resetFixture(&conn, NetState_inBattle, 1);
	assert(!waitResetConnections(NetState_inSetup));
	resetFixture(&conn, NetState_inBattle, 0);
	assert(waitResetConnections(NetState_inSetup));
	assert(conn.state == NetState_inSetup);

	// The first end-of-battle wait used to assert endingBattle2 after a
	// disconnect. The second wait used to report success on disconnect.
	for (unsigned failedWait = 1; failedWait <= 2; failedWait++) {
		resetFixture(&conn, NetState_endingBattle, failedWait);
		NetConnection_setReadyCallback(&conn, readyToEnd2Callback, NULL);
		assert(!readyForBattleEndPlayer(&conn));
		assert(GLOBAL(CurrentActivity) & CHECK_ABORT);
		assert(inputCalls == failedWait);
	}
	resetFixture(&conn, NetState_endingBattle, 0);
	NetConnection_setReadyCallback(&conn, readyToEnd2Callback, NULL);
	assert(readyForBattleEndPlayer(&conn));
	assert(conn.state == NetState_interBattle);
	assert(!(GLOBAL(CurrentActivity) & CHECK_ABORT));

	resetFixture(&conn, NetState_endingBattle, 0);
	conn.stateFlags.connected = false;
	assert(!readyForBattleEndPlayer(&conn));
	assert(inputCalls == 0 && (GLOBAL(CurrentActivity) & CHECK_ABORT));
	assert(!readyForBattleEndPlayer(NULL));

	// No network peers is the normal local-game case, not a failed handshake.
	netConnections[0] = netConnections[1] = NULL;
	assert(negotiateReadyConnections(true, NetState_interBattle));
	assert(waitResetConnections(NetState_inSetup));
	puts("Disconnect waits: handshake/reset failures propagate, battle-end disconnects abort safely, successful and offline paths preserved.");
	return 0;
}
