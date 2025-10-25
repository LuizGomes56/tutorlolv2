use super::*;

impl Generator<Champion> for Fiora {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (2, 0, _1)];
        ability![W, (1, 0, _1)];
        ability![E, (2, 0, _1)];
    }
}
