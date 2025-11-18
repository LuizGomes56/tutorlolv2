use super::*;

impl Generator<ItemData> for SlightlyMagicalBoots {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
