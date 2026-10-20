/* Host capabilities needed by the real graphics/units public headers. */
#ifndef UQM_VELOCITY_ORACLE_ENVIRONMENT_H
#define UQM_VELOCITY_ORACLE_ENVIRONMENT_H
#define HAVE_STRCASECMP_UQM 1
#define HAVE_WCHAR_T 1
#define HAVE_WINT_T 1
#define HAVE_READDIR_R 1
#define HAVE_SETENV 1
#ifdef UQM_TEST_HD
/* units.h needs this value from options.h; avoid pulling the unrelated SDL UI
 * into the arithmetic test. All units, structs and arithmetic macros stay real. */
#define OPTIONS_H
extern unsigned int resolutionFactor;
#endif
#endif
