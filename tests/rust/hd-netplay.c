/* Differential driver for the actual HD C and generated Rust netplay code.
 * The only substitutes are deterministic game callbacks and the browser room.
 * Both implementations use the real Emscripten heap, C ABI and byte bridge. */
#include <assert.h>
#include <errno.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <emscripten.h>
#define NETCONNECTION_INTERNAL
#include "uqm/supermelee/netplay/netconnection.h"
#include "uqm/supermelee/netplay/netrcv.h"
#include "uqm/supermelee/netplay/netsend.h"
#include "uqm/supermelee/netplay/packethandlers.h"
#include "uqm/supermelee/netplay/packet.h"
#include "uqm/supermelee/netplay/netmelee.h"
#include "uqm/supermelee/netplay/netmisc.h"
#include "libs/log.h"

extern void c_ref_webDataReady(NetConnection *);
extern int c_ref_sendPacket(NetConnection *, Packet *);
extern int c_ref_PacketHandler_Fleet(NetConnection *, const Packet_Fleet *);
extern int c_ref_PacketHandler_TeamName(NetConnection *, const Packet_TeamName *);
extern size_t rust_netplay_layout(unsigned);

PacketTypeData packetTypeData[PACKET_NUM];
static BattleStateData battleState;
static unsigned handled, disconnectAt, shipsChanged, namesChanged, cancelled;
static uint32_t trace;
static int lastError;
static unsigned long cases;
static const char *phase;

int uqm_lua_failed(void) { return 0; }
void log_add(log_Level level, const char *format, ...) { (void) level; (void) format; }
NetStatistics *NetConnection_getStatistics(NetConnection *conn) { return &conn->statistics; }
bool NetConnection_isConnected(const NetConnection *conn) { return conn->stateFlags.connected; }
NetConnectionStateData *NetConnection_getStateData(const NetConnection *conn) {
    (void) conn;
    return (NetConnectionStateData *) &battleState;
}
void NetConnection_webDisconnect(NetConnection *conn, int err) {
    if (conn->stateFlags.disconnected) return;
    conn->stateFlags.connected = false;
    conn->stateFlags.disconnected = true;
    lastError = err;
}
int Netplay_cancelConfirmation(NetConnection *conn) {
    cancelled++;
    conn->stateFlags.handshake.localOk = false;
    return 0;
}
void confirmationCancelled(MELEE_STATE *state, COUNT side) {
    (void) state;
    trace = trace * 33 + side + 1;
}
void Melee_RemoteChange_ship(MELEE_STATE *state, NetConnection *conn,
        COUNT side, FleetShipIndex index, MeleeShip ship) {
    (void) state; (void) conn;
    shipsChanged++;
    trace = trace * 33 + side;
    trace = trace * 33 + index;
    trace = trace * 33 + ship;
}
void Melee_RemoteChange_teamName(MELEE_STATE *state, NetConnection *conn,
        COUNT side, const char *name) {
    (void) state; (void) conn;
    namesChanged++;
    trace = trace * 33 + side;
    for (; *name; name++) trace = trace * 33 + (unsigned char) *name;
}
static int handlePacket(NetConnection *conn, const void *packet) {
    const unsigned char *bytes = packet;
    unsigned i;
    handled++;
    for (i = 0; i < sizeof(Packet_Init); i++) trace = trace * 33 + bytes[i];
    if (disconnectAt && handled == disconnectAt) NetConnection_webDisconnect(conn, ECONNRESET);
    return 0;
}

typedef struct {
    int result, error, disconnectError;
    unsigned handled, ships, names, cancelled;
    uint32_t trace;
    size_t buffered;
    int queued;
    NetStateFlags flags;
    NetStatistics statistics;
    unsigned char bytes[NETPLAY_READBUFSIZE];
} Snapshot;

