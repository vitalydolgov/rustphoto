// Image representation module.
//
// # Data Structures
//
// - `Pixel`: RGB color representation with 8-bit channels (0-255).
//
// - `Image`: Contains width, height (i32), and a flat pixel array.
//   Pixels are stored in row-major order: `pixels[y * width + x]`.
//
// # Type Conversions
//
// - Loading: `u32` (image crate) → `i32` (internal)
// - Saving: `i32` (internal) → `u32` (image crate)

#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_f32(r: f32, g: f32, b: f32) -> Self {
        Self {
            r: r.clamp(0.0, 255.0) as u8,
            g: g.clamp(0.0, 255.0) as u8,
            b: b.clamp(0.0, 255.0) as u8,
        }
    }

    pub fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
        }
    }
}

pub struct Image {
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) pixels: Vec<Pixel>,
}

impl Image {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let img = image::open(path)?;
        let rgb = img.to_rgb8();
        let (width, height) = rgb.dimensions();

        let pixels: Vec<Pixel> = rgb.pixels().map(|p| Pixel::new(p[0], p[1], p[2])).collect();

        Ok(Self {
            width: width as i32,
            height: height as i32,
            pixels,
        })
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = image::RgbImage::new(self.width as u32, self.height as u32);

        for (i, pixel) in self.pixels.iter().enumerate() {
            let x = (i as i32 % self.width) as u32;
            let y = (i as i32 / self.width) as u32;
            buffer.put_pixel(x, y, image::Rgb([pixel.r, pixel.g, pixel.b]));
        }

        buffer.save(path)?;
        Ok(())
    }
}
