use anyhow::Result;
use bmp::{
    consts::{BLUE, GREEN, LIME, RED, YELLOW},
    Image, Pixel,
};
use vec::{Painter2, Vec1, Vec2};

pub mod vec;

pub fn scene(im: &mut Image) {
    // line(im, Vec2(0, 480 - 1), Vec2(640, 0), |_| RED); // bottom left -> top right
    // line(im, Vec2(0, 0), Vec2(640-1, 480-1), |_| RED);
    tri(im, 200, 0.5, Vec2(320, 240), |_| YELLOW);
}

pub fn dot(im: &mut Image, at: Vec2, to: Pixel) {
    assert!(at.0 >= 0 && at.0 < im.get_width().into());
    assert!(at.1 >= 0 && at.1 < im.get_height().into());
    im.set_pixel(at.0.try_into().unwrap(), at.1.try_into().unwrap(), to);
}

pub fn rect(im: &mut Image, from: Vec2, to: Vec2, painter: Painter2) {
    for y in from.1..=to.1 {
        for x in from.0..=to.0 {
            dot(im, Vec2(x, y), painter(Vec2(x, y)));
        }
    }
}
pub fn centered_square(im: &mut Image, origin: Vec2, radius: Vec1, painter: Painter2) {
    let extended = Vec2(radius, radius);
    rect(im, origin - extended, origin + extended, painter);
}

pub fn tri(im: &mut Image, side_length: Vec1, angle: f32, origin: Vec2, painter: Painter2) {
    let scale = side_length as f32;
    let vert_point = Vec2(0, (angle.sin() * scale) as Vec1) + origin;
    let horiz_point = Vec2((angle.cos() * scale) as Vec1, 0) + origin;
    centered_square(im, origin, 10, |_| LIME);
    centered_square(im, horiz_point, 10, |_| LIME);
    centered_square(im, vert_point, 10, |_| LIME);
    line(im, horiz_point, origin, painter);
    line(im, horiz_point, vert_point, painter);
    line(im, vert_point, origin, painter);
}

pub fn line(im: &mut Image, from: Vec2, to: Vec2, painter: Painter2) {
    // y=mx+b
    let dx = to.0 - from.0;
    let dy = to.1 - from.1;

    let mut error = dx.abs() + -dy.abs();
    let mut pos = from;
    loop {
        eprintln!("--");
        dbg!(pos, dx, dy);
        dot(im, pos, painter(pos));
        if pos == to || error <=0 {
            break;
        }
        if (error * 2) > dy {
            if pos.0 == to.0 {
                break;
            }
            error -= dy.abs();
            pos.0 += dx.signum();
        }
        if (error * 2) <= dx {
            if pos.1 == to.1 {
                break;
            }
            error += dx.abs();
            pos.1 += dy.signum();
        }
        dbg!(error);
    }
}
