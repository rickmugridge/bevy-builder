use crate::builder::text_field_builder::TextField;
use bevy::app::{App, Plugin};
use bevy::color::palettes::basic::{GREEN, RED};
use bevy::prelude::*;

pub struct TextFieldPlugin;

impl Plugin for TextFieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, text_field_hover);
    }
}

fn text_field_hover(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<TextField>),
    >,
) {
    for (interaction, mut background_color) in interaction_query.iter_mut() {
        // println!(
        //     "Button over TextField {:?} {:?}",
        //     interaction, background_color
        // );
        match *interaction {
            Interaction::Pressed => {
                background_color.0 = RED.into();
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
