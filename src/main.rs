use graphicsmagick::{initialize, types::FilterTypes, types::GravityType, wand::MagickWand};
use std::path::PathBuf;
use anyhow::Context;
fn make_square(filename: &str) -> anyhow::Result<()> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("yeet.jpg");
    let path = path.to_str().context("get image path failed")?;

    initialize();

    let mut binding = MagickWand::new();
    let mut mw = binding
        .read_image(path)?
        .set_image_gravity(graphicsmagick::types::GravityType::CenterGravity)?
        .transform_image("", "2000x2000>").expect("yeet");

    if mw.get_image_height() == 2000 {
        let x = &mw.get_image_width() / 2;
        mw.extent_image(2200, 2200, (1100 - x).try_into().unwrap(), 100)?;
    } else {
        let y = &mw.get_image_height() / 2;
        mw.extent_image(2200, 2200, 100, (1100 - y).try_into().unwrap())?;
    }

    mw.write_image("square2.jpg")?;

    //let mut wand = MagickWand::new();
    //wand.read_image(filename).unwrap();
    //wand.set_gravity(GravityType_CenterGravity);
    //wand.extend_image(2200, 2200, -100, -100);
    //wand.crop_image(2200, 2200, 0, 0);

    //wand.write_image("square.jpeg");
    Ok(())
}
fn main() {
    make_square("yeet.jpg");
    ()
}
