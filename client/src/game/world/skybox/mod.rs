use bevy::asset::{load_internal_asset, weak_handle};
use bevy::core_pipeline::Skybox;
use bevy::image::CompressedImageFormats;
use bevy::prelude::*;
use bevy::render::renderer::RenderDevice;

pub mod image;
pub use image::*;
use wgpu_types::{TextureViewDescriptor, TextureViewDimension};

/// The `Handle` for the shader for the [`SkyboxMaterial`]
///
/// Generated with `Uuid::new_v4()`
pub const SKYBOX_SHADER_HANDLE: Handle<Shader> =
    weak_handle!("7c1959f7-9ad5-4c78-b7f6-fd57e1e1a76a");

/// A procedural skybox plugin for Bevy. Based on bevy_atmosphere and bevy_skybox, but over-simplified.
///
/// Requires bevy_asset, bevy_render, bevy_image in order to work.
///
/// Scans [`Camera3d`] that have [`SkyboxCamera`] component on it.
#[derive(Debug, Clone, Resource)]
pub struct SkyboxPlugin {
    /// String with path to image in [`AssetServer`].
    /// Is none if asset can't be loaded.
    /// Automatically creates a handle if it wasn't passed.
    image: Option<String>,
    /// Automatically set whenever image is changed.
    /// If handle is absent, check logs for error.
    handle: Option<Handle<Image>>,
}

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.clone())
            .add_systems(Startup, (check_device_features,))
            .add_systems(
                Update,
                (detect_new_cameras, load_skybox_image, create_skybox),
            );

        // todo: test usage of skybox shader.
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
    /// Accepts `image` that automatically creates its `handle`, so you don't need to pass it.
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
/// When added, a [`Skybox`] will be created as a child if [`SkyboxPlugin`] has image or handle.
/// When removed, that [`Skybox`] will also be removed.
#[derive(Component, Default, Debug, Clone)]
pub struct SkyboxCamera;

fn check_device_features(render_device: Res<RenderDevice>) {
    let features = render_device.features();
    let formats = CompressedImageFormats::from_features(features);
    if !formats.contains(CompressedImageFormats::NONE) {
        error!("Uncompressed format of images needs support for skybox to work.");
        return;
    }
}

fn load_skybox_image(
    assets: Res<AssetServer>,
    mut commands: Commands,
    mut plugin: ResMut<SkyboxPlugin>,
    camera_q: Query<Entity, With<SkyboxCamera>>,
) {
    if let Some(image) = &plugin.image {
        if let Some(handle) = &plugin.handle {
            let state = assets.load_state(handle.id());
            if state.is_failed() {
                error!("Skybox image can't be loaded.");
                *plugin = SkyboxPlugin::empty();
            }
        } else {
            plugin.handle = Some(assets.load(image));
        }
    } else {
        for cam in camera_q.iter() {
            commands.entity(cam).remove::<Skybox>();
        }
    }
}

fn create_skybox(
    assets: Res<AssetServer>,
    mut commands: Commands,
    mut plugin: ResMut<SkyboxPlugin>,
    mut images: ResMut<Assets<Image>>,
    camera_q: Query<Entity, (Added<Camera3d>, With<SkyboxCamera>)>,
) {
    if let Some(handle) = &plugin.handle {
        let state = assets.load_state(handle.id());
        if !state.is_loaded() {
            return;
        }

        match image::get_skybox(&images, handle) {
            Ok(mut image) => {
                if image.texture_descriptor.array_layer_count() != 1 {
                    error!("Array layer count is incorrect.");
                    *plugin = SkyboxPlugin::empty();
                }

                image.reinterpret_stacked_2d_as_array(6);
                if image.texture_descriptor.array_layer_count() != 6 {
                    error!("Array layer count is incorrect after reinterpreting.");
                    *plugin = SkyboxPlugin::empty();
                }

                image.texture_view_descriptor = Some(TextureViewDescriptor {
                    dimension: Some(TextureViewDimension::Cube),
                    ..default()
                });

                let skybox_handle = images.add(image);
                plugin.handle = Some(skybox_handle.clone());

                for cam in camera_q.iter() {
                    insert_skybox_camera(&mut commands, cam, &skybox_handle);
                }
                warn!("syccess!");
            }
            Err(e) => {
                error!("Skybox image is incorrect: {:?}", e);
                *plugin = SkyboxPlugin::empty();
            }
        }
    }
}

/// System that detects new cameras with [`SkyboxCamera`] component.
fn detect_new_cameras(
    mut commands: Commands,
    plugin: Res<SkyboxPlugin>,
    camera_q: Query<Entity, (Added<Camera3d>, With<SkyboxCamera>)>,
) {
    if let Some(handle) = &plugin.handle {
        for cam in camera_q.iter() {
            insert_skybox_camera(&mut commands, cam, handle);
        }
    }
}

/// Inserts `Skybox` to `cam` `Entity`.
fn insert_skybox_camera(commands: &mut Commands, cam: Entity, handle: &Handle<Image>) {
    commands.entity(cam).insert(Skybox {
        image: handle.clone(),
        brightness: 1000.,
        ..default()
    });
}
