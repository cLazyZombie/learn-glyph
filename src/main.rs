use ab_glyph::*;
use image::*;

fn main() {
    let font = FontRef::try_from_slice(include_bytes!("../fonts/CONSOLA.TTF")).unwrap();
    let glyph = font
        .glyph_id('A')
        .with_scale_and_position(100.0, point(0.0, 0.0));
    let outline_glyph = font.outline_glyph(glyph).unwrap();

    let bounds = outline_glyph.px_bounds();
    let mut buffer = vec![0.0; bounds.width() as usize * bounds.height() as usize];

    let mut glyph_image: RgbaImage =
        ImageBuffer::new(bounds.width() as u32, bounds.height() as u32);

    outline_glyph.draw(|x, y, v| {
        let idx = x as usize + (y as usize * bounds.width() as usize);
        buffer[idx] = v;

        //let pixel = [(v * 256.0) as u8, (v * 256.0) as u8, (v * 256.0) as u8].into();
        let pixel = [255, 255, 255, (v * 255.0) as u8].into();
        glyph_image.put_pixel(x, y, pixel);
    });

    glyph_image
        .save_with_format("./A.png", ImageFormat::Png)
        .unwrap();
}
