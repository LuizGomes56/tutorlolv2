pub fn convert_folder(source: &str, folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    match tutorlolv2_avif::convert_folder_avif(&format!("../{source}/{folder}")) {
        Ok(_) => println!("Convertion of '{folder}' finished"),
        Err(e) => eprintln!("Error converting '{folder}': {e:#?}"),
    }
    Ok(())
}

pub const IMG_FOLDERS: [&str; 8] = [
    "abilities",
    "centered",
    "champions",
    "items",
    "other",
    "runes",
    "splash",
    "stats",
];

pub fn img_convert_avif<const N: usize>(folders: [&'static str; N]) {
    for folder in folders {
        println!("Converting folder: {folder}");
        let _ = convert_folder("raw_img", folder);
    }
}

fn main() {
    img_convert_avif(IMG_FOLDERS);
}
