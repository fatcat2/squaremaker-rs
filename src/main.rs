use graphicsmagick::{initialize, types::FilterTypes, types::GravityType, wand::MagickWand};
use std::path::PathBuf;

fn make_square(filename: &str) -> Result<&str, &str> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("yeet.jpg");
    let path = path.to_str().context("get image path failed")?;

    let mut mw = MagickWand::new()
        .read_image(path)?
        .transform_image("", "2000x2000>")?
        .set_gravity(graphicsmagick::types::GravityType::CenterGravity)?
        .extent_image(2200, 2200, -100, -100)?
        .write_image("square.jpg")?;

    //let mut wand = MagickWand::new();
    //wand.read_image(filename).unwrap();
    //wand.set_gravity(GravityType_CenterGravity);
    //wand.extend_image(2200, 2200, -100, -100);
    //wand.crop_image(2200, 2200, 0, 0);

    //wand.write_image("square.jpeg");

    Ok("yeet")
}
fn main() {
    make_square("yeet.jpg");
    ()
}
