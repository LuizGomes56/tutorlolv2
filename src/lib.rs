#![allow(ambiguous_glob_reexports)]
#[cfg(feature = "dev")]
pub use tutorlolv2_dev::*;
pub use tutorlolv2_generated::*;
pub use tutorlolv2_math::*;

#[cfg(test)]
mod tests {
    use tutorlolv2_exports::generate_champion_html;

    #[test]
    fn generate_html() {
        generate_champion_html();
    }
}
