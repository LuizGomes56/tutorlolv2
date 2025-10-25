use crate::generators::{Generator, gen_factories::fac_items::ItemData};

tutorlolv2_macros::expand_dir!("../internal/items", |Name| {
    pub struct Name(pub ItemData);

    impl Name {
        pub fn new(data: ItemData) -> Box<dyn Generator<ItemData>> {
            Box::new(Self(data))
        }
    }

    impl ::core::ops::Deref for Name {
        type Target = ItemData;
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
