# Panel docking

`craft_ui::docking` lets an application move the same panel between docked tabs, side splits and
floating groups. The shared layer owns placement and interaction; the application owns each
panel's contents, commands, document state and persistence policy.

## Model and transactions

`Layout<P>` contains an optional root `Node<P>` and floating tab groups. A node can be:

- `Tabs`: ordered panel identities with one active index.
- `Split`: horizontal or vertical children, with a ratio or fixed first/second size.
- `Stack`: accordion entries with individual open state and optional content height.

Choose a stable, nonlocalized panel ID type implementing `Clone + Eq + Hash`. Rendering also
requires `Debug + Send + Sync + 'static`. A panel identity occurs at most once in the layout.
Keep transient document/sequence tabs separate when they represent documents inside one panel.

```rust
use craft_ui::docking::{Action, Floating, Layout, Node, Placement};

let mut layout = Layout {
    root: Some(Node::Tabs { panels: vec!["layers", "properties"], active: 0 }),
    floating: vec![Floating {
        panels: vec!["brushes"], active: 0, rect: [40.0, 80.0, 240.0, 300.0],
    }],
};
layout.apply(Action::Move {
    panel: "brushes", anchor: "layers",
    placement: Placement::Tab { before: Some("properties") },
})?;
# Ok::<(), craft_ui::docking::DockError>(())
```

Every transaction validates the original layout before cloning or recursive mutation. It checks
the resulting layout before replacing the original. Missing or self targets, duplicate IDs,
invalid active indices, excessive depth/count and invalid geometry return `DockError`; they do
not remove the source panel. Tab removals retain the selected panel by identity when possible.
Empty source groups collapse. Moving within an accordion preserves its entry state and height;
unaffected split sizing and accordion entries remain unchanged.

`OpenAt` adds a currently absent registered panel at an explicit tab or side destination. `Open`
is the simpler group-reveal operation. Registry checks remain app-owned: the shared model cannot
know whether an ID names an actual panel or whether its content is supported by the application.

Use `apply_with_permissions` to prevent moving/closing a document canvas or merging a panel into
it. `Permissions::PROTECTED` disallows these operations while allowing side splits around it.
An app that requires a permanently docked panel also checks that policy before accepting a
transaction; an empty shared root is valid when all panels float.

## Rendering and interaction

Existing renderers can convert their layout into the shared model for transactions and retain
their own panel chrome. `DropTarget` describes visible body/tab rectangles and whether a group
accepts tabs or splits. Supply targets back-to-front, including floating-window layer order.
`resolve_drop` returns a tab insertion or side/center placement and its preview rectangle.
The frontmost containing group blocks targets underneath even when it rejects a drop.

The optional `DockArea` renderer handles tab fitting and overflow, drag/drop previews,
Escape/focus-loss cancellation, split/accordion dividers and floating-group resize grips. It renders a
snapshot and returns `DockOutput.actions`; apply them through the application's command path.
With `serde` enabled, `Action` also serializes so pointer, keyboard and control-channel requests
can share that command path, including splitter changes, tab reorder and floating bounds.
Panel body callbacks receive stable global egui IDs, so moving a panel does not reset its widget
memory. The application should still keep durable panel/document data outside transient egui
widget memory.

`DockStyle` supplies colors, font, spacing and localized action labels. `show_with_limits` also
accepts per-panel minimum/maximum sizes. Visible geometry clamps to the available area; a narrow
viewport relaxes minima proportionally. Persisted preferred sizes are unchanged by this drawing
clamp. A divider gesture produces an explicit resize action.

Use `begin_drag` with the same area ID to carry an existing tab/header gesture into `DockArea`
when an app switches from its default arrangement to a customizable workspace.

Floating groups are egui areas in one viewport. Native child windows, cross-viewport drag
transport, OS window ownership and multi-monitor coordinate conversion remain the host's
responsibility. The model does not instantiate or reparent native windows.

## Persistence and visibility

