/* SPDX-License-Identifier: GPL-2.0-or-later
 * Configuration/stdio adapters for the Rust math implementation. The HD engine
 * chooses a different RNG multiplier for custom planet generation. Keep this
 * access in C until the owning game-state types have been migrated.
 */
#include "mthintrn.h"
#include <stdio.h>

DWORD uqm_hd_seed_a(void);
DWORD
uqm_hd_seed_a(void)
{
	return SeedA;
}

void
printBits(size_t size, const void *ptr)
{
	const unsigned char *bytes = ptr;
	while (size > 0)
	{
		int bit;
		--size;
		for (bit = 7; bit >= 0; --bit)
			putchar('0' + ((bytes[size] >> bit) & 1));
	}
	putchar('\n');
}
