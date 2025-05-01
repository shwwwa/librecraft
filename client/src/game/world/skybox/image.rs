use bevy::{platform::collections::HashMap, prelude::*};

use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use itertools::Itertools;

/// Error that could happened while processing generic skybox image.
#[derive(Debug, Clone, Copy)]
pub enum ImageError {
    /** Happens if image can't be converted to rgba8 standard. */
    DecodeFailed,
    /** Happens if background color was not identified through `find_background` algorithm. */
    BackgroundNotDetermined,
    /** Happens if background/non-background pixel was not found. */
    NetNotFound,
    /** Happens if something in image is not aligned. */
    NotAligned,
    /** Happens if something happened while copying a face. */
    CopyError,
    /** Happens if asset can't be retrieved from `Handle`. */
    AssetNotFound,
}

/// Gets the skybox mesh image in required format - six squares, one above the other.
pub fn get_skybox(
    images: &ResMut<Assets<Image>>,
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

            Ok(measurements.to_image(&dyn_image_rgba)?)
        }
        None => Err(ImageError::AssetNotFound),
    }
}

/// Search horizontally from the left to find the first non-background pixel.
pub fn search_from_left(rgba: &DynamicImage, bg: Rgba<u8>, y: u32) -> Result<u32, ImageError> {
    for x in 0..rgba.width() {
        if rgba.get_pixel(x, y) != bg {
            return Ok(x);
        }
    }
    Err(ImageError::NetNotFound)
}

/// Search horizontally from the right to find the last background pixel.
pub fn search_from_right(rgba: &DynamicImage, bg: Rgba<u8>, y: u32) -> Result<u32, ImageError> {
    for x in (0..rgba.width()).rev() {
        if rgba.get_pixel(x, y) != bg {
            return Ok(x + 1);
        }
    }
    Err(ImageError::NetNotFound)
}

/// Search vertically from the top to find the first non-background pixel.
pub fn search_from_top(rgba: &DynamicImage, bg: Rgba<u8>, x: u32) -> Result<u32, ImageError> {
    for y in 0..rgba.height() {
        if rgba.get_pixel(x, y) != bg {
            return Ok(y);
        }
    }
    Err(ImageError::NetNotFound)
}

/// Search vertically from the bottom to find the last background pixel.
pub fn search_from_bottom(rgba: &DynamicImage, bg: Rgba<u8>, x: u32) -> Result<u32, ImageError> {
    for y in (0..rgba.height()).rev() {
        if rgba.get_pixel(x, y) != bg {
            return Ok(y + 1);
        }
    }
    Err(ImageError::NetNotFound)
}

/// Searches for 8 points in the extreme sectors of image where we except background to be.
///
/// This is over complicated and heavy, but may help to recover from image errors.
pub fn find_background(rgba: &DynamicImage) -> Result<Rgba<u8>, ImageError> {
    let samples: Vec<Rgba<u8>> = (0..4)
        .cartesian_product(0..2)
        .map(|(x, y)| {
            rgba.get_pixel(
                (x * 2 + 1) * rgba.width() / 8,
                (y * 4 + 1) * rgba.height() / 6,
            )
        })
        .collect::<Vec<Rgba<u8>>>();

    // Find the most common background color.
    let mut sample_freq = HashMap::<Rgba<u8>, usize>::new();
    for sample in samples {
        *sample_freq.entry(sample).or_insert(0) += 1;
    }
    let mut sample_hist = sample_freq.drain().collect::<Vec<(Rgba<u8>, usize)>>();
    sample_hist.sort_by(|a, b| (a.1).cmp(&b.1));

    if let Some(background) = sample_hist.iter().last() {
        // At least half should be the background color.
        if background.1 >= 4 {
            Ok(background.0)
        } else {
            Err(ImageError::BackgroundNotDetermined)
        }
    } else {
        Err(ImageError::BackgroundNotDetermined)
    }
}

/// Measurements of positions in pixels.
/// See `image` module docs for the explanation of the indices.
pub struct ImageMeasurements {
    vec_x: Vec<u32>,
    vec_y: Vec<u32>,
}

impl ImageMeasurements {
    /// Transforms `DynamicImage` to bevy `Image` format, by copying 6 faces.
    pub fn to_image(&self, rgba: &DynamicImage) -> Result<Image, ImageError> {
        let side = self.measure_side_length();
        let mut image = RgbaImage::new(side, side * 6);

        // +X
        self.copy_face(rgba, &mut image, side, 3, 1, 0)?;
        // -X
        self.copy_face(rgba, &mut image, side, 1, 1, 1)?;
        // +Y
        self.copy_face(rgba, &mut image, side, 2, 0, 2)?;
        // -Y
        self.copy_face(rgba, &mut image, side, 2, 2, 3)?;
        // +Z
        self.copy_face(rgba, &mut image, side, 2, 1, 4)?;
        // -Z
        self.copy_face(rgba, &mut image, side, 0, 1, 5)?;

        let new_image = Image::from_dynamic(
            image::DynamicImage::from(image),
            true,
            bevy::asset::RenderAssetUsages::all(),
        );

        Ok(new_image)
    }

