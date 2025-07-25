# Notes, References to good examples

## Button

See `button.rs`.

Also see https://github.com/bevyengine/bevy/blob/latest/examples/ui/button.rs

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

For a Component, such as `OpenClose`, that needs mouse interaction, include the following in the component declaration:
 * `#[require(Interaction)]`

To debounce a mouse Press, use a resource to track the timing of the last Press. See DebounceMousePress

## Plugins

We used plugins for modularity, such as `TextFieldPlugin` in `text_field.rs`.

## Keyboard handling

See https://github.com/bevyengine/bevy/blob/main/examples/input/text_input.rs

See :

* fn `keyboard_input`(), which also accesses the single Text entity with a CursorTimer (ie, it's selected).

## Events

We want our `TextField` to signal any changes through an event. We use that to wire up between Edit and Display panels.

Question: Do we have to have a big dispatch on the events, based on id, or do we have a way for Query<> to help us?
Perhaps we can pass in a closure to TextField, and that's responsible for firing the right event.

See https://taintedcoders.com/bevy/events and https://github.com/bevyengine/bevy/blob/latest/examples/ecs/event.rs

## Children access in system

See react_to_content_update() in button.rs.