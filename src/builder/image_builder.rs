// I can't find any examples of using images in the UI for 0.16.

// use bevy::prelude::*;
// 
// pub struct ImageBuilder {
//     image: Image,
// }
// 
// impl ImageBuilder {
//     pub fn new() -> Self {
//         Self {
//             image: Image::default(),
//         }
//     }
// 
//     pub fn load_image(mut self, asset_server: &AssetServer, image_path: &str) -> Self {
//         self.image = Image::asset_server.load(image_path);
//         self
//     }
// 
//     pub fn build(self) -> Image {
//         self.image
//     }
// 
//     pub fn spawn(self, commands: &mut Commands) -> Entity {
//         commands.spawn(self.image).id()
//     }
// }
// 
// impl Default for ImageBuilder {
//     fn default() -> Self {
//         Self::new()
//     }
// }