#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#define FRIBIDI "ribidi"
#define FRIBIDI_NAME "Iglunix RiBidi"
#define FRIBIDI_VERSION "1.0.10"
#define FRIBIDI_MAJOR_VERSION 1
#define FRIBIDI_MINOR_VERSION 0
#define FRIBIDI_MICRO_VERSION 10

/* always true because we don't implement anything */
#define FRIBIDI_LEVEL_IS_RTL(level) 1

#define FRIBIDI_PAR_ON 'n'
#define FRIBIDI_PAR_LTR 'L'

#define FRIBIDI_FLAGS_DEFAULT 0
#define FRIBIDI_FLAGS_ARABIC 0

typedef uint32_t FriBidiChar;
typedef size_t FriBidiStrIndex;
typedef bool fribidi_boolean;
typedef size_t FriBidiCharType;
typedef uint32_t FriBidiLevel;
typedef FriBidiChar FriBidiBracketType;
typedef uint32_t FriBidiParType;
typedef size_t FriBidiJoiningType;
typedef size_t FriBidiArabicProp;
typedef uint32_t FriBidiFlags;

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
void fribidi_get_bracket_types(FriBidiChar const *, FriBidiStrIndex, FriBidiCharType const *, FriBidiBracketType *);
FriBidiLevel fribidi_get_par_embedding_levels_ex(
	FriBidiCharType const *,
	FriBidiBracketType const *,
	FriBidiStrIndex len,
	FriBidiParType *pbase_dir,
	FriBidiLevel *embedding_levels
);


FriBidiJoiningType fribidi_get_joining_type(FriBidiChar);
void fribidi_get_joining_types(FriBidiChar const *, FriBidiStrIndex, FriBidiJoiningType *);

FriBidiLevel fribidi_reorder_line(
	FriBidiFlags,
	FriBidiCharType const *,
	FriBidiStrIndex,
	FriBidiStrIndex,
	FriBidiParType const *,
	FriBidiLevel *,
	FriBidiChar *,
	FriBidiStrIndex *
);

void fribidi_join_arabic(
	FriBidiCharType const *,
	FriBidiStrIndex,
	FriBidiLevel const *,
	FriBidiArabicProp *
);

void fribidi_shape(
	FriBidiFlags,
	FriBidiLevel const *,
	FriBidiStrIndex,
	FriBidiArabicProp *,
	FriBidiChar *
);
