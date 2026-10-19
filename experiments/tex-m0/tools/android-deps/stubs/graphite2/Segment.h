#pragma once
#include "Font.h"
#ifdef __cplusplus
extern "C" {
#endif
gr_segment *gr_make_seg(const gr_font *, const gr_face *, gr_uint32, const gr_feature_val *, enum gr_encform, const void *, size_t, int);
void gr_seg_destroy(gr_segment *);
unsigned int gr_seg_n_slots(const gr_segment *);
const gr_slot *gr_seg_first_slot(gr_segment *);
float gr_seg_advance_X(const gr_segment *);
float gr_seg_advance_Y(const gr_segment *);
const gr_slot *gr_slot_next_in_segment(const gr_slot *);
float gr_slot_origin_X(const gr_slot *);
float gr_slot_origin_Y(const gr_slot *);
float gr_slot_advance_X(const gr_slot *, const gr_face *, const gr_font *);
float gr_slot_advance_Y(const gr_slot *, const gr_face *, const gr_font *);
unsigned int gr_slot_before(const gr_slot *);
unsigned int gr_slot_after(const gr_slot *);
unsigned short gr_slot_gid(const gr_slot *);
int gr_slot_can_insert_before(const gr_slot *);
#ifdef __cplusplus
}
#endif
