use crate::builder::button_builder::ButtonResponder;
use bevy::prelude::*;

pub const NORMAL: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERING: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED: Color = Color::srgb(0.35, 0.75, 0.35);

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (button_hover, button_press));
    }
}

fn button_hover(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
            }
            Interaction::Hovered => {
                *color = HOVERING.into();
            }
            Interaction::None => {
                *color = NORMAL.into();
            }
        }
    }
}

fn button_press(
    mut interaction_query: Query<(&Interaction, &ButtonResponder, Entity), Changed<Interaction>>,
    mut commands: Commands,
) {
    for (interaction, button_responder, button_entity) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(on_press) = button_responder.on_press.as_ref() {
                    on_press(button_entity, &mut commands);
                }
            }
            _ => {}
        }
    }
}
