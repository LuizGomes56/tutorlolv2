use super::*;

impl Generator<ItemData> for AxiomArc {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
