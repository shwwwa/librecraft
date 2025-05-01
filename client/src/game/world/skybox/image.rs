use bevy::prelude::*;

use image::{DynamicImage, GenericImageView, ImageReader, Rgba};
use itertools::Itertools;

use super::SkyboxPlugin;

/// Error that could happened while processing generic skybox image.
#[derive(Debug, Clone, Copy)]
pub enum ImageError {
    FileNotFound,
    DecodeFailed,
    /** Happens if background color was not identified through `find_background algorithm. */
    BackgroundNotDetermined,
    NetNotFound,
    NotAligned,
    CopyError,
}

pub fn get_skybox(
    mut images: ResMut<Assets<Image>>,
    handle: &Handle<Image>,
) -> Result<Image, ImageError> {
    images.get(handle.id());

    Err(ImageError::DecodeFailed)
}

pub struct ImageMeasurements {
    vec_x: Vec<u32>,
    vec_y: Vec<u32>,
}

impl ImageMeasurements {
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
