//! `f![…]` of v2 generators, which builds a [`Formula`] as it reads, as
//! `f![13 + (Level - 1) + 0.3 * BonusAd]` or `f![5 * p]`, where `0.3 * BonusAd`
//! couldn't be written with operators alone: `Mul` can't be implemented for a
//! primitive times a `CtxVar`. Each atom (a number, a `CtxVar`, a `Formula`) becomes
//! a `Formula` with [`IntoFormula`], and the operators between them are kept, so
//! Rust's precedence applies
//!
//! [`Formula`]: tutorlolv2_wiki::wikitext::Formula
//! [`IntoFormula`]: tutorlolv2_wiki::wikitext::IntoFormula

/// Builds a `Formula`, see the [module](self)
#[macro_export]
macro_rules! formula_v2 {
    ($($tokens:tt)*) => {
        $crate::formula_v2_munch!([] $($tokens)*)
    };
}

/// Converts the tokens of [`formula_v2`] one by one into `out`
#[doc(hidden)]
#[macro_export]
macro_rules! formula_v2_munch {
    ([$($out:tt)*]) => {
        ($($out)*)
    };
    ([$($out:tt)*] + $($rest:tt)*) => {
        $crate::formula_v2_munch!([$($out)* +] $($rest)*)
    };
    ([$($out:tt)*] - $($rest:tt)*) => {
        $crate::formula_v2_munch!([$($out)* -] $($rest)*)
    };
    ([$($out:tt)*] * $($rest:tt)*) => {
        $crate::formula_v2_munch!([$($out)* *] $($rest)*)
    };
    ([$($out:tt)*] / $($rest:tt)*) => {
        $crate::formula_v2_munch!([$($out)* /] $($rest)*)
    };
    ([$($out:tt)*] ($($inner:tt)*) $($rest:tt)*) => {
        $crate::formula_v2_munch!([$($out)* ($crate::formula_v2!($($inner)*))] $($rest)*)
    };
    ([$($out:tt)*] $atom:tt $($rest:tt)*) => {
        $crate::formula_v2_munch!(
            [$($out)* ::tutorlolv2_wiki::wikitext::IntoFormula::to_formula(&$atom)]
            $($rest)*
        )
    };
}
