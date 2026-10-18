# Shared UI visual regressions

`crates/ui/tests/visual_regression.rs` runs actual shared components through egui and renders
paired images with the same offscreen wgpu renderer. `visual/fixtures.rs` supplies independent
reference painters with fixed geometry and state colors. References do not call the shared
components or obtain expected rectangles from component responses. These fixtures specify the
shared components' visual contracts; they are not historical screenshots of applications.

## Coverage

| Surface | Paired cases | Dimensions exercised |
| --- | ---: | --- |
| Icon buttons (separate and single-shape frames), tabs, horizontal and vertical split handles | 372 | Light/dark, 1/1.5/2 pixels per point, normal/compact, normal/hover/pressed/disabled/focused; buttons and tabs also selected/selected-pressed |
| Toolbar fitting | 18 | Light/dark, three display scales, all groups visible / a group plus overflow / overflow only |
| Widget palette | 12 | Light/dark, three display scales, normal/compact; all five widget states, both fills, radius, foreground and preserved border/expansion |
| SVG preparation | 12 | Light/dark colors, three display scales, 12/24 point icons; literal recolor and stroke references |

The 414 positive pairs require identical RGBA pixels. The 402 wgpu pairs also require identical
tessellated vertices, indices, texture IDs and clip rectangles. SVG pairs compare the actual
image loader's raster output against literal prepared SVGs. Every SVG must contain visible pixels.

Controls receive real egui pointer movement, pointer press or focus requests; assertions verify
that the intended response state was reached. Disabled controls receive pointer input and must
remain inert. Selection takes precedence over a pointer press. Resize handles exercise a drag
and both split orientations. Compact fixtures test the same visual contracts at smaller sizes.

An additional 36 negative comparisons deliberately change normal fill color, inter-button
spacing or stroke width. Each must differ in both pixels and meshes. No tolerance, mask, crop,
perceptual score or automatic baseline update can turn a changed pixel into a passing result.
Toolbar unit tests separately reject malformed and excessive input, require a reachable overflow
control, and verify that accepted layouts preserve every group and fit the supplied width.

## Running and reviewing

```sh
cargo test -p craft-ui --test visual_regression
CRAFT_UI_VISUAL_DIR=/absolute/path/to/visuals cargo test -p craft-ui --test visual_regression
```

The optional directory receives one subdirectory per suite. Each case writes `-before.png`
(the independent reference), `-after.png` (the production component) and `-difference.png`
(changed pixels in magenta), plus a `comparison.tsv` with dimensions, changed pixel count,
mesh equality and whether a change is expected. Completed rows survive a subsequent failed
assertion. The fixture count and image dimensions are fixed and bounded; the largest control
image is 256 by 144 pixels. Tests fail if a renderer cannot initialize or render.

The test renderer compares both sides on the current machine. It does not assert cross-driver
pixel identity or substitute for application-specific native screenshots, complete themes,
text/font layout, command dispatch, window resizing, accessibility service acceptance, or
cross-platform runtime tests. Application baselines must retain their own source and binary
provenance. Review those captures separately when adopting or changing a shared adapter.
