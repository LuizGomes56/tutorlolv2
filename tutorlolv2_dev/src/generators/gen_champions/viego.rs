use super::*;

impl Generator<Champion> for Viego {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1), (0, 1, _2), (3, 0, _3), (3, 1, _4)];
        ability![W, (1, 0, _1)];
        ability![R, (0, 0, _1)];
    }
}
