use super::*;

impl Generator<Champion> for DrMundo {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1), (2, 0, _2), (2, 1, _3)];
        ability![W, (0, 0, _1), (0, 1, _2), (2, 0, _3)];
        ability![E, (0, 0, _1), (0, 1, _2), (3, 0, _3)];
    }
}
