use super::*;

impl Generator<ItemData> for GangplankPlaceholder {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
