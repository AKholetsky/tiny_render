use tgaimage::{TGAColor, TGAImage};


fn main() {
    let white = TGAColor::rgba(255, 255, 255, 255);
    let red = TGAColor::rgba(255, 0, 0, 255);

    let mut image = TGAImage::new(100, 100, 4);
    line(13, 20, 80, 40, &mut image, white); 
    line(20, 13, 40, 80, &mut image, red); 
    line(80, 40, 13, 20, &mut image, red); 
    image.flip_vertically();
    image.write_tga_file("outpu.tga", true);
}

fn line(x0: usize, y0: usize, x1: usize, y1: usize, image: &mut TGAImage, color: TGAColor) {
    let x_len = (x1 as f32 - x0 as f32) as f32;
    // let x_len = 1.0 - 2.0;
    // let y_len = (y1 - y0) as f32;
    
    for x in x0..=x1 {
        let t = (x - x0) as f32 / x_len;
        let y = y0 as f32 * (1. - t) + y1 as f32 * t;
        image.set(x, y as usize, &color);    
    }
}