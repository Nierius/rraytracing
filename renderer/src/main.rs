mod materials;
mod math;
mod renderer;
mod scene;
mod shapes;
mod util;
use std::{fs::File, io::Write};

use ::renderer::renderer::renderer::Renderer;
use clap::{command, Parser};
use shared::traits::Render;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    filename: Option<String>,

    #[arg(long, default_value_t = 400, value_parser = clap::value_parser!(i32).range(2..))]
    width: i32,

    #[arg(long, default_value_t = 225, value_parser = clap::value_parser!(i32).range(2..))]
    height: i32,

    #[arg(short, long, default_value_t = 10, value_parser = clap::value_parser!(i16).range(5..200))]
    samples_per_pixel: i16,
}

fn main() {
    let cli = Cli::parse();
    write_img(
        cli.filename.unwrap_or("out.ppm".to_string()),
        cli.width,
        cli.height,
        cli.samples_per_pixel,
    );
}

fn write_img(file: String, width: i32, height: i32, samples_per_pixel: i16) {
    println!(
        "Rendering image to file {} with size {}:{}. Samples per pixel: {}",
        file, width, height, samples_per_pixel
    );
    let renderer = Renderer::default();
    let frame = renderer.render(width, height, samples_per_pixel);
    let frame_str = frame
        .pixels
        .chunks(width as usize) // Divide into rows
        .into_iter()
        .rev() // Go rows from bottom to top
        .map(|row| {
            // Each pixel of row to string format
            row.into_iter()
                .map(|pixel| {
                    format!(
                        "{} {} {}",
                        (pixel.r * 256.0) as i16,
                        (pixel.g * 256.0) as i16,
                        (pixel.b * 256.0) as i16
                    )
                })
                .collect::<Vec<String>>()
                .join("\n")
        })
        .collect::<Vec<String>>()
        .join("\n"); // Combine rows

    let mut out_file = File::create(file).unwrap();
    out_file
        .write_all(format!("P3\n{width} {height}\n255\n{frame_str}").as_bytes())
        .unwrap();
}
