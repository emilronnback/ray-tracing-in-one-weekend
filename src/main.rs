//use std::fmt::format;
use indicatif::ProgressBar;
use lib::camera::Camera;
use lib::error::Error;
use lib::hittable::{Hittable, RotateY, Translate};
use lib::hittable_list::HittableList;
use lib::job::Job;
use lib::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use lib::mybox::MyBox;
use lib::ray::Ray;
use lib::rectangle::{XYRectangle, XZRectangle, YZRectangle};
use lib::sphere::Sphere;
use lib::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use lib::vec::Vec3;
use lib::{bvh_node::BVHNode, constant_medium::ConstantMedium};
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
    color: Vec3,
}

impl Pixel {
    fn new(color: Vec3) -> Pixel {
        Pixel { color }
    }
}

fn run() -> Result<(), Error> {
    // Image
    let mut aspect_ratio = 16.0 / 9.0;
    let mut image_width = 400;
    let mut samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let world: HittableList;
    let look_from: Vec3;
    let look_at: Vec3;
    let vfov: f64;
    let background: Vec3;

    match 0 {
        1 => {
            world = random_scene();
            background = Vec3::new(0.7, 0.8, 1.0);
            look_from = Vec3::new(13.0, 2.0, 3.0);
            look_at = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        2 => {
            world = two_spheres();
            background = Vec3::new(0.7, 0.8, 1.0);
            look_from = Vec3::new(13.0, 2.0, 3.0);
            look_at = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        3 => {
            world = two_perlin_spheres();
            background = Vec3::new(0.7, 0.8, 1.0);
            look_from = Vec3::new(13.0, 2.0, 3.0);
            look_at = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        4 => {
            world = earth();
            background = Vec3::new(0.7, 0.8, 1.0);
            look_from = Vec3::new(13.0, 2.0, 3.0);
            look_at = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        5 => {
            world = simple_light();
            samples_per_pixel = 400;
            background = Vec3::new(0.0, 0.0, 0.0);
            look_from = Vec3::new(26.0, 3.0, 6.0);
            look_at = Vec3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
        }
        6 => {
            world = cornell_box();
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            background = Vec3::new(0.0, 0.0, 0.0);
            look_from = Vec3::new(278.0, 278.0, -800.0);
            look_at = Vec3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        7 => {
            world = cornell_smoke();
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            background = Vec3::new(0.0, 0.0, 0.0);
            look_from = Vec3::new(278.0, 278.0, -800.0);
            look_at = Vec3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        _ => {
            world = final_scene();
            aspect_ratio = 1.0;
            image_width = 800;
            samples_per_pixel = 10000;
            background = Vec3::new(0.0, 0.0, 0.0);
            look_from = Vec3::new(478.0, 278.0, -600.0);
            look_at = Vec3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
    }

    let image_height = (image_width as f64 / aspect_ratio) as usize;

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
                &background,
                samples_per_pixel,
                max_depth,
                image_width,
                image_height,
                &camera,
            )
        })
        .collect();
    for o in outcome {
        for p in o.pixels {
            write_color(&mut file, p.color, samples_per_pixel)?;
        }
    }
    progress_bar.finish_with_message("Done!");
    let pixel_color = Vec3::random();
    write_color(&mut file, pixel_color, samples_per_pixel)?;
    Ok(())
}

fn work(
    job: &Job,
    world: &HittableList,
    background: &Vec3,
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
                let color = ray_color(&ray, background, &*world, max_depth);
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

fn ray_color(ray: &Ray, background: &Vec3, world: &impl Hittable, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = world.hit(ray, 0.001, std::f64::INFINITY) {
        let emitted = hit.material.emitted(hit.u, hit.v, &hit.point);
        if let Some((attenuation, scattered)) = hit.material.scatter(ray, &hit) {
            return emitted + attenuation * ray_color(&scattered, background, world, depth - 1);
        } else {
            return emitted;
        }
    } else {
        return *background;
    }
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
                        0.2,
                        center,
                        center_end,
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

    let perlin_texture = Arc::new(NoiseTexture::new_scaled(4.0));
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

fn earth() -> HittableList {
    let mut earth = HittableList::new();

    let earth_texture = Arc::new(ImageTexture::new_from_file("world.png"));
    let earth_surface = Arc::new(Lambertian::new_texture(earth_texture));
    let globe = Arc::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0, earth_surface));
    earth.add(globe);

    earth
}

fn simple_light() -> HittableList {
    let mut objects = HittableList::new();

    let perlin_texture = Arc::new(NoiseTexture::new_scaled(4.0));
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

    let diffuse_light = Arc::new(DiffuseLight::new_color(Vec3::new(4.0, 4.0, 4.0)));
    objects.add(Arc::new(XYRectangle::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        diffuse_light.clone(),
    )));

    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 8.0, 0.0),
        2.0,
        diffuse_light,
    )));

    objects
}

