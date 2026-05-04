use super::*;

impl Generator for Neeko {
    fn generate(&mut self) -> MayFail {
        self.attr(Area, [Q(_1), Q(Min), Q(Max), E(Void), R(Void)])?
            .combo([
                Ability(E(Void)),
                Ability(Q(Max)),
                Attack,
                Ability(W(Void)),
                Ability(R(Void)),
            ])?
            .combo([Ability(Q(_1)), Attack, Ability(W(Void))])?
            .progress(Stable)
            .end()
    }
}
