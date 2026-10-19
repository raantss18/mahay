#include "graphite2/Segment.h"
gr_face *gr_make_face_with_ops(const void *a, const gr_face_ops *b, unsigned int c) { (void)a; (void)b; (void)c; return 0; }
void gr_face_destroy(gr_face *f) { (void)f; }
const gr_feature_ref *gr_face_find_fref(const gr_face *f, gr_uint32 t) { (void)f; (void)t; return 0; }
gr_feature_val *gr_face_featureval_for_lang(const gr_face *f, gr_uint32 l) { (void)f; (void)l; return 0; }
int gr_fref_set_feature_value(const gr_feature_ref *r, gr_uint16 v, gr_feature_val *f) { (void)r; (void)v; (void)f; return 0; }
void gr_featureval_destroy(gr_feature_val *f) { (void)f; }
void gr_label_destroy(void *p) { (void)p; }
gr_segment *gr_make_seg(const gr_font *a, const gr_face *b, gr_uint32 c, const gr_feature_val *d, enum gr_encform e, const void *f, size_t g, int h) { (void)a;(void)b;(void)c;(void)d;(void)e;(void)f;(void)g;(void)h; return 0; }
void gr_seg_destroy(gr_segment *s) { (void)s; }
unsigned int gr_seg_n_slots(const gr_segment *s) { (void)s; return 0; }
const gr_slot *gr_seg_first_slot(gr_segment *s) { (void)s; return 0; }
float gr_seg_advance_X(const gr_segment *s) { (void)s; return 0; }
float gr_seg_advance_Y(const gr_segment *s) { (void)s; return 0; }
const gr_slot *gr_slot_next_in_segment(const gr_slot *s) { (void)s; return 0; }
float gr_slot_origin_X(const gr_slot *s) { (void)s; return 0; }
float gr_slot_origin_Y(const gr_slot *s) { (void)s; return 0; }
float gr_slot_advance_X(const gr_slot *s, const gr_face *f, const gr_font *t) { (void)s;(void)f;(void)t; return 0; }
float gr_slot_advance_Y(const gr_slot *s, const gr_face *f, const gr_font *t) { (void)s;(void)f;(void)t; return 0; }
unsigned int gr_slot_before(const gr_slot *s) { (void)s; return 0; }
unsigned int gr_slot_after(const gr_slot *s) { (void)s; return 0; }
unsigned short gr_slot_gid(const gr_slot *s) { (void)s; return 0; }
int gr_slot_can_insert_before(const gr_slot *s) { (void)s; return 0; }
