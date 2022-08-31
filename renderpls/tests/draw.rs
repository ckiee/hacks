use std::env::current_dir;
use std::fs;
use std::path::{PathBuf, Path};
use std::process::Command;

use anyhow::Result;
use bmp::consts::{WHITE, YELLOW};
use bmp::{consts::RED, Image};
use renderpls::vec::Vec2;
use renderpls::{line, tri};

fn test(name: &str, cb: fn(&mut Image)) {
    let mtest_dir = current_dir().unwrap().join("target").join("mtest");
    let _ = fs::create_dir(&mtest_dir);
    let mut im = Image::new(640, 480);
    cb(&mut im);

    let file = mtest_dir.join(name);
    let bmp = file.with_extension("bmp");
    let png = file.with_extension("png");

    im.save(&bmp).unwrap();
    let _ = fs::remove_file(&png);
    fn convert(resize: bool, from: &Path, to: &Path) {
        let mut res = vec!["-resize", "128x128"];
        if !resize {res.clear();}
        Command::new("convert")
            .arg(from)
            .args(res)
            .arg(to)
            .spawn()
            .unwrap();
    }
    convert(false, &bmp, &png.with_extension("hi.png"));
    convert(true, &bmp, &png);
    // convenient size for emacs viewing
}

#[test]
fn test_line_tl_to_br() {
    test("tl_to_br", |im| {
        line(im, Vec2(0, 0), Vec2(638, 479), |_| RED);
    });


    test("tl_to_br_sharp_slope", |im| { // hangs forever currently. TODO!
        line(im, Vec2(0, 0), Vec2(300 - 1, 480), |_| RED);
    });
}

#[test]
fn test_line_bl_to_tr() {
    test("bottomleft_to_topright", |im| {
        line(im, Vec2(0, 480 - 1), Vec2(640, 0), |_| RED); // bottom left -> top right
    });
}

#[test]
fn test_tri() {
    test("simple_tri", |im| { // also this hangs forever too since it makes sharp tris. TODO!
        tri(im, 200, 0.5, Vec2(320, 240), |_| YELLOW);
    });
}
