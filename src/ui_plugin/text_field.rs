use crate::builder::text_field_builder::TextField;
use bevy::app::{App, Plugin};
use bevy::color::palettes::basic::{GREEN, RED};
use bevy::prelude::*;

pub struct TextFieldPlugin;

impl Plugin for TextFieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (mouse_handling, tick_cursor));
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
    mut commands: Commands,
    mut cursor_timer: Query<(&mut Text, &mut TextField, &mut CursorTimer)>,
    time: Res<Time>,
) {
    for (mut text, mut text_field, mut cooldown) in &mut cursor_timer {
        cooldown.0.tick(time.delta());
        if cooldown.0.finished() {
            cooldown.0.reset();
            println!("Ticking cursor on {}", text.0);
            if text_field.cursor_on {
                text.pop();
                text_field.cursor_on = false;
            } else {
                text.0.push('|');
                text_field.cursor_on = true;
            }
            // commands.entity(entity).remove::<CursorTimer>();
        }
    }
}
