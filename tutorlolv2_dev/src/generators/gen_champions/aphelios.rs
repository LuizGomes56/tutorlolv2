use super::*;

impl Generator<Champion> for Aphelios {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![P, (2, 0, _1)];
    }
}
