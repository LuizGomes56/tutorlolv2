use super::*;

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q_0_0_0", Target::MINIMUM));
    ability!(w, (0, 0, "W_0_0_0", Target::MINIMUM));
    ability!(e, (0, 0, "E_0_0_0", Target::MINIMUM));
    ability!(r, (3, 0, "R_0_3_0", Target::MINIMUM));
}

#[writer_macros::test]
pub fn test() {}
