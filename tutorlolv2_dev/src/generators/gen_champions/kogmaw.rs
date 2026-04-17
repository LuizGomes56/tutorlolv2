use super::*;

impl Generator<Champion> for KogMaw {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.passive(Void, (0, 0), None, None)
            .ability(Key::Q, [(0, 0, Void)])
            .ability(Key::W, [(0, 1, Void)])
            .ability(Key::E, [(1, 0, Void)])
            .ability(Key::R, [(0, 0, Max), (0, 1, Min)])
            .attr(Area, [P(Void), E(Void), R(Min), R(Max)])?
            .combo([
                Ability(E(Void)),
                Attack,
                Ability(W(Void)),
                Ability(Q(Void)),
                Attack,
                Ability(W(Void)),
                Ability(R(Min)),
                Attack,
                Ability(W(Void)),
            ])?
            .progress(Stable);

        self.end()
    }
}
