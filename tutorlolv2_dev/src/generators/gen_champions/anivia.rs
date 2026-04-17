use super::*;

impl Generator<Champion> for Anivia {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(
            Key::Q,
            [
                (0, 0, _1Min), // Cast #1
                (2, 0, _1Max), // Cast #2
                (2, 2, Max),   // Total
            ],
        )
        .ability(Key::E, [(0, 0, Max), (0, 1, Min)])
        .ability(Key::R, [(0, 0, Min), (4, 0, Max)])
        .attr(Area, [Q(_1Min), Q(_1Max), Q(Max), R(Min), R(Max)])?
        .combo([
            Ability(Q(_1Min)),
            Attack,
            Ability(Q(_1Max)),
            Ability(E(Max)),
        ])?
        .progress(Stable);

        self.end()
    }
}
