mod builder;
mod display_panel;
mod edit_panel;
mod ui_plugin;

use crate::builder::node_builder::NodeBuilder;
use crate::display_panel::setup_display_panel;
use crate::edit_panel::setup_edit_panel;
use crate::ui_plugin::button::ButtonPlugin;
use crate::ui_plugin::text_field::TextFieldPlugin;
use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: [800., 600.].into(),
                title: "Bevy UI Builder".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((TextFieldPlugin, ButtonPlugin))
        .insert_resource(ClearColor(Color::srgb(0.05, 0.15, 0.25)))
        .add_systems(Startup, setup_panels)
        .run();
}

fn setup_panels(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let container = NodeBuilder::new()
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .column(vec![GridTrack::flex(1.0), GridTrack::flex(1.0)])
        .border(UiRect::all(Val::Px(5.0)))
        .build_and_spawn(&mut commands);
    let edit_panel = setup_edit_panel(&mut commands, &_asset_server);
    let display_panel = setup_display_panel(&mut commands, &_asset_server);
    commands.entity(container).add_children(&[edit_panel, display_panel]);
}
