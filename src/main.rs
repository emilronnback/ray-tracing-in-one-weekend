//use std::fmt::format;
use indicatif::ProgressBar;
use lib::camera::Camera;
use lib::error::Error;
use lib::hittable::Hittable;
use lib::hittable_list::HittableList;
use lib::material::{Lambertian, Metal};
use lib::ray::Ray;
use lib::sphere::Sphere;
use lib::vec::Vec3;
use rand;
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
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 10;

    // World

    let material_ground = Lambertian::new(0.8, 0.8, 0.0);
    let material_center = Lambertian::new(0.7, 0.3, 0.3);
    let material_left = Metal::new(0.8, 0.8, 0.8);
    let material_right = Metal::new(0.8, 0.6, 0.2);

    let mut world = HittableList::default();

    let ground_sphere = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, &material_ground);
    world.add(&ground_sphere);

    let center_sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, &material_center);
    world.add(&center_sphere);
    let left_sphere = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, &material_left);
    world.add(&left_sphere);
    let right_sphere = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, &material_right);
    world.add(&right_sphere);
    // Camera
    let camera = Camera::new();

    // Render

    let file = File::create("out.ppm")?;
    let mut file = BufWriter::new(file);
    write!(file, "P3\n{} {}\n255\n", image_width, image_height)?;

    let progress_bar = ProgressBar::new(image_height as u64);
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rand::random::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, max_depth);
            }
            write_color(&mut file, pixel_color, samples_per_pixel)?;
        }
        progress_bar.inc(1);
    }
    progress_bar.finish_with_message("Done!");

    Ok(())
}

fn write_color(
    writer: &mut impl Write,
    pixel_color: Vec3,
    samples_per_pixel: i32,
) -> Result<(), Error> {
    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1.0 / samples_per_pixel as f64;
    let mut color = pixel_color * scale;
    color.x = color.x.sqrt();
    color.y = color.y.sqrt();
    color.z = color.z.sqrt();

    write!(
        writer,
        "{} {} {}\n",
        (256.0 * color.x.clamp(0.0, 0.999)) as u64,
        (256.0 * color.y.clamp(0.0, 0.999)) as u64,
        (256.0 * color.z.clamp(0.0, 0.999)) as u64,
    )?;
    Ok(())
}

fn ray_color(ray: &Ray, world: &impl Hittable, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    if let Some(hit) = world.hit(ray, 0.001, std::f64::INFINITY) {
        if let Some((attenuation, scattered)) = hit.material.scatter(ray, &hit) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        } else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        // let target = hit.point + hit.normal + Vec3::random_unit_vector();
        // return 0.5 * ray_color(&Ray::new(hit.point, target - hit.point), world, depth - 1);
    }

    let unit_direction = Vec3::unit_vector(ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}
