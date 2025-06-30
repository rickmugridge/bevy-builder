use crate::builder::node_builder::{NodeBuilder, NodeBundle};
use crate::ui_plugin::number_plugin::NumberChangedEvent;
use bevy::color::palettes::basic::{RED, TEAL};
use bevy::prelude::*;
use bevy::ui::ContentSize;
use bevy::ui::widget::TextNodeFlags;
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
    content: String,
    font: TextFont,
    color: Color,
    justify_text: JustifyText,
    linebreak: LineBreak,
    shadow_color: Color,
    shadow_offset: Vec2,
    node_bundle: NodeBundle,
}

impl TextFieldBuilder {
    pub fn new() -> Self {
        Self {
            text_field: TextField::default(),
            content: "Default".to_string(),
            font: TextFont::default(),
            color: Color::BLACK,
            justify_text: JustifyText::default(),
            linebreak: LineBreak::default(),
            shadow_color: Color::NONE,
            shadow_offset: Vec2::ZERO,
            node_bundle: NodeBundle::default(),
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

    pub fn content<S: Into<String>>(mut self, content: S) -> Self {
        self.content = content.into();
        self
    }

    pub fn font(mut self, font: TextFont) -> Self {
        self.font = font;
        self
    }

    pub fn node(mut self, node_bundle: NodeBundle) -> Self {
        self.node_bundle = node_bundle;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn justify_text(mut self, justify_text: JustifyText) -> Self {
        self.justify_text = justify_text;
        self
    }

    pub fn linebreak(mut self, linebreak: LineBreak) -> Self {
        self.linebreak = linebreak;
        self
    }

    pub fn shadow(mut self, color: Color, offset: Vec2) -> Self {
        self.shadow_color = color;
        self.shadow_offset = offset;
        self
    }
    
    pub fn spawn(self, commands: &mut Commands) -> Entity {
        let text = (
            Text::new(self.content),
            self.text_field,
            self.font,
            TextColor(self.color),
            TextLayout::new(self.justify_text, self.linebreak),
            TextShadow {
                color: self.shadow_color,
                offset: self.shadow_offset,
            },
            TextNodeFlags::default(),
            ContentSize::default(),
            self.node_bundle,
        );
        let border_node = NodeBuilder::new()
            .border_of(Val::Px(1.), TEAL.into())
            .background_color(Color::WHITE)
            .spawn(commands);
        let text_entity = commands.spawn(text).id();
        let error_text = commands
            .spawn((
                Text::new("Error"),
                TextColor(RED.into()),
                Visibility::Hidden,
            ))
            .id();
        commands
            .entity(border_node)
            .add_children(&[text_entity, error_text]);
        border_node
    }
}
