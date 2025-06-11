use crate::builder::box_builder::BoxBuilder;
use crate::builder::node_builder::NodeBuilder;
use crate::builder::text_builder::TextBuilder;
use crate::builder::text_field_builder::TextFieldBuilder;
use bevy::asset::AssetServer;
use bevy::prelude::*;

pub fn setup_edit_panel(commands: &mut Commands, _asset_server: &Res<AssetServer>) -> Entity {
    // let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let border_node = BoxBuilder::new().full_box().build_and_spawn(commands);

    let container = NodeBuilder::new()
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
        .align_items(AlignItems::Start)
        .justify_content(JustifyContent::Start)
        .row(vec![GridTrack::min_content(), GridTrack::flex(1.0)])
        .row_gap(Val::Px(5.0))
        .border(UiRect::all(Val::Px(5.0)))
        .build_and_spawn(commands);

    let label = TextBuilder::new()
        .content("Edit Button:")
        .build_and_spawn(commands);
    let text = TextFieldBuilder::new()
        .id("text-field")
        .node(
            NodeBuilder::new()
                .border(UiRect::all(Val::Px(5.0)))
                .margin(UiRect::all(Val::Px(5.0)))
                .build(),
        )
        .on_change(|id, s, commands| println!("on_change of {}: {}", id, s))
        .build_and_spawn(commands);
    commands.entity(border_node).add_child(container);
    commands.entity(container).add_children(&[label, text]);
    border_node
}
