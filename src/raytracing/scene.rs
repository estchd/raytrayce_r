use rand::{Rng, thread_rng};
use std::sync::Arc;
use crate::raytracing::camera::Camera;
use crate::raytracing::color::Color;
use crate::raytracing::hit_record::HitRecord;
use crate::raytracing::hittable::Hittable;
use crate::raytracing::hittable::sphere::Sphere;
use crate::raytracing::materials::dielectric::Dielectric;
use crate::raytracing::materials::lambertian::Lambertian;
use crate::raytracing::materials::metal::Metal;
use crate::raytracing::ray::Ray;
use crate::raytracing::vector_3d::Vec3;

#[derive(Default, Clone)]
pub struct RaytracingScene {
    pub camera: Camera,
    pub hittables: Vec<Arc<dyn Hittable + 'static + Send + Sync>>
}

impl RaytracingScene {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(),
            hittables: vec![]
        }
    }
    
    pub fn create_scene(aspect_ratio: f64) -> Self {
        let mut rand = thread_rng();

        let mut hittables: Vec<Arc<dyn Hittable + Send + Sync + 'static>> = vec![];

        let ground_material = Arc::new(Lambertian::create(Color::create(0.5,0.5,0.5,1.0)));
        let ground_sphere = Arc::new(Sphere::create(Vec3::create(0.0, -1000.0, 0.0), 1000.0, ground_material));
        hittables.push(ground_sphere);

        let center_material = Arc::new(Dielectric::create(1.5));
        let center_sphere = Arc::new(Sphere::create(Vec3::create(0.0, 1.0, 0.0), 1.0, center_material));
        hittables.push(center_sphere);

        let left_material = Arc::new(Lambertian::create(Color::create(0.4,0.2,0.1,1.0)));
        let left_sphere = Arc::new(Sphere::create(Vec3::create(-4.0, 1.0, 0.0), 1.0, left_material));
        hittables.push(left_sphere);

        let right_material = Arc::new(Metal::create(Color::create(0.7,0.6,0.5,1.0), 0.0));
        let right_sphere = Arc::new(Sphere::create(Vec3::create(4.0, 1.0, 0.0), 1.0, right_material));
        hittables.push(right_sphere);

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = rand.gen_range(0.0..1.0);
                let center = Vec3::create(a as f64 + 0.9 * rand.gen_range(0.0..1.0), 0.2, b as f64 + 0.9 * rand.gen_range(0.0..1.0));

                if (center - Vec3::create(4.0,0.2,0.0)).length() > 0.9 {
                    if choose_mat < 0.8 {
                        let albedo = Color {
                            r: rand.gen_range(0.0..=1.0) * rand.gen_range(0.0..1.0),
                            g: rand.gen_range(0.0..=1.0) * rand.gen_range(0.0..1.0),
                            b: rand.gen_range(0.0..=1.0) * rand.gen_range(0.0..1.0),
                            a: 1.0
                        };
                        let sphere_material = Arc::new(Lambertian::create(albedo));
                        let sphere = Arc::new(Sphere::create(center, 0.2, sphere_material));
                        hittables.push(sphere);
                    }
                    else if choose_mat < 0.95 {
                        let albedo = Color {
                            r: rand.gen_range(0.5..=1.0),
                            g: rand.gen_range(0.5..=1.0),
                            b: rand.gen_range(0.5..=1.0),
                            a: 1.0,
                        };
                        let fuzz = rand.gen_range(0.0..=0.5);
                        let sphere_material = Arc::new(Metal::create(albedo, fuzz));
                        let sphere = Arc::new(Sphere::create(center, 0.2, sphere_material));
                        hittables.push(sphere);
                    }
                    else {
                        let sphere_material = Arc::new(Dielectric::create(1.5));
                        let sphere = Arc::new(Sphere::create(center, 0.2, sphere_material));
                        hittables.push(sphere);
                    }
                }
            }
        }

        let look_from = Vec3::create(13.0,2.0,3.0);
        let look_at = Vec3::create(0.0,0.0,0.0);
        let focus_distance = 10.0;

        let camera = Camera::create(
            look_from,
            look_at,
            Vec3::create(0.0, 1.0, 0.0),
            20.0,
            aspect_ratio,
            focus_distance,
            0.01
        );

        RaytracingScene {
            camera,
            hittables
        }
    }
}

impl Hittable for RaytracingScene {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_t = t_max;

        for hittable in &self.hittables {
            if let Some(new_record) = hittable.hit(ray, t_min, closest_t) {
                closest_t = new_record.t;
                hit_record = Some(new_record);
            }
        }

        hit_record
    }
}
