use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Component, Event, EventReader, Node, Query, UiRect, Val};

#[derive(Event, Debug)]
pub struct NumberChangedEvent {
    pub source_id: String,
    pub value: f32,
}

#[derive(Component, Debug)]
pub enum BorderSizeChangeReactor {
    Active { source_id: String },
    Inactive,
}

impl Default for BorderSizeChangeReactor {
    fn default() -> Self {
        Self::Inactive
    }
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
        for (mut node, reactor) in query.iter_mut() {
            if let BorderSizeChangeReactor:: Active { source_id } = reactor {
                if event.source_id == *source_id {
                    node.border = UiRect::all(Val::Px(event.value));
                }
            }
        }
    }
}
