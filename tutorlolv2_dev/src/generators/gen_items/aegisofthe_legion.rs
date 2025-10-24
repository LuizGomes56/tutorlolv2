use super::*;

impl Generator<Item> for AegisoftheLegion {
    #[item_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Item> {}
}
