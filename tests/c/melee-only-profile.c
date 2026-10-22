/* Exercise the actual dedicated native loop with owned session stubs.
 * No ship, combat, or netplay state is injected into a running engine. */
#include <assert.h>
#include <stdlib.h>
#include <string.h>
#include "uqm/restart.c"

GLOBDATA GlobData;
ACTIVITY LastActivity, NextActivity;
QUEUE race_q[2];
int webNetplayRole = -1;
NetplayOptions netplayOptions;
OPT_ENABLABLE optShipSeed;
static unsigned sessions, localSeeds, queueResets, flushes, reloads, frees;
static int initialRole = -1;

void ReinitQueue (QUEUE *queue) { (void)queue; ++queueResets; }
void SetDefaultMenuRepeatDelay (void) {}
void FlushInput (void) { ++flushes; }
DWORD SeedRandomNumbers (void) { ++localSeeds; return 123; }
void ReloadMasterShipList (void (*yield)(void)) { (void)yield; ++reloads; }
void FreeGameData (void) { ++frees; }
void Melee (void)
{
    int expected[] = {initialRole, 0, 1, -1};
    assert (sessions < 4 && webNetplayRole == expected[sessions]);
    assert (!LastActivity && !NextActivity && !GLOBAL (CurrentActivity));
    assert (optShipSeed == OPTVAL_DISABLED);
    if (sessions > 0 && webNetplayRole >= 0) assert (netplayOptions.inputDelay == 3);
    assert (MAIN_THREAD_EM_ASM_INT({ return Module.gameScreen === 'melee'; }));
    ++sessions;
    GLOBAL (CurrentActivity) = CHECK_ABORT;
}
static void finish (void)
{
    assert (sessions == 4 && queueResets == 8 && flushes == 4 && reloads == 4 && frees == 4);
    assert (localSeeds == (initialRole < 0 ? 2u : 1u));
    assert (MAIN_THREAD_EM_ASM_INT({
        return Module.screenHistory.length === 8 &&
            Module.screenHistory.every(function (screen, index) {
                return screen === (index % 2 ? 'meleeMenu' : 'melee');
            });
    }));
    puts ("Native profile: direct launch, one-shot local/network routing and safe repeated returns passed.");
    exit (0);
}

void SleepThread (TimePeriod delay)
{
    assert (delay == ONE_SECOND / 30 && webNetplayRole == -1);
    assert (MAIN_THREAD_EM_ASM_INT({
        return Module.supportsNetplayHook === true && Module.gameScreen === 'meleeMenu';
    }));
    if (sessions == 4) finish ();
    assert (sessions > 0); /* Queued initial local/network starts never wait. */
    MAIN_THREAD_EM_ASM({
        if ($0 === 1) Module.pendingRole = 0;
        else if ($0 === 2) Module.pendingRole = 1;
        else Module.pendingLocal = true;
    }, sessions);
}

int main (int argc, char **argv)
{
    int classic = argc > 1 && !strcmp (argv[1], "classic");
    initialRole = argc > 1 && !strcmp (argv[1], "network") ? 0 : -1;
    webNetplayRole = initialRole;
    MAIN_THREAD_EM_ASM({
        Module.meleeOnly = !$0;
        Module.pendingLocal = $1 < 0;
        Module.pendingRole = -1;
        Module.takePendingLocalMelee = function () {
            var result = Module.pendingLocal; Module.pendingLocal = false; return result;
        };
        Module.takePendingNetplayRole = function () {
            var result = Module.pendingRole; Module.pendingRole = -1; return result;
        };
        Module.screenHistory = [];
        Object.defineProperty(Module, 'gameScreen', {
            get: function () { return this.screenHistory[this.screenHistory.length - 1]; },
            set: function (screen) { this.screenHistory.push(screen); }
        });
    }, classic, initialRole);
    assert (!!UQM_MeleeOnlyProfile () == !classic);
    MAIN_THREAD_EM_ASM({ Module.meleeOnly = !Module.meleeOnly; });
    assert (!!UQM_MeleeOnlyProfile () == !classic); /* Launch profile is latched. */
    if (classic) {
        puts ("Native profile: ordinary edition remains opt-out.");
        return 0;
    }
    runMeleeOnly ();
    assert (!"The Melee-only loop must never return a campaign");
    return 0;
}
