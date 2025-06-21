use crate::builder::node_builder::NodeBuilder;
use crate::builder::text_builder::TextBuilder;
use crate::builder::text_field_builder::TextFieldBuilder;
use crate::edit::button_edit_panel::ButtonBorderColoration;
use crate::ui_plugin::color_sample_plugin::ColoringBox;
use crate::ui_plugin::open_close_plugin::{OpenClose, OpenCloseReactor};
use bevy::color::palettes::basic::GREEN;
use bevy::color::Color;
use bevy::prelude::{Commands, Component, Display, Entity, GridTrack, Val};

pub fn setup_colour_edit_panel(
    commands: &mut Commands,
    colour_destination_id: String,
    open_destination_id: String,
) -> Entity {
    let rgb_entity = commands
        .spawn((
            ButtonBorderColoration {
                destination_id: colour_destination_id.clone(),
            },
            EditRed(0.0),
            EditGreen(0.0),
            EditBlue(0.0),
        ))
        .id();
    let key_values_panel = NodeBuilder::new()
        .key_value_pairs()
        .background_color(GREEN.into())
        .build_and_spawn(commands);
    let red_label = make_text(commands, "Red:");
    let red = coloration(commands, move |value, commands| {
        println!("Update EditRed({value})");
        commands.entity(rgb_entity).insert((EditRed(value), ColorValueChanged));
    });
    let green_label = make_text(commands, "Green:");
    let green = coloration(commands, move |value, commands| {
        println!("Update EditGreen({value})");
        commands.entity(rgb_entity).insert((EditGreen(value), ColorValueChanged));
    });
    let blue_label = make_text(commands, "Blue:");
    let blue = coloration(commands, move |value, commands| {
        println!("Update EditBlue({value})");
        commands.entity(rgb_entity).insert((EditBlue(value), ColorValueChanged));
    });
    commands
        .entity(key_values_panel)
        .add_children(&[red_label, red, green_label, green, blue_label, blue])
        .insert(OpenCloseReactor {
            destination_id: open_destination_id.clone(),
            open_state: Display::Grid,
        });
    let outer_panel = NodeBuilder::new()
        .row(vec![GridTrack::min_content(), GridTrack::min_content()])
        .build_and_spawn(commands);
    let color_box = make_color_sample(commands, colour_destination_id, open_destination_id);
    commands
        .entity(outer_panel)
        .add_children(&[color_box, key_values_panel]);
    outer_panel
}

fn make_text(commands: &mut Commands, s: &str) -> Entity {
    TextBuilder::new().content(s).build_and_spawn(commands)
}

fn make_color_sample(
    commands: &mut Commands,
    colour_destination_id: String,
    open_destination_id: String,
) -> Entity {
    let color_box = NodeBuilder::new()
        .height(Val::Px(20.0))
        .border_of(Val::Px(1.0), Color::BLACK)
        .background_color(Color::WHITE)
        .build();
    commands
        .spawn((
            color_box,
            ColoringBox {
                destination_id: colour_destination_id,
            },
            OpenClose {
                destination_id: open_destination_id,
                open: false,
            },
        ))
        .id()
}

fn coloration<F>(commands: &mut Commands, on_change: F) -> Entity
where
    F: Fn(f32, &mut Commands) + Send + Sync + 'static,
{
    TextFieldBuilder::new()
        .node(NodeBuilder::new().text_field_node().build())
        .content("0.0")
        .on_change(move |button_text, commands| {
            if let Ok(value) = button_text.parse::<f32>() {
                println!("colour has updated to in coloration: {value}");
                let clamped_value = value.clamp(0.0, 1.0);
                on_change(clamped_value, commands);
            }
        })
        .build_and_spawn(commands)
}

#[derive(Component, Debug)]
pub struct EditRed(pub f32);

#[derive(Component, Debug)]
pub struct EditGreen(pub f32);

#[derive(Component, Debug)]
pub struct EditBlue(pub f32);

#[derive(Component)]
pub struct ColorValueChanged;
