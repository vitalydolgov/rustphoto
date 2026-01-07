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

    pub fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
        }
    }

    pub fn to_int(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}

#[derive(Clone)]
pub struct Image {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) pixels: Vec<Pixel>,
}

impl Image {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let img = image::open(path)?;
        let rgb = img.to_rgb8();
        let (width, height) = rgb.dimensions();

        let pixels: Vec<Pixel> = rgb.pixels().map(|p| Pixel::new(p[0], p[1], p[2])).collect();

        Ok(Self {
            width: width as usize,
            height: height as usize,
            pixels,
        })
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = image::RgbImage::new(self.width as u32, self.height as u32);

        for (i, pixel) in self.pixels.iter().enumerate() {
            let x = (i % self.width) as u32;
            let y = (i / self.width) as u32;
            buffer.put_pixel(x, y, image::Rgb([pixel.r, pixel.g, pixel.b]));
        }

        buffer.save(path)?;
        Ok(())
    }
}
