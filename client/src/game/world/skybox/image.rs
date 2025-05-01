use bevy::prelude::*;

use image::{DynamicImage, GenericImageView, ImageReader, Rgba};
use itertools::Itertools;

use super::SkyboxPlugin;

/// Error that could happened while processing generic skybox image.
#[derive(Debug, Clone, Copy)]
pub enum ImageError {
    /** Happens if image can't be converted to rgba8 standard. */
    DecodeFailed,
    /** Happens if background color was not identified through `find_background` algorithm. */
    BackgroundNotDetermined,
    NetNotFound,
    NotAligned,
    CopyError,
    /** Happens if asset can't be retrieved from `Handle`. */
    AssetNotFound,
}

pub fn get_skybox(
    images: ResMut<Assets<Image>>,
    handle: &Handle<Image>,
) -> Result<Image, ImageError> {
    match images.get(handle.id()) {
        Some(image) => {
            let dyn_image = image
                .clone()
                .try_into_dynamic()
                .map_err(|_| ImageError::DecodeFailed)?;
            let dyn_image_rgba = DynamicImage::ImageRgba8(dyn_image.to_rgba8());

            let measurements = ImageMeasurements::find_measurements(&dyn_image_rgba)?;

            Err(ImageError::CopyError)
        }
        None => Err(ImageError::AssetNotFound),
    }
}

pub struct ImageMeasurements {
    vec_x: Vec<u32>,
    vec_y: Vec<u32>,
}

impl ImageMeasurements {
    pub fn to_image(&self, rgba: &DynamicImage) -> Result<Image, ImageError> {
        Err(ImageError::NetNotFound)
    }

    pub fn find_measurements(rgba: &DynamicImage) -> Result<Self, ImageError> {
        let background = Self::find_background(&rgba)?;

        warn!("{:?}", background);

        Ok(ImageMeasurements {
            vec_x: vec![],
            vec_y: vec![],
        })
    }

    pub fn find_background(rgba: &DynamicImage) -> Result<Rgba<u8>, ImageError> {
        let samples: Vec<Rgba<u8>> = (0..4)
            .cartesian_product(0..2)
            .map(|(x, y)| {
                rgba.get_pixel(
                    (x * 2 + 1) * rgba.width() / 8,
                    (y * 4 + 1) * rgba.height() / 6,
                )
            })
            .collect();

        Ok(samples[0])
    }
}
