use super::*;

impl Generator<ItemData> for BunnyHop {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
