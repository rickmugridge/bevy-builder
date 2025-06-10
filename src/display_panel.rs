use crate::NORMAL_BUTTON;
use crate::builder::box_builder::BoxBuilder;
use crate::builder::button_builder::ButtonBuilder;
use crate::builder::node_builder::NodeBuilder;
use crate::builder::text_builder::TextBuilder;
use bevy::asset::AssetServer;
use bevy::prelude::*;

pub fn setup_display_panel(commands: &mut Commands, _asset_server: &Res<AssetServer>) -> Entity {
    let border_node = BoxBuilder::new().full_box().build_and_spawn(commands);

    let button = ButtonBuilder::new(
        NodeBuilder::new()
            .width(Val::Px(150.0))
            .height(Val::Px(65.0))
            .justify_content(JustifyContent::Center)
            .align_items(AlignItems::Center)
            .border(UiRect::all(Val::Px(5.0)))
            .build(),
        TextBuilder::new().build_and_spawn(commands),
    )
    .justify_content(JustifyContent::Center)
    .align_items(AlignItems::Center)
    .background_color(NORMAL_BUTTON)
    .border_color(Color::WHITE)
    .build_and_spawn(commands);
    commands.entity(border_node).add_child(button);
    border_node
}
