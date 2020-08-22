extern crate rand;
use wasm_bindgen::prelude::*;
use rand::Rng;

pub fn generate_rgb(max: i32) -> String {
    let mut colors: Vec<String> = Vec::new();
    let mut rgb: Vec<String> = Vec::new();

    for _ in 0..max {
        for _ in 0..3 {
            let new: i32 = rand::thread_rng().gen_range(0, 255);
            rgb.push(new.to_string());
        }
        colors.push(format!("rgb({})", rgb.join(",")));
        rgb.clear();
    }

    return colors.join(" ");
}

pub fn generate_hsl(max: i32) -> String {
    let mut colors: Vec<String> = Vec::new();
    let mut hsl: Vec<String> = Vec::new();

    for _ in 0..max {
        for _ in 0..3 {
            let new: i32 = rand::thread_rng().gen_range(0, 100);
            hsl.push(new.to_string());
        }
        colors.push(format!("hsl({})", hsl.join(",")));
        hsl.clear();
    }

    return colors.join(" ");
}

pub fn generate_hex(max: i32) -> String {
    let mut colors: Vec<String> = Vec::new();
    let mut hex: Vec<String> = Vec::new();

    for _ in 0..max {
        for _ in 0..3 {
            let new: i32 = rand::thread_rng().gen_range(0, 255);

            if new < 10 {
                hex.push(format!("0{}", new));
            } else {
                hex.push(format!("{:X}", new));
            }
        }
        colors.push(format!("#{}", hex.join("")));
        hex.clear();
    }

    return colors.join(" ");
}

pub fn print_error() -> String {
    return "Wrong request".to_string();
}

#[wasm_bindgen]
pub fn generate_colors(max: i32, mode: &str) -> String {
    match mode {
        "rgb" => generate_rgb(max),
        "hex" => generate_hex(max),
        "hsl" => generate_hsl(max),
        _ => print_error()
    }
}
