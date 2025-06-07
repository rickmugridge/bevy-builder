use bevy::app::{App, Plugin, Startup};
use bevy::color::Color;
use bevy::prelude::{default, Camera2d, Commands, JustifyText, Node, PositionType, Text, TextColor, TextFont, TextLayout, Val};

pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_text);
    }
}

pub fn spawn_text(mut commands: Commands) { // https://taintedcoders.com/bevy/ui
    commands.spawn(Camera2d); // ui camera
    let text = "Hello world!";

    commands.spawn((
        Text::new(text),
        TextFont {
            font_size: 100.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        },
    ));
}
