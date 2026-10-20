/* Empty stand-in: xetex-core.h only includes this header; the C code never uses fontconfig types.
 * The Rust side declares the few functions it calls itself (tectonic_bridge_fontconfig::sys). */
#pragma once
typedef struct _FcPattern FcPattern;
