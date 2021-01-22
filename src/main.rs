use bardecoder;
extern crate image;
use image::{DynamicImage, ImageBuffer};
use quirc::QrCoder;
use rascam::*;
use std::fs::File;
use std::io::Write;
use std::{thread, time};

extern crate quirc;

fn main() {
    let info = info().unwrap();
    if info.cameras.len() < 1 {
        println!("Found 0 cameras. Exiting");
        // note that this doesn't run destructors
        ::std::process::exit(1);
    }
    loop {
        read_qr_from_camera(&info.cameras[0]);
    }
}

fn read_qr_from_camera(info: &CameraInfo) {
    const WIDTH: u32 = 3280;
    const HEIGHT: u32 = 2464;

    let mut camera = SimpleCamera::new(info.clone()).unwrap();
    let settings = CameraSettings {
        width: WIDTH,
        height: HEIGHT,
        ..CameraSettings::default()
    };
    camera.configure(settings);
    camera.activate().unwrap();

    let sleep_duration = time::Duration::from_millis(2000);
    thread::sleep(sleep_duration);

    println!("Taking image");
    let img = camera.take_one().unwrap();
    // File::create("image.jpg")
    //     .unwrap()
    //     .write_all(&camera.take_one().unwrap())
    //     .unwrap();

    let decoder = bardecoder::default_decoder();
    // let image_file = image::open("image.jpg").unwrap();
    let image_file = image::load_from_memory(&img).unwrap().to_luma();

    let mut quirc = QrCoder::new().unwrap();

    println!("Decoding");
    // let results = decoder.decode(&image_file);
    let results = quirc.codes(&image_file, WIDTH, HEIGHT).unwrap();
    for result in results {
        match result {
            Ok(msg) => {
                // println!("{}", msg)
                println!("{:?}", msg)
            }
            Err(e) => {
                // println!("{}", e)
                println!("{:?}", e)
            }
        }
    }
}
