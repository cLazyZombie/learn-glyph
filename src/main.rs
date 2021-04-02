//use ab_glyph::*;
use glyph_brush_layout::{ab_glyph::*, *};
use image::*;

fn main() {
    glyph_brush("hello", "world");
    glyph_to_image('Q');
    glyph_to_image('K');
    glyph_to_image('s');
    glyph_to_image('M');
}

fn glyph_to_image(ch: char) {
    eprintln!("---- image {}", ch);

    let font = FontRef::try_from_slice(include_bytes!("../fonts/CONSOLA.TTF")).unwrap();
    let glyph = font
        .glyph_id(ch)
        .with_scale_and_position(100.0, point(0.0, 0.0));
    let outline_glyph = font.outline_glyph(glyph).unwrap();
    let bounds = outline_glyph.px_bounds();
    eprintln!(
        "bounds: {:?}, width: {}, height: {}",
        bounds,
        bounds.width(),
        bounds.height()
    );

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
    let image_name = format!("./{}.png", ch);

    glyph_image
        .save_with_format(&image_name, ImageFormat::Png)
        .unwrap();
}

fn glyph_brush(s1: &str, s2: &str) {
    let font_1 = FontRef::try_from_slice(include_bytes!("../fonts/CONSOLA.TTF")).unwrap();
    let font_2 = FontRef::try_from_slice(include_bytes!("../fonts/NanumBarunGothic.TTF")).unwrap();

    let fonts = &[font_1, font_2];

    let glyphs = Layout::default()
        .v_align(VerticalAlign::Top)
        .h_align(HorizontalAlign::Left)
        .calculate_glyphs(
            fonts,
            &SectionGeometry {
                screen_position: (150.0, 50.0),
                ..SectionGeometry::default()
            },
            &[
                SectionText {
                    text: s1,
                    scale: PxScale::from(40.0),
                    font_id: FontId(0),
                },
                SectionText {
                    text: s2,
                    scale: PxScale::from(60.0),
                    font_id: FontId(1),
                },
            ],
        );

    // create image
    let mut glyphs_image: RgbaImage = ImageBuffer::new(512, 512);
    for section_glyph in &glyphs {
        let font = &fonts[section_glyph.font_id];
        let outline_glyph = font.outline_glyph(section_glyph.glyph.clone()).unwrap();
        let bounds = dbg!(outline_glyph.px_bounds());
        outline_glyph.draw(|x, y, v| {
            let pixel = [255, 255, 255, (v * 255.0) as u8].into();
            let x = x + bounds.min.x as u32;
            let y = y + bounds.min.y as u32;
            glyphs_image.put_pixel(x, y, pixel);
        })
    }

    let image_name = "./glyph_brush.png";
    glyphs_image
        .save_with_format(image_name, ImageFormat::Png)
        .unwrap();
}
