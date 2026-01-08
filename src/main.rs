mod rustphoto;

use std::io::{stdin, stdout, Write};
use std::ops::ControlFlow;

use rustphoto::image::{Image, Pixel};
use rustphoto::transforms::*;

fn cmd_load(parts: &[&str]) -> Option<Image> {
    if parts.len() < 2 {
        println!("Usage: load <path>");
        return None;
    }

    let path = expand_path(parts[1]);

    match Image::load(&path) {
        Ok(img) => {
            println!("Image loaded: {}x{}", img.width, img.height);
            Some(img)
        }
        Err(e) => {
            println!("Error: {}", e);
            None
        }
    }
}

fn cmd_save(parts: &[&str], image: &Image) {
    if parts.len() < 2 {
        println!("Usage: save <path>");
        return;
    }

    let path = expand_path(parts[1]);

    match image.save(&path) {
        Ok(_) => println!("Image saved: {}", path),
        Err(e) => println!("Error: {}", e),
    }
}

fn cmd_crop(parts: &[&str], image: &Image) -> Option<Image> {
    if parts.len() < 5 {
        println!("Usage: crop <x> <y> <width> <height>");
        return None;
    }

    let x = parse_number(parts[1])?;
    let y = parse_number(parts[2])?;
    let width = parse_number(parts[3])?;
    let height = parse_number(parts[4])?;

    let transform = Crop::new(x, y, width, height);

    match transform.apply(image) {
        Ok(result) => Some(result),
        Err(e) => {
            println!("Error: {}", e);
            None
        }
    }
}

fn cmd_flip(parts: &[&str], image: &Image) -> Option<Image> {
    if parts.len() < 2 {
        println!("Usage: flip <h|v>");
        return None;
    }

    let axis = match parts[1] {
        "h" => FlipAxis::Horizontal,
        "v" => FlipAxis::Vertical,
        _ => {
            println!("Invalid axis. Use 'h' (horizontal) or 'v' (vertical)");
            return None;
        }
    };

    let transform = Flip::new(axis);

    match transform.apply(image) {
        Ok(result) => Some(result),
        Err(e) => {
            println!("Error: {}", e);
            None
        }
    }
}

fn cmd_rotate(parts: &[&str], image: &Image) -> Option<Image> {
    if parts.len() < 2 {
        println!("Usage: rotate <90|180|270>");
        return None;
    }

    let angle = match parts[1] {
        "90" => RotateAngle::Deg90,
        "180" => RotateAngle::Deg180,
        "270" => RotateAngle::Deg270,
        _ => {
            println!("Invalid angle. Use 90, 180, or 270");
            return None;
        }
    };

    let transform = Rotate::new(angle);

    match transform.apply(image) {
        Ok(result) => Some(result),
        Err(e) => {
            println!("Error: {}", e);
            None
        }
    }
}

fn cmd_fit(parts: &[&str], image: &Image) -> Option<Image> {
    if parts.len() < 3 {
        println!("Usage: fit <max_width> <max_height>");
        return None;
    }

    let max_width = parse_number(parts[1])?;
    let max_height = parse_number(parts[2])?;

    let transform = Fit::new(max_width, max_height);

    match transform.apply(image) {
        Ok(result) => Some(result),
        Err(e) => {
            println!("Error: {}", e);
            None
        }
    }
}

fn cmd_invert(image: &Image) -> Option<Image> {
    let transform = Invert::new();

    match transform.apply(image) {
        Ok(result) => Some(result),
        Err(e) => {
            println!("Error: {}", e);
            None
        }
    }
}

fn cmd_grayscale(image: &Image) -> Option<Image> {
    let transform = Grayscale::new();

    match transform.apply(image) {
        Ok(result) => Some(result),
        Err(e) => {
            println!("Error: {}", e);
            None
        }
    }
}

fn cmd_brightness(parts: &[&str], image: &Image) -> Option<Image> {
    if parts.len() < 2 {
        println!("Usage: brightness <factor>");
        return None;
    }

    let factor = parse_float(parts[1])?;
    let transform = Brightness::new(factor);

    match transform.apply(image) {
        Ok(result) => Some(result),
        Err(e) => {
            println!("Error: {}", e);
            None
        }
    }
}

fn cmd_contrast(parts: &[&str], image: &Image) -> Option<Image> {
    if parts.len() < 2 {
        println!("Usage: contrast <factor>");
        return None;
    }

    let factor = parse_float(parts[1])?;
    let transform = Contrast::new(factor);

    match transform.apply(image) {
        Ok(result) => Some(result),
        Err(e) => {
            println!("Error: {}", e);
            None
        }
    }
}

fn cmd_tint(parts: &[&str], image: &Image) -> Option<Image> {
    if parts.len() < 3 {
        println!("Usage: tint <hex_color> <intensity>");
        return None;
    }

    let hex_color = parse_hex_color(parts[1])?;
    let intensity = parse_float(parts[2])?;

    let color = Pixel::from_hex(hex_color);
    let transform = Tint::new(color, intensity);

    match transform.apply(image) {
        Ok(result) => Some(result),
        Err(e) => {
            println!("Error: {}", e);
            None
        }
    }
}

