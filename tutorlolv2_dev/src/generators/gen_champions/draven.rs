use super::*;

impl Generator<Champion> for Draven {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1)];
        ability![E, (0, 0, _1)];
        ability![R, (0, 0, _1), (0, 1, _2), (4, 0, _3), (4, 1, _4)];
    }
}
