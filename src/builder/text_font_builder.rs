use bevy::prelude::*;
use bevy::text::{FontSmoothing, LineHeight};

pub struct TextFontBuilder {
    font: TextFont,
}

impl Default for TextFontBuilder {
    fn default() -> Self {
        Self {
            font: TextFont::default(),
        }
    }
}

impl TextFontBuilder {
    pub fn new() -> Self {
        TextFontBuilder::default()
    }

    pub fn load_font(mut self, asset_server: &AssetServer, font_name: &str) -> Self {
        self.font.font = asset_server.load(font_name);
        self
    }

    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font.font_size = font_size;
        self
    }

    pub fn line_height(mut self, line_height: LineHeight) -> Self {
        self.font.line_height = line_height;
        self
    }

    pub fn font_smoothing(mut self, font_smoothing: FontSmoothing) -> Self {
        self.font.font_smoothing = font_smoothing;
        self
    }

    pub fn build(self) -> TextFont {
        self.font
    }
}
