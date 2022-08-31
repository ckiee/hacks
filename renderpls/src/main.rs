use anyhow::Result;
use bmp::{
    consts::{BLUE, GREEN, LIME, RED, YELLOW},
    Image, Pixel,
};
use renderpls::scene;
use renderpls::vec::{Painter2, Vec1, Vec2};

fn main() -> Result<()> {
    let mut im = Image::new(640, 480);

    scene(&mut im);

    im.save("target/test.bmp")?;

    Ok(())
}
