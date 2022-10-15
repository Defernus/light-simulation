use glam::DVec2;
use light::{camera::Camera, canvas::Canvas, config::CONFIG, world::World};

fn main() {
    dotenv::dotenv().ok();

    std::fs::create_dir_all(&CONFIG.out_dir).unwrap();

    let width: u32 = 512;
    let height: u32 = 512;

    let mut world = World::new();
    let camera = Camera {
        hole_radius: 0.01,
        focal_length: 1.0,
        sensor_size: DVec2::new(2.0, 2.0),
    };
    let mut canvas = Canvas::new(width, height);

    let iters = 100;

    for i in 0..iters {
        println!("{}/{}", i, iters);
        world.process(camera, &mut canvas);

        canvas.update_fading();

        if i % 10 == 0 {
            canvas.save(format!("{}/step-{}.png", CONFIG.out_dir, i));
        }
    }

    canvas.save(format!("{}/final.png", CONFIG.out_dir));
}
