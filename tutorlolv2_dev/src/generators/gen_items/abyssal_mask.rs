use super::*;

impl Generator<Item> for AbyssalMask {
    #[item_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Item> {}
}
