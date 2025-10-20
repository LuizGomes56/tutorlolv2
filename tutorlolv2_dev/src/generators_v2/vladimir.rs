use super::*;

impl Generator for Vladimir {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 1, _1), (2, 0, _2)];
        ability![W, (1, 0, _1), (1, 1, _2)];
        ability![E, (4, 0, _1), (4, 1, _2)];
        ability![R, (1, 1, _1)];
    }
}
