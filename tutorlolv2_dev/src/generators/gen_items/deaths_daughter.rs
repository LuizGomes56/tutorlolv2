use super::*;

impl Generator<ItemData> for DeathsDaughter {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
