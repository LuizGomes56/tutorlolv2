use super::*;

impl Generator<ItemData> for SlightlyMagicalBoots {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
