use bevy::app::{App, Startup};
use bevy::color::Color;
use bevy::prelude::{default, BackgroundColor, Commands, Entity, JustifyContent, Node, Plugin, UiRect, Val};

// This doesn't keep showing the box
// Created starting with https://taintedcoders.com/bevy/ui

pub struct BoxedPlugin;
impl Plugin for BoxedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_box_in_container);
    }
}

fn spawn_box_in_container(mut commands: Commands) {
    let child = spawn_box(&mut commands);
    let parent = spawn_single_container(&mut commands);
    commands.entity(parent).add_children(&[child]);
}

fn spawn_single_container(commands: &mut Commands) -> Entity {
    let container = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        ..default()
    };
    commands.spawn(container).id()
}

fn spawn_box(commands: &mut Commands) -> Entity {
    let square_node = Node {
        width: Val::Px(200.),
        border: UiRect::all(Val::Px(2.)),
        ..default()
    };

    let square_color = Color::srgb(0.65, 0.65, 0.65);

    let square = (square_node, BackgroundColor(square_color));
    commands.spawn(square).id()
}