    /// Copies a face as part of new image creation.
    fn copy_face(
        &self,
        rgba: &DynamicImage,
        image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        side: u32,
        x_idx: usize,
        y_idx: usize,
        out_idx: usize,
    ) -> Result<(), ImageError> {
        let offset_x = (self.vec_x[x_idx + 1] - self.vec_x[x_idx] - side) / 2;
        let offset_y = (self.vec_y[y_idx + 1] - self.vec_y[y_idx] - side) / 2;
        image
            .copy_from(
                &rgba
                    .view(
                        self.vec_x[x_idx] + offset_x,
                        self.vec_y[y_idx] + offset_y,
                        side,
                        side,
                    )
                    .to_image(),
                0,
                side * (out_idx as u32),
            )
            .map_err(|_| ImageError::CopyError)
    }

    /// Finds the dimensions of the skybox net in the image.
    pub fn find_measurements(rgba: &DynamicImage) -> Result<Self, ImageError> {
        // Find the background color.
        let background = find_background(&rgba)?;
        // Measure the x values of the vertical edges of the net.
        let dy = rgba.height() / 6;
        let mid_x_min = search_from_left(&rgba, background, dy * 3)?;
        let mid_x_max = search_from_right(&rgba, background, dy * 3)?;
        let top_x_min = search_from_left(&rgba, background, dy * 1)?;
        let top_x_max = search_from_right(&rgba, background, dy * 1)?;
        let bot_x_min = search_from_left(&rgba, background, dy * 5)?;
        let bot_x_max = search_from_right(&rgba, background, dy * 5)?;
        if (top_x_min as i32 - bot_x_min as i32).abs() > 8 {
            return Err(ImageError::NotAligned);
        }
        if (top_x_max as i32 - bot_x_max as i32).abs() > 8 {
            return Err(ImageError::NotAligned);
        }
        let short_x_min = (top_x_min + bot_x_min) / 2;
        let short_x_max = (top_x_max + bot_x_max) / 2;
        // Assuming the shape, calculate the x values of the vertices and check them.
        let vec_x = vec![
            mid_x_min,
            (short_x_min + mid_x_min) / 2,
            short_x_min,
            short_x_max,
            mid_x_max,
        ];
        let mut diff_x = vec_x
            .as_slice()
            .windows(2)
            .map(|w| w[1] as i32 - w[0] as i32)
            .collect::<Vec<i32>>();
        diff_x.sort_unstable();
        if diff_x[3] - diff_x[0] > 16 {
            return Err(ImageError::NotAligned);
        }

        // Measure the y values of the horizontal edges of the net.
        let mid_y_min = search_from_top(&rgba, background, (vec_x[2] + vec_x[3]) / 2)?;
        let mid_y_max = search_from_bottom(&rgba, background, (vec_x[2] + vec_x[3]) / 2)?;
        let left_y_min = search_from_top(&rgba, background, vec_x[1])?;
        let left_y_max = search_from_bottom(&rgba, background, vec_x[1])?;
        let right_y_min = search_from_top(&rgba, background, (vec_x[3] + vec_x[4]) / 2)?;
        let right_y_max = search_from_bottom(&rgba, background, (vec_x[3] + vec_x[4]) / 2)?;
        if (left_y_min as i32 - right_y_min as i32).abs() > 8 {
            return Err(ImageError::NotAligned);
        }
        if (left_y_max as i32 - right_y_max as i32).abs() > 8 {
            return Err(ImageError::NotAligned);
        }
        let short_y_min = (left_y_min + right_y_min) / 2;
        let short_y_max = (left_y_max + right_y_max) / 2;

        // Assuming the shape, calculate the y values to return and check them.
        let vec_y = vec![mid_y_min, short_y_min, short_y_max, mid_y_max];
        let mut diff_y = vec_y
            .as_slice()
            .windows(2)
            .map(|w| w[1] as i32 - w[0] as i32)
            .collect::<Vec<i32>>();
        diff_y.sort_unstable();
        if diff_y[2] - diff_y[0] > 16 {
            return Err(ImageError::NotAligned);
        }

        Ok(ImageMeasurements { vec_x, vec_y })
    }

    /// Determines the size of each image in the net, assuming that they all have to be the same
    /// and are all square, so that we can copy pixel for pixel into the output without needing to scale.
    ///
    /// Use minimums to avoid overlapping outside the net or even the source image.
    fn measure_side_length(&self) -> u32 {
        let min_x = self
            .vec_x
            .windows(2)
            .map(|x| x[1] - x[0])
            .min()
            .expect("Four x intervals");
        let min_y = self
            .vec_y
            .windows(2)
            .map(|y| y[1] - y[0])
            .min()
            .expect("Three y intervals");
        let side = min_x.min(min_y);
        side
    }
}
