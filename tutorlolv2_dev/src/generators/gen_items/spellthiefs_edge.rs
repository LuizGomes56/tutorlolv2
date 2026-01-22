use super::*;

impl Generator<ItemData> for SpellthiefsEdge {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
