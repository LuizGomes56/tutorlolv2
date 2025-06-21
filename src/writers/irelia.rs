use super::*;

// #![auto_generated]
// ! #![unstable] [X] "06/11/2025" | "25.11"
// #![preserve]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q", Target::MINIMUM));
    ability!(
        w,
        (2, 0, "W", Target::MINIMUM),
        (2, 1, "W_MAX", Target::MAXIMUM)
    );
    ability!(e, (2, 0, "E", Target::MINIMUM));
    ability!(r, (0, 0, "R", Target::MINIMUM));
    merge_ability!("W");
}

#[writer_macros::test]
pub fn test() {}
