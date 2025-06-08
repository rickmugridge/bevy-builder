mod builder;
mod ui;

use crate::builder::button_builder::ButtonBuilder;
use crate::builder::node_builder::NodeBuilder;
use crate::builder::text_builder::TextBuilder;
use bevy::{prelude::*, winit::WinitSettings};
use crate::builder::text_field_builder::TextFieldBuilder;
// use crate::ui::button2::ButtonPlugin2;
// use crate::builder::button_builder::ButtonBuilder;
use crate::ui::flex::FlexPlugin;
// use crate::ui::boxed::BoxedPlugin;
// use crate::ui::text::TextPlugin;
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
        .insert_resource(ClearColor(Color::srgb(0.05, 0.15, 0.25)))
        .add_systems(Startup, setup_panels)
        .run();
}

fn setup_panels(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn(Camera2d);
    let container = NodeBuilder::new()
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .column(vec![GridTrack::flex(1.0), GridTrack::min_content()])
        .border(UiRect::all(Val::Px(5.0)))
        .build_and_spawn(&mut commands);
    let text = TextFieldBuilder::new()
        .node(NodeBuilder::new().border(UiRect::all(Val::Px(5.0))).margin(UiRect::all(Val::Px(5.0))).build())
        .build_and_spawn(&mut commands);
    let button = ButtonBuilder::new(
        NodeBuilder::new()
            .width(Val::Px(150.0))
            .height(Val::Px(65.0))
            .justify_content(JustifyContent::Center)
            .align_items(AlignItems::Center)
            .border(UiRect::all(Val::Px(5.0)))
            .build(),
        TextBuilder::new().build_and_spawn(&mut commands),
    )
    .justify_content(JustifyContent::Center)
    .align_items(AlignItems::Center)
    .background_color(NORMAL_BUTTON)
    .border_color(Color::WHITE)
    .build_and_spawn(&mut commands);
    commands.entity(container).add_children(&[text, button]);
}

fn _temporary() {
    App::new()
        .add_plugins((DefaultPlugins, FlexPlugin))
        // .add_plugins((DefaultPlugins, TextPlugin))
        // .add_plugins((DefaultPlugins, ButtonPlugin2))
        // .add_plugins((DefaultPlugins, BoxedPlugin))
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .run();
}
