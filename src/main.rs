// 
// termsprite
// ShaiyaJ - 2023
//

use std::env;
use std::path::Path;
use std::io::{stdout};

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};

use image::io::Reader as ImageReader;
use image::error::ImageError;
use image::GenericImageView;


// Types
//
#[derive(Debug)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    legacy_col: Color
}

impl Default for Pixel {
    fn default() -> Self {
        Pixel {
            r: 0,
            g: 0,
            b: 0,
            legacy_col: Color::Red
        }
    }
}

// File handling
//
fn open_file(path: &Path) -> Result<Vec<Vec<Pixel>>, ImageError> {
    // Open image
    let img = ImageReader::open(path)?.decode()?;

    // Push each pixel to a 2d vec (pixel_vec)
    let mut pixel_vec: Vec<Vec<Pixel>> = vec![]; 

    for pixel in img.pixels() {
        let x: usize = pixel.0 as usize;
        let y: usize = pixel.1 as usize;
        let r = pixel.2[0];
        let g = pixel.2[1];
        let b = pixel.2[2];
        let legacy_col = calculate_legacy_color(r,g,b);

        if pixel_vec.len() == y {
            pixel_vec.push(vec![]);
        }

        if pixel_vec[y].len() == x {
            pixel_vec[y].push(Pixel {r,g,b,legacy_col});
        }
    }

    return Ok(pixel_vec);
}


// Terminal IO
//

fn calculate_legacy_color(r: u8, g: u8, b: u8) -> Color {
    let color_set_rgb: Vec<Pixel> = vec![
        Pixel {r: 0,   g: 0,   b: 0,   legacy_col: Color::Black},       // Black
        Pixel {r: 128, g: 128, b: 128, legacy_col: Color::DarkGrey},    // DarkGrey
        Pixel {r: 255, g: 0,   b: 0,   legacy_col: Color::Red},         // Red
        Pixel {r: 128, g: 0,   b: 0,   legacy_col: Color::DarkRed},     // DarkRed
        Pixel {r: 0,   g: 255, b: 0,   legacy_col: Color::Green},       // Green
        Pixel {r: 0,   g: 128, b: 0,   legacy_col: Color::DarkGreen},   // DarkGreen
        Pixel {r: 255, g: 255, b: 0,   legacy_col: Color::Yellow},      // Yellow
        Pixel {r: 128, g: 128, b: 0,   legacy_col: Color::DarkYellow},  // DarkYellow
        Pixel {r: 0,   g: 0,   b: 255, legacy_col: Color::Blue},        // Blue
        Pixel {r: 0,   g: 0,   b: 128, legacy_col: Color::DarkBlue},    // DarkBlue
        Pixel {r: 255, g: 0,   b: 255, legacy_col: Color::Magenta},     // Magenta
        Pixel {r: 128, g: 0,   b: 128, legacy_col: Color::DarkMagenta}, // DarkMagenta
        Pixel {r: 0,   g: 255, b: 255, legacy_col: Color::Cyan},        // Cyan
        Pixel {r: 0,   g: 128, b: 128, legacy_col: Color::DarkCyan},    // DarkCyan
        Pixel {r: 255, g: 255, b: 255, legacy_col: Color::White},       // White
        Pixel {r: 50,  g: 50,  b: 50,  legacy_col: Color::Grey},        // Grey
    ];

    let pixel_sum: u64 = (r as u64) + (g as u64) + (b as u64);
    let mut low_cost: f64 = 10000.0;
    let mut cost: f64;
    let mut col: Color = Color::Red;

    for color in color_set_rgb {
        let color_sum: u64 = (color.r as u64) + (color.g as u64) + (color.b as u64);
        cost = color_sum as f64 - pixel_sum as f64;
        cost = cost.powf(2.0);

        //println!("{}", cost);

        if cost < low_cost {
            low_cost = cost;
            col = color.legacy_col;
        }
    }

    return col;

}

fn output(upper_pixel: &Pixel, lower_pixel: &Pixel, legacy: bool) -> std::io::Result<()> {
    let foreground: Color;
    let background: Color;

    if legacy {
        foreground = calculate_legacy_color(lower_pixel.r, lower_pixel.g, lower_pixel.b);
        background = calculate_legacy_color(upper_pixel.r, upper_pixel.g, upper_pixel.b);
    } else {
        foreground = Color::Rgb {r: lower_pixel.r, g: lower_pixel.g, b: lower_pixel.b};
        background = Color::Rgb {r: upper_pixel.r, g: upper_pixel.g, b: upper_pixel.b};
    }

    execute!(
        stdout(),
        SetBackgroundColor(background),
        SetForegroundColor(foreground),
        Print("▄"),
        ResetColor
    )?;

    Ok(())
}

fn main() {
    // Parsing command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {                             // No args provided
        println!("Invalid args");
        println!("USAGE: termsprite <path> OPTIONS...");
        println!("    OPTIONS:");
        println!("        -l\tLegacy mode (for old terminals)");
        return;
    }

    let path = &args[1];
    let options = &args[2..]; 

    let legacy = options.contains(&String::from("-l"));
    

    // Opening file
    let file_path = Path::new(path);

    let content: Vec<Vec<Pixel>> = match open_file(&file_path) {
        Err(e) => {
            eprintln!("{}", format!("ERROR: Open file operation failed: {}", e));
            return;
        },
        Ok(c) => c,
    }; 

    for y in (1..content.len()).step_by(2) {
        for x in 0..content[y].len() {
            let _ = output(&content[y-1][x], &content[y][x], legacy);
        }

        match execute!(
            stdout(),
            Print("\n")
        ) {
            Err(_) => eprintln!("ERROR: Failed to write to terminal, please check if your terminal is compatible with crossterm (https://docs.rs/crossterm/latest/crossterm/)"),
            Ok(_) => {}
        };
    }
}
