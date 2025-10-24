use super::*;

impl Generator<Champion> for Neeko {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1), (1, 0, _2), (2, 0, _3), (2, 1, _4)];
        ability![W, (1, 0, _1)];
        ability![E, (0, 0, _1)];
        ability![R, (2, 0, _1)];
    }
}
