use bardecoder;
extern crate image;
use image::{DynamicImage, ImageBuffer};
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

    // Raspberry Pi HQ Cam resolution / 5
    const WIDTH: u32 = 811;
    const HEIGHT: u32 = 608;

    let mut camera = SimpleCamera::new(info.cameras[0].clone()).unwrap();
    let settings = CameraSettings {
        width: WIDTH,
        height: HEIGHT,
        ..CameraSettings::default()
    };
    camera.configure(settings);
    camera.activate().unwrap();

    let sleep_duration = time::Duration::from_millis(2000);
    thread::sleep(sleep_duration);

    loop {
        let img = camera.take_one().expect("Failed to take image");
        read_qr_from_camera(img);
    }
}

fn read_qr_from_camera(img: Vec<u8>) {
    println!("Taking image");

    let decoder = bardecoder::default_decoder();
    let image_file = image::load_from_memory(&img).unwrap();

    println!("Decoding");
    let results = decoder.decode(&image_file);
    for result in results {
        match result {
            Ok(msg) => {
                println!("{}", msg)
            }
            Err(e) => {
                println!("{}", e)
            }
        }
    }
}
