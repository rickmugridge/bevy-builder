use crate::builder::node_builder::NodeBuilder;
use crate::builder::text_builder::TextBuilder;
use crate::builder::text_field_builder::TextFieldBuilder;
use crate::edit::add_key_value_pairs::add_key_value_pairs;
use crate::edit::colour_panel::setup_colour_edit_panel;
use crate::edit::sources::{
    BUTTON_BACKGROUND_COLOR_SOURCE, BUTTON_BORDER_COLOR_SOURCE, BUTTON_BORDER_SIZE_SOURCE,
    BUTTON_TEXT_SOURCE,
};
use crate::edit_plugin::text_edit_plugin::TextContentChange;
use bevy::asset::AssetServer;
use bevy::color::palettes::basic::{BLUE, GREEN, RED};
use bevy::prelude::*;

pub fn setup_button_edit_panel(
    commands: &mut Commands,
    _asset_server: &Res<AssetServer>,
) -> Entity {
    let left_container = NodeBuilder::new()
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
        .align_items(AlignItems::Start)
        .justify_content(JustifyContent::Stretch)
        .row(vec![GridTrack::min_content(), GridTrack::flex(1.0)])
        .row_gap(Val::Px(5.0))
        .border(UiRect::all(Val::Px(5.0)), RED.into())
        .background_color(BLUE.into())
        .build_and_spawn(commands);
    let top_label = TextBuilder::new()
        .content("Edit Button:")
        .build_and_spawn(commands);
    let key_values_panel = setup_key_value_panel(commands);
    commands
        .entity(left_container)
        .add_children(&[top_label, key_values_panel]);
    left_container
}

fn setup_key_value_panel(commands: &mut Commands) -> Entity {
    let key_values_panel = NodeBuilder::new()
        .key_value_pairs()
        .background_color(GREEN.into())
        .build_and_spawn(commands);
    let text = setup_text_edit_panel(commands, "Default", BUTTON_TEXT_SOURCE);
    let border_color_edit = setup_colour_edit_panel(commands, BUTTON_BORDER_COLOR_SOURCE);
    let border_size_edit = setup_border_size_edit(commands);
    let background_color_edit = setup_colour_edit_panel(commands, BUTTON_BACKGROUND_COLOR_SOURCE);
    let pairs = &[
        ("Text:", text),
        ("Border colour:", border_color_edit),
        ("Border size in Px:", border_size_edit),
        ("Background colour:", background_color_edit),
    ];
    add_key_value_pairs(pairs, key_values_panel, commands);
    key_values_panel
}

fn setup_border_size_edit(commands: &mut Commands) -> Entity {
    TextFieldBuilder::new()
        .content("1.0")
        .on_change_to_number(BUTTON_BORDER_SIZE_SOURCE)
        .build_and_spawn(commands)
}

fn setup_text_edit_panel<S: Into<String>>(
    commands: &mut Commands,
    default_content: S,
    source_id: &'static str,
) -> Entity {
    TextFieldBuilder::new()
        .node(NodeBuilder::new().text_field_node().build())
        .on_change(|button_text, commands| {
            commands.queue(|w: &mut World| {
                println!(
                    "Queue string update to {} to {}",
                    button_text,
                    source_id.to_string()
                );
                w.send_event(TextContentChange {
                    source_id: source_id.into(),
                    contents: button_text,
                });
            })
        })
        .content(default_content)
        // .on_change(|id, s, commands| println!("on_change of {}: {}", id, s))
        .build_and_spawn(commands)
}
