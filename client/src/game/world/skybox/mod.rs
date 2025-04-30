use bevy::asset::{load_internal_asset, weak_handle};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;

/// The `Handle` for the shader for the `SkyboxMaterial`
/// Generated with `Uuid::new_v4()`
pub const SKYBOX_SHADER_HANDLE: Handle<Shader> =
    weak_handle!("7c1959f7-9ad5-4c78-b7f6-fd57e1e1a76a");

#[derive(Debug, Clone, Copy)]
pub struct SkyboxPlugin;

/// A procedural sky plugin for the bevy game engine. Based on bevy_atmosphere but over-simplified.
impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            SKYBOX_SHADER_HANDLE,
            "shaders/skybox.wgsl",
            Shader::from_wgsl
        );
    }
}

/// A marker `Component` for a `Camera` that receives a skybox.
///
/// When added, a `Skybox` will be created as a child.
/// When removed, that `Skybox` will also be removed.
#[derive(Component, Default, Debug, Clone)]
pub struct SkyboxCamera {
    /// Controls whether or not the skybox will be seen only on certain render layers.
    pub render_layers: Option<RenderLayers>,
}

/// A marker `Component` for skybox entities.
///
/// Added for the skybox generated when an `SkyboxCamera` is detected.
#[derive(Component, Debug, Clone, Copy)]
pub struct Skybox;
