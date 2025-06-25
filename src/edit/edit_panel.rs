use crate::builder::node_builder::NodeBuilder;
use crate::builder::text_builder::TextBuilder;
use crate::edit::button_edit_panel::setup_button_edit_panel;
use bevy::asset::AssetServer;
use bevy::color::palettes::css::YELLOW;
use bevy::prelude::Val::Px;
use bevy::prelude::*;

pub fn setup_edit_panel(commands: &mut Commands, _asset_server: &Res<AssetServer>) -> Entity {
    let top_label = TextBuilder::new()
        .content("Edit:")
        .build_and_spawn(commands);
    let border_node = NodeBuilder::new()
        .border_of(Px(2.), YELLOW.into())
        .height(Val::Percent(100.))
        .row(vec![GridTrack::min_content(), GridTrack::flex(1.0)])
        .build_and_spawn(commands);
    let button_edit_panel = setup_button_edit_panel(commands, _asset_server);
    commands
        .entity(border_node)
        .add_children(&[top_label, button_edit_panel]);
    border_node
}
