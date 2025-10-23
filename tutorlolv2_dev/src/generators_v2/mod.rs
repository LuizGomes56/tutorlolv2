pub mod gen_champions;
pub mod gen_decl;
pub mod gen_factories;
pub mod gen_items;
pub mod gen_runes;
pub mod gen_utils;

pub trait Generator<T> {
    fn generate(self: Box<Self>) -> MayFail<T>;
}

pub type MayFail<T = ()> = Result<T, Box<dyn std::error::Error>>;
