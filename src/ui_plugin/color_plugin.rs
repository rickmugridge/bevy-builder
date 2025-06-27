use bevy::app::{App, Plugin, Update};
use bevy::color::Color;
use bevy::prelude::*;

// This responds to changes in RGB and fires a ColorChangedEvent.
// That event can be used to change the color of borders, backgrounds, text, etc

#[derive(Component, Debug)]
pub struct Coloration {
    pub source_id: String,
}

#[derive(Event, Debug)]
pub struct ColorChangedEvent {
    pub source_id: String,
    pub color: Color,
}

#[derive(Component, Debug)]
pub struct BackgroundColorChangeReactor {
    pub source_id: String,
}

#[derive(Component, Debug)]
pub struct BorderColorChangeReactor {
    pub source_id: String,
}

#[derive(Component, Debug)]
pub struct TextColorChangeReactor {
    pub source_id: String,
}

#[derive(Component, Debug)]
pub struct EditRed(pub f32);

#[derive(Component, Debug)]
pub struct EditGreen(pub f32);

#[derive(Component, Debug)]
pub struct EditBlue(pub f32);

#[derive(Component)]
pub struct ColorValueChanged;

pub struct ColorPlugin;

impl Plugin for ColorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ColorChangedEvent>().add_systems(
            Update,
            (
                update_color.run_if(|query: Query<(), With<ColorValueChanged>>| !query.is_empty()),
                update_background_color,
                update_border_color,
                update_text_color,
            ),
        );
    }
}

fn update_color(
    coloration_query: Query<
        (&Coloration, &EditRed, &EditGreen, &EditBlue, Entity),
        With<ColorValueChanged>,
    >,
    mut commands: Commands,
) {
    for (Coloration { source_id }, EditRed(red), EditGreen(green), EditBlue(blue), entity) in
        coloration_query.iter()
    {
        let color = Color::srgb(*red, *green, *blue);
        // println!("update_color {source_id} to {:?}", color);
        let source_id = source_id.clone();
        commands.queue(move |w: &mut World| {
            w.send_event(ColorChangedEvent { source_id, color });
        });
        commands.entity(entity).remove::<ColorValueChanged>();
    }
}

fn update_border_color(
    mut events: EventReader<ColorChangedEvent>,
    mut query: Query<(&mut BorderColor, &BorderColorChangeReactor)>,
) {
    for event in events.read() {
        // println!("Update border color: {}, {:?}", event.source_id, event.color);
        for (mut border_color, BorderColorChangeReactor { source_id }) in query.iter_mut() {
            // println!("Update border color: {source_id} and {}, {:?}", event.source_id, event.color);
            if event.source_id == *source_id {
                border_color.0 = event.color;
            }
        }
    }
}

fn update_background_color(
    mut events: EventReader<ColorChangedEvent>,
    mut query: Query<(&mut BackgroundColor, &BackgroundColorChangeReactor)>,
) {
    for event in events.read() {
        for (mut background_color, BackgroundColorChangeReactor { source_id }) in query.iter_mut() {
            // println!("Update background color: {source_id} and {}, {:?}", event.source_id, event.color);
            if event.source_id == *source_id {
                background_color.0 = event.color;
            }
        }
    }
}

fn update_text_color(
    mut events: EventReader<ColorChangedEvent>,
    mut query: Query<(&mut TextColor, &TextColorChangeReactor)>,
) {
    for event in events.read() {
        for (mut text_color, TextColorChangeReactor { source_id }) in query.iter_mut() {
            if event.source_id == *source_id {
                text_color.0 = event.color;
            }
        }
    }
}
