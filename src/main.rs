#![allow(dead_code)]

use crate::application::{RaytracingApplication, RaytracingApplicationSettings};

mod rendering;
mod image;
mod raytracing;
mod gui;
mod application;
mod directx;
mod window;

fn main() {
    let app_settings = RaytracingApplicationSettings {
        width: 1200,
        height: 800,
        x: 50,
        y: 50,
        window_name: "Raytrace Window".to_string()
    };
    let application = RaytracingApplication::create(app_settings);

    application.run();
}