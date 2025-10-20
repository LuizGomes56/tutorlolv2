use super::*;

impl Generator for Renekton {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1), (0, 5, _2)];
        ability![W, (0, 0, _1), (0, 1, _2), (1, 0, _3)];
        ability![E, (0, 0, _1), (3, 0, _2), (4, 1, _3), (4, 2, _4)];
        ability![R, (1, 0, _1), (1, 1, _2)];
    }
}
