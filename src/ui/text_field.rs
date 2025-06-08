use bevy::app::{App, Plugin};
use bevy::prelude::{BackgroundColor, Button, Changed, Children, Interaction, Query, With};
use crate::builder::text_field_builder::TextField;

pub struct TextFieldPlugin;

impl Plugin for TextFieldPlugin {
    fn build(&self, app: &mut App) {}
}

fn text_field_hover(mut interaction_query: Query<
    (
        &Interaction,
        &mut BackgroundColor,
        &mut TextField,
        &Children,
    ),
    (Changed<Interaction>, With<Button>),
>) {
    todo!()
}