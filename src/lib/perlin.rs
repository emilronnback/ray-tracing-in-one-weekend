use crate::vec::Vec3;
use rand::Rng;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    random_vectors: [Vec3; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}
impl Perlin {
    pub fn new() -> Self {
        let mut random_vectors = [Vec3::new(0.0, 0.0, 0.0); POINT_COUNT];
        for r in &mut random_vectors {
            *r = Vec3::unit_vector(Vec3::random_range(-1.0, 1.0));
        }
        Perlin {
            random_vectors,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Vec3) -> f64 {
        let mut u = p.x - p.x.floor();
        let mut v = p.y - p.y.floor();
        let mut w = p.z - p.z.floor();

        u = u * u * (3.0 - 2.0 * u);
        v = v * v * (3.0 - 2.0 * v);
        w = w * w * (3.0 - 2.0 * w);

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let index = (self.perm_x[((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize])
                        as usize;
                    c[di as usize][dj as usize][dk as usize] = self.random_vectors[index];
                }
            }
        }

        Self::perlin_interpolation(c, u, v, w)
    }

    fn perlin_generate_perm() -> [i32; POINT_COUNT] {
        let mut p: [i32; POINT_COUNT] = [0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            p[i] = i as i32;
        }
        Self::permute(&mut p, POINT_COUNT);
        p
    }

    fn permute(p: &mut [i32; POINT_COUNT], n: usize) {
        let mut rng = rand::thread_rng();
        for i in (1..n).rev() {
            let target = rng.gen_range(0..i);
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }
    }
    fn perlin_interpolation(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accumulator = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accumulator += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * Vec3::dot(&c[i][j][k], &weight_v);
                }
            }
        }

        accumulator
    }

    pub fn turb(&self, p: &Vec3, depth: i32) -> f64 {
        let mut accumulator = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for i in 0..depth {
            accumulator += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accumulator.abs()
    }
}
