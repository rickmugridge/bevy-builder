use crate::builder::node_builder::NodeBuilder;
use crate::builder::text_builder::TextBuilder;
use crate::builder::text_field_builder::TextFieldBuilder;
use crate::ui_plugin::color_plugin::{
    BackgroundColorChangeReactor, ColorValueChanged, Coloration, EditBlue, EditGreen, EditRed,
};
use crate::ui_plugin::open_close_plugin::{OpenClose, OpenCloseReactor};
use bevy::color::Color;
use bevy::color::palettes::basic::{GREEN, WHITE};
use bevy::prelude::{Commands, Display, Entity, GridTrack, Interaction, Val};

pub fn setup_colour_edit_panel(
    commands: &mut Commands,
    color_source_id: &str
) -> Entity {
    let mut open_close_source_id = color_source_id.to_string();
    open_close_source_id.push_str("_open_close");
    let rgb_entity = commands
        .spawn((
            Coloration {
                source_id: color_source_id.into(),
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
        // println!("Update EditRed({value})");
        commands
            .entity(rgb_entity)
            .insert((EditRed(value), ColorValueChanged));
    });
    let green_label = make_text(commands, "Green:");
    let green = coloration(commands, move |value, commands| {
        // println!("Update EditGreen({value})");
        commands
            .entity(rgb_entity)
            .insert((EditGreen(value), ColorValueChanged));
    });
    let blue_label = make_text(commands, "Blue:");
    let blue = coloration(commands, move |value, commands| {
        // println!("Update EditBlue({value})");
        commands
            .entity(rgb_entity)
            .insert((EditBlue(value), ColorValueChanged));
    });
    // commands.entity(rgb_entity).log_components();
    commands
        .entity(key_values_panel)
        .add_children(&[red_label, red, green_label, green, blue_label, blue])
        .insert(OpenCloseReactor {
            source_id: open_close_source_id.clone(),
            open_state: Display::Grid,
        });
    let outer_panel = NodeBuilder::new()
        .row(vec![GridTrack::min_content(), GridTrack::min_content()])
        .build_and_spawn(commands);
    let colour_panel = make_colour_panel(commands, color_source_id, open_close_source_id);
    commands
        .entity(outer_panel)
        .add_children(&[colour_panel, key_values_panel]);
    outer_panel
}

fn make_colour_panel(
    commands: &mut Commands,
    color_source_id: &str,
    open_source_id: String,
) -> Entity {
    let color_box = make_color_sample(commands, color_source_id);
    let colour_panel = NodeBuilder::new()
        .column(vec![GridTrack::min_content(), GridTrack::flex(1.0)])
        .build_and_spawn(commands);
    let plus_label = make_open_label(commands, open_source_id.into());
    commands
        .entity(colour_panel)
        .add_children(&[plus_label, color_box]);
    colour_panel
}

fn make_open_label(commands: &mut Commands, open_source_id: String) -> Entity {
    let text = TextBuilder::new().content("+").build_and_spawn(commands);
    commands.entity(text).insert((
        Interaction::default(),
        OpenClose {
            source_id: open_source_id,
            open: true,
        },
    ));
    let panel = NodeBuilder::new()
        .background_color(GREEN.into())
        .border_of(Val::Px(1.), WHITE.into())
        .build_and_spawn(commands);
    commands.entity(panel).add_children(&[text]);
    panel
}

fn make_text(commands: &mut Commands, s: &str) -> Entity {
    TextBuilder::new().content(s).build_and_spawn(commands)
}

fn make_color_sample(commands: &mut Commands, source_id: &str) -> Entity {
    let color_box = NodeBuilder::new()
        .height(Val::Px(25.0))
        .background_color(Color::BLACK)
        .build();
    commands
        .spawn((
            color_box,
            BackgroundColorChangeReactor {
                source_id: source_id.into(),
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
                // println!("colour has updated in coloration: {value}");
                let clamped_value = value.clamp(0.0, 1.0);
                on_change(clamped_value, commands);
            }
        })
        .build_and_spawn(commands)
}
