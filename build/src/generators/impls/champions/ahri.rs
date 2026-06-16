use super::*;

impl Generator for Ahri {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Min) /* Damage Per Pass */])
            .ability(
                Key::W,
                [
                    (0, Min), /* Primary Magic Damage */
                    (2, _1),  /* Subsequent Magic Damage */
                    (4, Max), /* Total Single-Target Damage */
                ],
            )
            .ability(Key::E, [(1, Void) /* Magic Damage */])
            .ability(Key::R, [(0, Min) /* Magic Damage */]);

        let qmax = self.merge_damage([Q(Min)], |[qmin]| f![(qmin * MagicMultiplier) + qmin])?;
        let rmax = self.merge_damage([R(Min)], |[rmin]| f![3 * (rmin)])?;

        self.clone_to(Q(Min), Q(Max), qmax)?
            .clone_to(R(Min), R(Max), rmax)?
            .damage_type(Q(Max), Mixed)?
            .combo([Attack, Ability(E(Void)), Ability(Q(Max)), Ability(W(Max))])?
            .combo([
                Ability(R(Min)),
                Ability(E(Void)),
                Attack,
                Ability(W(Max)),
                Ability(Q(Max)),
                Attack,
                Ability(R(Min)),
                Attack,
                Ability(R(Min)),
            ])?
            .end()
    }
}
