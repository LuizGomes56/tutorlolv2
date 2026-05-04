use super::*;

impl Generator for KogMaw {
    fn generate(&mut self) -> MayFail {
        self.attr(Area, [P(Void), E(Void), R(Min), R(Max)])?
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
            .progress(Stable)
            .end()
    }
}
