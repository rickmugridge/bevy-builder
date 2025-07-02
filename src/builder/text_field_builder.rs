use crate::builder::node_builder::{NodeBuilder, NodeBundle};
use crate::builder::text_builder::{TextBuilder, TextBundle};
use crate::ui_plugin::number_plugin::NumberChangedEvent;
use bevy::color::palettes::basic::{RED, TEAL};
use bevy::prelude::*;
use std::sync::Arc;

#[derive(Component, Default)]
#[require(Interaction)]
pub struct TextField {
    pub on_change: Option<TextChangeCallback>,
    pub validate: Option<TextValidationCallback>,
    pub invalid: bool,
}

type TextChangeCallback = Arc<dyn Fn(String, &mut Commands) + Send + Sync>;
type TextValidationCallback = Arc<dyn Fn(String) -> Option<String> + Send + Sync>; // None means it's valid

pub struct TextFieldBuilder {
    text_field: TextField,
    text_bundle: TextBundle,
    border_node_bundle: NodeBundle,
    error_text_bundle: TextBundle,
}

impl TextFieldBuilder {
    pub fn new() -> Self {
        Self {
            text_field: TextField::default(),
            text_bundle: TextBundle::default(),
            border_node_bundle: NodeBuilder::new()
                .border_of(Val::Px(1.), TEAL.into())
                .background_color(Color::WHITE)
                .bundle(),
            error_text_bundle: TextBuilder::new()
                .content("Error")
                .color(RED.into())
                .bundle(),
        }
    }

    pub fn on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(String, &mut Commands) + Send + Sync + 'static,
    {
        self.text_field.on_change = Some(Arc::new(callback));
        self
    }

    pub fn on_change_to_number(self, source_id: impl Into<String>) -> Self {
        let source_id = source_id.into();
        self.on_change(move |content, commands| {
            if let Ok(value) = content.parse::<f32>() {
                let source_id = source_id.clone();
                commands.queue(move |w: &mut World| {
                    w.send_event(NumberChangedEvent { source_id, value });
                });
            }
        })
        .validate_is_number()
    }

    pub fn validate<F>(mut self, callback: F) -> Self
    where
        F: Fn(String) -> Option<String> + Send + Sync + 'static,
    {
        self.text_field.validate = Some(Arc::new(callback));
        self
    }

    pub fn validate_is_number(self) -> Self {
        self.validate(|content| content.parse::<f32>().err().map(|_| "Not a number".into()))
    }

    pub fn text_bundle(mut self, text_bundle: TextBundle) -> Self {
        self.text_bundle = text_bundle;
        self
    }

    pub fn spawn(self, commands: &mut Commands) -> Entity {
        let border_node = commands.spawn(self.border_node_bundle).id();
        let text_entity = commands.spawn((self.text_bundle, self.text_field)).id();
        let error_text = commands
            .spawn((self.error_text_bundle, Visibility::Hidden))
            .id();
        commands
            .entity(border_node)
            .add_children(&[text_entity, error_text]);
        border_node
    }
}
