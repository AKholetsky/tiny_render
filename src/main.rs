use tgaimage::{TGAColor, TGAImage};
use cgmath::Vector2;

fn main() {
    let white = TGAColor::rgb(255, 255, 255);
    let green = TGAColor::rgb(0, 255, 0);
    let red = TGAColor::rgb(255, 0, 0);


    let width = 800;
    let height = 800;
    let mut image = TGAImage::new(width, height, 3);

    // let mut t0: [Vector2<f32>; 3] = [Vector2::new(10, 70), Vector2::new(50, 160), Vector2::new(70, 80)]; 
    // let mut t1: [Vector2<f32>; 3] = [Vector2::new(180, 50), Vector2::new(150, 1), Vector2::new(70, 180)]; 
    // let mut t2: [Vector2<f32>; 3] = [Vector2::new(180, 150), Vector2::new(120, 160), Vector2::new(130, 180)]; 

    triangle(Vector2::new(10., 70.), Vector2::new(50., 160.), Vector2::new(70., 80.), &mut image, red);
    triangle(Vector2::new(180., 50.), Vector2::new(150., 1.), Vector2::new(70., 180.), &mut image, white);
    triangle(Vector2::new(180., 150.), Vector2::new(120., 160.), Vector2::new(130., 180.), &mut image, green);

    image.flip_vertically();
    image.write_tga_file("outpu.tga", true);
}

fn triangle(t0: Vector2<f32>, t1: Vector2<f32>, t2: Vector2<f32>, image: &mut TGAImage, color: TGAColor) {
    // if t0.y == t1.y && t0.y == t2.y { return };

    let mut t0 = t0;
    let mut t1 = t1;
    let mut t2 = t2;
    let green = TGAColor::rgb(0, 255, 0);
    let red = TGAColor::rgb(255, 0, 0);
    if t0.y > t1.y { std::mem::swap(&mut t0, &mut t1) };
    if t0.y > t2.y { std::mem::swap(&mut t0, &mut t2) };
    if t1.y > t2.y { std::mem::swap(&mut t1, &mut t2) };

    let t2_minus_t0 = t2 - t0;
    let t1_minus_t0 = t1 - t0;

    let total_height = t2.y - t0.y;
    for y in t0.y as i32..t1.y as i32 {
        let segment_height = t1.y - t0.y + 1.;
        let alpha = (y as f32 - t0.y) / total_height;
        let beta = (y as f32 - t0.y) / segment_height;

        let a = t0 + t2_minus_t0 * alpha;
        let b = t0 + t1_minus_t0 * beta;
        image.set(a.x as usize, y as usize, &red);
        image.set(b.x as usize, y as usize, &green); 
        // println!("A {:?} B {:?} y {}", A, B, y);
    }
    // line(t0, t1, image, green);
    // line(t1, t2, image, green);
    // line(t2, t0, image, red);
}

// fn line(p0: Vector2<f32>, p1: Vector2<f32>, image: &mut TGAImage, color: TGAColor) {
//     let mut p0 = p0;
//     let mut p1 = p1;

//     let mut steep = false;
//     if (p0.x - p1.x).abs() < (p0.y - p1.y).abs() {
//         std::mem::swap(&mut p0.x, &mut p0.y);
//         std::mem::swap(&mut p1.x, &mut p1.y);
//         steep = true;
//     }
    
//     if p0.x > p1.x {
//         std::mem::swap(&mut p0, &mut p1);
//     }
    
//     for x in p0.x..=p1.x {
//         let t = (x as f32 - p0.x as f32)/(p1.x as f32 - p0.x as f32);
//         let y = p0.y as f32 * (1. - t) + p1.y as f32 * t;
//         if steep {
//             image.set(y as usize, x as usize, &color);
//         } else {
//             image.set(x as usize, y as usize, &color);    
//         }
//     }
// }