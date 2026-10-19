/* Translation-only forward declaration of the same existing C object.
 * C2Rust loses repeated block-scoped externs; a file-scope declaration gives
 * every reference the same canonical declaration without changing behavior.
 */
#include "uqm/starmap.h"
extern const STAR_DESC starmap_array[];
