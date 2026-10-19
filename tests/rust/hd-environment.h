/* Minimal game-state inputs for the real MegaMod math/random.h. The HD RNG
 * header includes the entire graphics/sound/UI engine for just these two
 * values. Keep the actual RNG header and its SeedA/SeedQ/SeedR macros, while
 * replacing those unrelated engine dependencies with controlled test inputs.
 * OPTVAL_PLANET/OPTVAL_PRIME match setupmenu.h's OPT_SEED enumeration.
 */
#ifndef UQM_HD_ORACLE_ENVIRONMENT_H
#define UQM_HD_ORACLE_ENVIRONMENT_H
#include <stddef.h>
#include <stdio.h>
#include "libs/compiler.h"
#define UQM_GLOBDATA_H_
#define UQM_SETUPMENU_H_
#define OPTVAL_PRIME 0
#define OPTVAL_PLANET 1
extern int oracle_hd_seed_type;
extern DWORD oracle_hd_sis_Seed;
#define optSeedType oracle_hd_seed_type
#define GLOBAL_SIS(member) oracle_hd_sis_##member
#endif
