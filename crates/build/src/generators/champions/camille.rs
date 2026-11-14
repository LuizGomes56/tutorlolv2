use super::*;

impl Generator for Camille {
    fn generate(&mut self) -> MayFail {
        let outer_cone = self.value("Outer Cone Bonus Damage", Key::W)?;

        self.take("Bonus Physical Damage", Q(Min))?
            .take("Increased Mixed Damage", Q(Max))?
            .damage_type(Q(Min), Physical)
            .damage_type(Q(Max), Mixed)
            .take("Physical Damage", W(Min))?
            .derive(W(Max), W(Min), |w| f![w + outer_cone])?
            .comment(W(Max), "Outer cone damage")
            .take("Physical Damage", E(Void))?
            .take("Bonus Magic Damage", R(Void))?
            .combos([
                &[
                    Ability(E(Void)),
                    Ability(W(Max)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                ],
                &[
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Min)),
                ],
                &[
                    Attack,
                    Ability(Q(Min)),
                    Ability(E(Void)),
                    Ability(W(Max)),
                    Ability(E(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                    Ability(Q(Min)),
                ],
                &[
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                ],
                &[
                    Attack,
                    Ability(Q(Min)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(W(Max)),
                ],
                &[
                    Ability(Q(Min)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(W(Max)),
                ],
                &[Ability(E(Void)), Ability(E(Void))],
            ])?
            .end()
    }
}
