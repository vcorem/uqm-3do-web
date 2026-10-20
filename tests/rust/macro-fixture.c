/* Translator regression: preserve individual Clang-expanded macro calls.
 * In particular MakeLinks aliases a function-like macro, as UQM prim.h does.
 * Reconstructing that object macro from one call loses the other arguments. */
typedef unsigned char BYTE;
typedef unsigned short COUNT;
typedef unsigned int DWORD;
#define MAKE_DWORD(lo, hi) ((DWORD)(COUNT)(lo) | ((DWORD)(COUNT)(hi) << 16))
#define MakeLinks MAKE_DWORD
#define DynamicLinks MAKE_DWORD
#define GetPredLink(value) ((COUNT)(value))
#define GetSuccLink(value) ((COUNT)((DWORD)(value) >> 16))
#define SetPrimLinks(prim, pred, succ) ((prim)->links = DynamicLinks(pred, succ))
#define NestedLinks(pred, succ) DynamicLinks(GetSuccLink(pred), GetPredLink(succ))
#define SizeOf sizeof
#define CAST_VALUE(type, value) ((type)(value))
#define CastValue CAST_VALUE
#define STEP(value) (++(value))
#define GET_FIELD(pointer, field) ((pointer)->field)
#define GetField GET_FIELD
#define OFFSET(type, field) ((DWORD)__builtin_offsetof(type, field))
#define JOIN_INNER(a, b) a##b
#define JOIN(a, b) JOIN_INNER(a, b)
#define SELECT(name) JOIN(value_, name)
#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define Max MAX
struct Primitive { DWORD links; BYTE kind; COUNT index; };
struct Other { BYTE prefix[3]; DWORD value; BYTE suffix; };
static const DWORD value_one = 17;
static const DWORD value_two = 29;

DWORD macro_fixture(DWORD selector, DWORD a, DWORD b) {
    struct Primitive primitive = {0, 7, 19};
    struct Other other = {{1, 2, 3}, a, 4};
    switch (selector) {
    case 0: return MakeLinks(65535, 65535);
    case 1: return MakeLinks(329, 0);
    case 2: return DynamicLinks(a, b);
    case 3: return DynamicLinks(GetPredLink(a), GetSuccLink(b));
    case 4: SetPrimLinks(&primitive, a, b); return primitive.links;
    case 5: return NestedLinks(DynamicLinks(a, b), DynamicLinks(b, a));
    case 6: return SizeOf(BYTE) + 256 * SizeOf(COUNT) + 65536 * SizeOf(DWORD);
    case 7: return CastValue(BYTE, a) + (CastValue(COUNT, b) << 8);
    case 8: return SELECT(one) * a + SELECT(two) * b;
    case 9: return SizeOf(primitive.kind) + 16 * SizeOf(primitive.index) + 256 * SizeOf(primitive.links);
    case 10: { DWORD result = DynamicLinks(STEP(a), STEP(b)); return result ^ a ^ (b << 8); }
    case 11: return GetField(&primitive, kind) + GetField(&other, value);
    case 12: return OFFSET(struct Primitive, index) + 256 * OFFSET(struct Other, suffix);
    case 13: return Max(CastValue(BYTE, a), CastValue(COUNT, b));
    default: return DynamicLinks(GetSuccLink(a), GetPredLink(b));
    }
}
