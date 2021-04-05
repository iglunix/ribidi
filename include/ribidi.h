#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

typedef uint32_t FriBidiChar;
typedef size_t FriBidiStrIndex;
typedef bool fribidi_boolean;
typedef size_t FriBidiCharType;
typedef uint32_t FriBidiLevel;
typedef FriBidiChar FriBidiBracketType;
typedef uint32_t FriBidiParType;
typedef size_t FriBidiJoiningType;
typedef size_t FriBidiArabicProp;

FriBidiCharType fribidi_get_bidi_type(FriBidiChar);
void fribidi_get_bidi_types(FriBidiChar *, FriBidiStrIndex, FriBidiCharType *);
char const *fribidi_get_bidi_type_name(FriBidiCharType);

fribidi_boolean fribidi_log2vis(
	FriBidiChar const *,
	FriBidiStrIndex,
	FriBidiCharType const *,
	FriBidiChar *,
	FriBidiStrIndex *,
	FriBidiStrIndex *,
	FriBidiLevel *
);

FriBidiBracketType fribidi_get_bracket(FriBidiChar);
FriBidiLevel fribidi_get_par_embedding_levels_ex(
	FriBidiCharType const *,
	FriBidiBracketType const *,
	FriBidiStrIndex len,
	FriBidiParType *pbase_dir,
	FriBidiLevel *embedding_levels
);


FriBidiJoiningType fribidi_get_joining_type(FriBidiChar);
void fribidi_get_joining_types(FriBidiChar const *, FriBidiStrIndex, FriBidiJoiningType *);