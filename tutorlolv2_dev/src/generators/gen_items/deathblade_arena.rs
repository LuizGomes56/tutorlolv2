use super::*;

impl Generator<ItemData> for DeathbladeArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
