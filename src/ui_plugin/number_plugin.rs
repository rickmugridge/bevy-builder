use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Component, Event, EventReader, Node, Query, UiRect, Val};

#[derive(Event, Debug)]
pub struct NumberChangedEvent {
    pub source_id: String,
    pub value: f32,
}

#[derive(Component, Debug)]
pub struct BorderSizeChangeReactor {
    pub source_id: String,
}

pub struct NumberPlugin;

impl Plugin for NumberPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NumberChangedEvent>()
            .add_systems(Update, update_border_size);
    }
}

fn update_border_size(
    mut events: EventReader<NumberChangedEvent>,
    mut query: Query<(&mut Node, &BorderSizeChangeReactor)>,
) {
    for event in events.read() {
        for (mut node, BorderSizeChangeReactor { source_id }) in query.iter_mut() {
            if event.source_id == *source_id {
                node.border = UiRect::all(Val::Px(event.value));
            }
        }
    }
}
