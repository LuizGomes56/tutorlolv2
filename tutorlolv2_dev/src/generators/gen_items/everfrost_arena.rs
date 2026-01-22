use super::*;

impl Generator<ItemData> for EverfrostArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
