use crate::MayFail;
use serde::{Deserialize, Serialize};

pub mod gen_champions;
pub mod gen_factories;
pub mod gen_items;
pub mod gen_utils;

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub enum Progress {
    Preserve,
    Stable,
    #[default]
    Unstable,
}

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
pub trait Generator {
    fn generate(&mut self) -> MayFail;
}

pub trait GeneratorExt<T>
where
    Self: Generator,
{
    fn end(self: Box<Self>) -> MayFail<T>;
    fn call(mut self: Box<Self>) -> MayFail<T> {
        self.generate()?;
        self.end()
    }
}
