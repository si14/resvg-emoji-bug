use std::sync::Arc;
use resvg::usvg;



pub fn main() {
    #[allow(unused_mut, unused_assignments)]
    let mut kind = "without-raster-images";

    #[cfg(feature = "with-raster-images")]
    {
        kind = "with-raster-images";
    }


    let mut fontdb = usvg::fontdb::Database::new();
    fontdb.load_font_data(include_bytes!("../fonts/nunito/Nunito-Regular.ttf").to_vec());
    fontdb.load_font_data(include_bytes!("../fonts/apple_color_emoji/AppleColorEmojiSubset.ttf").to_vec());

    let options = usvg::Options {
        fontdb: Arc::new(fontdb),
        ..usvg::Options::default()
    };

    let svg = include_str!("../test.svg");
    let tree = usvg::Tree::from_str(svg, &options).unwrap();

    let pixmap_size = tree.size().to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();

    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

    let filename = format!("{kind}.png");
    pixmap.save_png(&filename).unwrap();
    println!("Test image saved to {filename}");
}