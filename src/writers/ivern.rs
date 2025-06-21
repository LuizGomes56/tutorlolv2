use super::*;

// #![auto_generated]
// ! #![unstable] [X] "06/11/2025" | "25.11"
// #![preserve]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q", Target::MINIMUM));
    ability!(
        w,
        (0, 0, "W_0_0_0_BONUS", Target::MINIMUM),
        (2, 0, "W_0_2_0_BONUS", Target::MINIMUM)
    );
    ability!(e, (1, 0, "E", Target::MINIMUM));
}

#[writer_macros::test]
pub fn test() {}
