/* Exercise the actual online-room exit helper while stubbing only the popup. */
#include <assert.h>
#include <stdio.h>
#include <string.h>
#include "../../melee.c"

GLOBDATA GlobData;
int webNetplayRole = -1;
static unsigned popups;
static const char *lastMessage;

void DoPopupWindow(const char *message) {
	// CHECK_ABORT would make the real popup immediately disappear.
	assert(!(GLOBAL(CurrentActivity) & CHECK_ABORT));
	popups++;
	lastMessage = message;
}

int main(void) {
	const char *message = "Top player: connection closed.";
	// Local browser play preserves its legacy behavior.
	memset(&GlobData, 0, sizeof GlobData);
	assert(!closeWebMeleeRoom(message));
	assert(popups == 0 && !(GLOBAL(CurrentActivity) & CHECK_ABORT));
	for (int role = 0; role < 2; role++) {
		webNetplayRole = role;
		GLOBAL(CurrentActivity) = SUPER_MELEE;
		unsigned before = popups;
		assert(closeWebMeleeRoom(message));
		assert(popups == before + 1 && lastMessage == message);
		assert(GLOBAL(CurrentActivity) & CHECK_ABORT);
		// Cleanup/intentional quit must not show another modal.
		assert(closeWebMeleeRoom(message));
		assert(popups == before + 1);
	}
	webNetplayRole = -1;
	GLOBAL(CurrentActivity) = 0;
	assert(!closeWebMeleeRoom(message));
	assert(!(GLOBAL(CurrentActivity) & CHECK_ABORT));
	puts("Setup disconnect: both online roles show one popup and abort; local play and intentional exit are preserved.");
	return 0;
}
