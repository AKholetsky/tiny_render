use tgaimage::{TGAColor, TGAImage};
use obj::{Obj, SimplePolygon, IndexTuple};
use std::path::Path;


fn main() {
    let african_head = Obj::<SimplePolygon>::load(&Path::new("resources/african_head.obj")).unwrap();

    let white = TGAColor::rgb(255, 255, 255);

    let width = 800;
    let height = 800;
    let mut image = TGAImage::new(width, height, 3);

    for face in african_head.objects[0].groups[0].polys.iter() {
        for i in 0..3 {
            let IndexTuple(a, _, _) = face[i];
            let IndexTuple(d, _, _) = face[(i + 1) % 3];
            let v0 = african_head.position[a];    
            let v1 = african_head.position[d];

            let x0 = (v0[0] + 1.) * (width as f32 / 2.);
            let y0 = (v0[1] + 1.) * (height as f32 / 2.);
            let x1 = (v1[0] + 1.) * (width as f32 / 2.);
            let y1 = (v1[1] + 1.) * (height as f32 / 2.);
            line(x0 as i32, y0 as i32, x1 as i32, y1 as i32, &mut image, white);
        }
    }
    image.flip_vertically();
    image.write_tga_file("outpu.tga", true);
}

fn line(mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, image: &mut TGAImage, color: TGAColor) {
    let mut steep = false;
    if (x0 - x1).abs() < (y0 - y1).abs() {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
        steep = true;
    }

    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    
    let derror2 = dy.abs() * 2;
    let mut error2 = 0;
    let mut y = y0;

    for x in x0..=x1 {
        if steep {
            image.set(y as usize, x as usize, &color);
        } else {
            image.set(x as usize, y as usize, &color);    
        }
        error2 += derror2;
        if error2 > dx {
            y += if y1 > y0 { 1 } else  { -1 };
            error2 -= dx * 2;
        }
    }
}