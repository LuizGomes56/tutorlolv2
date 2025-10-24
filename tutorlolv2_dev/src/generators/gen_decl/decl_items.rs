use crate::{
    generators::Generator,
    items::{MerakiItem, Item},
};

tutorlolv2_macros::expand_dir!("../internal/items", |Name| {
    pub struct Name(pub MerakiItem);

    impl Name {
        pub fn new(data: MerakiItem) -> Box<dyn Generator<Item>> {
            Box::new(Self(data))
        }
    }

    impl ::core::ops::Deref for Name {
        type Target = MerakiItem;
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
