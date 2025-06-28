use crate::edit_plugin::text_edit_plugin::{TextColorReactor, TextContentReactor};
use bevy::prelude::*;
use bevy::ui::ContentSize;
use bevy::ui::widget::TextNodeFlags;
use crate::builder::node_builder::NodeBundle;

pub struct TextBuilder {
    content: String,
    font: TextFont,
    color: Color,
    justify_text: JustifyText,
    linebreak: LineBreak,
    shadow_color: Color,
    shadow_offset: Vec2,
    inner_node_bundle: NodeBundle,
    outer_node_bundle: NodeBundle,
    text_content_reactor: Option<TextContentReactor>,
    text_color_reactor: Option<TextColorReactor>,
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
            inner_node_bundle: NodeBundle::default(),
            outer_node_bundle: NodeBundle::default(),
            text_content_reactor: None,
            text_color_reactor: None,
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

    pub fn inner_node_bundle(mut self, inner_node_bundle: NodeBundle) -> Self {
        self.inner_node_bundle = inner_node_bundle;
        self
    }

    pub fn outer_node_bundle(mut self, outer_node_bundle: NodeBundle) -> Self {
        self.outer_node_bundle = outer_node_bundle;
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

    pub fn text_content_reactor(mut self, source_id: &str) -> Self {
        self.text_content_reactor = Some(TextContentReactor { source_id : source_id.into()});
        self
    }

    pub fn text_color_reactor(mut self, source_id:  &str) -> Self {
        self.text_color_reactor = Some(TextColorReactor { source_id: source_id.into() });
        self
    }

    pub fn build_and_spawn(self, commands: &mut Commands) -> Entity {
        let outer = commands.spawn(self.outer_node_bundle).id();
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
            self.inner_node_bundle,
        );
        let text_entity = commands.spawn(text).id();
        if let Some(text_content_reactor) = self.text_content_reactor {
            commands.entity(text_entity).insert(text_content_reactor);
        }
       if let Some(text_color_reactor) = self.text_color_reactor {
            commands.entity(text_entity).insert(text_color_reactor);
        }
        commands.entity(outer).add_child(text_entity);
        outer
    }
}
