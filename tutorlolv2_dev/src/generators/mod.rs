pub mod gen_champions;
pub mod gen_decl;
pub mod gen_factories;
pub mod gen_items;
pub mod gen_runes;
pub mod gen_utils;

/// Base generator trait, that returns a type that will be serialized into a
/// JSON file to be read by the `tutorlolv2_build` script and generate Rust code,
/// to be compiled to avoid execution overhead for parsing strings and doing
/// calculations that were known at compile-time.
///
/// Since every item and champion receives its own struct, they're placed
/// in a `Box<Self>` because their execution is done with dynamic dispatch, so all of
/// them can be executed with a simple `for` loop, instead of hardcoding them all or
/// expanding macros to achieve a static dispatch.
///
/// Also, with each champion, item, and rune having its own file, with its name, it is
/// easier to fix their generators in case some breaking change occur.
pub trait Generator<T> {
    fn generate(self: Box<Self>) -> crate::MayFail<T>;
}
