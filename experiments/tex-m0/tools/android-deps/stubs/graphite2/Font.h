#pragma once
#include "Types.h"
#ifdef __cplusplus
extern "C" {
#endif
typedef struct gr_face gr_face; typedef struct gr_font gr_font; typedef struct gr_feature_ref gr_feature_ref;
typedef struct gr_feature_val gr_feature_val; typedef struct gr_segment gr_segment; typedef struct gr_slot gr_slot;
typedef struct gr_face_ops { size_t size; const void *(*get_table)(const void *, unsigned int, size_t *); void (*release_table)(const void *, const void *); } gr_face_ops;
enum gr_face_options { gr_face_preloadAll = 4 };
gr_face *gr_make_face_with_ops(const void *, const gr_face_ops *, unsigned int);
void gr_face_destroy(gr_face *);
const gr_feature_ref *gr_face_find_fref(const gr_face *, gr_uint32);
gr_feature_val *gr_face_featureval_for_lang(const gr_face *, gr_uint32);
int gr_fref_set_feature_value(const gr_feature_ref *, gr_uint16, gr_feature_val *);
void gr_featureval_destroy(gr_feature_val *);
void gr_label_destroy(void *);
#ifdef __cplusplus
}
#endif
