use crate::builder::node_builder::NodeBuilder;
use crate::builder::text_builder::TextBuilder;
use crate::builder::text_field_builder::TextFieldBuilder;
use crate::edit::colour_panel::setup_colour_edit_panel;
use crate::ui_plugin::button_edit_plugin::{ButtonNameChange, DESTINATION_BUTTON};
use bevy::asset::AssetServer;
use bevy::color::palettes::basic::{BLUE, GREEN, RED};
use bevy::prelude::*;

pub const TO_BUTTON_BACKGROUND_COLOR_OPEN_CLOSE: &str = "TO_BUTTON_BACKGROUND_COLOR_OPEN_CLOSE";
pub const TO_BUTTON_BACKGROUND_COLOR_HOVER: &str = "TO_BUTTON_BACKGROUND_COLOR_HOVER";

#[derive(Component, Debug)]
pub struct ButtonBorderColoration {
    pub destination_id: String,
}

pub fn setup_button_edit_panel(
    commands: &mut Commands,
    _asset_server: &Res<AssetServer>,
) -> Entity {

    let top_label = make_text(commands, "Edit Button:");
    let key_values_panel = NodeBuilder::new()
        .key_value_pairs()
        .background_color(GREEN.into())
        .build_and_spawn(commands);
    let label_text = make_text(commands, "Text:");
    let text = setup_text_edit_panel(commands, "Default", DESTINATION_BUTTON);
    let background_label = make_text(commands, "Border colour:");
    let background_color_edit = setup_colour_edit_panel(commands,
                                                        DESTINATION_BUTTON.to_string(),
                                                        TO_BUTTON_BACKGROUND_COLOR_OPEN_CLOSE.to_string());
    commands.entity(key_values_panel).add_children(&[
        label_text,
        text,
        background_label,
        background_color_edit,
    ]);
    
    let row_container = NodeBuilder::new()
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
        .align_items(AlignItems::Start)
        .justify_content(JustifyContent::Stretch)
        .row(vec![GridTrack::min_content(), GridTrack::flex(1.0)])
        .row_gap(Val::Px(5.0))
        .border(UiRect::all(Val::Px(5.0)), RED.into())
        .background_color(BLUE.into())
        .build_and_spawn(commands);
    commands
        .entity(row_container)
        .add_children(&[top_label, key_values_panel]);
    row_container
}

fn setup_text_edit_panel<S: Into<String>>(
    commands: &mut Commands,
    default_content: S,
    destination_id: &'static str,
) -> Entity {
    TextFieldBuilder::new()
        .node(NodeBuilder::new().text_field_node().build())
        .on_change(|button_text, commands| {
            commands.queue(|w: &mut World| {
                println!(
                    "Queue string update to {} to {}",
                    button_text,
                    destination_id.to_string()
                );
                w.send_event(ButtonNameChange {
                    destination_id: destination_id.to_string(),
                    button_text,
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
