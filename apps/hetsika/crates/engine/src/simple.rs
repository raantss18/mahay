//! The Simple workspace's story model (decision D27): the first video track of the active sequence
//! is a *story*, a row of clips in order. Add a clip at the end, cut at the playhead, trim to the
//! playhead, remove, move earlier or later, change speed, add a title. Each is one engine command
//! (so one undo step), nothing here edits the project behind the engine's back.

use serde_json::{Value, json};

use hetsika_project::{ClipId, ItemId};
use hetsika_time::{TICKS_PER_SECOND, Tick};

use crate::{EngineError, Result, Session};

/// One clip of the story, for the strip the Simple workspace draws.
#[derive(Clone, Debug, PartialEq)]
pub struct StoryClip {
    pub id: ClipId,
    pub item: ItemId,
    pub name: String,
    pub start: Tick,
    pub duration: Tick,
    /// 1.0 = normal speed.
    pub speed: f64,
    /// A title or other graphic, not footage.
    pub graphic: bool,
}

impl StoryClip {
    pub fn end(&self) -> Tick {
        self.start + self.duration
    }

    pub fn seconds(&self) -> f64 {
        self.duration.0 as f64 / TICKS_PER_SECOND as f64
    }
}

/// The clips of the story (the first video track), in order.
pub fn story(s: &Session) -> Vec<StoryClip> {
    let Some(seq) = s.active_sequence() else { return Vec::new() };
    let Some(track) = seq.video_tracks.first() else { return Vec::new() };
    let mut clips: Vec<StoryClip> = track
        .items
        .iter()
        .map(|c| StoryClip { id: c.id, item: c.item, name: c.name.clone(), start: c.start, duration: c.duration, speed: c.speed, graphic: c.graphic.is_some() })
        .collect();
    clips.sort_by_key(|c| c.start);
    clips
}

/// Where the story ends (zero when it is empty).
pub fn story_end(s: &Session) -> Tick {
    story(s).last().map_or(Tick::ZERO, StoryClip::end)
}

/// The story clip under the playhead, if any.
pub fn clip_at_playhead(s: &Session) -> Option<StoryClip> {
    let t = s.playhead();
    story(s).into_iter().find(|c| c.start <= t && t < c.end())
}

/// The selected story clip (the first of the selection that is in the story).
pub fn selected(s: &Session) -> Option<StoryClip> {
    let story = story(s);
    s.state.selection.iter().find_map(|id| story.iter().find(|c| c.id == *id).cloned())
}

fn need_sequence(s: &Session) -> Result<()> {
    s.active_sequence().map(|_| ()).ok_or(EngineError::NoSequence)
}

/// Select `clip` and put the playhead at its start.
pub fn select(s: &mut Session, clip: &StoryClip) -> Result<()> {
    s.execute("timeline.select", json!({"clips": [clip.id.0]}))?;
    s.set_playhead(clip.start);
    // `set_playhead` may have re-selected what is under it: the clip is what the caller picked.
    s.execute("timeline.select", json!({"clips": [clip.id.0]}))?;
    Ok(())
}

/// Put `item` (a project item: footage, a picture) at the end of the story, picture and sound.
pub fn append(s: &mut Session, item: ItemId) -> Result<()> {
    need_sequence(s)?;
    let at = story_end(s);
    s.execute("timeline.place", json!({"item": item.0, "track": "V1", "time": at.0, "insert": false}))?;
    Ok(())
}

/// Cut the story at the playhead (the clip under it becomes two).
pub fn split_at_playhead(s: &mut Session) -> Result<()> {
    need_sequence(s)?;
    let clip = clip_at_playhead(s).ok_or_else(|| EngineError::Other("the playhead is not on a clip".into()))?;
    s.execute("timeline.select", json!({"clips": [clip.id.0]}))?;
    let r = s.execute("sequence.addEdit", json!({}))?;
    if r.get("cuts").and_then(Value::as_u64) == Some(0) {
        return Err(EngineError::Other("nothing to cut here: move the playhead inside a clip".into()));
    }
    Ok(())
}

/// Remove the selected clip and close the gap.
pub fn remove_selected(s: &mut Session) -> Result<()> {
    need_sequence(s)?;
    let clip = selected(s).ok_or_else(|| EngineError::Other("select a clip first".into()))?;
    s.execute("edit.rippleDelete", json!({"clips": [clip.id.0]}))?;
    Ok(())
}

/// Keep the selected clip from the playhead on (cut its start off) and close the gap.
pub fn trim_start_to_playhead(s: &mut Session) -> Result<()> {
    need_sequence(s)?;
    let clip = selected(s).ok_or_else(|| EngineError::Other("select a clip first".into()))?;
    s.execute("edit.rippleDeleteHead", json!({"clips": [clip.id.0]}))?;
    Ok(())
}

