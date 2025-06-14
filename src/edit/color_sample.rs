use crate::builder::node_builder::NodeBuilder;
use crate::edit::button_edit_panel::ButtonBorderColoration;
use crate::edit::colour_panel::{EditBlue, EditGreen, EditRed};
use bevy::app::{App, Plugin, Update};
use bevy::color::Color;
use bevy::prelude::{BackgroundColor, Commands, Component, Entity, Node, Query, Val, With};

#[derive(Component, Debug)]
pub struct ColoringBox {
    destination_id: String,
}

pub struct ColorSamplePlugin;

impl Plugin for ColorSamplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, change_color);
    }
}

fn change_color(
    mut node_query: Query<(&mut BackgroundColor, &ColoringBox), With<Node>>,
    coloration_query: Query<(&ButtonBorderColoration, &EditRed, &EditGreen, &EditBlue)>,
) {
    for (mut background, coloringBox) in node_query.iter_mut() {
        for (
            ButtonBorderColoration { destination_id },
            EditRed(red),
            EditGreen(green),
            EditBlue(blue),
        ) in coloration_query.iter()
        {
            if coloringBox.destination_id == *destination_id {
                let new_color = Color::srgb(*red, *green, *blue);
                if background.0 != new_color {
                    background.0 = new_color;
                }
            }
        }
    }
}

pub fn make_color_sample(commands: &mut Commands, destination_id: String) -> Entity {
    let color_box = NodeBuilder::new()
        .height(Val::Px(20.0))
        .border_of(Val::Px(1.0), Color::BLACK)
        .background_color(Color::WHITE)
        .build();
    commands
        .spawn((color_box, ColoringBox { destination_id }))
        .id()
}
