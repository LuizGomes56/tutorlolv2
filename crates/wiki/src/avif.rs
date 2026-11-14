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
    const SPEED: u8 = 1;

    println!("Converting {source:?} to {destination:?}");
    let img = ImageReader::open(source)?
        .with_guessed_format()?
        .decode()?
        .to_rgba8();

    let width = img.width();
    let height = img.height();
    let pixels = img.into_raw();

    let pixels_rgba = pixels
        .chunks_exact(4)
        .map(|chunk| RGBA8::new(chunk[0], chunk[1], chunk[2], chunk[3]))
        .collect::<Vec<RGBA8>>();

    let res = ravif::Encoder::new()
        .with_quality(QUALITY)
        .with_speed(SPEED)
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
    let mut components = path.components().peekable();

    while let Some(component) = components.next() {
        result.push(component.as_os_str());
        // Raw sources live one level below the existing image output folders.
        if component == Component::Normal("img".as_ref())
            && components.peek() == Some(&Component::Normal("raw".as_ref()))
        {
            components.next();
        }
    }

    result.set_extension("avif");
    result
}

pub fn convert_folder_avif(folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::read_dir(folder)?
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

pub fn convert_folder(folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    match convert_folder_avif(&format!("img/raw/{folder}")) {
        Ok(_) => println!("Convertion of {folder:?} finished"),
        Err(e) => eprintln!("Error converting {folder:?}: {e:#?}"),
    }

    Ok(())
}

pub fn img_convert_avif() {
    for folder in [
        "abilities",
        "centered",
        "champions",
        "items",
        "other",
        "runes",
        "splash",
        "stats",
    ] {
        println!("Converting folder: {folder:?}");
        let _ = convert_folder(folder);
    }
}

#[cfg(test)]
mod tests {
    use super::to_img_path;
    use std::path::Path;

    #[test]
    fn raw_images_convert_into_existing_image_directories() {
        assert_eq!(
            to_img_path(Path::new("img/raw/champions/Ahri.png")),
            Path::new("img/champions/Ahri.avif")
        );
        assert_eq!(
            to_img_path(Path::new("img/raw/splash/Ahri_0.jpg")),
            Path::new("img/splash/Ahri_0.avif")
        );
        assert_eq!(
            to_img_path(Path::new("/project/img/raw/items/1001.png")),
            Path::new("/project/img/items/1001.avif")
        );
        assert_eq!(
            to_img_path(Path::new("img/items/1001.png")),
            Path::new("img/items/1001.avif")
        );
    }
}
