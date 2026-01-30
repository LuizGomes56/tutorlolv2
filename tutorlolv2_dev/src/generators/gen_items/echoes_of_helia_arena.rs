use super::*;

impl Generator<ItemData> for EchoesOfHeliaArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
