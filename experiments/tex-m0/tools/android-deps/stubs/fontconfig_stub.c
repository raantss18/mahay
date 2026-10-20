/* Minimal stand-in for fontconfig on Android (there is none): reports "no system fonts".
 * Tectonic finds bundle fonts by file name through its own I/O layer, so this is enough for
 * documents that load fonts from the bundle. Only the symbols tectonic_bridge_fontconfig uses. */
#include <stdarg.h>
#include <stdlib.h>
typedef int FcBool;
typedef struct { int nfont; int sfont; void **fonts; } FcFontSet;
int FcInit(void) { return 1; }
void *FcConfigGetCurrent(void) { static int x; return &x; }
void *FcNameParse(const char *n) { (void)n; return calloc(1, 16); }
void *FcObjectSetBuild(const char *f, ...) { (void)f; return calloc(1, 16); }
void *FcObjectSetCreate(void) { return calloc(1, 16); }
FcBool FcObjectSetAdd(void *os, const char *o) { (void)os; (void)o; return 1; }
void FcObjectSetDestroy(void *os) { free(os); }
void FcPatternReference(void *p) { (void)p; }
void FcPatternDestroy(void *p) { free(p); }
FcFontSet *FcFontList(void *c, void *p, void *os) { (void)c; (void)p; (void)os; return calloc(1, sizeof(FcFontSet)); }
void FcFontSetDestroy(FcFontSet *fs) { free(fs); }
int FcPatternGetString(void *p, const char *o, int n, const char **s) { (void)p; (void)o; (void)n; (void)s; return 1; /* FcResultNoMatch */ }
int FcPatternGetInteger(void *p, const char *o, int n, int *i) { (void)p; (void)o; (void)n; (void)i; return 1; }
