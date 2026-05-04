use super::*;

impl Generator for Anivia {
    fn generate(&mut self) -> MayFail {
        self.attr(Area, [Q(_1Min), Q(_1Max), Q(Max), R(Min), R(Max)])?
            .combo([
                Ability(Q(_1Min)),
                Attack,
                Ability(Q(_1Max)),
                Ability(E(Max)),
            ])?
            .progress(Preserve)
            .end()
    }
}
