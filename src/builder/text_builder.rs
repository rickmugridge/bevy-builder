use crate::edit_plugin::text_edit_plugin::TextContentReactor;
use bevy::prelude::*;
use bevy::ui::ContentSize;
use bevy::ui::widget::TextNodeFlags;

pub struct TextBuilder {
    content: String,
    font: TextFont,
    color: Color,
    justify_text: JustifyText,
    linebreak: LineBreak,
    shadow_color: Color,
    shadow_offset: Vec2,
    node: Node,
    text_content_reactor: Option<TextContentReactor>,
}

impl TextBuilder {
    pub fn new() -> Self {
        Self {
            content: "Default".to_string(),
            font: TextFont::default(),
            color: Color::WHITE,
            justify_text: JustifyText::default(),
            linebreak: LineBreak::default(),
            shadow_color: Color::NONE,
            shadow_offset: Vec2::ZERO,
            node: Node::default(),
            text_content_reactor: None,
        }
    }

    pub fn content<S: Into<String>>(mut self, content: S) -> Self {
        self.content = content.into();
        self
    }

    pub fn font(mut self, font: TextFont) -> Self {
        self.font = font;
        self
    }

    pub fn node(mut self, node: Node) -> Self {
        self.node = node;
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

    pub fn text_content_reactor(mut self, source_id: String) -> Self {
        self.text_content_reactor = Some(TextContentReactor { source_id });
        self
    }

    pub fn build_and_spawn(self, commands: &mut Commands) -> Entity {
        let text = (
            Text::new(self.content),
            self.font,
            TextColor(self.color),
            TextLayout::new(self.justify_text, self.linebreak),
            TextShadow {
                color: self.shadow_color,
                offset: self.shadow_offset,
            },
            TextNodeFlags::default(),
            ContentSize::default(),
            self.node,
        );
        let id = commands.spawn(text).id();
        if let Some(text_content_reactor) = self.text_content_reactor {
            commands.entity(id).insert(text_content_reactor);       
        }
        id
    }
}
