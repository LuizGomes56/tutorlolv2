use crate::{
    champions::{CdnChampion, Champion},
    generators::{Generator, gen_factories::fac_champions::ChampionData},
};

tutorlolv2_macros::expand_dir!("../internal/champions", |Name| {
    pub struct Name(pub ChampionData);

    impl Name {
        pub fn new(data: CdnChampion) -> Box<dyn Generator<Champion>> {
            Box::new(Self(ChampionData::new(data)))
        }
    }

    impl ::core::ops::Deref for Name {
        type Target = ChampionData;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl ::core::ops::DerefMut for Name {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
});
