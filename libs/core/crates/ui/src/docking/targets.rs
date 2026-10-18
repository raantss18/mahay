use egui::{Pos2, Rect, pos2};

use super::{Placement, Zone};

/// One visible group, supplied in back-to-front order to [`resolve_drop`]. An empty tab strip
/// or a group that rejects tabs may still accept side splits, e.g. a protected document canvas.
#[derive(Clone, Debug)]
pub struct DropTarget<P> {
    pub anchor: P,
    pub rect: Rect,
    pub tab_strip: Option<Rect>,
    pub tabs: Vec<(P, Rect)>,
    pub allow_splits: bool,
    pub accepts_tabs: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DropProposal<P> {
    pub anchor: P,
    pub placement: Placement<P>,
    pub preview: Rect,
    pub insertion: Option<Rect>,
}

pub(super) fn usable(rect: Rect) -> bool {
    rect.is_finite() && rect.width() > 0.0 && rect.height() > 0.0 && rect.width() <= 1_000_000.0 && rect.height() <= 1_000_000.0
}

/// Tab header is the center; otherwise the nearest quarter-width/height edge wins.
/// Outside, non-finite, empty and inverted rectangles have no target.
pub fn drop_zone(rect: Rect, tab_height: f32, pointer: Pos2) -> Option<Zone> {
    if !usable(rect) || !pointer.is_finite() || !rect.contains(pointer) || !tab_height.is_finite() || tab_height < 0.0 {
        return None;
    }
    if pointer.y < rect.top() + tab_height.min(rect.height()) {
        return Some(Zone::Center);
    }
    let x = (pointer.x - rect.left()) / rect.width();
    let y = (pointer.y - rect.top()) / rect.height();
    let distances = [(x, Zone::Left), (1.0 - x, Zone::Right), (y, Zone::Top), (1.0 - y, Zone::Bottom)];
    let (distance, zone) = distances.into_iter().min_by(|a, b| a.0.total_cmp(&b.0))?;
    Some(if distance > 0.25 { Zone::Center } else { zone })
}

/// Preview of the region occupied after a split or group merge.
pub fn zone_rect(rect: Rect, zone: Zone) -> Option<Rect> {
    if !usable(rect) {
        return None;
    }
    let center = rect.min + rect.size() * 0.5;
    Some(match zone {
        Zone::Center => rect,
        Zone::Left => Rect::from_min_max(rect.min, pos2(center.x, rect.bottom())),
        Zone::Right => Rect::from_min_max(pos2(center.x, rect.top()), rect.max),
        Zone::Top => Rect::from_min_max(rect.min, pos2(rect.right(), center.y)),
        Zone::Bottom => Rect::from_min_max(pos2(rect.left(), center.y), rect.max),
    })
}

/// Resolve a pointer against visible groups. The frontmost containing group occludes groups
/// behind it even if it rejects the drop. A strip reorders by midpoint; the body of the source
/// group is a no-op at center. Floating groups normally set `allow_splits` to false.
pub fn resolve_drop<P: Clone + Eq>(panel: &P, pointer: Pos2, targets: &[DropTarget<P>]) -> Option<DropProposal<P>> {
    if !pointer.is_finite() || targets.len() > super::MAX_PANELS {
        return None;
    }
    let target = targets.iter().rev().find(|target| usable(target.rect) && target.rect.contains(pointer))?;
    if target.tabs.len() > super::MAX_PANELS {
        return None;
    }
    let anchor = if &target.anchor == panel { target.tabs.iter().find(|(p, _)| p != panel).map(|(p, _)| p)? } else { &target.anchor };
    let strip = target.tab_strip.filter(|r| usable(*r) && r.contains(pointer));
    if let Some(strip) = strip {
        if !target.accepts_tabs {
            return None;
        }
        let before = target.tabs.iter().filter(|(_, r)| usable(*r)).find(|(_, r)| pointer.x < r.center().x);
        let before_panel = before.map(|(p, _)| p.clone());
        let x = before.map(|(_, r)| r.left()).unwrap_or_else(|| {
            target.tabs.iter().filter(|(_, r)| usable(*r)).map(|(_, r)| r.right()).fold(strip.left(), f32::max).min(strip.right())
        });
        let insertion = Rect::from_min_max(pos2((x - 1.0).max(strip.left()), strip.top()), pos2((x + 1.0).min(strip.right()), strip.bottom()));
        return Some(DropProposal {
            anchor: anchor.clone(),
            placement: Placement::Tab { before: before_panel },
            preview: target.rect,
            insertion: Some(insertion),
        });
    }
    let zone = if target.allow_splits { drop_zone(target.rect, 0.0, pointer)? } else { Zone::Center };
    if zone == Zone::Center {
        if !target.accepts_tabs || target.tabs.iter().any(|(p, _)| p == panel) {
            return None;
        }
    } else if !target.allow_splits {
        return None;
    }
    Some(DropProposal { anchor: anchor.clone(), placement: Placement::Split(zone), preview: zone_rect(target.rect, zone)?, insertion: None })
}
