//! `f![…]` shared by all generators, which builds a [`Formula`] as it reads, as
//! `f![13 + (Level - 1) + 0.3 * BonusAd]` or `f![5 * p]`, where `0.3 * BonusAd`
//! couldn't be written with operators alone: `Mul` can't be implemented for a
//! primitive times a `CtxVar`. Each atom (a number, a `CtxVar`, a `Formula`) becomes
//! a `Formula` with [`IntoFormula`], and the operators between them are kept, so
//! Rust's precedence applies
//!
//! [`Formula`]: wiki::wikitext::Formula
//! [`IntoFormula`]: wiki::wikitext::IntoFormula

/// Builds a `Formula`, see the [module](self)
#[macro_export]
macro_rules! formula {
    ($($tokens:tt)*) => {
        $crate::formula_munch!([] $($tokens)*)
    };
}

/// Converts the tokens of [`formula`] one by one into `out`
#[doc(hidden)]
#[macro_export]
macro_rules! formula_munch {
    ([$($out:tt)*]) => {
        ($($out)*)
    };
    ([$($out:tt)*] + $($rest:tt)*) => {
        $crate::formula_munch!([$($out)* +] $($rest)*)
    };
    ([$($out:tt)*] - $($rest:tt)*) => {
        $crate::formula_munch!([$($out)* -] $($rest)*)
    };
    ([$($out:tt)*] * $($rest:tt)*) => {
        $crate::formula_munch!([$($out)* *] $($rest)*)
    };
    ([$($out:tt)*] / $($rest:tt)*) => {
        $crate::formula_munch!([$($out)* /] $($rest)*)
    };
    ([$($out:tt)*] ($($inner:tt)*) $($rest:tt)*) => {
        $crate::formula_munch!([$($out)* ($crate::formula!($($inner)*))] $($rest)*)
    };
    ([$($out:tt)*] $atom:tt $($rest:tt)*) => {
        $crate::formula_munch!(
            [$($out)* ::wiki::wikitext::IntoFormula::to_formula(&$atom)]
            $($rest)*
        )
    };
}
