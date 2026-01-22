use super::*;

impl Generator<ItemData> for ElixirOfSkill {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
