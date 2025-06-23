use crate::edit::button_edit_panel::ButtonBorderColoration;
use crate::edit::colour_panel::{ColorValueChanged, EditBlue, EditGreen, EditRed};
use bevy::app::{App, Plugin, Update};
use bevy::color::Color;
use bevy::prelude::{BackgroundColor, Commands, Component, Entity, IntoScheduleConfigs, Node, Query, With};

#[derive(Component, Debug)]
pub struct ColoringBox {
    pub destination_id: String,
}

pub struct ColorSamplePlugin;

impl Plugin for ColorSamplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, change_color.run_if(|query: Query<(), With<ColorValueChanged>>| !query.is_empty())
        );
    }
}

fn change_color(
    coloration_query: Query<(&ButtonBorderColoration, &EditRed, &EditGreen, &EditBlue, Entity), With<ColorValueChanged>>,
    mut node_query: Query<(&mut BackgroundColor, &ColoringBox), With<Node>>,
    mut commands: Commands,
) {
    for (
        ButtonBorderColoration { destination_id },
        EditRed(red),
        EditGreen(green),
        EditBlue(blue),
        entity,
    ) in coloration_query.iter()
    {
        println!("change_color 1st inner {entity}: {destination_id}");
        for (mut background, coloring_box) in node_query.iter_mut() {
            println!(
                "change_color inner {}, {} from {:?} to {:?}",
                coloring_box.destination_id,
                *destination_id,
                background.0,
                Color::srgb(*red, *green, *blue)
            );
            if coloring_box.destination_id == *destination_id {
                let new_color = Color::srgb(*red, *green, *blue);
                if background.0 != new_color {
                    println!(
                        "Change background colour XXXX from {:?} to {:?}",
                        background.0, new_color
                    );
                    background.0 = new_color;
                }
            }
        }
        commands.entity(entity).remove::<ColorValueChanged>();
    }
}
