extern crate leptess;

use std::path::Path;
use leptess::{leptonica, tesseract};

fn main() {
    let mut api = tesseract::TessApi::new(None, "eng").unwrap();

    let pix = leptonica::pix_read(Path::new("./tests/di.png")).unwrap();
    api.set_image(&pix);

    if api.get_source_y_resolution() <= 0 {
        api.set_source_resolution(70)
    }
}
