use bevy::app::{App, Plugin, Update};
use bevy::color::Color;
use bevy::prelude::{Component, Event, EventReader, Query, Text, TextColor};

pub struct TextEditPlugin;

impl Plugin for TextEditPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (react_to_content_update, react_to_colour_update))
            .add_event::<TextContentChange>()
            .add_event::<TextColorChange>();
    }
}

#[derive(Event, Debug)]
pub struct TextContentChange {
    pub source_id: String,
    pub contents: String,
}

#[derive(Debug, Component)]
pub enum TextContentReactor {
    Active { source_id: String },
    Inactive,
}

impl Default for TextContentReactor {
    fn default() -> Self {
        Self::Inactive
    }
}

fn react_to_content_update(
    mut events: EventReader<TextContentChange>,
    mut text_query: Query<(&mut Text, &TextContentReactor)>,
) {
    for event in events.read()
    {
        for (mut text, reactor) in text_query.iter_mut() {
            // println!(
            //     "Event received for  {:?}",
            //     event
            // );
            if let TextContentReactor::Active{source_id} = reactor {
                if *event.source_id == *source_id {
                    text.0 = event.contents.clone();
                }
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
pub enum TextColorReactor {
    Active { source_id: String },
    Inactive,
}

impl Default for TextColorReactor {
    fn default() -> Self {
        Self::Inactive
    }
}

fn react_to_colour_update(
    mut events: EventReader<TextColorChange>,
    mut text_query: Query<(&mut TextColor, &TextColorReactor)>,
) {
    for event in events.read() {
        for (mut text_color, reactor) in text_query.iter_mut() {
            println!(
                "Event received for  {:?}",
                event
            );
            if let TextColorReactor::Active{source_id} = reactor {
                if *event.source_id == *source_id {
                    text_color.0 = event.color;
                }
            }
        }
    }
}
