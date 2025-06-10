//! This example illustrates how to create a ui_plugin that changes color and text based on its
//! interaction state.
//!

// https://github.com/bevyengine/bevy/blob/latest/examples/ui/button.rs

use bevy::color::palettes::basic::{GREEN, RED};
use bevy::prelude::*;

const NORMAL: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERING: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED: Color = Color::srgb(0.35, 0.75, 0.35);

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_hover);
    }
}

fn button_hover(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED.into();
                border_color.0 = RED.into();
            }
            Interaction::Hovered => {
                *color = HOVERING.into();
                border_color.0 = GREEN.into();
            }
            Interaction::None => {
                *color = NORMAL.into();
                border_color.0 = Color::WHITE;
            }
        }
    }
}
