use crate::ui_plugin::debounce_mouse_press_plugin::DebounceMousePress;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{
    Commands, Component, Display, Event, EventReader, Interaction, Node, Query, ResMut, World,
};

pub struct OpenClosePlugin;

impl Plugin for OpenClosePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (open_close, open_close_reactor))
            .add_event::<OpenCloseChanged>();
    }
}

#[derive(Component, Debug)]
#[require(Interaction)]
pub struct OpenClose {
    pub destination_id: String,
    pub open: bool,
}

#[derive(Component, Debug)]
pub struct OpenCloseReactor {
    pub destination_id: String,
    pub open_state: Display,
}

#[derive(Event, Debug)]
pub struct OpenCloseChanged {
    pub destination_id: String,
    pub open: bool,
}

fn open_close(
    mut commands: Commands,
    query: Query<(&Interaction, &mut OpenClose)>,
    mut debounce: ResMut<DebounceMousePress>,
) {
    for (interaction, mut open_close) in query {
        let destination_id = open_close.destination_id.clone();
        let new_open_state = !open_close.open;
        match *interaction {
            Interaction::Pressed => {
                if debounce.pressed() {
                    println!("Open close {new_open_state}");
                    open_close.open = new_open_state;
                    commands.queue(move |w: &mut World| {
                        w.send_event(OpenCloseChanged {
                            destination_id,
                            open: new_open_state,
                        });
                    })
                }
            }
            _ => {}
        }
    }
}

fn open_close_reactor(
    mut events: EventReader<OpenCloseChanged>,
    mut query: Query<(&OpenCloseReactor, &mut Node)>,
) {
    for OpenCloseChanged {
        destination_id,
        open,
    } in events.read()
    {
        println!("Open close reactor {destination_id} {open}");
        for (reactor, mut node) in query.iter_mut() {
            if reactor.destination_id == *destination_id {
                if *open {
                    node.display = reactor.open_state;
                } else {
                    node.display = Display::None;
                }
            }
        }
    }
}