static void reset(NetConnection *conn, unsigned char *buffer) {
    memset(conn, 0, sizeof *conn);
    memset(buffer, 0, NETPLAY_READBUFSIZE);
    memset(&battleState, 0, sizeof battleState);
    conn->readBuf = conn->readEnd = buffer;
    conn->state = NetState_inSetup;
    conn->stateFlags.connected = true;
    handled = shipsChanged = namesChanged = cancelled = trace = 0;
    disconnectAt = 0;
    lastError = errno = 0;
    MAIN_THREAD_EM_ASM({
        Module.webNet = ({state:'ready', role:0, incoming:[], incomingBytes:0, stats:{},
            send:function(bytes) { this.sent=bytes; return true; },
            close:function() { this.state='closed'; }});
    });
}
static void enqueue(const void *data, size_t length) {
    MAIN_THREAD_EM_ASM({
        Module.webNet.incoming.push(HEAPU8.slice($0, $0 + $1));
        Module.webNet.incomingBytes += $1;
    }, data, length);
}
static Snapshot snapshot(NetConnection *conn, int result) {
    Snapshot value;
    memset(&value, 0, sizeof value);
    value.result = result;
    value.error = errno;
    value.disconnectError = lastError;
    value.handled = handled;
    value.ships = shipsChanged;
    value.names = namesChanged;
    value.cancelled = cancelled;
    value.trace = trace;
    value.buffered = conn->readEnd - conn->readBuf;
    value.queued = MAIN_THREAD_EM_ASM_INT({ return Module.webNet.incomingBytes; });
    value.flags = conn->stateFlags;
    value.statistics = conn->statistics;
    memcpy(value.bytes, conn->readBuf, sizeof value.bytes);
    return value;
}
static void compare(Snapshot reference, Snapshot rust) {
    cases++;
    if (memcmp(&reference, &rust, sizeof reference)) {
        fprintf(stderr, "%s differential mismatch in case %lu: C result=%d errno=%d handled=%u; Rust result=%d errno=%d handled=%u\n",
                phase, cases, reference.result, reference.error, reference.handled,
                rust.result, rust.error, rust.handled);
        assert(0);
    }
}

static void check_layout(void) {
    const size_t expected[] = {
        sizeof(NetConnection), sizeof(NetStateFlags), sizeof(NetStatistics),
        offsetof(NetConnection, stateFlags), offsetof(NetConnection, statistics),
        offsetof(NetConnection, readBuf), offsetof(NetConnection, readEnd),
        sizeof(Packet_Fleet), sizeof(Packet_TeamName), sizeof(BattleStateData),
        offsetof(BattleStateData, meleeState), sizeof(NetConnection), sizeof(NetConnection)
    };
    size_t i;
    for (i = 0; i < sizeof expected / sizeof expected[0]; i++)
        assert(rust_netplay_layout(i) == expected[i]);
}

