use super::*;

// #![stable]

impl Generator<Champion> for Akali {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![P::Void, (0, 1)];
        ability![Q, (0, 0, Void)];
        ability![E, (0, 0, _1Min), (2, 0, _1Max), (2, 1, Max)];
        ability![R, (0, 0, _1), (2, 0, _2Min), (2, 1, _2Max)];

        attr![
            Area => [Q::Void, R::_1, R::_2Min, R::_2Max],
            Onhit => [P::Void]
        ];

        merge![E::_1Min - E::_1Max, R::_2Min - R::_2Max];
    }
}