fn cmd_colorize(parts: &[&str], image: &Image) -> Option<Image> {
    if parts.len() < 2 {
        println!("Usage: colorize <hex_color>");
        return None;
    }

    let hex_color = parse_hex_color(parts[1])?;
    let color = Pixel::from_hex(hex_color);
    let transform = Colorize::new(color);

    match transform.apply(image) {
        Ok(result) => Some(result),
        Err(e) => {
            println!("Error: {}", e);
            None
        }
    }
}

fn cmd_boxblur(image: &Image) -> Option<Image> {
    let transform = BoxBlur::new();

    match transform.apply(image) {
        Ok(result) => Some(result),
        Err(e) => {
            println!("Error: {}", e);
            None
        }
    }
}

fn cmd_blur(image: &Image) -> Option<Image> {
    let transform = GaussianBlur::new();

    match transform.apply(image) {
        Ok(result) => Some(result),
        Err(e) => {
            println!("Error: {}", e);
            None
        }
    }
}

fn parse_command(
    command: &str,
    current_image: &mut Option<Image>,
    previous_image: &mut Option<Image>,
) -> ControlFlow<()> {
    let parts: Vec<&str> = command.split_whitespace().collect();

    if parts.is_empty() {
        return ControlFlow::Continue(());
    }

    match parts[0] {
        "exit" => {
            return ControlFlow::Break(());
        }
        "load" => {
            if let Some(img) = cmd_load(&parts) {
                *current_image = Some(img);
                *previous_image = None;
            }

            return ControlFlow::Continue(());
        }
        "help" => {
            println!(concat!(
                "Available commands: load, save, crop, flip, rotate, fit, ",
                "invert, grayscale, brightness, contrast, tint, colorize, ",
                "blur, boxblur, undo, help, exit"
            ));
            return ControlFlow::Continue(());
        }
        _ => {}
    }

    let Some(image) = current_image else {
        println!("No image loaded");
        return ControlFlow::Continue(());
    };

    match parts[0] {
        "save" => cmd_save(&parts, image),
        "undo" => {
            if let Some(prev) = previous_image.take() {
                *current_image = Some(prev);
            } else {
                println!("Nothing to undo");
            }
        }
        "crop" => {
            if let Some(result) = cmd_crop(&parts, image) {
                *previous_image = current_image.take();
                *current_image = Some(result);
            }
        }
        "flip" => {
            if let Some(result) = cmd_flip(&parts, image) {
                *previous_image = current_image.take();
                *current_image = Some(result);
            }
        }
        "rotate" => {
            if let Some(result) = cmd_rotate(&parts, image) {
                *previous_image = current_image.take();
                *current_image = Some(result);
            }
        }
        "fit" => {
            if let Some(result) = cmd_fit(&parts, image) {
                *previous_image = current_image.take();
                *current_image = Some(result);
            }
        }
        "invert" => {
            if let Some(result) = cmd_invert(image) {
                *previous_image = current_image.take();
                *current_image = Some(result);
            }
        }
        "grayscale" => {
            if let Some(result) = cmd_grayscale(image) {
                *previous_image = current_image.take();
                *current_image = Some(result);
            }
        }
        "brightness" => {
            if let Some(result) = cmd_brightness(&parts, image) {
                *previous_image = current_image.take();
                *current_image = Some(result);
            }
        }
        "contrast" => {
            if let Some(result) = cmd_contrast(&parts, image) {
                *previous_image = current_image.take();
                *current_image = Some(result);
            }
        }
        "tint" => {
            if let Some(result) = cmd_tint(&parts, image) {
                *previous_image = current_image.take();
                *current_image = Some(result);
            }
        }
        "colorize" => {
            if let Some(result) = cmd_colorize(&parts, image) {
                *previous_image = current_image.take();
                *current_image = Some(result);
            }
        }
        "blur" => {
            if let Some(result) = cmd_blur(image) {
                *previous_image = current_image.take();
                *current_image = Some(result);
            }
        }
        "boxblur" => {
            if let Some(result) = cmd_boxblur(image) {
                *previous_image = current_image.take();
                *current_image = Some(result);
            }
        }
        _ => println!("Unknown command: {}", command),
    }

    ControlFlow::Continue(())
}

fn main() {
    println!("Welcome to RustPhoto CLI!");
    println!("Type 'exit' to quit");

    let mut current_image: Option<Image> = None;
    let mut previous_image: Option<Image> = None;

    loop {
        print!("> ");

        if let Err(e) = stdout().flush() {
            eprintln!("Error: {}", e);
            break;
        }

        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(0) => break, // EOF
            Ok(_) => {
                if let ControlFlow::Break(()) =
                    parse_command(input.trim(), &mut current_image, &mut previous_image)
                {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}

// Utils

fn expand_path(path: &str) -> String {
    if let Some(rest) = path.strip_prefix("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(rest).to_string_lossy().to_string();
        }
    }

    path.to_string()
}

fn parse_number(s: &str) -> Option<i32> {
    s.parse().ok().or_else(|| {
        println!("Invalid number");
        None
    })
}

fn parse_float(s: &str) -> Option<f32> {
    s.parse().ok().or_else(|| {
        println!("Invalid number");
        None
    })
}

fn parse_hex_color(s: &str) -> Option<u32> {
    if s.starts_with("#") {
        u32::from_str_radix(&s[1..], 16).ok()
    } else {
        u32::from_str_radix(s, 16).ok()
    }
}
