use super::*;

impl Generator<ItemData> for EchoesOfHelia {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
