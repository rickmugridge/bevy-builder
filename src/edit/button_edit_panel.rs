use crate::builder::node_builder::NodeBuilder;
use crate::builder::text_builder::TextBuilder;
use crate::builder::text_field_builder::TextFieldBuilder;
use crate::edit::colour_panel::setup_colour_edit_panel;
use crate::edit_plugin::button_edit_plugin::{ButtonNameChange, DESTINATION_BUTTON};
use bevy::asset::AssetServer;
use bevy::color::palettes::basic::{BLUE, GREEN, RED};
use bevy::prelude::*;

pub const BUTTON_BORDER_COLOR_OPEN_CLOSE: &str = "BUTTON_BORDER_COLOR_OPEN_CLOSE";

#[derive(Component, Debug)]
pub struct ButtonBorderColoration {
    pub source_id: String,
}

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
    let top_label = make_text(commands, "Edit Button:");
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
    let text = setup_text_edit_panel(commands, "Default", DESTINATION_BUTTON);
    let background_color_edit = setup_colour_edit_panel(
        commands,
        DESTINATION_BUTTON.to_string(),
        BUTTON_BORDER_COLOR_OPEN_CLOSE.to_string(),
    );
     let pairs = &[("Text:", text), ("Border colour:", background_color_edit)];
    add_key_value_pairs(pairs, key_values_panel, commands);
    key_values_panel
}

fn add_key_value_pairs(
    pairs: &[(&str, Entity)],
    key_values_panel: Entity,
    commands: &mut Commands,
) {
    let mut children: Vec<Entity> = vec![];
    pairs.iter().for_each(|(s, entity)| {
        children.push(make_text(commands, s));
        children.push(*entity);
    });
    commands.entity(key_values_panel).add_children(&children);
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
                w.send_event(ButtonNameChange {
                    source_id: source_id.to_string(),
                    text: button_text,
                });
            })
        })
        .content(default_content)
        // .on_change(|id, s, commands| println!("on_change of {}: {}", id, s))
        .build_and_spawn(commands)
}

fn make_text(commands: &mut Commands, s: &str) -> Entity {
    TextBuilder::new().content(s).build_and_spawn(commands)
}
