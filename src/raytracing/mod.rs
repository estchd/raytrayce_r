use std::sync::Arc;
use rand::{Rng, thread_rng};
use workers_pool::{TaskState, Worker};
use workers_pool::TaskState::Finished;
use scene::RaytracingScene;
use crate::raytracing::color::Color;
use crate::raytracing::hittable::Hittable;
use crate::raytracing::ray::Ray;
use crate::raytracing::work::RaytracingWork;

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
pub mod work;
pub mod raytracer;

#[derive(Default, Clone)]
pub struct RaytracingContext {
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: usize,
    pub max_bounces: usize,
    pub scene: Arc<RaytracingScene>
}

#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub struct RaytracingResult {
    pub x: u32,
    pub y: u32,
    pub pixel_color: Color,
}

#[derive(Debug, Clone, Default)]
pub struct RaytracingWorker {
    pub current_task_state: Option<RaytracingWorkerTaskState>
}

#[derive(Clone, Debug)]
pub struct RaytracingWorkerTaskState {
    pub data: RaytracingWork,
}

impl Worker for RaytracingWorker {
    type Data = RaytracingWork;
    type Result = RaytracingResult;
    type Context = RaytracingContext;

    fn execute(&mut self, data: Option<Self::Data>, context: &Arc<Self::Context>) -> (Option<Self::Result>, TaskState) {
        let mut rand = thread_rng();

        let data = match data {
            None => {
                match &mut self.current_task_state {
                    None => {
                        return (None, TaskState::Finished);
                    }
                    Some(state) => {
                        state
                    }
                }

            }
            Some(data) => {
                self.current_task_state = Some(RaytracingWorkerTaskState {
                    data,
                });

                match &mut self.current_task_state {
                    None => panic!(""),
                    Some(state) => {
                        state
                    }
                }
            }
        };

        let work = data.data.get_next_work_pixel();

        let work = match work {
            None => {
                self.current_task_state = None;
                return (None, Finished);
            }
            Some(work) => {
                work
            }
        };

        let image_width = context.image_width;
        let image_height = context.image_height;

        let x = work.0;
        let y = work.1;

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
            x,
            y,
            pixel_color: color
        };
        (Some(result), TaskState::Continue)
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

pub const NEAR_ZERO_THRESHOLD: f64 = f64::EPSILON;
