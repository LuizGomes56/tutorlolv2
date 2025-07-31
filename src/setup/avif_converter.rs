#![cfg(feature = "dev")]
use image::{GenericImage, GenericImageView, ImageReader, RgbaImage};
use ravif::RGBA8;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

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

    let file = fs::File::create(&destination)?;
    let mut buf = std::io::BufWriter::new(file);
    buf.write_all(&res.avif_file)?;
    buf.flush()?;

    Ok(())
}

pub async fn convert_folder_avif(folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    let entries = fs::read_dir(folder)?;

    let mut handles = Vec::<tokio::task::JoinHandle<()>>::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|e| e.to_str()) != Some("svg") {
            let file_stem = path.file_stem().unwrap().to_str().unwrap().to_string();
            let source = format!("{}/{}", folder, path.file_name().unwrap().to_string_lossy());
            let destination = format!("{}/{}.avif", folder, file_stem);
            if fs::metadata(&destination).is_ok() {
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

#[derive(Deserialize, Serialize)]
struct Frame {
    name: String,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

#[derive(Deserialize, Serialize)]
struct SpriteSheetMetadata {
    frames: Vec<Frame>,
}

pub fn concat_sprite_jsons() {
    #[derive(Serialize)]
    struct SpritesheetSchema {
        f: u8,
        w: u32,
        h: u32,
        x: u32,
        y: u32,
    }

    let mut map = FxHashMap::default();
    let folders = ["abilities", "champions", "items"];

    for folder in folders {
        let entries = fs::read_dir(format!("sprite/{}", folder)).unwrap();
        let mut inner_map = FxHashMap::default();

        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            let file_stem = path.file_stem().unwrap().to_str().unwrap().to_string();
            if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("json") {
                let data = fs::read_to_string(&path).unwrap();
                let metadata: SpriteSheetMetadata = serde_json::from_str(&data).unwrap();
                for frame in metadata.frames {
                    inner_map.insert(
                        frame.name,
                        SpritesheetSchema {
                            f: file_stem.split('_').last().unwrap().parse().unwrap(),
                            w: frame.width,
                            h: frame.height,
                            x: frame.x,
                            y: frame.y,
                        },
                    );
                }
            }
        }
        map.insert(folder, inner_map);
    }

    let data = serde_json::to_string_pretty(&map).unwrap();
    fs::write("sprite/sprite.json", data).unwrap();
}

pub fn clean_sprite_folder() -> Result<(), String> {
    let folders = ["abilities", "champions", "items"];
    let safety_dir = Path::new("safety_copy/original_sprite");
    fs::create_dir_all(safety_dir)
        .map_err(|e| format!("Falha ao criar pasta {:?}: {}", safety_dir, e))?;

    for folder in folders {
        let sprite_folder = format!("sprite/{}", folder);
        let sprite_path = Path::new(&sprite_folder);
        if !sprite_path.exists() {
            println!("⚠️  Pasta não existe: {}", sprite_folder);
            continue;
        }
        let dest_folder = safety_dir.join(folder);
        fs::create_dir_all(&dest_folder)
            .map_err(|e| format!("Falha ao criar pasta {:?}: {}", dest_folder, e))?;
        let entries = fs::read_dir(&sprite_path)
            .map_err(|e| format!("Falha ao ler pasta {}: {}", sprite_folder, e))?;
        let mut moved_count = 0;
        for entry in entries {
            let entry = entry.map_err(|e| format!("Erro ao ler entrada: {}", e))?;
            let path = entry.path();
            if path.is_file() {
                let extension = path.extension().and_then(|e| e.to_str());
                if extension == Some("png") || extension == Some("json") {
                    let file_name = path
                        .file_name()
                        .ok_or("Nome de arquivo inválido")?
                        .to_string_lossy();

                    let dest_path = dest_folder.join(&*file_name);
                    fs::rename(&path, &dest_path).map_err(|e| {
                        format!("Falha ao mover {:?} para {:?}: {}", path, dest_path, e)
                    })?;

                    moved_count += 1;
                    println!("📦 Movido: {} -> {:?}", file_name, dest_path);
                }
            }
        }

        println!("✅ Pasta '{}': {} arquivos movidos", folder, moved_count);
        if let Ok(entries) = fs::read_dir(&sprite_path) {
            if entries.count() == 0 {
                if let Err(e) = fs::remove_dir(&sprite_path) {
                    println!(
                        "⚠️  Não foi possível remover pasta vazia {}: {}",
                        sprite_folder, e
                    );
                } else {
                    println!("🗑️  Pasta vazia removida: {}", sprite_folder);
                }
            }
        }
    }
    println!(
        "🎉 Limpeza concluída! Arquivos movidos para: {:?}",
        safety_dir
    );
    Ok(())
}

pub fn generate_spritesheet() -> Result<(), String> {
    let folders = ["abilities", "champions", "items"];
    let chunk_size = 64;
    let cols = 8;

    let mut corrupted_images: FxHashMap<String, Vec<String>> = FxHashMap::default();

    for folder in &folders {
        let src_dir = Path::new("safety_copy/original_img").join(folder);
        let dst_dir = Path::new("sprite").join(folder);
        fs::create_dir_all(&dst_dir)
            .map_err(|_| format!("Não foi possível criar {:?}", dst_dir))?;

        let mut files: Vec<_> = fs::read_dir(&src_dir)
            .map_err(|_| format!("Falha ao ler {:#?}", src_dir))?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("png"))
            .collect();
        files.sort();

        let mut folder_corrupted = Vec::new();

        for (chunk_idx, chunk) in files.chunks(chunk_size).enumerate() {
            let mut valid_images = Vec::new();
            let mut tile_w = 0u32;
            let mut tile_h = 0u32;
            let mut dimensions_set = false;

            for path in chunk {
                match image::open(path) {
                    Ok(img) => {
                        if !dimensions_set {
                            let (w, h) = img.dimensions();
                            tile_w = w;
                            tile_h = h;
                            dimensions_set = true;
                        }
                        valid_images.push((path, img));
                    }
                    Err(_) => {
                        let file_stem = path
                            .file_stem()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .into_owned();
                        folder_corrupted.push(file_stem);
                        println!("⚠️  Imagem corrompida ignorada: {:?}", path);
                    }
                }
            }

            if valid_images.is_empty() {
                println!(
                    "❌ Chunk {} da pasta '{}' não possui imagens válidas",
                    chunk_idx, folder
                );
                continue;
            }

            let rows = ((valid_images.len() as u32) + cols - 1) / cols;
            let sprite_w = cols * tile_w;
            let sprite_h = rows * tile_h;
            let mut canvas = RgbaImage::new(sprite_w, sprite_h);
            let mut metadata = Vec::with_capacity(valid_images.len());

            for (i, (path, img)) in valid_images.iter().enumerate() {
                let rgba_img = img.to_rgba8();
                let x = (i as u32 % cols) * tile_w;
                let y = (i as u32 / cols) * tile_h;

                match canvas.copy_from(&rgba_img, x, y) {
                    Ok(_) => {
                        metadata.push(Frame {
                            name: path
                                .file_stem()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .into_owned(),
                            x,
                            y,
                            width: tile_w,
                            height: tile_h,
                        });
                    }
                    Err(_) => {
                        let file_stem = path
                            .file_stem()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .into_owned();
                        folder_corrupted.push(file_stem);
                        println!("⚠️  Erro ao compor imagem: {:?}", path);
                    }
                }
            }

            if !metadata.is_empty() {
                let out_png = dst_dir.join(format!("sprite_{}.png", chunk_idx));
                canvas
                    .save(&out_png)
                    .map_err(|_| format!("Falha ao salvar {:?}", out_png))?;
                println!("Finished {:?} — {} images", out_png, metadata.len());

                let meta = SpriteSheetMetadata { frames: metadata };
                let out_json = dst_dir.join(format!("sprite_{}.json", chunk_idx));
                let file = File::create(&out_json)
                    .map_err(|_| format!("Não foi possível criar {:?}", out_json))?;
                serde_json::to_writer_pretty(file, &meta)
                    .map_err(|_| format!("Falha ao escrever {:?}", out_json))?;
                println!("✔ {:?} (metadata)", out_json);
            }
        }

        if !folder_corrupted.is_empty() {
            corrupted_images.insert(folder.to_string(), folder_corrupted);
            println!("📝 Pasta '{}' imagens corrompidas encontradas", folder,);
        }
    }

    if !corrupted_images.is_empty() {
        let corrupted_report_path = Path::new("sprite").join("corrupted_images.json");
        let corrupted_file = File::create(&corrupted_report_path)
            .map_err(|_| format!("Não foi possível criar {:?}", corrupted_report_path))?;
        serde_json::to_writer_pretty(corrupted_file, &corrupted_images)
            .map_err(|_| format!("Falha ao escrever relatório de imagens corrompidas"))?;
        println!(
            "\n📊 Relatório de imagens corrompidas salvo em: {:?}",
            corrupted_report_path
        );
        let total_corrupted: usize = corrupted_images.values().map(|v| v.len()).sum();
        println!("📈 Total de imagens corrompidas: {}", total_corrupted);
        for (folder, files) in &corrupted_images {
            println!("   - {}: {} imagens", folder, files.len());
        }
    } else {
        println!("\n🎉 Nenhuma imagem corrompida encontrada!");
    }

    Ok(())
}
