use bevy::prelude::*;
use bevy_pixel::{prelude::*, texture::TexturePixelCamera};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), TexturePixelCamera::default()));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("tile_0006.png"),
        ..Default::default()
    });
}