static void fleet_packets(void) {
    unsigned state, resetFlag, ready, side, count, lengthIndex, variant;
    const unsigned lengths[] = {0, 7, 8, 9, 10, 12, 35, 36, 40, 100};
    const unsigned ships[] = {0, 24, 25, 254, 255};
    union { Packet_Fleet packet; unsigned char bytes[128]; } data;
    unsigned char buffer[NETPLAY_READBUFSIZE];
    NetConnection conn;
    phase = "fleet packet";
    for (state = 0; state < 2; state++)
    for (resetFlag = 0; resetFlag < 3; resetFlag++)
    for (ready = 0; ready < 2; ready++)
    for (side = 0; side < 4; side++)
    for (count = 0; count < 18; count++)
    for (lengthIndex = 0; lengthIndex < sizeof lengths / sizeof lengths[0]; lengthIndex++) {
        Snapshot samples[2];
        memset(&data, 0, sizeof data);
        data.packet.header.len = hton16(lengths[lengthIndex]);
        data.packet.header.type = hton16(PACKET_FLEET);
        data.packet.side = side;
        data.packet.numShips = hton16(count);
        for (variant = 0; variant < 18; variant++) {
            data.packet.ships[variant].index = variant % MELEE_FLEET_SIZE;
            data.packet.ships[variant].ship = variant % NUM_MELEE_SHIPS;
        }
        for (variant = 0; variant < 2; variant++) {
            int result;
            reset(&conn, buffer);
            conn.player = count % 2;
            conn.stateFlags.discriminant = lengthIndex % 2;
            conn.state = state ? NetState_inBattle : NetState_inSetup;
            conn.stateFlags.reset.localReset = resetFlag == 1;
            conn.stateFlags.reset.remoteReset = resetFlag == 2;
            conn.stateFlags.handshake.localOk = ready;
            result = variant ? PacketHandler_Fleet(&conn, &data.packet)
                             : c_ref_PacketHandler_Fleet(&conn, &data.packet);
            samples[variant] = snapshot(&conn, result);
        }
        compare(samples[0], samples[1]);
    }
    /* Explicitly reject out-of-range ship indexes/types while accepting the
     * MELEE_NONE marker, independently of the differential comparison. */
    for (count = 0; count < 16; count++)
    for (lengthIndex = 0; lengthIndex < sizeof ships / sizeof ships[0]; lengthIndex++) {
        Snapshot samples[2];
        memset(&data, 0, sizeof data);
        data.packet.header.len = hton16(12);
        data.packet.header.type = hton16(PACKET_FLEET);
        data.packet.numShips = hton16(1);
        data.packet.ships[0].index = count;
        data.packet.ships[0].ship = ships[lengthIndex];
        for (variant = 0; variant < 2; variant++) {
            int result;
            reset(&conn, buffer);
            result = variant ? PacketHandler_Fleet(&conn, &data.packet)
                             : c_ref_PacketHandler_Fleet(&conn, &data.packet);
            assert(result == ((count >= MELEE_FLEET_SIZE ||
                    (ships[lengthIndex] >= NUM_MELEE_SHIPS && ships[lengthIndex] != MELEE_NONE)) ? -1 : 0));
            samples[variant] = snapshot(&conn, result);
        }
        compare(samples[0], samples[1]);
    }
}

static void name_packets(void) {
    unsigned state, resetFlag, ready, side, length, variant, i;
    union { Packet_TeamName packet; unsigned char bytes[128]; } data;
    unsigned char buffer[NETPLAY_READBUFSIZE];
    NetConnection conn;
    phase = "team-name packet";
    for (state = 0; state < 2; state++)
    for (resetFlag = 0; resetFlag < 3; resetFlag++)
    for (ready = 0; ready < 2; ready++)
    for (side = 0; side < 4; side++)
    for (length = 0; length < 71; length++) {
        Snapshot samples[2];
        memset(&data, 0, sizeof data);
        for (i = sizeof(Packet_TeamName); i < sizeof data.bytes; i++) data.bytes[i] = 'A' + i % 26;
        data.packet.header.len = hton16(length);
        data.packet.header.type = hton16(PACKET_TEAMNAME);
        data.packet.side = side;
        for (variant = 0; variant < 2; variant++) {
            int result;
            reset(&conn, buffer);
            conn.player = length % 2;
            conn.stateFlags.discriminant = length % 3 == 0;
            conn.state = state ? NetState_inBattle : NetState_inSetup;
            conn.stateFlags.reset.localReset = resetFlag == 1;
            conn.stateFlags.reset.remoteReset = resetFlag == 2;
            conn.stateFlags.handshake.localOk = ready;
            result = variant ? PacketHandler_TeamName(&conn, &data.packet)
                             : c_ref_PacketHandler_TeamName(&conn, &data.packet);
            samples[variant] = snapshot(&conn, result);
        }
        compare(samples[0], samples[1]);
    }
}

