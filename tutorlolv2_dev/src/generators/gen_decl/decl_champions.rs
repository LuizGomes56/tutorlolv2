use crate::{
    MayFail,
    champions::{Champion, MerakiChampion},
    generators::{Generator, gen_factories::fac_champions::ChampionData},
};

tutorlolv2_macros::expand_dir!("../internal/champions", |Name| {
    pub struct Name(pub ChampionData);

    impl Name {
        pub fn new(data: MerakiChampion) -> Box<dyn Generator<Champion>> {
            Box::new(Self(ChampionData::new(data)))
        }

        pub fn end(self) -> MayFail<Champion> {
            self.0.end()
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
