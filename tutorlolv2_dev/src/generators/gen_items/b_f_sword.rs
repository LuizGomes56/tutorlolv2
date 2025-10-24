use super::*;

impl Generator<Item> for BFSword {
    #[item_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Item> {}
}
