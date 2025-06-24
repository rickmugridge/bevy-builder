use bevy::app::{App, Plugin, Update};
use bevy::color::Color;
use bevy::prelude::{Component, Event, EventReader, Query};
use bevy::ui::BorderColor;

pub struct BorderEditPlugin;

impl Plugin for BorderEditPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, react_to_update)
            .add_event::<BorderColorChange>();
    }
}

#[derive(Event, Debug)]
pub struct BorderColorChange {
    pub source_id: String,
    pub color: Color,
}

#[derive(Debug, Component)]
pub struct BorderColorReactor {
    pub source_id: String,
}

fn react_to_update(
    mut events: EventReader<BorderColorChange>,
    mut query: Query<(&mut BorderColor, &BorderColorReactor)>,
) {
    for BorderColorChange { source_id, color } in events.read() {
        for (mut border_color, reactor) in query.iter_mut() {
            println!(
                "Event received for Text: {} with colour: {:?}",
                source_id, color
            );
            if *source_id == reactor.source_id {
                border_color.0 = *color;
            }
        }
    }
}
