use super::*;

impl Generator<Champion> for Ziggs {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (1, 0, _1)];
        ability![W, (1, 0, _1)];
        ability![E, (1, 0, _1), (1, 1, _2), (1, 2, _3)];
        ability![R, (1, 0, _1), (1, 1, _2)];
    }
}
