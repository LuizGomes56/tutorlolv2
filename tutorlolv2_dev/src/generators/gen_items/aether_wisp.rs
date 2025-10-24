use super::*;

impl Generator<Item> for AetherWisp {
    #[item_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Item> {}
}
