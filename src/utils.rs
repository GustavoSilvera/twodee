use image::png::PNGEncoder;
use image::ColorType;

pub fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let encoder = PNGEncoder::new(std::fs::File::create(filename)?);
    encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::RGB(8))?;
    Ok(())
}