/// Keep the selected clip up to the playhead (cut its end off) and close the gap.
pub fn trim_end_to_playhead(s: &mut Session) -> Result<()> {
    need_sequence(s)?;
    let clip = selected(s).ok_or_else(|| EngineError::Other("select a clip first".into()))?;
    let t = s.playhead();
    if !(clip.start < t && t < clip.end()) {
        return Err(EngineError::Other("move the playhead inside the clip".into()));
    }
    s.execute("timeline.select", json!({"clips": [clip.id.0]}))?;
    s.execute("sequence.addEdit", json!({}))?;
    // The second half starts at the cut.
    let tail = story(s).into_iter().find(|c| c.start == t).ok_or_else(|| EngineError::Other("the cut was not made".into()))?;
    s.execute("edit.rippleDelete", json!({"clips": [tail.id.0]}))?;
    s.execute("timeline.select", json!({"clips": [clip.id.0]}))?;
    Ok(())
}

/// Swap the selected clip with its neighbour: `-1` earlier, `1` later.
pub fn move_selected(s: &mut Session, direction: i32) -> Result<()> {
    need_sequence(s)?;
    let story = story(s);
    let at = story.iter().position(|c| s.state.selection.contains(&c.id)).ok_or_else(|| EngineError::Other("select a clip first".into()))?;
    let other = match direction.signum() {
        -1 if at > 0 => at - 1,
        1 if at + 1 < story.len() => at + 1,
        _ => return Ok(()),
    };
    let (a, b) = (&story[at], &story[other]);
    // The earlier clip of the pair goes last, the later one first; the pair keeps its span.
    let first = a.start.min(b.start);
    let (early, late) = if a.start < b.start { (a, b) } else { (b, a) };
    // Park the early clip past the end so the late one can take the start, then put it after.
    let park = story_end(s) + Tick(TICKS_PER_SECOND);
    let (track, _) = (json!("V1"), ());
    s.execute("timeline.move", json!({"moves": [{"clip": early.id.0, "track": track, "time": park.0}], "insert": false, "linked": false}))?;
    s.execute("timeline.move", json!({"moves": [{"clip": late.id.0, "track": track, "time": first.0}], "insert": false, "linked": false}))?;
    let after = first + late.duration;
    s.execute("timeline.move", json!({"moves": [{"clip": early.id.0, "track": track, "time": after.0}], "insert": false, "linked": false}))?;
    Ok(())
}

/// Set the speed of the selected clip, in percent (100 = normal); the clips after it follow.
pub fn set_speed(s: &mut Session, percent: f64) -> Result<()> {
    need_sequence(s)?;
    if !percent.is_finite() || !(1.0..=1000.0).contains(&percent) {
        return Err(EngineError::Other("speed must be between 1% and 1000%".into()));
    }
    let clip = selected(s).ok_or_else(|| EngineError::Other("select a clip first".into()))?;
    s.execute("clip.speedDuration", json!({"clips": [clip.id.0], "speed": percent, "ripple": true}))?;
    Ok(())
}

