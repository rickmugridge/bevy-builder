use bevy::app::{App, Plugin};
use bevy::color::palettes::basic::{GREEN, RED};
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;
use crate::builder::text_field_builder::TextField;

pub struct TextFieldPlugin;

impl Plugin for TextFieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                react_to_deselect,
                mouse_handling,
                tick_cursor,
                keyboard_input,
            ),
        )
        .add_event::<TextFieldDeselected>();
    }
}

#[derive(Component, Debug)]
struct CursorTimer {
    timer: Timer,
    on: bool,
    position: usize,
}

impl CursorTimer {
    pub fn new(position: usize) -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            on: false,
            position,
        }
    }
}

#[derive(Event, Debug)]
pub struct TextFieldDeselected {
    pub current_id: Entity,
}

fn react_to_deselect(
    mut commands: Commands,
    mut cursor_query: Query<(Entity, &mut Text, &CursorTimer)>,
    mut events: EventReader<TextFieldDeselected>,
) {
    for (entity, mut text, cursor) in cursor_query.iter_mut() {
        for TextFieldDeselected { current_id } in events.read() {
            if *current_id != entity {
                text.remove(cursor.position);
                commands.entity(entity).remove::<CursorTimer>();
            }
        }
    }
}

fn mouse_handling(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut Text, &mut BackgroundColor, Entity),
        (Changed<Interaction>, With<TextField>, Without<CursorTimer>),
    >,
) {
    for (interaction, mut text, mut background_color, entity) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                background_color.0 = RED.into();
                let length = text.0.len();
                text.0.insert(length, '_');
                commands.entity(entity).insert(CursorTimer::new(length)); // also signals it is selected
                deselect_others(&mut commands, entity);
            }
            Interaction::Hovered => {
                background_color.0 = GREEN.into();
            }
            Interaction::None => {
                background_color.0 = Color::WHITE;
            }
        }
    }
}

fn deselect_others(commands: &mut Commands, entity: Entity) {
    commands.queue(move |w: &mut World| {
        w.send_event(TextFieldDeselected { current_id: entity });
    })
}

fn tick_cursor(mut cursor_timer: Query<(&mut Text, &mut CursorTimer)>, time: Res<Time>) {
    for (mut text, mut cursor) in &mut cursor_timer {
        cursor.timer.tick(time.delta());
        if cursor.timer.finished() {
            cursor.timer.reset();
            // println!("Ticking cursor on {} at {}", text.0, cursor.position);
            if cursor.on {
                text.remove(cursor.position);
                text.0.insert(cursor.position, '_');
                cursor.on = false;
            } else {
                text.remove(cursor.position);
                text.0.insert(cursor.position, '|');
                cursor.on = true;
            }
        }
    }
}

fn keyboard_input(
    mut commands: Commands,
    mut events: EventReader<KeyboardInput>,
    edit_text: Single<(
        &mut Text,
        &mut TextField,
        &mut CursorTimer,
        Entity,
        &ChildOf,
    )>,
    children_query: Query<&Children>,
) {
    let (mut text, mut text_field, mut cursor, entity, parent) = edit_text.into_inner();
    let mut error_text: Option<Entity> = None;
    if let Ok(children) = children_query.get(parent.0) {
        // Get the second child if it exists
        if children.len() >= 2 {
            error_text = Some(children[1]);
            // commands.entity(second_child).insert(Visibility::Visible);
        }
    }

    for event in events.read() {
        if !event.state.is_pressed() {
            continue;
        }
        match (&event.logical_key, &event.text) {
            (Key::Enter, _) => {
                text.remove(cursor.position);
                commands.entity(entity).remove::<CursorTimer>();
            }
            (Key::Backspace, _) if cursor.position > 0 => {
                text.remove(cursor.position - 1);
                cursor.position -= 1;
                check_on_change(&mut commands, &mut text, &mut text_field, &mut cursor);
                validate(
                    &text,
                    &mut text_field,
                    &mut cursor,
                    &mut commands,
                    error_text,
                );
            }
            (Key::ArrowLeft, _) if cursor.position > 0 => {
                text.remove(cursor.position);
                text.insert(cursor.position - 1, '|');
                cursor.position -= 1;
            }
            (Key::ArrowRight, _) if cursor.position < text.0.len() => {
                text.remove(cursor.position);
                text.insert(cursor.position + 1, '|');
                cursor.position += 1;
            }
            (_, Some(inserted_text)) => {
                if inserted_text.chars().all(is_printable_char) {
                    text.insert_str(cursor.position, inserted_text);
                    cursor.position += 1;
                    check_on_change(&mut commands, &mut text, &mut text_field, &mut cursor);
                    validate(
                        &text,
                        &mut text_field,
                        &mut cursor,
                        &mut commands,
                        error_text,
                    );
                }
            }
            _ => continue,
        }
    }
}

fn check_on_change(
    mut commands: &mut Commands,
    text: &mut Mut<Text>,
    text_field: &mut Mut<TextField>,
    cursor: &mut Mut<CursorTimer>,
) {
    if let Some(on_change) = &text_field.on_change {
        let text = text_without_cursor(text.0.clone(), &cursor);
        on_change(text, &mut commands);
    }
}

fn validate(
    text: &Mut<Text>,
    text_field: &mut Mut<TextField>,
    cursor: &Mut<CursorTimer>,
    commands: &mut Commands,
    error_text: Option<Entity>,
) {
    if let Some(error_text) = error_text {
        if let Some(validate) = &text_field.validate {
            if let Some(error) = validate(text_without_cursor(text.0.clone(), &cursor)) {
                if !text_field.invalid {
                    text_field.invalid = true;
                    commands.entity(error_text).insert((
                        Visibility::Visible,
                        Text::new(error),
                        TextColor(RED.into()),
                    ));
                }
            } else {
                if text_field.invalid {
                    text_field.invalid = false;
                    commands.entity(error_text).insert(Visibility::Hidden);
                }
            }
        }
    }
 }

fn text_without_cursor(mut text: String, cursor: &CursorTimer) -> String {
    text.remove(cursor.position);
    text
}

// this logic is taken from egui-winit:
// https://github.com/emilk/egui/blob/adfc0bebfc6be14cee2068dee758412a5e0648dc/crates/egui-winit/src/lib.rs#L1014-L1024
fn is_printable_char(chr: char) -> bool {
    let is_in_private_use_area = ('\u{e000}'..='\u{f8ff}').contains(&chr)
        || ('\u{f0000}'..='\u{ffffd}').contains(&chr)
        || ('\u{100000}'..='\u{10fffd}').contains(&chr);

    !is_in_private_use_area && !chr.is_ascii_control()
}
