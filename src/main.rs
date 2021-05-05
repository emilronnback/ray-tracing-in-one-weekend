//use std::fmt::format;
use indicatif::ProgressBar;
use lib::error::Error;
use lib::ray::Ray;
use lib::vec::Vec3;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}
fn run() -> Result<(), Error> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 600;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let file = File::create("out.ppm")?;
    let mut file = BufWriter::new(file);
    write!(file, "P3\n{} {}\n255\n", image_width, image_height)?;

    let progress_bar = ProgressBar::new(image_height as u64);
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&ray);

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

fn ray_color(ray: &Ray) -> Vec3 {
    let unit_direction = Vec3::unit_vector(ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}