/// Add a title with `text` at the playhead, as its own clip above the story.
pub fn add_title(s: &mut Session, text: &str) -> Result<()> {
    need_sequence(s)?;
    let text = text.trim();
    if text.is_empty() {
        return Err(EngineError::Other("write the title first".into()));
    }
    s.execute("graphics.newText", json!({"text": text.chars().take(200).collect::<String>()}))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A story of three demo clips in a fresh sequence.
    fn session() -> (Session, Vec<ItemId>) {
        let mut s = Session::default();
        s.execute("file.newSequence", json!({"name": "Story"})).unwrap();
        s.execute("file.importDemoFootage", json!({})).unwrap();
        let items: Vec<ItemId> = s.project.items.iter().filter(|(_, it)| matches!(it.kind, hetsika_project::ItemKind::Media(_))).map(|(id, _)| *id).take(3).collect();
        assert_eq!(items.len(), 3, "demo footage imported");
        for it in &items {
            append(&mut s, *it).unwrap();
        }
        (s, items)
    }

    /// Select the `n`th clip of the story.
    fn pick(s: &mut Session, n: usize) {
        let clip = story(s)[n].clone();
        select(s, &clip).unwrap();
    }

    fn names(s: &Session) -> Vec<ItemId> {
        story(s).iter().map(|c| c.item).collect()
    }

    #[test]
    fn clips_are_appended_back_to_back() {
        let (s, items) = session();
        let story = story(&s);
        assert_eq!(story.len(), 3);
        assert_eq!(story[0].start, Tick::ZERO);
        for w in story.windows(2) {
            assert_eq!(w[0].end(), w[1].start, "no gap between clips");
        }
        assert_eq!(names(&s), items);
    }

    #[test]
    fn cutting_at_the_playhead_makes_two_clips_with_the_same_total() {
        let (mut s, _) = session();
        let total = story_end(&s);
        let first = story(&s)[0].clone();
        s.set_playhead(Tick(first.duration.0 / 2));
        split_at_playhead(&mut s).unwrap();
        assert_eq!(story(&s).len(), 4);
        assert_eq!(story_end(&s), total);
        // Outside any clip it says so instead of doing nothing.
        s.set_playhead(total + Tick(TICKS_PER_SECOND * 5));
        assert!(split_at_playhead(&mut s).is_err());
    }

    #[test]
    fn removing_a_clip_closes_the_gap() {
        let (mut s, items) = session();
        let second = story(&s)[1].clone();
        select(&mut s, &second).unwrap();
        remove_selected(&mut s).unwrap();
        assert_eq!(names(&s), vec![items[0], items[2]]);
        let story = story(&s);
        assert_eq!(story[0].end(), story[1].start);
    }

    #[test]
    fn trimming_to_the_playhead_keeps_the_rest_of_the_story_together() {
        let (mut s, _) = session();
        let before = story(&s);
        let second = before[1].clone();
        select(&mut s, &second).unwrap();
        // End: keep the first half of the second clip.
        s.set_playhead(second.start + Tick(second.duration.0 / 2));
        trim_end_to_playhead(&mut s).unwrap();
        let after = story(&s);
        assert_eq!(after.len(), 3);
        assert!((after[1].duration.0 - second.duration.0 / 2).abs() < TICKS_PER_SECOND / 10);
        assert_eq!(after[1].end(), after[2].start, "the next clip followed");
        // Start: keep the second half of the first clip.
        select(&mut s, &after[0]).unwrap();
        s.set_playhead(Tick(after[0].duration.0 / 2));
        s.execute("timeline.select", json!({"clips": [after[0].id.0]})).unwrap();
        trim_start_to_playhead(&mut s).unwrap();
        let last = story(&s);
        assert_eq!(last.len(), 3);
        assert_eq!(last[0].start, Tick::ZERO);
        assert!(last[0].duration < after[0].duration);
    }

    #[test]
    fn a_clip_moves_earlier_and_later_keeping_the_story_whole() {
        let (mut s, items) = session();
        let total = story_end(&s);
        pick(&mut s, 0);
        move_selected(&mut s, 1).unwrap();
        assert_eq!(names(&s), vec![items[1], items[0], items[2]]);
        assert_eq!(story_end(&s), total);
        for w in story(&s).windows(2) {
            assert_eq!(w[0].end(), w[1].start);
        }
        // At the end, "later" does nothing (and is not an error).
        pick(&mut s, 2);
        move_selected(&mut s, 1).unwrap();
        assert_eq!(names(&s), vec![items[1], items[0], items[2]]);
        // And back.
        pick(&mut s, 1);
        move_selected(&mut s, -1).unwrap();
        assert_eq!(names(&s), vec![items[0], items[1], items[2]]);
    }

    #[test]
    fn speed_changes_the_duration_and_the_later_clips_follow() {
        let (mut s, _) = session();
        let before = story(&s);
        select(&mut s, &before[0]).unwrap();
        set_speed(&mut s, 200.0).unwrap();
        let after = story(&s);
        assert!((after[0].duration.0 as f64 - before[0].duration.0 as f64 / 2.0).abs() < TICKS_PER_SECOND as f64 / 10.0);
        assert_eq!(after[0].end(), after[1].start);
        assert!(set_speed(&mut s, 0.0).is_err());
        assert!(set_speed(&mut s, f64::NAN).is_err());
    }

    #[test]
    fn a_title_is_added_above_the_story_and_empty_text_is_refused() {
        let (mut s, _) = session();
        let clips = story(&s).len();
        add_title(&mut s, "Salama").unwrap();
        assert_eq!(story(&s).len(), clips, "the title is not part of the story track");
        assert!(s.active_sequence().unwrap().video_tracks.len() >= 2);
        assert!(add_title(&mut s, "   ").is_err());
    }

    #[test]
    fn without_a_sequence_everything_refuses_and_nothing_panics() {
        let mut s = Session::default();
        assert!(story(&s).is_empty());
        assert!(split_at_playhead(&mut s).is_err());
        assert!(remove_selected(&mut s).is_err());
        assert!(move_selected(&mut s, 1).is_err());
        assert!(set_speed(&mut s, 100.0).is_err());
        assert!(add_title(&mut s, "x").is_err());
    }
}
