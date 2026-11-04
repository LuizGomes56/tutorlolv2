use super::*;

impl Generator for Ahri {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [("Damage Per Pass", Min)])
            .ability(
                Key::W,
                [
                    ("Primary Magic Damage", Min),
                    ("Subsequent Magic Damage", _1),
                    ("Total Single-Target Damage", Max),
                ],
            )
            .ability(Key::E, [("Magic Damage", Void)])
            .ability(Key::R, [("Magic Damage", Min)]);

        let qmax = self.merge_damage([Q(Min)], |[qmin]| f![(qmin * MagicMultiplier) + qmin])?;
        let rmax = self.merge_damage([R(Min)], |[rmin]| f![3 * (rmin)])?;

        self.clone_to(Q(Min), Q(Max), qmax)?
            .clone_to(R(Min), R(Max), rmax)?
            .damage_type(Q(Max), Mixed)?
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
