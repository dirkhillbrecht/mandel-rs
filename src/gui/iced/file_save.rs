// For reading and opening files
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::gui::iced::pixels::Pixels;

/// Write the given Pixels data into a PNG file with the given name
pub fn write_image_png(name: String, pixels: Pixels) {
    let path = Path::new(&name);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, pixels.size.width as u32, pixels.size.height as u32); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    //    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2)); // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = png::SourceChromaticities::new(
        // Using unscaled instantiation here
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000),
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&pixels.pixels).unwrap(); // Save
}

/// Show a file name selection dialog and return the selected file name if one is given, None otherwise
pub fn show_save_file_dialog() -> Option<String> {
    use rfd::FileDialog;

    FileDialog::new()
        //    .set_directory("/")
        .save_file()
        .map(|s| s.into_os_string())
        .map(|s| s.into_string())
        .and_then(|r| r.ok())
}

// end of file
