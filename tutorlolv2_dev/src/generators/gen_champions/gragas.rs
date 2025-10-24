use super::*;

impl Generator<Champion> for Gragas {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (1, 0, _1), (1, 1, _2), (1, 3, _3), (1, 4, _4)];
        ability![W, (0, 0, _1), (1, 0, _2), (1, 1, _3)];
        ability![E, (0, 0, _1)];
        ability![R, (0, 0, _1)];
    }
}
