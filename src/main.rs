//use std::fmt::format;
use indicatif::ProgressBar;
use lib::error::Error;
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
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.25;

            let ir: u64 = (255.999 * r) as u64;
            let ig: u64 = (255.999 * g) as u64;
            let ib: u64 = (255.999 * b) as u64;

            write!(file, "{} {} {}\n", ir, ig, ib)?;
        }
        progress_bar.inc(1);
    }
    progress_bar.finish_with_message("Done!");

    Ok(())
}
