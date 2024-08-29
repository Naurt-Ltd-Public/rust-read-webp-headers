use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

const FILEPATH: &'static str = "images/naurt_phone.webp";

/// A WebP header contains the string "RIFF", the filesize in bytes, and the string "WEBP".
#[derive(Debug)]
pub struct WebpHeader {
    riff: String,
    file_size: u32,
    webp: String,
    chunk_type: String,
    chunk_header: ExtenderChunkHeader,
}

#[derive(Debug)]
pub struct ExtenderChunkHeader {
    icc_profile: bool,
    alpha: bool,
    exif_metadata: bool,
    xmp_metadata: bool,
    animation: bool,
    width: u32,
    height: u32,
}

// Source: https://developers.google.com/speed/webp/docs/webp_lossless_bitstream_specification
// Source: https://developers.google.com/speed/webp/docs/riff_container

impl WebpHeader {
    fn new_from_buf_reader<R>(reader: &mut R) -> Self
    where
        R: Read,
    {
        // WebP header is 12 bytes. 4 for RIFF, 4 for file size, 4 for WebP.

        let mut four_byte_buffer = [0; 4];

        reader.read(&mut four_byte_buffer).unwrap();

        let riff = String::from_utf8(four_byte_buffer.to_vec()).unwrap();

        reader.read(&mut four_byte_buffer).unwrap();

        // We need to add 8 to get the total file size as it doesn't include the riff and webp strings.
        // Little endian encoding.
        let file_size = ((four_byte_buffer[0] as u32)
            | (four_byte_buffer[1] as u32) << 8
            | (four_byte_buffer[2] as u32) << 16
            | (four_byte_buffer[3] as u32) << 24)
            + 8;

        reader.read(&mut four_byte_buffer).unwrap();

        let webp = String::from_utf8(four_byte_buffer.to_vec()).unwrap();

        // The chunk header is 8 bytes. First 4 should contain the header type. 'VP8 ', 'VP8L' or 'VP8X'
        // TODO: Does nect 4 bytes have the size in??
        let mut eight_byte_buffer = [0; 8];

        reader.read(&mut eight_byte_buffer).unwrap();

        let chunk_type = String::from_utf8(eight_byte_buffer[0..4].to_vec()).unwrap();

        // reader.read(&mut four_byte_buffer).unwrap();
        // println!("Next four bytes: {:?}", four_byte_buffer);
        // This one is slightly different. We want to extract the individual bits form our four bytes.

        reader.read(&mut four_byte_buffer).unwrap();

        // First two bits are ignored. Reserved.

        // ICC profile is the third bit, so we shift by 3. This is still within our first byte.
        let icc_profile_mask = 1 << (8 - 3);
        let icc_profile = (icc_profile_mask & four_byte_buffer[0]) > 0;

        // Alpha profile is the fourth bit.
        let alpha_mask = 1 << (8 - 4);
        let alpha = (alpha_mask & four_byte_buffer[0]) > 0;

        // Exif metadata is the fifth bit.
        let exif_metadata_mask = 1 << (8 - 5);
        let exif_metadata = (exif_metadata_mask & four_byte_buffer[0]) > 0;

        // XMP metadata is the sixth bit.
        let xmp_metadata_mask = 1 << (8 - 6);
        let xmp_metadata = (xmp_metadata_mask & four_byte_buffer[0]) > 0;

        // Alpha profile is the seventh bit.
        let animation_mask = 1 << (8 - 7);
        let animation = (animation_mask & four_byte_buffer[0]) > 0;

        // This is the first byte finished as the last bit is reserved.

        // The next 24 bits are reserved and just 0. Making up the rest of the 4 bytes.

        // The width and height to come are the next 6 bytes. So let's read in three at a time now as they are 24 bit.

        let mut three_byte_buffer = [0; 3];

        reader.read(&mut three_byte_buffer).unwrap();

        let width = ((three_byte_buffer[0] as u32)
            | (three_byte_buffer[1] as u32) << 8
            | (three_byte_buffer[2] as u32) << 16)
            + 1;

        reader.read(&mut three_byte_buffer).unwrap();

        let height = ((three_byte_buffer[0] as u32)
            | (three_byte_buffer[1] as u32) << 8
            | (three_byte_buffer[2] as u32) << 16)
            + 1;

        let chunk_header = ExtenderChunkHeader {
            icc_profile,
            alpha,
            exif_metadata,
            xmp_metadata,
            animation,
            width,
            height,
        };

        return Self {
            riff,
            file_size,
            webp,
            chunk_type,
            chunk_header,
        };
    }
}
fn main() {
    let file = File::open(FILEPATH).unwrap();

    let mut reader = BufReader::new(file);

    let header = WebpHeader::new_from_buf_reader(&mut reader);

    println!("Header: {:?}", header);
}

fn basic_read_and_print() {
    let file = File::open(FILEPATH).unwrap();

    let mut reader = BufReader::new(file);

    let mut four_byte_buffer = [0; 4];

    reader.read(&mut four_byte_buffer).unwrap();

    println!("Header: {:?}", four_byte_buffer);

    let my_string = String::from_utf8(four_byte_buffer.to_vec().to_ascii_uppercase()).unwrap();

    println!("RIFF Starting string {}", my_string);

    reader.read(&mut four_byte_buffer).unwrap();

    println!("File size bytes: {:?}", four_byte_buffer);

    // Little endian
    println!(
        "File size in bytes: {}",
        (four_byte_buffer[0] as u32) << 0
            | (four_byte_buffer[1] as u32) << 8
            | (four_byte_buffer[2] as u32) << 16
            | (four_byte_buffer[3] as u32) << 24
    );

    reader.read(&mut four_byte_buffer).unwrap();

    println!(
        "WEBP String: {:?}",
        String::from_utf8(four_byte_buffer.to_vec().to_ascii_uppercase()).unwrap()
    );

    reader.read(&mut four_byte_buffer).unwrap();

    // This last piece varies based on the format. 'VP8 'for simple lossy, 'VP8L' for lossless
    println!(
        "VP8 space: {:?}",
        String::from_utf8(four_byte_buffer.to_vec().to_ascii_uppercase()).unwrap()
    );

    // Read in the first chunk of data - this contains a header which has height x width.
    reader.read(&mut four_byte_buffer).unwrap();

    println!("Bytes in stream: {:?}", four_byte_buffer);

    let mut single_byte_buffer = [0; 1];

    reader.read(&mut four_byte_buffer).unwrap();

    println!("Single buffer: {:?}", single_byte_buffer);

    // Read in the first chunk of data - this contains a header which has height x width.
    reader.read(&mut four_byte_buffer).unwrap();

    println!("Size and stuff in stream: {:?}", four_byte_buffer);
}
