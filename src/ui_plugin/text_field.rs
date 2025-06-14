use crate::builder::text_field_builder::TextField;
use bevy::app::{App, Plugin};
use bevy::color::palettes::basic::{GREEN, RED};
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;

pub struct TextFieldPlugin;

impl Plugin for TextFieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (mouse_handling, tick_cursor, keyboard_input));
    }
}

#[derive(Component, Debug)]
struct CursorTimer(Timer);

impl CursorTimer {
    pub fn new() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Repeating))
    }
}

fn mouse_handling(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Entity, &mut TextField),
        (Changed<Interaction>, With<TextField>),
    >,
) {
    for (interaction, mut background_color, entity, mut text_field) in interaction_query.iter_mut()
    {
        // println!(
        //     "Button over TextField {:?} {:?}",
        //     interaction, background_color
        // );
        match *interaction {
            Interaction::Pressed => {
                background_color.0 = RED.into();
                commands.entity(entity).insert(CursorTimer::new()); // also signals it is selected
                text_field.cursor_on = false;
            }
            Interaction::Hovered => {
                background_color.0 = GREEN.into();
            }
            Interaction::None => {
                background_color.0 = Color::WHITE;
                // todo set this as not selected??
            }
        }
    }
}

fn tick_cursor(
    mut cursor_timer: Query<(&mut Text, &mut TextField, &mut CursorTimer)>,
    time: Res<Time>,
) {
    for (mut text, mut text_field, mut cooldown) in &mut cursor_timer {
        cooldown.0.tick(time.delta());
        if cooldown.0.finished() {
            cooldown.0.reset();
            // println!("Ticking cursor on {} at {}", text.0, text_field.cursor_position);
            if text_field.cursor_on {
                text.remove(text_field.cursor_position);
                text_field.cursor_on = false;
            } else {
                text.0.insert(text_field.cursor_position, '|');
                text_field.cursor_on = true;
            }
        }
    }
}

fn keyboard_input(
    mut commands: Commands,
    mut events: EventReader<KeyboardInput>,
    edit_text: Single<(&mut Text, &mut TextField, Entity), With<CursorTimer>>,
) {
    println!("Keyboard input");
    let (mut text, mut text_field, entity) = edit_text.into_inner();
    for event in events.read() {
        // Only trigger changes when the key is first pressed.
        if !event.state.is_pressed() {
            continue;
        }
        // println!(
        //     "KB: {:?}/{:?} for {} at {}",
        //     &event.logical_key, &event.text, text.0, text_field.cursor_position
        // );

        match (&event.logical_key, &event.text) {
            (Key::Enter, _) => {
                commands.entity(entity).remove::<CursorTimer>();
                if text_field.cursor_on {
                    text.remove(text_field.cursor_position);
                    text_field.cursor_on = false;
                }
            }
            (Key::Backspace, _) => {
                text.remove(text_field.cursor_position - 1);
                text_field.cursor_position -= 1;
                if let Some(on_change) = &text_field.on_change {
                    let text = text_without_cursor(text.0.clone(), &text_field);
                    on_change(text, &mut commands);
                }
            }
            (Key::ArrowLeft, _) if text_field.cursor_position > 0 => {
                if text_field.cursor_on {
                    text.remove(text_field.cursor_position);
                    text.insert(text_field.cursor_position - 1, '|');
                }
                text_field.cursor_position -= 1;
            }
            (Key::ArrowRight, _) if text_field.cursor_position < text.0.len() => {
                if text_field.cursor_on {
                    text.remove(text_field.cursor_position);
                    text.insert(text_field.cursor_position + 1, '|');
                }
                text_field.cursor_position += 1;
            }
            (_, Some(inserted_text)) => {
                if inserted_text.chars().all(is_printable_char) {
                    text.insert_str(text_field.cursor_position, inserted_text);
                    text_field.cursor_position += 1;
                    if let Some(on_change) = &text_field.on_change {
                        let text = text_without_cursor(text.0.clone(), &text_field);
                        on_change(text, &mut commands);
                    }
                }
            }
            _ => continue,
        }
    }

    fn text_without_cursor(mut text: String, text_field: &TextField) -> String {
        if text_field.cursor_on {
            text.remove(text_field.cursor_position);
        }
        text
    }
}

// this logic is taken from egui-winit:
// https://github.com/emilk/egui/blob/adfc0bebfc6be14cee2068dee758412a5e0648dc/crates/egui-winit/src/lib.rs#L1014-L1024
fn is_printable_char(chr: char) -> bool {
    let is_in_private_use_area = ('\u{e000}'..='\u{f8ff}').contains(&chr)
        || ('\u{f0000}'..='\u{ffffd}').contains(&chr)
        || ('\u{100000}'..='\u{10fffd}').contains(&chr);

    !is_in_private_use_area && !chr.is_ascii_control()
}
