/* Stub of the graphite2 headers: only what Tectonic and its harfbuzz use. Graphite fonts are
 * unsupported (the functions return failure); OpenType/AAT shaping is unaffected. */
#pragma once
#include <stddef.h>
#include <stdint.h>
typedef uint8_t gr_uint8; typedef uint16_t gr_uint16; typedef uint32_t gr_uint32; typedef int16_t gr_int16;
enum gr_encform { gr_utf8 = 1, gr_utf16 = 2, gr_utf32 = 4 };
