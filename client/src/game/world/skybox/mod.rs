use bevy::asset::{load_internal_asset, weak_handle};
use bevy::prelude::*;

pub mod image;
pub use image::*;

/// A procedural skybox plugin for Bevy. Based on bevy_atmosphere and bevy_skybox but over-simplified.
#[derive(Debug, Clone, Resource)]
pub struct SkyboxPlugin {
    /// String with path to image in [`AssetServer`].
    /// Automatically creates a handle if it wasn't passed.
    image: Option<String>,
    handle: Option<Handle<Image>>,
}

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.clone());
        load_internal_asset!(
            app,
            SKYBOX_SHADER_HANDLE,
            "shaders/skybox.wgsl",
            Shader::from_wgsl
        );
    }
}

impl SkyboxPlugin {
    /// Contains procedural skybox for Bevy.
    /// Accepts `image` that automatically creates its handle, or you can just pass it with file.
    ///
    /// Creates [`Skybox`] entity from all cameras that have [`SkyboxCamera`] component.
    pub fn from_image_file(image: &str) -> SkyboxPlugin {
        Self {
            image: Some(image.to_owned()),
            handle: None,
        }
    }

    /// Removes [`Skybox`] entity from all cameras that have [`SkyboxCamera`] component.
    pub fn empty() -> SkyboxPlugin {
        Self {
            image: None,
            handle: None,
        }
    }
}

/// A marker `Component` for a `Camera` that receives a skybox.
///
/// When added, a `Skybox` will be created as a child if [`SkyboxPlugin`] has image or handle.
/// When removed, that `Skybox` will also be removed.
#[derive(Component, Default, Debug, Clone)]
pub struct SkyboxCamera;

/// A marker `Component` for skybox entities.
///
/// Added for the skybox generated when an `SkyboxCamera` is detected.
#[derive(Component, Debug, Clone, Copy)]
pub struct Skybox;

/// The `Handle` for the shader for the [`SkyboxMaterial`]
///
/// Generated with `Uuid::new_v4()`
pub const SKYBOX_SHADER_HANDLE: Handle<Shader> =
    weak_handle!("7c1959f7-9ad5-4c78-b7f6-fd57e1e1a76a");
