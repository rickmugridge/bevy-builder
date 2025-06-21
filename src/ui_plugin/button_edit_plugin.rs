use crate::builder::button_builder::ButtonId;
use crate::edit::button_edit_panel::ButtonBorderColoration;
use crate::edit::colour_panel::{EditBlue, EditGreen, EditRed};
use bevy::app::{App, Plugin, Update};
use bevy::color::Color;
use bevy::prelude::{Children, Event, EventReader, Query, Text};
use bevy::ui::BorderColor;

pub struct ButtonEditPlugin;

impl Plugin for ButtonEditPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (react_to_content_update, react_to_border_colour_update),
        )
        .add_event::<ButtonNameChange>();
    }
}

fn react_to_content_update(
    mut button_query: Query<(&ButtonId, &Children)>,
    mut events: EventReader<ButtonNameChange>,
    mut text_query: Query<&mut Text>,
) {
    for (button_id, children) in button_query.iter_mut() {
        for ButtonNameChange {
            destination_id,
            button_text,
        } in events.read()
        {
            println!(
                "Event received for button: {} with button_text: {}",
                destination_id, button_text
            );
            if *destination_id == button_id.0 {
                let mut text = text_query.get_mut(children[0]).unwrap();
                // println!("Event received for button: {} with old content: {}", destination_id, text.0);
                text.0 = button_text.to_string();
            }
        }
    }
}

fn react_to_border_colour_update(
    mut button_query: Query<(&ButtonId, &mut BorderColor)>,
    coloration_query: Query<(&ButtonBorderColoration, &EditRed, &EditGreen, &EditBlue)>,
) {
    for (button_id, mut border_color) in button_query.iter_mut() {
        for (
            ButtonBorderColoration { destination_id },
            EditRed(red),
            EditGreen(green),
            EditBlue(blue),
        ) in coloration_query.iter()
        {
            if *destination_id == button_id.0 {
                let new_color = Color::srgb(*red, *green, *blue);
                if border_color.0 != new_color {
                    println!(
                        "Button border colour change for {destination_id} from {:?} to {:?}",
                        border_color.0, new_color
                    );
                    border_color.0 = new_color;
                }
            }
        }
    }
}

#[derive(Event, Debug)]
pub struct ButtonNameChange {
    pub destination_id: String,
    pub button_text: String,
}

pub const DESTINATION_BUTTON: &str = "DESTINATION_BUTTON";
