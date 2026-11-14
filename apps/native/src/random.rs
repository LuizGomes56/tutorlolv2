use std::hash::{BuildHasher, RandomState};
use strum::VariantArray;

/// A random variant, like the web app's `EnumCast::random`.
/// `RandomState` is seeded randomly per process and per call, so no RNG crate is needed.
pub fn random_variant<T: VariantArray + Copy>() -> T {
    let seed = RandomState::new().hash_one(());

    T::VARIANTS[seed as usize % T::VARIANTS.len()]
}
