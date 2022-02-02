use std::sync::{Arc, Mutex};
use std::time::Instant;

use clap::Parser;
use num_cpus;
use pbr::ProgressBar;
use rand::prelude::*;
use rayon::prelude::*;

use toy_raytracer_rust::image;
use toy_raytracer_rust::math::Float;
use toy_raytracer_rust::scene::{create_scene, Scene};
use toy_raytracer_rust::vec3::Vec3;

#[derive(Parser, Debug)]
#[clap(version)]
struct Opts {
    #[clap(short, long, default_value = "weekend")]
    scene: Scene,

    #[clap(short, long, default_value = "1200")]
    image_width: usize,

    #[clap(short, long, default_value = "1.5")]
    aspect_ratio: Float,

    #[clap(short, long, default_value = "0")]
    num_threads: usize,

    #[clap(short, long, default_value = "500")]
    samples_per_pixel: usize,

    #[clap(short, long, default_value = "50")]
    max_depth: i32,

    #[clap(short, long, default_value = "output.png")]
    output: String,
}

fn main() {
    // 解析命令行参数
    let opts: Opts = Opts::parse();

    // 确定图像大小
    let aspect_ratio = opts.aspect_ratio;
    let image_width = opts.image_width;
    let image_height = (image_width as Float / aspect_ratio) as usize;
    let samples_per_pixel = opts.samples_per_pixel;

    // 创建场景
    let (camera, world, ray_color) = create_scene(opts.scene, aspect_ratio);

    // 配置 Rayon
    if 0 < opts.num_threads && opts.num_threads <= num_cpus::get() {
        rayon::ThreadPoolBuilder::new()
            .num_threads(opts.num_threads)
            .build_global()
            .unwrap();
    }
    let num_threads = rayon::current_num_threads();

    // 渲染
    println!(
        "Rendering {}x{} image, {} sample(s) per pixel with {} threads, max depth {}",
        image_width, image_height, samples_per_pixel, num_threads, opts.max_depth
    );
    let start_time = Instant::now();
    let pbar = Arc::new(Mutex::new(ProgressBar::new(image_height as u64)));
    let pixels = (0..image_height)
        .into_par_iter()
        .map(|y| {
            let mut rng = rand::thread_rng();
            let pbar_lock = pbar.clone();
            pbar_lock.lock().unwrap().inc();
            (0..image_width)
                .into_iter()
                .map(|x| {
                    let mut color = Vec3::zeros();
                    for _ in 0..samples_per_pixel {
                        let u = (x as Float + rng.gen::<Float>()) / (image_width - 1) as Float;
                        let v =
                            1. - (y as Float + rng.gen::<Float>()) / (image_height - 1) as Float;
                        let ray = camera.ray(u, v);
                        color = color + ray_color(&ray, &world, opts.max_depth);
                    }
                    // 计算平均色彩并应用 Gamma 校正
                    (color / samples_per_pixel as Float)
                        .apply(Float::sqrt)
                        .into_color()
                        .as_vec_u8()
                })
                .flatten()
                .collect::<Vec<u8>>()
        })
        .flatten()
        .collect::<Vec<u8>>();

    // 保存渲染结果
    match image::write_png(opts.output.as_str(), image_width, image_height, &pixels) {
        Ok(_) => {
            let time_cost = start_time.elapsed().as_millis() as Float / 1000.;
            let message = format!(
                "Rendered in {:.3} secs, saved to {}",
                time_cost, opts.output
            );
            pbar.lock().unwrap().finish_print(message.as_str());
        }
        Err(e) => println!("Error: {:?}", e),
    }
}