fn cornell_box() -> HittableList {
    let mut objects = HittableList::new();

    let red = Arc::new(Lambertian::new_color(Vec3::new(0.64, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_color(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_color(Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_color(Vec3::new(15.0, 15.0, 15.0)));

    objects.add(Arc::new(YZRectangle::new(
        0.0, 555.0, 0.0, 555.0, 555.0, green,
    )));
    objects.add(Arc::new(YZRectangle::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(XZRectangle::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    objects.add(Arc::new(XZRectangle::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XZRectangle::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XYRectangle::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let box1 = Arc::new(MyBox::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    objects.add(box1);

    let box2 = Arc::new(MyBox::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white,
    ));
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    objects.add(box2);

    objects
}
fn cornell_smoke() -> HittableList {
    let mut objects = HittableList::new();

    let red = Arc::new(Lambertian::new_color(Vec3::new(0.64, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_color(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_color(Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_color(Vec3::new(7.0, 7.0, 7.0)));

    objects.add(Arc::new(YZRectangle::new(
        0.0, 555.0, 0.0, 555.0, 555.0, green,
    )));
    objects.add(Arc::new(YZRectangle::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(XZRectangle::new(
        113.0, 443.0, 127.0, 432.0, 554.0, light,
    )));
    objects.add(Arc::new(XZRectangle::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XZRectangle::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XYRectangle::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let box1 = Arc::new(MyBox::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));

    let box2 = Arc::new(MyBox::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white,
    ));
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));

    objects.add(Arc::new(ConstantMedium::new_color(
        box1,
        0.01,
        Vec3::new(0.0, 0.0, 0.0),
    )));
    objects.add(Arc::new(ConstantMedium::new_color(
        box2,
        0.01,
        Vec3::new(1.0, 1.0, 1.0),
    )));

    objects
}

fn final_scene() -> HittableList {
    let mut boxes1 = HittableList::new();
    let ground = Arc::new(Lambertian::new_color(Vec3::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    let mut rng = rand::thread_rng();
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + (i as f64 * w);
            let z0 = -1000.0 + (j as f64 * w);
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rng.gen_range(1.0..101.0);
            let z1 = z0 + w;
            boxes1.add(Arc::new(MyBox::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }
    let mut objects = HittableList::new();

    objects.add(Arc::new(BVHNode::new_hittablelist(&boxes1, 0.0, 1.0)));

    let light = Arc::new(DiffuseLight::new_color(Vec3::new(7.0, 7.0, 7.0)));
    objects.add(Arc::new(XZRectangle::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    )));

    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);

    let moving_sphere_material = Arc::new(Lambertian::new_color(Vec3::new(0.7, 0.3, 0.1)));
    objects.add(Arc::new(Sphere::new_moving(
        50.0,
        center1,
        center2,
        0.0,
        1.0,
        moving_sphere_material,
    )));

    objects.add(Arc::new(Sphere::new(
        Vec3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary = Arc::new(Sphere::new(
        Vec3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    objects.add(boundary.clone());
    objects.add(Arc::new(ConstantMedium::new_color(
        boundary,
        0.2,
        Vec3::new(0.2, 0.4, 0.9),
    )));
    let boundary = Arc::new(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    objects.add(Arc::new(ConstantMedium::new_color(
        boundary,
        0.0001,
        Vec3::new(1.0, 1.0, 1.0),
    )));

    let earth_material = Arc::new(Lambertian::new_texture(Arc::new(
        ImageTexture::new_from_file("world.png"),
    )));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(400.0, 200.0, 400.0),
        100.0,
        earth_material,
    )));
    let pertext = Arc::new(NoiseTexture::new_scaled(0.1));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new_texture(pertext)),
    )));

    let mut boxes2 = HittableList::new();
    let white = Arc::new(Lambertian::new_color(Vec3::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            Vec3::random_range(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }

    objects.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BVHNode::new_hittablelist(&boxes2, 0.0, 1.0)),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    objects
}
