use super::*;

impl Generator<Champion> for Tryndamere {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (1, 1, _1)];
        ability![W, (1, 0, _1)];
        ability![E, (0, 0, _1)];
    }
}
