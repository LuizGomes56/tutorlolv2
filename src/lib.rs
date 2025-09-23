#![allow(ambiguous_glob_reexports)]
#[cfg(feature = "dev")]
pub use tutorlolv2_dev::*;
pub use tutorlolv2_gen::*;
pub use tutorlolv2_math::*;

#[cfg(test)]
mod tests {
    use std::thread::spawn;
    use tutorlolv2_exports::*;

    #[test]
    fn generate_html() {
        let champions = spawn(generate_champion_html);
        let items = spawn(generate_item_html);
        let runes = spawn(generate_rune_html);

        champions.join().unwrap();
        items.join().unwrap();
        runes.join().unwrap();
    }
}
