use super::image::{Image, Pixel};

// Image transformations module.
//
// # Coordinate Type Conventions
//
// - `i32`: All coordinates (x, y), dimensions (width, height), sizes, and offsets.
//
// - `usize`: Only for array indexing: `pixels[idx as usize]`.
//   Required by Rust's Vec/slice indexing.
//
// - `u32`: Only at image library boundaries (converting to/from the `image` crate).
//
// - `f32`: Intermediate calculations for scaling, blending, and color operations.
//
// ## Indexing Pattern
//
// All coordinate arithmetic is done in i32, with a single cast to usize for indexing:
// ```
// let idx = (y * width + x) as usize;
// let pixel = pixels[idx];
// ```

#[derive(Debug)]
pub enum TransformError {
    OutOfBounds,
}

impl std::fmt::Display for TransformError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransformError::OutOfBounds => write!(f, "Region is out of bounds"),
        }
    }
}

impl std::error::Error for TransformError {}

pub trait Transformation {
    fn apply(&self, image: &Image) -> Result<Image, TransformError>;
}

// Geometric transformations

pub struct Crop {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Crop {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

impl Transformation for Crop {
    fn apply(&self, image: &Image) -> Result<Image, TransformError> {
        if self.x + self.width > image.width || self.y + self.height > image.height {
            return Err(TransformError::OutOfBounds);
        }

        let mut pixels = Vec::with_capacity((self.width * self.height) as usize);

        for dy in 0..self.height {
            for dx in 0..self.width {
                let src_idx = ((self.y + dy) * image.width + (self.x + dx)) as usize;
                pixels.push(image.pixels[src_idx]);
            }
        }

        Ok(Image {
            width: self.width,
            height: self.height,
            pixels,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FlipAxis {
    Horizontal,
    Vertical,
}

pub struct Flip {
    axis: FlipAxis,
}

impl Flip {
    pub fn new(axis: FlipAxis) -> Self {
        Self { axis }
    }
}

impl Transformation for Flip {
    fn apply(&self, image: &Image) -> Result<Image, TransformError> {
        let mut pixels = vec![Pixel::new(0, 0, 0); (image.width * image.height) as usize];

        match self.axis {
            FlipAxis::Horizontal => {
                for y in 0..image.height {
                    for x in 0..image.width {
                        let src_idx = (y * image.width + x) as usize;
                        let dst_x = image.width - 1 - x;
                        let dst_idx = (y * image.width + dst_x) as usize;
                        pixels[dst_idx] = image.pixels[src_idx];
                    }
                }
            }
            FlipAxis::Vertical => {
                for y in 0..image.height {
                    for x in 0..image.width {
                        let src_idx = (y * image.width + x) as usize;
                        let dst_y = image.height - 1 - y;
                        let dst_idx = (dst_y * image.width + x) as usize;
                        pixels[dst_idx] = image.pixels[src_idx];
                    }
                }
            }
        }

        Ok(Image {
            width: image.width,
            height: image.height,
            pixels,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RotateAngle {
    Deg90,
    Deg180,
    Deg270,
}

pub struct Rotate {
    angle: RotateAngle,
}

impl Rotate {
    pub fn new(angle: RotateAngle) -> Self {
        Self { angle }
    }
}

impl Transformation for Rotate {
    fn apply(&self, image: &Image) -> Result<Image, TransformError> {
        let (width, height) = match self.angle {
            RotateAngle::Deg90 | RotateAngle::Deg270 => (image.height, image.width),
            RotateAngle::Deg180 => (image.width, image.height),
        };

        let mut pixels = vec![Pixel::new(0, 0, 0); (width * height) as usize];

        match self.angle {
            RotateAngle::Deg90 => {
                for y in 0..image.height {
                    for x in 0..image.width {
                        let src_idx = (y * image.width + x) as usize;
                        let dst_x = image.height - 1 - y;
                        let dst_y = x;
                        let dst_idx = (dst_y * image.height + dst_x) as usize;
                        pixels[dst_idx] = image.pixels[src_idx];
                    }
                }
            }
            RotateAngle::Deg180 => {
                for y in 0..image.height {
                    for x in 0..image.width {
                        let src_idx = (y * image.width + x) as usize;
                        let dst_x = image.width - 1 - x;
                        let dst_y = image.height - 1 - y;
                        let dst_idx = (dst_y * image.width + dst_x) as usize;
                        pixels[dst_idx] = image.pixels[src_idx];
                    }
                }
            }
            RotateAngle::Deg270 => {
                for y in 0..image.height {
                    for x in 0..image.width {
                        let src_idx = (y * image.width + x) as usize;
                        let dst_x = y;
                        let dst_y = image.width - 1 - x;
                        let dst_idx = (dst_y * image.height + dst_x) as usize;
                        pixels[dst_idx] = image.pixels[src_idx];
                    }
                }
            }
        }

        Ok(Image {
            width,
            height,
            pixels,
        })
    }
}

pub struct Fit {
    max_width: i32,
    max_height: i32,
}

impl Fit {
    pub fn new(max_width: i32, max_height: i32) -> Self {
        Self {
            max_width,
            max_height,
        }
    }
}

impl Transformation for Fit {
    fn apply(&self, image: &Image) -> Result<Image, TransformError> {
        let scale_x = self.max_width as f32 / image.width as f32;
        let scale_y = self.max_height as f32 / image.height as f32;
        let scale = scale_x.min(scale_y).min(1.0);

        let new_width = (image.width as f32 * scale) as i32;
        let new_height = (image.height as f32 * scale) as i32;

        let mut pixels = vec![Pixel::new(0, 0, 0); (new_width * new_height) as usize];

        let x_ratio = image.width as f32 / new_width as f32;
        let y_ratio = image.height as f32 / new_height as f32;

        for dst_y in 0..new_height {
            for dst_x in 0..new_width {
                let src_x = (dst_x as f32 * x_ratio) as i32;
                let src_y = (dst_y as f32 * y_ratio) as i32;

                let src_idx = (src_y * image.width + src_x) as usize;
                let dst_idx = (dst_y * new_width + dst_x) as usize;

                pixels[dst_idx] = image.pixels[src_idx];
            }
        }

        Ok(Image {
            width: new_width,
            height: new_height,
            pixels,
        })
    }
}

// Pixel-to-pixel transformations

pub struct Invert;

impl Invert {
    pub fn new() -> Self {
        Self
    }
}

impl Transformation for Invert {
    fn apply(&self, image: &Image) -> Result<Image, TransformError> {
        let pixels: Vec<Pixel> = image
            .pixels
            .iter()
            .map(|p| Pixel::new(255 - p.r, 255 - p.g, 255 - p.b))
            .collect();

        Ok(Image {
            width: image.width,
            height: image.height,
            pixels,
        })
    }
}

pub struct Grayscale;

impl Grayscale {
    pub fn new() -> Self {
        Self
    }
}

impl Transformation for Grayscale {
    fn apply(&self, image: &Image) -> Result<Image, TransformError> {
        let pixels: Vec<Pixel> = image
            .pixels
            .iter()
            .map(|p| {
                let gray = ((p.r as u16 + p.g as u16 + p.b as u16) / 3) as u8;
                Pixel::new(gray, gray, gray)
            })
            .collect();

        Ok(Image {
            width: image.width,
            height: image.height,
            pixels,
        })
    }
}

pub struct Brightness {
    factor: f32,
}

impl Brightness {
    pub fn new(factor: f32) -> Self {
        Self { factor }
    }
}

impl Transformation for Brightness {
    fn apply(&self, image: &Image) -> Result<Image, TransformError> {
        let pixels: Vec<Pixel> = image
            .pixels
            .iter()
            .map(|p| {
                Pixel::from_f32(
                    p.r as f32 * self.factor,
                    p.g as f32 * self.factor,
                    p.b as f32 * self.factor,
                )
            })
            .collect();

        Ok(Image {
            width: image.width,
            height: image.height,
            pixels,
        })
    }
}

pub struct Contrast {
    factor: f32,
}

impl Contrast {
    pub fn new(factor: f32) -> Self {
        Self { factor }
    }
}

impl Transformation for Contrast {
    fn apply(&self, image: &Image) -> Result<Image, TransformError> {
        let pixels: Vec<Pixel> = image
            .pixels
            .iter()
            .map(|p| {
                Pixel::from_f32(
                    (p.r as f32 - 128.0) * self.factor + 128.0,
                    (p.g as f32 - 128.0) * self.factor + 128.0,
                    (p.b as f32 - 128.0) * self.factor + 128.0,
                )
            })
            .collect();

        Ok(Image {
            width: image.width,
            height: image.height,
            pixels,
        })
    }
}

pub struct Tint {
    color: Pixel,
    intensity: f32,
}

impl Tint {
    pub fn new(color: Pixel, intensity: f32) -> Self {
        Self { color, intensity }
    }
}

impl Transformation for Tint {
    fn apply(&self, image: &Image) -> Result<Image, TransformError> {
        let blend =
            |src: u8, tint: u8| src as f32 * (1.0 - self.intensity) + tint as f32 * self.intensity;

        let pixels: Vec<Pixel> = image
            .pixels
            .iter()
            .map(|p| {
                Pixel::from_f32(
                    blend(p.r, self.color.r),
                    blend(p.g, self.color.g),
                    blend(p.b, self.color.b),
                )
            })
            .collect();

        Ok(Image {
            width: image.width,
            height: image.height,
            pixels,
        })
    }
}

pub struct Colorize {
    color: Pixel,
}

impl Colorize {
    pub fn new(color: Pixel) -> Self {
        Self { color }
    }
}

impl Transformation for Colorize {
    fn apply(&self, image: &Image) -> Result<Image, TransformError> {
        let pixels: Vec<Pixel> = image
            .pixels
            .iter()
            .map(|p| {
                let gray = (p.r as u16 + p.g as u16 + p.b as u16) / 3;
                let factor = gray as f32 / 255.0;
                Pixel::from_f32(
                    self.color.r as f32 * factor,
                    self.color.g as f32 * factor,
                    self.color.b as f32 * factor,
                )
            })
            .collect();

        Ok(Image {
            width: image.width,
            height: image.height,
            pixels,
        })
    }
}

// Kernel filters

trait KernelTransformation {
    fn kernel(&self) -> &Kernel;
}

impl<T: KernelTransformation> Transformation for T {
    fn apply(&self, image: &Image) -> Result<Image, TransformError> {
        let kernel = self.kernel();
        let mut pixels = vec![Pixel::new(0, 0, 0); (image.width * image.height) as usize];

        for y in 0..image.height {
            for x in 0..image.width {
                let window = KernelWindow {
                    image,
                    center_x: x,
                    center_y: y,
                };
                let idx = (y * image.width + x) as usize;
                pixels[idx] = window.apply_kernel(&kernel);
            }
        }

        Ok(Image {
            width: image.width,
            height: image.height,
            pixels,
        })
    }
}

struct Kernel {
    size: i32,
    values: Vec<f32>,
}

impl Kernel {
    fn new(size: i32, values: Vec<f32>) -> Kernel {
        assert!(size % 2 == 1, "Kernel size must be odd");
        assert_eq!(values.len(), (size * size) as usize);

        Self { size, values }
    }

    fn normalize(&mut self) {
        let sum: f32 = self.values.iter().sum();

        if sum.abs() > f32::EPSILON {
            for val in &mut self.values {
                *val /= sum;
            }
        }
    }

    fn normalized(mut self) -> Self {
        self.normalize();
        self
    }

    fn get(&self, dx: i32, dy: i32) -> f32 {
        let center = self.size / 2;
        let x = center + dx;
        let y = center + dy;
        self.values[(y * self.size + x) as usize]
    }
}

struct KernelWindow<'a> {
    image: &'a Image,
    center_x: i32,
    center_y: i32,
}

impl<'a> KernelWindow<'a> {
    fn get(&self, dx: i32, dy: i32) -> Pixel {
        let x = (self.center_x + dx).clamp(0, self.image.width - 1);
        let y = (self.center_y + dy).clamp(0, self.image.height - 1);
        self.image.pixels[(y * self.image.width + x) as usize]
    }

    fn apply_kernel(&self, kernel: &Kernel) -> Pixel {
        let offset = kernel.size / 2;

        let mut r: f32 = 0.0;
        let mut g: f32 = 0.0;
        let mut b: f32 = 0.0;

        for dy in -offset..=offset {
            for dx in -offset..=offset {
                let pixel = self.get(dx, dy);
                let k = kernel.get(dx, dy);

                r += pixel.r as f32 * k;
                g += pixel.g as f32 * k;
                b += pixel.b as f32 * k;
            }
        }

        Pixel::from_f32(r, g, b)
    }
}

pub struct GaussianBlur {
    kernel: Kernel,
}

impl GaussianBlur {
    pub fn new() -> Self {
        // 3x3 Gaussian kernel:
        // 1  2  1
        // 2  4  2
        // 1  2  1
        let kernel = Kernel::new(3, vec![1.0, 2.0, 1.0, 2.0, 4.0, 2.0, 1.0, 2.0, 1.0]).normalized();
        Self { kernel }
    }
}

impl KernelTransformation for GaussianBlur {
    fn kernel(&self) -> &Kernel {
        &self.kernel
    }
}

pub struct Sharpen {
    kernel: Kernel,
}

impl Sharpen {
    pub fn new() -> Self {
        // 3x3 sharpen kernel:
        //  0 -1  0
        // -1  5 -1
        //  0 -1  0
        let kernel = Kernel::new(3, vec![0.0, -1.0, 0.0, -1.0, 5.0, -1.0, 0.0, -1.0, 0.0]);
        Self { kernel }
    }
}

impl KernelTransformation for Sharpen {
    fn kernel(&self) -> &Kernel {
        &self.kernel
    }
}

pub struct EdgeDetect {
    kernel: Kernel,
}

impl EdgeDetect {
    pub fn new() -> Self {
        // 3x3 edge detection kernel:
        // -1 -1 -1
        // -1  8 -1
        // -1 -1 -1
        let kernel = Kernel::new(3, vec![-1.0, -1.0, -1.0, -1.0, 8.0, -1.0, -1.0, -1.0, -1.0]);
        Self { kernel }
    }
}

impl KernelTransformation for EdgeDetect {
    fn kernel(&self) -> &Kernel {
        &self.kernel
    }
}

pub struct Emboss {
    kernel: Kernel,
}

impl Emboss {
    pub fn new() -> Self {
        // 3x3 emboss kernel:
        // -2 -1  0
        // -1  1  1
        //  0  1  2
        let kernel = Kernel::new(3, vec![-2.0, -1.0, 0.0, -1.0, 1.0, 1.0, 0.0, 1.0, 2.0]);
        Self { kernel }
    }
}

impl KernelTransformation for Emboss {
    fn kernel(&self) -> &Kernel {
        &self.kernel
    }
}
