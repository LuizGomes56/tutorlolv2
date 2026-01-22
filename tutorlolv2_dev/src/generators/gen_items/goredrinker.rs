use super::*;

impl Generator<ItemData> for Goredrinker {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
