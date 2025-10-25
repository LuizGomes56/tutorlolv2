use super::*;

impl Generator<Champion> for Akshan {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1), (1, 0, _2), (3, 0, _3)];
        ability![E, (4, 0, _1)];
        ability![R, (4, 0, _1), (4, 1, _2), (4, 2, _3), (4, 3, _4)];
    }
}
