pub(super) mod aatrox;
pub(super) mod ahri;
pub(self) use super::base::{
    IterationTarget, extract_as_linear_range, extract_percentage_range, get_from_pattern,
    read_from_file, replace_keys,
};
pub(super) use crate::model::champions::{Ability, CdnAbility, CdnChampion, Champion};
