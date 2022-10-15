use glam::DVec2;
use light::{camera::Camera, canvas::Canvas, config::CONFIG, world::World};

#[show_image::main]
fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    std::fs::create_dir_all(&CONFIG.out_dir).unwrap();

    let width: u32 = 512;
    let height: u32 = 512;

    let mut world = World::new();
    let camera = Camera {
        hole_radius: 0.1,
        focal_length: 1.0,
        sensor_size: DVec2::new(2.0, 2.0),
    };
    let mut canvas = Canvas::new(width, height);

    let mut itt = 0;
    loop {
        let start_time = std::time::Instant::now();
        world.process(camera, &mut canvas);

        canvas.update_fading();

        if itt % 10 == 0 {
            canvas.save(format!("{}/step-{}.png", CONFIG.out_dir, itt));
        }

        canvas.show();

        log::info!("Iteration {}; Time: {:?}", itt, start_time.elapsed());
        itt += 1;
    }
}
