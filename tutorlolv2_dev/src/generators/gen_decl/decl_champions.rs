use crate::{
    MayFail,
    champions::{Champion, MerakiChampion},
    generators::{Generator, gen_factories::fac_champions::ChampionData},
};

tutorlolv2_macros::expand_dir!("../internal/champions", |Name| {
    pub struct Name(pub ChampionData);

    impl Name {
        pub fn name() -> &'static str {
            stringify!(Name)
        }

        pub fn new(data: MerakiChampion) -> Box<dyn Generator<Champion>> {
            Box::new(Self(ChampionData::new(data)))
        }

        pub fn end(self) -> MayFail<Champion> {
            println!("Ending generator for {}", Self::name());
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