Enable `craft-ui`'s optional `serde` feature to serialize the shared model, or retain an existing
schema through an adapter. The serialization feature does not choose a filesystem location or
write preferences. Applications validate panel IDs, payload size and schema version before
committing loaded state. Bound recursive deserialization/conversion in the app as well as using
`Layout::validate`; the validator cannot retroactively protect an unbounded parser.

To hide a panel without forgetting its placement, capture `Layout::location`, then close it.
Keep the resulting `Location` with the app's visibility state. `restore` uses stable neighbor
IDs, restores captured fixed sizing/floating bounds and recreates a previously closed accordion
group. If a neighbor was removed, it returns an unchanged layout and `MissingTarget`; choose an
explicit fallback such as a default side or remembered floating rectangle. Restoring multiple
panels closed from the same group in reverse close order preserves neighbor relationships.

## Acceptance

The shared tests cover lossless moves, tab reorder, stale targets, atomic errors, protected
canvases, accordion state, fixed sizing, hidden-panel restoration, bounded malformed layouts,
and a deterministic sequence of 800 panel moves. Pointer tests exercise dragging a floating
panel into the main tab strip and canceling a drag.

`tests/docking_workflows.rs` renders a document and populated Layers/Properties/Brushes panels,
then uses actual pointer events to merge, detach and redock Brushes. It checks retained values
and widget identity at two widths, two display scales and both egui themes. To retain captures:

```sh
CRAFT_UI_DOCK_VISUAL_DIR=/absolute/output/path cargo test -p craft-ui --test docking_workflows --all-features
```

These are offscreen egui checks. An adopting application still needs command, content, workspace
reload, theme and input tests using its own renderer and persistence adapter.

## Application chrome and activation

`DockArea::show_customized` accepts one mutable `DockContent<P>` adapter, with body,
panel-menu, group-header and label methods. Apps queue commands in these callbacks and
apply them after rendering the readonly snapshot. `group_header_width` reserves a bounded
trailing portion of the existing tab strip; its child UI is clipped to that region.
Collapsed accordion entries still have panel menus; collapsing tab groups is host policy.

`label` returns `WidgetText`, including an application-shaped galley, used for tab width,
painting and menu labels. The separate logical label callback supplies tab accessibility
text. `action_label` independently shapes the localized stock Float/Move/Close menu
text without receiving a panel identity. Its default returns unchanged logical `WidgetText`;
custom visual text never replaces the action's logical accessibility label. Panel targets
and overflow entries continue to use `label`. Original `show` and `show_with_limits` call
sites remain supported.

Apply `Action::Activate` in the host, then use `DockArea::raise_panel(panel)` on the next
frame to raise the owning in-viewport floating layer and focus the panel tab. This request
does not mutate the serialized layout or persist z-order. Floating area identity still
derives from the first panel; native-window identity remains host-owned.

Floating groups have a distinct 12-point grip in the existing tab strip. Moving it emits
`MoveFloating` for the whole group; dragging an individual tab detaches just that panel.
`DockOutput::group_drag` and `group_drop` expose the group identity and release position
for app-owned atomic group docking policies. Escape/lost focus restores the group's
starting rectangle. No generic group docking or app command registry is inferred.

Hosts supply overlay rectangles via `blocked_rects` before showing the surface. These
block drop previews, release actions and float fallback at the pointer, including overlays
appearing mid-drag. Hosts must provide current overlay geometry or cancel docking when
modal state prevents interaction; later-drawn overlays cannot be discovered retroactively.

`DockArea::chrome_has_focus(ctx, layout)` classifies focus on rendered dock tabs,
collapse buttons, overflow, split/stack dividers, floating resize corners and group
grips. It excludes panel content and custom header/menu widgets and rejects invalid
layouts. Hosts can permit global commands from chrome while reserving Enter, Space,
Tab, Escape and arrow keys for activation, traversal, cancellation and resizing.
Content widgets retain their usual keyboard ownership. The renderer owns the IDs;
hosts do not need to reconstruct them.
