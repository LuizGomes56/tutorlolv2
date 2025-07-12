#![cfg(feature = "dev-routes")]
use image::ImageReader;
use ravif::RGBA8;
use std::io::Write;

fn convert_to_avif(
    source: &str,
    destination: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Converting {} to {}", source, destination);
    let img = ImageReader::open(source)?
        .with_guessed_format()?
        .decode()?
        .to_rgba8();
    let (width, height) = (img.width(), img.height());
    let pixels = img.into_raw();

    let pixels_rgba: Vec<RGBA8> = pixels
        .chunks_exact(4)
        .map(|chunk| RGBA8::new(chunk[0], chunk[1], chunk[2], chunk[3]))
        .collect();

    let res = ravif::Encoder::new()
        .with_quality(80f32)
        .with_speed(1)
        .encode_rgba(ravif::Img::new(
            &pixels_rgba,
            width as usize,
            height as usize,
        ))?;

    let file = std::fs::File::create(&destination)?;
    let mut buf = std::io::BufWriter::new(file);
    buf.write_all(&res.avif_file)?;
    buf.flush()?;

    Ok(())
}

pub async fn convert_folder_avif(folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    let entries = std::fs::read_dir(format!("img/{}", folder))?;

    let mut handles = Vec::<tokio::task::JoinHandle<()>>::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|e| e.to_str()) != Some("svg") {
            let file_stem = path.file_stem().unwrap().to_str().unwrap().to_string();
            let source = format!(
                "img/{}/{}",
                folder,
                path.file_name().unwrap().to_string_lossy()
            );
            let destination = format!("img/{}/{}.avif", folder, file_stem);
            if std::fs::metadata(&destination).is_ok() {
                continue;
            }

            let handle =
                tokio::task::spawn_blocking(move || match convert_to_avif(&source, &destination) {
                    Ok(_) => (),
                    Err(e) => eprintln!("Erro ao converter '{}': {}", source, e),
                });

            handles.push(handle);

            if handles.len() >= 8 {
                for h in handles.drain(..) {
                    let _ = h.await;
                }
            }
        }
    }

    for h in handles {
        let _ = h.await;
    }

    Ok(())
}
