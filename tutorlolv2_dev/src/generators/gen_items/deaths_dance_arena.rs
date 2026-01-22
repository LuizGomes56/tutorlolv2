use super::*;

impl Generator<ItemData> for DeathsDanceArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
