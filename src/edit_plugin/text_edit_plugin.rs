use bevy::app::{App, Plugin, Update};
use bevy::color::Color;
use bevy::prelude::{Component, Event, EventReader, Query, Text, TextColor};

pub struct TextEditPlugin;

impl Plugin for TextEditPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (react_to_content_update, react_to_colour_update))
            .add_event::<TextContentChange>();
    }
}

#[derive(Event, Debug)]
pub struct TextContentChange {
    pub source_id: String,
    pub contents: String,
}

#[derive(Debug, Component)]
pub struct TextContentReactor {
    pub source_id: String,
}

fn react_to_content_update(
    mut events: EventReader<TextContentChange>,
    mut text_query: Query<(&mut Text, &TextContentReactor)>,
) {
    for TextContentChange {
        source_id,
        contents,
    } in events.read()
    {
        for (mut text, reactor) in text_query.iter_mut() {
            println!(
                "Event received for Text: {} with content: {}",
                source_id, contents
            );
            if *source_id == reactor.source_id {
                text.0 = contents.clone();
            }
        }
    }
}

#[derive(Event, Debug)]
pub struct TextColorChange {
    pub source_id: String,
    pub color: Color,
}

#[derive(Debug, Component)]
pub struct TextColorReactor {
    pub source_id: String,
}

fn react_to_colour_update(
    mut events: EventReader<TextColorChange>,
    mut text_query: Query<(&mut TextColor, &TextColorReactor)>,
) {
    for TextColorChange { source_id, color } in events.read() {
        for (mut text_color, reactor) in text_query.iter_mut() {
            println!(
                "Event received for Text: {} with content: {:?}",
                source_id, color
            );
            if *source_id == reactor.source_id {
                text_color.0 = *color;
            }
        }
    }
}
