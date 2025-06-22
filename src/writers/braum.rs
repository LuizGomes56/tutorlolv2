use super::*;

// #![stable] "06/11/2025" | "25.11"

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    passive!("P", (0, 2), Target::MINIMUM, (Some(2), None));
    passive!("P_MAX", (0, 1), Target::MAXIMUM);
    ability!(q, (0, 0, "Q", Target::MINIMUM));
    ability!(r, (0, 0, "R", Target::MINIMUM));
    merge_ability!("P");
}
