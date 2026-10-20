#include <unicode/ucnv.h>
UConverter *ucnv_open(const char *name, UErrorCode *err) { (void)name; if (err) *err = U_FILE_ACCESS_ERROR; return 0; }
void ucnv_close(UConverter *c) { (void)c; }
int32_t ucnv_toAlgorithmic(UConverterType t, UConverter *c, char *target, int32_t cap, const char *src, int32_t len, UErrorCode *err) { (void)t;(void)c;(void)target;(void)cap;(void)src;(void)len; if (err) *err = U_UNSUPPORTED_ERROR; return 0; }
