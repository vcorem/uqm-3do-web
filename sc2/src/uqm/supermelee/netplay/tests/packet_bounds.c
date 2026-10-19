/* Regression tests for untrusted variable-length setup packets. */
#include <assert.h>
#include <stdio.h>
#include <string.h>
#include "../packethandlers.c"

PacketTypeData packetTypeData[PACKET_NUM];
static BattleStateData battleState;
static unsigned shipsChanged;
static unsigned namesChanged;

void log_add(log_Level level, const char *format, ...) {
	(void) level;
	(void) format;
}
NetConnectionStateData *NetConnection_getStateData(const NetConnection *conn) {
	(void) conn;
	return (NetConnectionStateData *) &battleState;
}
int Netplay_cancelConfirmation(NetConnection *conn) {
	(void) conn;
	assert(!"Invalid packets must not alter confirmation state");
	return 0;
}
void confirmationCancelled(MELEE_STATE *state, COUNT side) {
	(void) state;
	(void) side;
}
void Melee_RemoteChange_ship(MELEE_STATE *state, NetConnection *conn,
		COUNT side, FleetShipIndex index, MeleeShip ship) {
	(void) state; (void) conn; (void) side; (void) index; (void) ship;
	shipsChanged++;
}
void Melee_RemoteChange_teamName(MELEE_STATE *state, NetConnection *conn,
		COUNT side, const char *name) {
	(void) state; (void) conn; (void) side;
	assert(strcmp(name, "Hi") == 0);
	namesChanged++;
}

int main(void) {
	NetConnection conn;
	union { Packet_Fleet fleet; Packet_TeamName name; uint8 bytes[32]; } packet;
	memset(&conn, 0, sizeof conn);
	conn.state = NetState_inSetup;
	memset(&packet, 0, sizeof packet);
	packet.fleet.header.len = hton16(sizeof(Packet_Fleet));
	packet.fleet.numShips = hton16(1);
	// The old sizeof(pointer) check accepted this missing ship entry on Wasm.
	assert(PacketHandler_Fleet(&conn, &packet.fleet) == -1);
	assert(errno == EBADMSG && shipsChanged == 0);
	packet.fleet.numShips = 0;
	packet.fleet.side = 2;
	assert(PacketHandler_Fleet(&conn, &packet.fleet) == -1);
	packet.fleet.side = 0;
	packet.fleet.numShips = hton16(1);
	packet.fleet.header.len = hton16(12);
	assert(PacketHandler_Fleet(&conn, &packet.fleet) == 0);
	assert(shipsChanged == 1);

	memset(&packet, 0, sizeof packet);
	packet.name.header.len = hton16(sizeof(Packet_TeamName));
	conn.stateFlags.handshake.localOk = true;
	assert(PacketHandler_TeamName(&conn, &packet.name) == -1);
	assert(errno == EBADMSG && namesChanged == 0);
	packet.name.header.len = hton16(10);
	packet.name.side = 2;
	assert(PacketHandler_TeamName(&conn, &packet.name) == -1);
	conn.stateFlags.handshake.localOk = false;
	packet.name.side = 0;
	memcpy(packet.name.name, "Hi", 3);
	assert(PacketHandler_TeamName(&conn, &packet.name) == 0);
	assert(namesChanged == 1);
	puts("Setup packet bounds: truncated fleets/names and invalid sides rejected; valid packets preserved.");
	return 0;
}
