# Notes, References to good examples

## Timer

See https://taintedcoders.com/bevy/timers

See `text_field.rs`:

* struct CursorTimer
* fn tick_cursor()

## Mouse Handling

See https://taintedcoders.com/bevy/ui#reacting-to-a-button-press

See `text_field.rs`:
 * fn `mouse_handling`() with `*interaction` of `Pressed`, `Hovered`, `None`

See `text_field_builder.rs`:
 * An `Interaction` component is required by struct `TextField`, needed to pick up mouse events

## Plugins

We used plugins for modularity, such as `TextFieldPlugin` in `text_field.rs`.

## Keyboard handling

See https://github.com/bevyengine/bevy/blob/main/examples/input/text_input.rs
