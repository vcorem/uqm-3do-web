/* Rename only exports: the oracle compiles unchanged upstream C algorithms. */
#define InitQueue oracle_InitQueue
#define UninitQueue oracle_UninitQueue
#define ReinitQueue oracle_ReinitQueue
#define AllocLink oracle_AllocLink
#define FreeLink oracle_FreeLink
#define PutQueue oracle_PutQueue
#define InsertQueue oracle_InsertQueue
#define RemoveQueue oracle_RemoveQueue
#define CountLinks oracle_CountLinks
#define ForAllLinks oracle_ForAllLinks
#define BoxIntersect oracle_BoxIntersect
#define BoxUnion oracle_BoxUnion
#define _clip_line oracle_clip_line
