use std::f32::consts::{PI, TAU};

use anyhow::Result;
use bmp::{
    consts::{
        BLUE, BLUE_VIOLET, CORAL, DARK_GREEN, DARK_RED, DARK_VIOLET, GREEN, GREY, LIME,
        PALE_TURQUOISE, PINK, PURPLE, RED, YELLOW,
    },
    Image, Pixel,
};
use vec::{Painter2, Vec1, Vec2};

pub mod vec;

pub fn scene(im: &mut Image) {
    // line(im, Vec2(0, 480 - 1), Vec2(640, 0), |_| RED); // bottom left -> top right
    // line(im, Vec2(0, 0), Vec2(640-1, 480-1), |_| RED);
    // tri(im, 200, 0.5, Vec2(320, 240), |_| YELLOW);

    ascii_ch(im, Vec2(100, 20), 'H', |_| CORAL);
    // ascii_ch(im, Vec2(100, 50), 'I', |_| CORAL);
    // ascii_ch(im, Vec2(100, 60), '!', |_| CORAL);
    straight_line(im, Vec2(0, 240), Vec2(640 - 1, 240), |_| BLUE);
    straight_line(im, Vec2(320, 0), Vec2(320, 480 - 1), |_| BLUE);
    let count = 50;
    // good: 0,2,3,5,10
    // good-except-abs(bpn)==1: 2,7,8
    // bad: 1,4,6,9
    for i in 0..=count {
        let a = (TAU / (count as f32)) * (i as f32);
        let offset = Vec2((a.cos() * 100.0) as i64, (a.sin() * 100.0) as i64);
        eprintln!("{i} {{a={a}}}, off=[{} | {}]}}", offset.0, offset.1);
        let base = Vec2(320, 240);
        line(im, base, base + offset, |_| RED);

        im.save(format!("target/test-{i:06}.bmp")).unwrap();
    }

    // let quart = 0.26f32;
    // line(
    //     im,
    //     Vec2(320, 240),
    //     Vec2(320, 240) + Vec2((quart.cos() * 100.0) as i64, (quart.sin() * 100.0) as i64),
    //     |_| RED,
    // );
}

#[inline]
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

pub fn straight_line(im: &mut Image, from: Vec2, to: Vec2, painter: Painter2) {
    if from.0 == to.0 {
        let min = from.1.min(to.1);
        let max = from.1.max(to.1);
        for y in min..=max {
            let coord = Vec2(from.0, y);
            dot(im, coord, painter(coord));
        }
    } else if from.1 == to.1 {
        let min = from.0.min(to.0);
        let max = from.0.max(to.0);
        for x in min..=max {
            let coord = Vec2(x, from.1);
            dot(im, coord, painter(coord));
        }
    } else {
        panic!("straight_line was called with a non-straight pair {from:?} -> {to:?}");
    }
}

pub fn xy_uncombined_line(im: &mut Image, from: Vec2, to: Vec2, _painter: Painter2) {
    straight_line(im, Vec2(to.0, from.1), Vec2(to.0, to.1), |_| YELLOW);
    straight_line(im, Vec2(to.0, to.1), Vec2(from.0, to.1), |_| GREEN);
}

/// [`origin`] is top left, as is standard for this library.
// TODO: ugh so it seems the font isn't encoded as I thought it'd be
// I was trying [8*ch + 8*y+yi] then poking at the bitfield (8x8 font, x8 is in a byte)
// thinking that each byte would be a row of pixels
// (I converted it to that since it was a boolean array before which wastes 7 bits for each element)
//
// but it seems it's the other way around and was just incidentally sorta-working. i think.
// anyway, enough programmy for now
pub fn ascii_ch(im: &mut Image, origin: Vec2, ch: char, painter: Painter2) {
    assert!(ch.is_ascii());
    let uch = ch as u8;
    let font = include_bytes!("./8x8.bin");
    // for y in 0..8 {
    //     let byte = font[(8 * uch + y) as usize];
    //     for x in 0..8 {
    //         let at = Vec2(x as i64, y as i64);
    //         if (byte & (1 << x)) != 0 {
    //             dot(im, origin + at, painter(origin + at));
    //         }
    //     }
    // }

    for y in 0..(470 / 13) {
        for x in 0..(600 / 12) {
            let begin = Vec2((x * 12) as i64, (y * 13) as i64);
            // rect(
            //     im,
            //     begin,
            //     begin + Vec2(8, 8),
            //     if x % 2 == 0 && y % 2 == 0 {
            //         |_| DARK_VIOLET
            //     } else {
            //         |_| DARK_GREEN
            //     },
            // );

            for yi in 0..8 {
                let byte = font[((8*65) + yi).min(font.len()-1) as usize];
                for bi in 0..8 {
                    let at = begin + Vec2(yi as i64, bi as i64);
                    if (byte & (1 << bi)) != 0 {
                        dot(im, at, painter(at));
                    }
                }
            }
        }
    }
}

pub fn line(im: &mut Image, from: Vec2, to: Vec2, painter: Painter2) {
    // Most, least significant. b{from,to} relative to that space.
    let (bdx, bdy, bfrom, bto, retrans) = {
        let dx = to.0 - from.0;
        let dy = to.1 - from.1;
        if dx.abs() > dy.abs() {
            (dx, dy, from, to, false)
        } else {
            (dy, dx, from.swap(), to.swap(), true)
        }
    };

    let retransf = match retrans {
        false => |vec: Vec2| vec,
        true => |vec: Vec2| vec.swap(),
    };
    let bdot = |im: &mut Image, at: Vec2, to: Pixel| dot(im, retransf(at), to);

    let mut pos = bfrom;
    {
        bdot(im, pos, YELLOW);
        bdot(im, bto, LIME);
    }

    // we increment dl every pm%(dm/dl)==0. more complicated because lhs is usually non-int
    let ndx = (bdx.checked_div(bdy).unwrap_or(0)) as i64;
    dbg!(ndx);
    for m in 0..bdx.abs() {
        // pos.0 += dm.signum();
        bdot(im, pos, painter(pos));
    }
}
