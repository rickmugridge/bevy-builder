use bevy::app::{App, Plugin, Update};
use bevy::color::Color;
use bevy::prelude::{Component, Event, EventReader, Query};
use bevy::ui::BackgroundColor;

pub struct BackgroundColorEditPlugin;

impl Plugin for BackgroundColorEditPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (react_to_update,))
            .add_event::<BackgroundColorChange>();
    }
}

#[derive(Event, Debug)]
pub struct BackgroundColorChange {
    pub source_id: String,
    pub color: Color,
}

#[derive(Debug, Component)]
pub struct BackgroundColorReactor {
    pub source_id: String,
}

fn react_to_update(
    mut events: EventReader<BackgroundColorChange>,
    mut query: Query<(&mut BackgroundColor, &BackgroundColorReactor)>,
) {
    for BackgroundColorChange { source_id, color } in events.read() {
        for (mut background_color, reactor) in query.iter_mut() {
            println!(
                "Event received for Text: {} with content: {:?}",
                source_id, color
            );
            if *source_id == reactor.source_id {
                background_color.0 = color.clone();
            }
        }
    }
}
