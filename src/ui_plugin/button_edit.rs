use crate::builder::button_builder::ButtonId;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Children, Event, EventReader, Query, Text};

pub struct ButtonEditPlugin;

impl Plugin for ButtonEditPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, react_to_content_update)
            .add_event::<ButtonNameChange>();
    }
}

fn react_to_content_update(
    mut button_query: Query<(&ButtonId, &Children)>,
    mut events: EventReader<ButtonNameChange>,
    mut text_query: Query<&mut Text>,
) {
    println!("react_to_content_update");
    for (button_id, children) in button_query.iter_mut() {
        for ButtonNameChange {
            destination_id,
            button_text,
        } in events.read()
        {
            println!("Event received for button: {} with button_text: {}", destination_id, button_text);
            if *destination_id == button_id.0 {
                let mut text = text_query.get_mut(children[0]).unwrap();
                // println!("Event received for button: {} with old content: {}", destination_id, text.0);
                text.0 = button_text.to_string();
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
