use super::*;

impl Generator for Ahri {
    fn generate(&mut self) -> MayFail {
        self.take("Damage Per Pass", Q(Min))?
            .take("Primary Magic Damage", W(Min))?
            .take("Subsequent Magic Damage", W(_1))?
            .take("Total Single-Target Damage", W(Max))?
            .take("Magic Damage", E(Void))?
            .take("Magic Damage", R(Min))?
            .derive(Q(Max), Q(Min), |qmin| f![(qmin * MagicMultiplier) + qmin])?
            .derive(R(Max), R(Min), |rmin| f![3 * (rmin)])?
            .damage_type(Q(Max), Mixed)
            .combos([
                &[Attack, Ability(E(Void)), Ability(Q(Max)), Ability(W(Max))],
                &[
                    Ability(R(Min)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Max)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(R(Min)),
                    Attack,
                    Ability(R(Min)),
                ],
            ])?
            .end()
    }
}
