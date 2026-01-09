# RustPhoto

A command-line image processing tool written in Rust.

## Features

### Geometric Transformations
- **crop** - Extract a region from an image
- **flip** - Flip horizontally or vertically
- **rotate** - Rotate 90°, 180°, or 270°
- **fit** - Resize to fit within maximum dimensions

### Pixel Transformations
- **invert** - Invert colors
- **grayscale** - Convert to grayscale
- **brightness** - Adjust brightness
- **contrast** - Adjust contrast
- **tint** - Apply color tint
- **colorize** - Apply color mapping

### Kernel Filters
- **blur** - Gaussian blur (3x3)
- **sharpen** - Sharpen filter
- **edge** - Edge detection
- **emboss** - Emboss effect

### Compression
- **compress** - Save as JPEG with target file size (quality 1-100, binary search)

## Building

```bash
cargo build --release
```

## Usage

Run the interactive CLI:

```bash
cargo run
```

### Commands

```
load <path>                           Load an image
save <path>                           Save current image
compress <path> <max_size_kb>         Save as JPEG with target size
crop <x> <y> <width> <height>         Crop region
flip <h|v>                            Flip horizontal or vertical
rotate <90|180|270>                   Rotate image
fit <max_width> <max_height>          Resize to fit
invert                                Invert colors
grayscale                             Convert to grayscale
brightness <factor>                   Adjust brightness (e.g., 1.2)
contrast <factor>                     Adjust contrast (e.g., 1.5)
tint <hex_color> <intensity>          Apply tint (e.g., FF0000 0.3)
colorize <hex_color>                  Colorize with color
blur                                  Apply Gaussian blur
sharpen                               Sharpen image
edge                                  Detect edges
emboss                                Apply emboss effect
undo                                  Undo last transformation
help                                  Show available commands
exit                                  Quit
```

### Example Session

```
> load ~/photo.jpg
Image loaded: 1920x1080
> grayscale
> blur
> sharpen
> save ~/edited.jpg
Image saved: /Users/you/edited.jpg
> compress ~/small.jpg 50
Compressed to 49 KB (49823 bytes): /Users/you/small.jpg
```

## Architecture

The project separates library code (`src/rustphoto/`) from the CLI interface (`src/main.rs`). The architecture described below applies to the library code.

### Coordinate System
- All coordinates and dimensions use `i32`
- Only converted to `usize` at array indexing points
- Kernel operations use signed offsets for relative positioning

### Image Storage
- Pixels stored in flat vector, row-major order
- RGB format, 8-bit channels (0-255)
- Index calculation: `(y * width + x) as usize`

### Transformation Trait
All transformations implement the `Transformation` trait:

```rust
pub trait Transformation {
    fn apply(&self, image: &Image) -> Result<Image, TransformError>;
}
```

Kernel filters use `KernelTransformation` with a blanket implementation.

## License

MIT
