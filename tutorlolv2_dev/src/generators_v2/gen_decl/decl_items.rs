use crate::{
    generators_v2::Generator,
    items::{CdnItem, Item},
};

tutorlolv2_macros::expand_dir!("../internal/items", |Name| {
    pub struct Name(pub CdnItem);

    impl Name {
        pub fn new(data: CdnItem) -> Box<dyn Generator<Item>> {
            Box::new(Self(data))
        }
    }

    impl ::core::ops::Deref for Name {
        type Target = CdnItem;
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
