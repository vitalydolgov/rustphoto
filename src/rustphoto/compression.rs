// JPEG compression with quality-based size targeting.
//
// Uses binary search on JPEG quality parameter (1-100) to find the highest
// quality setting that produces a file within the target size constraint.
// Minimum quality 10 prevents extremely degraded output.

use super::error::ProcessError;
use super::image::Image;
use image::codecs::jpeg::JpegEncoder;
use std::io::Cursor;

fn encode_jpeg_to_buffer(image: &Image, quality: u8) -> Result<Vec<u8>, ProcessError> {
    let mut buffer = image::RgbImage::new(image.width as u32, image.height as u32);

    for (i, pixel) in image.pixels.iter().enumerate() {
        let x = (i as i32 % image.width) as u32;
        let y = (i as i32 / image.width) as u32;
        buffer.put_pixel(x, y, image::Rgb([pixel.r, pixel.g, pixel.b]));
    }

    let mut encoded = Cursor::new(Vec::new());
    let mut encoder = JpegEncoder::new_with_quality(&mut encoded, quality);
    encoder
        .encode(
            buffer.as_raw(),
            image.width as u32,
            image.height as u32,
            image::ExtendedColorType::Rgb8,
        )
        .map_err(|e| ProcessError::JpegEncoding(Box::new(e)))?;

    Ok(encoded.into_inner())
}

pub fn save_jpeg_compressed(
    image: &Image,
    path: &str,
    max_size: usize,
) -> Result<(), ProcessError> {
    let mut low = 1u8;
    let mut high = 100u8;
    let mut best: Option<(u8, Vec<u8>)> = None;

    while low <= high {
        let mid = (low + high) / 2;
        let encoded = encode_jpeg_to_buffer(image, mid)?;
        let size = encoded.len();

        if size <= max_size {
            best = Some((mid, encoded));

            if size as f64 >= max_size as f64 * 0.99 {
                break;
            }

            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    match best {
        Some((_quality, data)) => {
            std::fs::write(path, data).map_err(|e| ProcessError::FileWrite {
                path: path.to_string(),
                source: Box::new(e),
            })?;
            Ok(())
        }
        None => {
            let min_size = encode_jpeg_to_buffer(image, 10)?.len();
            Err(ProcessError::CompressionTargetTooSmall {
                target_kb: max_size / 1024,
                target_bytes: max_size,
                min_kb: min_size / 1024,
                min_bytes: min_size,
            })
        }
    }
}
