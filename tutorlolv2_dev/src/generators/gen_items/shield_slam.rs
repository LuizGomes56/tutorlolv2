use super::*;

impl Generator<ItemData> for ShieldSlam {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
