use image::ImageReader;
use ravif::RGBA8;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{
    fs,
    io::Write,
    path::{Component, Path, PathBuf},
};

fn convert_to_avif(
    source: &PathBuf,
    destination: &PathBuf,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    const QUALITY: f32 = 80.0;

    println!("Converting {source:?} to {destination:?}");
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

    let file = fs::File::create(destination)?;
    let mut buf = std::io::BufWriter::new(file);
    buf.write_all(&res.avif_file)?;
    buf.flush()?;

    Ok(())
}

fn to_img_path(path: &Path) -> PathBuf {
    let mut result = PathBuf::new();
    let mut replaced_first_folder = false;

    for component in path.components() {
        match component {
            Component::Prefix(prefix) => result.push(prefix.as_os_str()),
            Component::RootDir => result.push(component.as_os_str()),
            Component::CurDir => result.push(component.as_os_str()),
            Component::ParentDir => result.push(component.as_os_str()),
            Component::Normal(part) => match !replaced_first_folder {
                true => {
                    match part == "img" {
                        true => {
                            result.push(part);
                        }
                        false => {
                            result.push("img");
                        }
                    }
                    replaced_first_folder = true;
                }
                false => {
                    result.push(part);
                }
            },
        }
    }

    result.set_extension("avif");
    result
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
                let destination = to_img_path(&path);

                if fs::metadata(&destination).is_ok() {
                    return;
                }

                if let Err(e) = convert_to_avif(&path, &destination) {
                    eprintln!("Error while attempting to convert '{path:?}': {e}");
                }
            }
        });

    Ok(())
}