static void receive_packets(void) {
    const unsigned char init[] = {0,12,0,0,0,4,0,0,0,8,5,0};
    const unsigned char malformed[][4] = {{8,1,0,0}, {0,12,255,255}, {0,4,0,0}};
    unsigned char stream[sizeof init * 200], buffer[NETPLAY_READBUFSIZE];
    NetConnection conn;
    unsigned split, variant, i, scenario;
    phase = "fragmented/coalesced receive";
    for (i = 0; i < 200; i++) memcpy(stream + sizeof init * i, init, sizeof init);
    for (split = 1; split < sizeof init * 3; split++) {
        Snapshot samples[2][2];
        for (variant = 0; variant < 2; variant++) {
            reset(&conn, buffer);
            enqueue(stream, split);
            (variant ? webDataReady : c_ref_webDataReady)(&conn);
            samples[variant][0] = snapshot(&conn, 0);
            enqueue(stream + split, sizeof stream - split);
            (variant ? webDataReady : c_ref_webDataReady)(&conn);
            assert(handled == 200 && lastError == 0 && conn.readEnd == buffer);
            samples[variant][1] = snapshot(&conn, 0);
        }
        compare(samples[0][0], samples[1][0]);
        compare(samples[0][1], samples[1][1]);
    }
    phase = "malformed/disconnected receive";
    for (scenario = 0; scenario < 7; scenario++) {
        Snapshot samples[2];
        for (variant = 0; variant < 2; variant++) {
            reset(&conn, buffer);
            if (scenario < 3) enqueue(malformed[scenario], sizeof malformed[0]);
            if (scenario == 3) {
                enqueue(stream, sizeof stream);
                disconnectAt = 2;
            }
            if (scenario == 4) conn.readEnd += NETPLAY_READBUFSIZE;
            if (scenario == 5) MAIN_THREAD_EM_ASM({ Module.webNet.state = 'closed'; });
            if (scenario == 6) MAIN_THREAD_EM_ASM({ Module.webNet.incoming.push([1,2]); Module.webNet.incomingBytes = 2; });
            (variant ? webDataReady : c_ref_webDataReady)(&conn);
            assert(lastError != 0 && !conn.stateFlags.connected);
            samples[variant] = snapshot(&conn, 0);
        }
        compare(samples[0], samples[1]);
    }
}

static void send_packets(void) {
    Packet_Init packet;
    unsigned char buffer[NETPLAY_READBUFSIZE];
    NetConnection conn;
    unsigned scenario, variant;
    phase = "send/backpressure/disconnect";
    memset(&packet, 0, sizeof packet);
    packet.header.len = hton16(sizeof packet);
    packet.header.type = hton16(PACKET_INIT);
    packet.protoVersion.minor = 4;
    for (scenario = 0; scenario < 5; scenario++) {
        Snapshot samples[2];
        for (variant = 0; variant < 2; variant++) {
            int result;
            reset(&conn, buffer);
            if (scenario == 1) MAIN_THREAD_EM_ASM({ Module.webNet.send = function() { return false; }; });
            if (scenario == 2) MAIN_THREAD_EM_ASM({ Module.webNet.state = 'closed'; });
            if (scenario == 3) MAIN_THREAD_EM_ASM({ Module.webNet.send = function() { throw new Error('send failed'); }; });
            if (scenario == 4) MAIN_THREAD_EM_ASM({ Module.webNet.send = function() { this.state='closed'; return false; }; });
            result = variant ? sendPacket(&conn, (Packet *) &packet)
                             : c_ref_sendPacket(&conn, (Packet *) &packet);
            assert(result == (scenario ? -1 : 0));
            assert(errno == (scenario == 0 ? 0 : scenario == 1 ? EAGAIN : ECONNRESET));
            if (!scenario) assert(MAIN_THREAD_EM_ASM_INT({ return Module.webNet.sent.length === 12 && Module.webNet.sent[5] === 4; }));
            samples[variant] = snapshot(&conn, result);
        }
        compare(samples[0], samples[1]);
    }
}

int main(void) {
    unsigned i;
    for (i = 0; i < PACKET_NUM; i++) packetTypeData[i].name = "test packet";
    packetTypeData[PACKET_INIT].len = sizeof(Packet_Init);
    packetTypeData[PACKET_INIT].handler = handlePacket;
    check_layout();
    fleet_packets();
    name_packets();
    receive_packets();
    send_packets();
    printf("{\"cases\":%lu,\"layout_checks\":13,\"passed\":true}\n", cases);
    return 0;
}
