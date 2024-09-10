

pub fn render(pixels: &mut [u8]) {
    let (width, height) = (800, 600);
    for y in 0..height {
        for x in 0..width {
            let rgb = [0.0, 0.0, 0.0];
            let i = y * width + x;
            pixels[i * 3 + 0] = (rgb[0] * 255.0) as u8;
            pixels[i * 3 + 1] = (rgb[1] * 255.0) as u8;
            pixels[i * 3 + 2] = (rgb[2] * 255.0) as u8;
        }
    }
}
