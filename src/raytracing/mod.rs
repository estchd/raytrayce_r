use std::sync::Arc;
use rand::{Rng, thread_rng};
use workers_pool::Worker;
use scene::RaytracingScene;
use crate::{Color};
use crate::raytracing::hittable::Hittable;
use crate::raytracing::ray::Ray;

pub mod vector_2d;
pub mod vector_3d;
pub mod color;
pub mod texture;
pub mod pixel;
pub mod scene;
pub mod ray;
pub mod camera;
pub mod hittable;
pub mod hit_record;
pub mod materials;

pub const SAMPLES_PER_PIXEL: usize = 500;
pub const MAX_BOUNCES: usize = 50;
pub const NEAR_ZERO_THRESHOLD: f64 = f64::EPSILON;

#[derive(Default, Clone)]
pub struct RaytracingContext {
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: usize,
    pub max_bounces: usize,
    pub scene: RaytracingScene
}

pub struct RaytracingWorkData {
    pub x: u32,
    pub y: u32
}

pub struct RaytracingResult {
    pub x: u32,
    pub y: u32,
    pub pixel_color: Color,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct RaytracingWorker {}

impl Worker for RaytracingWorker {
    type Data = RaytracingWorkData;
    type Result = RaytracingResult;
    type Context = RaytracingContext;

    fn execute(&mut self, data: Self::Data, context: &Arc<Self::Context>) -> Self::Result {
        let mut rand = thread_rng();

        let image_width = context.image_width;
        let image_height = context.image_height;
        let x = data.x;
        let y = data.y;

        let mut color = Color::new();

        for _ in 0..context.samples_per_pixel {
            let u_offset = rand.gen_range(0.0..1.0);
            let v_offset = rand.gen_range(0.0..1.0);
            let u = (x as f64 + u_offset) / image_width as f64;
            let v = (y as f64 + v_offset) / image_height as f64;

            let ray = context.scene.camera.cast_ray(u,v);
            let new_color = ray_color(&ray, &context.scene, context.max_bounces);
            color = Color {
                r: color.r + new_color.r,
                g: color.g + new_color.g,
                b: color.b + new_color.b,
                a: 1.0
            };
        }

        let scale = 1.0 / context.samples_per_pixel as f32;

        let color = Color {
            r: (color.r * scale).sqrt(),
            g: (color.g * scale).sqrt(),
            b: (color.b * scale).sqrt(),
            a: 1.0
        };

        let result = RaytracingResult {
            x: data.x,
            y: data.y,
            pixel_color: color
        };

        return result;
    }
}

fn ray_color(ray: &Ray, scene: &RaytracingScene, depth: usize) -> Color {
    if depth == 0 {
        return Color::new();
    }

    if let Some(hit_record) = scene.hit(ray, 0.001, f64::INFINITY) {
        return if let Some(material) = &hit_record.material {
            let scattered = material.scatter(&ray, &hit_record);
            if let Some((attenuation, ray)) = scattered {
                let new_color = ray_color(&ray, scene, depth - 1);

                Color {
                    r: attenuation.r * new_color.r,
                    g: attenuation.g * new_color.g,
                    b: attenuation.b * new_color.b,
                    a: 1.0
                }
            } else {
                Color::create(0.0, 0.0, 0.0, 1.0)
            }
        } else {
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0
            }
        }
    }

    let unit_direction = ray.direction.normalized();

    let t = 0.5 * (unit_direction.y + 1.0);

    let color = Color {
        r: ((1.0 - t) + t * 0.5) as f32,
        g: ((1.0 - t) + t * 0.7) as f32,
        b: ((1.0 - t) + t * 1.0) as f32,
        a: 1.0
    };

    color
}
