//use std::fmt::format;
use indicatif::ProgressBar;
use lib::error::Error;
use lib::vec::Vec3;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}
fn run() -> Result<(), Error> {
    let file = File::create("out.ppm")?;
    let mut file = BufWriter::new(file);

    let image_width = 256;
    let image_height = 256;

    write!(file, "P3\n{} {}\n255\n", image_width, image_height)?;

    let progress_bar = ProgressBar::new(image_height);
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let pixel_color = Vec3::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25,
            );

            write!(
                file,
                "{} {} {}\n",
                (255.999 * pixel_color.x) as u64,
                (255.999 * pixel_color.y) as u64,
                (255.999 * pixel_color.z) as u64
            )?;
        }
        progress_bar.inc(1);
    }
    progress_bar.finish_with_message("Done!");

    Ok(())
}
