use glam::DVec2;
use image::RgbImage;
use light::{camera::Camera, canvas, world::World};

fn main() {
    let width: u32 = 512;
    let height: u32 = 512;

    let mut world = World::new();
    let camera = Camera {
        focal_length: 1.0,
        hole_width: 0.1,
        sensor_size: DVec2::new(2.0, 2.0),
    };
    let mut canvas = canvas::Canvas::new(width, height);

    let iters = 100;

    for i in 0..iters {
        println!("{}/{}", i, iters);
        world.process(camera, &mut canvas);
    }

    canvas.save("out.png");
}
