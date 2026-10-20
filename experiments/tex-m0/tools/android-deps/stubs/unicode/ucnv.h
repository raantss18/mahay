/* The NDK's libicu.so (API 31+) exports no converter API (ucnv_*). This header declares just
 * what XeTeX uses; the stub implementation (ucnv_stub.c) reports every encoding as unavailable,
 * so \XeTeXinputencoding to a legacy charset fails while UTF-8 input (the default) works. */
#pragma once
#include <unicode/utypes.h>
#ifdef __cplusplus
extern "C" {
#endif
typedef struct UConverter UConverter;
typedef enum { UCNV_UTF32_BigEndian = 7, UCNV_UTF32_LittleEndian = 8 } UConverterType;
UConverter *ucnv_open(const char *name, UErrorCode *err);
void ucnv_close(UConverter *c);
int32_t ucnv_toAlgorithmic(UConverterType t, UConverter *c, char *target, int32_t cap, const char *src, int32_t len, UErrorCode *err);
#ifdef __cplusplus
}
#endif
