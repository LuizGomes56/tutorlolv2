use crate::{
    CdnChampion, Champion, generators_v2::MayFail, setup::generators::champion_v2::GeneratorData,
};

pub trait Generator {
    fn generate(self: Box<Self>) -> MayFail<Champion>;
}

tutorlolv2_macros::generate_structs!("../internal/champions");
