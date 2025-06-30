use crate::builder::button_builder::ButtonBuilder;
use crate::builder::node_builder::NodeBuilder;
use crate::builder::text_builder::TextBuilder;
use crate::edit::sources::{BUTTON_BACKGROUND_COLOR_SOURCE, BUTTON_BORDER_COLOR_SOURCE, BUTTON_BORDER_SIZE_SOURCE, BUTTON_TEXT_SOURCE};
use bevy::asset::AssetServer;
use bevy::color::palettes::basic::YELLOW;
use bevy::color::palettes::css::GREEN;
use bevy::prelude::Val::Px;
use bevy::prelude::*;

pub fn setup_display_panel(commands: &mut Commands, _asset_server: &Res<AssetServer>) -> Entity {
    let border_node = NodeBuilder::new()
        .border_of(Px(3.), YELLOW.into())
        .height(Val::Percent(100.))
        .row(vec![GridTrack::min_content()])
        .build_and_spawn(commands);

    let button_node = NodeBuilder::new()
        .justify_content(JustifyContent::Center)
        .align_items(AlignItems::Center)
        .border(UiRect::all(Val::Px(2.0)), GREEN.into())
        .border_color_change_reactor(BUTTON_BORDER_COLOR_SOURCE)
        .border_size_change_reactor(BUTTON_BORDER_SIZE_SOURCE)
        .background_color_change_reactor(BUTTON_BACKGROUND_COLOR_SOURCE)
       .build();
    let button = ButtonBuilder::new(
        button_node,
        TextBuilder::new()
            .content("Default")
            .text_content_reactor(BUTTON_TEXT_SOURCE)
            .build_and_spawn(commands),
    )
    .build_and_spawn(commands);
    commands.entity(border_node).add_child(button);
    border_node
}
