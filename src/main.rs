use std::{
    fs::File,
    io::{BufReader, Read},
};

use image::ImageReader;

const FILEPATH: &'static str = "images/naurt_phone.webp";

fn main() {
    let img = ImageReader::open(FILEPATH).unwrap().decode().unwrap();

    let file = File::open(FILEPATH).unwrap();

    let mut reader = BufReader::new(file);

    let mut header_buffer = [0; 4];

    reader.read(&mut header_buffer).unwrap();

    println!("Header: {:?}", header_buffer);

    let my_string = String::from_utf8(header_buffer.to_vec().to_ascii_uppercase()).unwrap();

    println!("RIFF Starting string {}", my_string);

    reader.read(&mut header_buffer).unwrap();

    println!("File size bytes: {:?}", header_buffer);

    // Little endian
    println!(
        "File size in bytes: {}",
        (header_buffer[0] as u32) << 0
            | (header_buffer[1] as u32) << 8
            | (header_buffer[2] as u32) << 16
            | (header_buffer[3] as u32) << 24
    );

    reader.read(&mut header_buffer).unwrap();

    println!(
        "WEBP String: {:?}",
        String::from_utf8(header_buffer.to_vec().to_ascii_uppercase()).unwrap()
    );

    reader.read(&mut header_buffer).unwrap();

    // This last piece varies based on the format. 'VP8 'for simple lossy, 'VP8L' for lossless
    println!(
        "VP8 space: {:?}",
        String::from_utf8(header_buffer.to_vec().to_ascii_uppercase()).unwrap()
    );
}
