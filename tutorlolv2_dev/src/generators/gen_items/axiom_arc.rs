use super::*;

impl Generator<ItemData> for AxiomArc {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
