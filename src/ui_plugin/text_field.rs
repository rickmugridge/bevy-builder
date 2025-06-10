use std::io::Cursor;
use crate::builder::text_field_builder::TextField;
use bevy::app::{App, Plugin};
use bevy::color::palettes::basic::{GREEN, RED};
use bevy::prelude::*;

pub struct TextFieldPlugin;

impl Plugin for TextFieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (text_field_hover,tick_cursor));
    }
}

#[derive(Debug, Component)]
struct CursorTimer(Timer);

impl CursorTimer {
    pub fn new() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Repeating))
    }
}

fn text_field_hover(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Entity),
        (Changed<Interaction>, With<TextField>),
    >,
) {
    for (interaction, mut background_color, entity) in interaction_query.iter_mut() {
        // println!(
        //     "Button over TextField {:?} {:?}",
        //     interaction, background_color
        // );
        match *interaction {
            Interaction::Pressed => {
                background_color.0 = RED.into();
                commands.entity(entity).insert(CursorTimer::new());
                // todo Set this as the one selected text field
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
    mut cursor_timer: Query<(Entity, &mut CursorTimer)>,
    time: Res<Time>,
) {
    for (entity, mut cooldown) in &mut cursor_timer {
        cooldown.0.tick(time.delta());

        // todo Instead of the following, add or subtract the cursor from the TextField content
        if cooldown.0.finished() {
            cooldown.0.reset();
            println!("Ticking cursor");
            // commands.entity(entity).remove::<CursorTimer>();
        }
    }
}

