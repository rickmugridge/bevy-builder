use crate::ButtonNameChange;
use crate::builder::button_builder::ButtonId;
use bevy::color::palettes::basic::{GREEN, RED};
use bevy::prelude::*;

const NORMAL: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERING: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED: Color = Color::srgb(0.35, 0.75, 0.35);

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (button_hover, react_to_content_update));
    }
}

fn react_to_content_update(
    mut button_query: Query<(&mut Button, &ButtonId, &Children)>,
    mut events: EventReader<ButtonNameChange>,
    mut text_query: Query<&mut Text>,
) {
    for (button, button_id, children) in button_query.iter_mut() {
        for ButtonNameChange {destination_id, name} in events.read() {
            if *destination_id == button_id.0 {
                let mut text = text_query.get_mut(children[0]).unwrap();
                println!("Event received for button: {} with old content: {}", destination_id, text.0);
                text.0 = name.to_string();
            }
        }
    }
}

fn button_hover(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
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
