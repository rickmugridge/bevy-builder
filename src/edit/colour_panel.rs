use crate::builder::node_builder::NodeBuilder;
use crate::builder::text_builder::TextBuilder;
use crate::builder::text_field_builder::TextFieldBuilder;
use bevy::color::palettes::basic::GREEN;
use bevy::prelude::{Commands, Component, Entity};

pub fn setup_border_edit_panel(commands: &mut Commands, rgb_entity: Entity) -> Entity {
    commands
        .entity(rgb_entity)
        .insert((EditRed(0.0), EditGreen(0.0), EditBlue(0.0)));
    let key_values_panel = NodeBuilder::new()
        .key_value_pairs()
        .background_color(GREEN.into())
        .build_and_spawn(commands);
    let red_label = make_text(commands, "Red:");
    let red = coloration(commands, move |value, commands| {
        commands.entity(rgb_entity).insert(EditRed(value));
    });
    let green_label = make_text(commands, "Green:");
    let green = coloration(commands, move |value, commands| {
        commands.entity(rgb_entity).insert(EditGreen(value));
    });
    let blue_label = make_text(commands, "Blue:");
    let blue = coloration(commands, move |value, commands| {
        commands.entity(rgb_entity).insert(EditBlue(value));
    });
    commands
        .entity(key_values_panel)
        .add_children(&[red_label, red, green_label, green, blue_label, blue]);
    key_values_panel
}

fn make_text(commands: &mut Commands, s: &str) -> Entity {
    TextBuilder::new().content(s).build_and_spawn(commands)
}

fn coloration<F>(commands: &mut Commands, on_change: F) -> Entity
where
    F: Fn(f32, &mut Commands) + Send + Sync + 'static,
{
    TextFieldBuilder::new()
        .node(NodeBuilder::new().text_field_node().build())
        .content("0.0")
        .on_change(move |button_text, commands| {
            println!("Update to colour component: {}", button_text);
            if let Ok(value) = button_text.parse::<f32>() {
                let clamped_value = value.clamp(0.0, 1.0);
                on_change(clamped_value, commands);
            }
        })
        .build_and_spawn(commands)
}

#[derive(Component, Debug)]
struct EditRed(f32);

#[derive(Component, Debug)]
struct EditGreen(f32);

#[derive(Component, Debug)]
struct EditBlue(f32);
