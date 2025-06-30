use crate::builder::text_builder::TextBuilder;
use bevy::prelude::{Commands, Entity};

pub fn add_key_value_pairs(
    pairs: &[(&str, Entity)],
    key_values_panel: Entity,
    commands: &mut Commands,
) {
    let mut children: Vec<Entity> = vec![];
    pairs.iter().for_each(|(s, entity)| {
        children.push(TextBuilder::new().content(*s).spawn(commands));
        children.push(*entity);
    });
    commands.entity(key_values_panel).add_children(&children);
}
