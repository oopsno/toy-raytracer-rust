#[derive(Debug)]
pub enum ImageError {
    IOError(std::io::Error),
    PNGEncodingError(png::EncodingError),
}

pub fn write_png(
    filename: &str,
    width: usize,
    height: usize,
    pixels: &Vec<u8>,
) -> Result<bool, ImageError> {
    let path = std::path::Path::new(filename);
    let file = std::fs::File::create(path).map_err(ImageError::IOError)?;
    let ref mut w = std::io::BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder
        .write_header()
        .map_err(ImageError::PNGEncodingError)?;
    writer
        .write_image_data(pixels)
        .map_err(ImageError::PNGEncodingError)
        .map(|()| true)
}
