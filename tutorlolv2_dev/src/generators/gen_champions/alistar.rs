use super::*;

impl Generator for Alistar {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, Void)])
            .ability(Key::W, [(0, 0, Void)])
            .ability(Key::E, [(0, 0, Min), (0, 1, Max)])
            .combo([Ability(W(Void)), Ability(Q(Void)), Attack, Ability(E(Max))])?
            .progress(Stable)
            .end()
    }
}
