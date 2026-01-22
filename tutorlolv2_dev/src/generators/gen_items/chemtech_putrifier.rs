use super::*;

impl Generator<ItemData> for ChemtechPutrifier {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
