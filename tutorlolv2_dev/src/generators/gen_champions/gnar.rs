use super::*;

impl Generator for Gnar {
    fn generate(&mut self) -> MayFail {
        self.attr(
            Area,
            [Q(Min), Q(Max), Q(Mega), W(Mega), E(Mega), R(Min), R(Max)],
        )?
        .combo([Ability(Q(Min)), Attack, Attack, Ability(W(Void))])?
        .combo([
            Ability(W(Mega)),
            Attack,
            Ability(Q(Mega)),
            Attack,
            Ability(R(Min)),
            Attack,
            Ability(Q(Mega)),
            Attack,
        ])?
        .progress(Stable)
        .end()
    }
}
