use std::f32::consts::{PI, TAU};

use anyhow::Result;
use bmp::{
    consts::{
        BLUE, BLUE_VIOLET, CORAL, DARK_RED, GREEN, LIME, PALE_TURQUOISE, PINK, PURPLE, RED, YELLOW,
    },
    Image, Pixel,
};
use vec::{Painter2, Vec1, Vec2};

pub mod vec;

pub fn scene(im: &mut Image) {
    // line(im, Vec2(0, 480 - 1), Vec2(640, 0), |_| RED); // bottom left -> top right
    // line(im, Vec2(0, 0), Vec2(640-1, 480-1), |_| RED);
    // tri(im, 200, 0.5, Vec2(320, 240), |_| YELLOW);

    straight_line(im, Vec2(0, 240), Vec2(640 - 1, 240), |_| BLUE);
    straight_line(im, Vec2(320, 0), Vec2(320, 480 - 1), |_| BLUE);
    let count = 10000;
    // good: 0,2,3,5,10
    // good-except-abs(bpn)==1: 2,7,8
    // bad: 1,4,6,9
    for i in 0..=count {
        eprintln!("{i}");
        let a = ((TAU / (count as f32)) * (i as f32)) - PI;
        let base = Vec2(320, 240);
        line(
            im,
            base,
            base + Vec2((a.cos() * 100.0) as i64, (a.sin() * 100.0) as i64),
            |_| RED,
        );
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

pub fn line(im: &mut Image, ofrom: Vec2, oto: Vec2, painter: Painter2) {
    let (from, to) = if ofrom.0 < oto.0 && ofrom.1 < oto.1 {
        (ofrom, oto)
    } else {
        (oto, ofrom)
    };

    // Most, least significant. b{from,to} relative to that space.
    let (dm, dl, bfrom, bto, retrans) = {
        let dx = to.0 - from.0;
        let dy = to.1 - from.1;
        if dx.abs() > dy.abs() {
            (dx, dy, from, to, false)
        } else {
            (dy, dx, from.swap(), to.swap(), true)
        }
    };

    let mut bpos = bfrom;
    dbg!(bpos, dm, dl);

    {
        let pos = if retrans { bpos.swap() } else { bpos };
        dot(im, pos, LIME);
        dot(im, to, LIME);
    }

    // we increment dl every pm%(dm/dl)==0. more complicated because lhs is usually non-int
    let ex_every = (dm.checked_div(dl).unwrap_or(0)) as i64;
    let mod_corrections = {
        let mut v = vec![];
        match dm.checked_rem(dl).map(|i| i as f32) {
            Some(mut work) => {
                for _ in 0..30 {
                    work = work.sqrt();
                    if !(work == 0.0 || work.is_nan()) {
                        let every = ((dm as f32) / work) as i64;
                        dbg!(work, dm, every);
                        // Eventually we converge on a result, stop generating when that
                        // happens. TODO: This is because divisor==1 (since work < 1.5)
                        match v.last() {
                            Some(last) if *last == every => {
                                break;
                            }
                            _ => {}
                        };
                        v.push(every);
                    }
                }
            }
            None => {}
        }
        v
    };
    dbg!(ex_every, &mod_corrections);

    let mut bpn = 0i64;
    for m in 0..dm.abs() {
        bpos.0 += dm.signum();
        let pos = if retrans { bpos.swap() } else { bpos };
        // dbg!(bpos, dm, dl, ex_every, ey_every);
        if dl != 0 {
            if m % ex_every == 0 {
                bpos.1 += dl.signum();
                if bpn != 0 {
                    bpos.1 += bpn.signum();
                    bpn -= bpn.signum();
                }
                dot(im, Vec2(pos.0, to.1 + 10), BLUE_VIOLET);
            }
        }
        for (ic, corr) in mod_corrections.iter().enumerate() {
            if m % corr == 0 {
                bpn -= dl.signum();
                dot(
                    im,
                    Vec2(pos.0, to.1 + 11),
                    [YELLOW, PURPLE, PINK, CORAL, PALE_TURQUOISE, DARK_RED][ic % 6],
                );
            }
        }

        dot(im, pos, painter(pos));
    }
    eprintln!("bpn should be 0 if pixel hit: {bpn}");
}
