use super::*;

impl Generator<Champion> for Renekton {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1), (0, 5, _2)];
        ability![W, (0, 0, _1), (0, 1, _2), (1, 0, _3)];
        ability![
            E,
            (0, 0, _1),
            (0, 0, _2),
            (2, 1, _3),
            (2, 2, _4),
            (2, 3, _5)
        ];
        ability![R, (1, 0, _1), (1, 1, _2)];
    }
}
