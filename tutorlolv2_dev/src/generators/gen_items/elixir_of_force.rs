use super::*;

impl Generator<ItemData> for ElixirOfForce {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
