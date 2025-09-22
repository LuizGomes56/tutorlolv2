#![allow(ambiguous_glob_reexports)]
#[cfg(feature = "dev")]
pub use tutorlolv2_dev::*;
pub use tutorlolv2_generated::*;
pub use tutorlolv2_math::*;

#[cfg(test)]
mod tests {
    use tutorlolv2_exports::*;

    #[test]
    fn generate_html() {
        generate_champion_html();
        generate_item_html();
        generate_rune_html();
    }
}
