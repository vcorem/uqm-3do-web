/* Host capabilities needed by the real graphics/units public headers. */
#ifndef UQM_VELOCITY_ORACLE_ENVIRONMENT_H
#define UQM_VELOCITY_ORACLE_ENVIRONMENT_H
/* Respect generated platform capabilities before supplying oracle fallbacks. */
#include "config.h"
#ifndef HAVE_STRCASECMP_UQM
#define HAVE_STRCASECMP_UQM 1
#endif
#ifndef HAVE_WCHAR_T
#define HAVE_WCHAR_T 1
#endif
#ifndef HAVE_WINT_T
#define HAVE_WINT_T 1
#endif
#ifndef HAVE_READDIR_R
#define HAVE_READDIR_R 1
#endif
#ifndef HAVE_SETENV
#define HAVE_SETENV 1
#endif
#ifdef UQM_TEST_HD
/* units.h needs this value from options.h; avoid pulling the unrelated SDL UI
 * into the arithmetic test. All units, structs and arithmetic macros stay real. */
#define OPTIONS_H
extern unsigned int resolutionFactor;
#endif
#endif
