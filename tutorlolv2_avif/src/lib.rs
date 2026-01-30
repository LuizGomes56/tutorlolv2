use image::ImageReader;
use ravif::RGBA8;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{fs, io::Write};

fn convert_to_avif(
    source: &str,
    destination: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    const QUALITY: f32 = 80.0;

    println!("Converting {source} to {destination}");
    let img = ImageReader::open(source)?
        .with_guessed_format()?
        .decode()?
        .to_rgba8();
    let width = img.width();
    let height = img.height();
    let pixels = img.into_raw();

    let pixels_rgba: Vec<RGBA8> = pixels
        .chunks_exact(4)
        .map(|chunk| RGBA8::new(chunk[0], chunk[1], chunk[2], chunk[3]))
        .collect();

    let res = ravif::Encoder::new()
        .with_quality(QUALITY)
        .with_speed(1)
        .encode_rgba(ravif::Img::new(
            &pixels_rgba,
            width as usize,
            height as usize,
        ))?;

    let file = fs::File::create(&destination)?;
    let mut buf = std::io::BufWriter::new(file);
    buf.write_all(&res.avif_file)?;
    buf.flush()?;

    Ok(())
}

pub fn convert_folder_avif(folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    let entries = fs::read_dir(folder)?;

    entries
        .into_iter()
        .filter_map(Result::ok)
        .par_bridge()
        .for_each(|entry| {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|e| e.to_str()) != Some("svg") {
                let file_stem = path.file_stem().unwrap().to_str().unwrap().to_string();
                let source = format!("{folder}/{}", path.file_name().unwrap().to_string_lossy());
                let target = folder.split("/").nth(1).unwrap();
                let destination = format!("img/{target}/{file_stem}.avif");
                if fs::metadata(&destination).is_ok() {
                    return;
                }

                if let Err(e) = convert_to_avif(&source, &destination) {
                    eprintln!("Error while attempting to convert '{source}': {e}");
                }
            }
        });

    Ok(())
}
