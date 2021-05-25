//use std::fmt::format;
use indicatif::ProgressBar;
use lib::bvh_node::BVHNode;
use lib::camera::Camera;
use lib::error::Error;
use lib::hittable::Hittable;
use lib::hittable_list::HittableList;
use lib::job::Job;
use lib::material::{Dielectric, Lambertian, Metal};
use lib::ray::Ray;
use lib::sphere::Sphere;
use lib::texture::{CheckerTexture, NoiseTexture};
use lib::vec::Vec3;
use rand;
use rand::Rng;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::Arc;
use std::vec::Vec;

fn main() {
    if let Err(e) = run() {
        println!("ERROR: {}", e);
    }
}

struct Outcome {
    pixels: Vec<Pixel>,
}
impl Outcome {
    fn new() -> Outcome {
        Outcome { pixels: Vec::new() }
    }
}

struct Pixel {
    x: u32,
    y: u32,
    color: Vec3,
}

impl Pixel {
    fn new(color: Vec3) -> Pixel {
        Pixel { x: 0, y: 0, color }
    }
}

fn run() -> Result<(), Error> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let world: HittableList;
    let look_from: Vec3;
    let look_at: Vec3;
    let vfov: f64;

    match 0 {
        1 => {
            world = random_scene();
            look_from = Vec3::new(13.0, 2.0, 3.0);
            look_at = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        2 => {
            world = two_spheres();
            look_from = Vec3::new(13.0, 2.0, 3.0);
            look_at = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        _ => {
            world = two_perlin_spheres();
            look_from = Vec3::new(13.0, 2.0, 3.0);
            look_at = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
    }

    // Camera
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let distance_to_focus = 10.0;
    let aperture = 0.0;

    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        distance_to_focus,
        0.0,
        1.0,
    );

    // Render

    let file = File::create("out.ppm")?;
    let mut file = BufWriter::new(file);
    write!(file, "P3\n{} {}\n255\n", image_width, image_height)?;
    let jobs = lib::job::create_jobs(image_height, image_width);
    let progress_bar = ProgressBar::new(image_height as u64);
    let outcome: Vec<Outcome> = jobs
        //        .par_iter()
        .par_iter()
        .map(|j| {
            progress_bar.inc(1);
            work(
                j,
                &world,
                samples_per_pixel,
                max_depth,
                image_width,
                image_height,
                &camera,
            )
        })
        .collect();
    let mut outcomes = 0;
    let mut writes = 0;
    for o in outcome {
        outcomes += 1;
        for p in o.pixels {
            writes += 1;
            write_color(&mut file, p.color, samples_per_pixel)?;
        }
    }
    progress_bar.finish_with_message("Done!");
    println!("outcomes: {}, writes: {}", outcomes, writes);
    let pixel_color = Vec3::random();
    write_color(&mut file, pixel_color, samples_per_pixel)?;
    Ok(())
}

fn work(
    job: &Job,
    world: &HittableList,
    samples_per_pixel: i32,
    max_depth: i32,
    image_width: usize,
    image_height: usize,
    camera: &Camera,
) -> Outcome {
    let mut outcome = Outcome::new();
    let height_range = job.height_range.clone();
    for j in height_range.rev() {
        let width_range = job.width_range.clone();
        for i in width_range {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rand::random::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                let color = ray_color(&ray, &*world, max_depth);
                //                println!("color: {}", color);
                pixel_color += color;
            }
            outcome.pixels.push(Pixel::new(pixel_color));
        }
    }
    outcome
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
    }

    let unit_direction = Vec3::unit_vector(ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}
#[allow(dead_code)]
fn scene1() -> HittableList {
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new_color(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new_color(Vec3::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0));

    let ground_sphere = Arc::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(ground_sphere);

    let mut big_spheres = HittableList::new();
    let center_sphere = Arc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center));
    big_spheres.add(center_sphere);
    let left_sphere = Arc::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    big_spheres.add(left_sphere);
    let right_sphere = Arc::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right));
    big_spheres.add(right_sphere);
    let bvh_node = Arc::new(BVHNode::new_hittablelist(&big_spheres, 0.0, 1.1));
    world.add(bvh_node);
    world
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let checker = Arc::new(CheckerTexture::new_color(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    //    let material_ground = Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    let ground_sphere = Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_texture(checker)),
    ));
    world.add(ground_sphere);
    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let material_choise = rng.gen::<f64>();
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 * 0.9 * rng.gen::<f64>(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if material_choise < 0.8 {
                    //diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    let material_sphere = Arc::new(Lambertian::new_color(albedo));
                    let center_end = center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    world.add(Arc::new(Sphere::new_moving(
                        center,
                        center_end,
                        0.2,
                        0.0,
                        1.0,
                        material_sphere,
                    )));
                } else if material_choise < 0.95 {
                    //metal
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.5..1.0);
                    let material_sphere = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, material_sphere)));
                } else {
                    //glass
                    let material_sphere = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, material_sphere)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new_color(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

fn two_spheres() -> HittableList {
    let mut objects = HittableList::new();

    let checker = Arc::new(CheckerTexture::new_color(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));

    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -10.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_texture(checker.clone())),
    )));

    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new_texture(checker)),
    )));

    objects
}

fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new();

    let perlin_texture = Arc::new(NoiseTexture::new());
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_texture(perlin_texture.clone())),
    )));

    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new_texture(perlin_texture)),
    )));

    objects
}
