use super::*;

impl Generator<Champion> for Morgana {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1)];
        ability![W, (0, 0, _1), (0, 1, _2), (0, 2, _3), (0, 3, _4)];
        ability![R, (0, 1, _1), (0, 2, _2)];
    }
}
