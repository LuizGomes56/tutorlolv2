use super::*;

impl Generator for Leblanc {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(Min))?
            .map_change(Q(Min), "Base damage per hit")?
            .take("Total Magic Damage", Q(Max))?
            .take("Magic Damage", W(Void))?
            .take("Fracture Magic Damage", E(_1))?
            .take("Magic Damage", E(Min))?
            .take("Total Damage", E(Max))?
            .take("Application Magic Damage", R(_1))?
            .take("Mark Magic Damage", R(_2))?
            .take("Magic Damage", R(_3))?
            .take("Orb Magic Damage", R(_4))?
            .take("Fracture Magic Damage", R(_5))?
            .take_nth("Total Magic Damage", 1, R(_6))?
            // The mimics of Sigil of Malice and Ethereal Chains, written in notes
            .map_change(R(_1), "Base damage")?
            .map_change(R(_1), "AP ratio")?
            .map_change(R(_4), "Base damage")?
            .map_change(R(_4), "AP ratio")?;

        // The mark and the total of Sigil of Malice deal its damage twice
        let (mode, per_hit) = self.map_value(Key::Q, "Base damage per hit")?;

        self.map_base(Q(Max), mode, per_hit.times(2.0))?;

        let (mode, mimic) = self.map_value(Key::R, "Base damage")?;
        let (_, mimic_ap) = self.map_value(Key::R, "AP ratio")?;
        let orb = self.damage(R(_4))?;
        let fracture = self.damage(R(_5))?;

        self.map_base(R(_2), mode, mimic.times(2.0))?
            .map_ratio(R(_2), mode, &[AbilityPower], mimic_ap.times(2.0))?
            .set(R(_6), f![orb + fracture])
            .combos([
                &[
                    Ability(R(_1)),
                    Ability(Q(Max)),
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(R(_1)),
                    Ability(E(Max)),
                    Ability(Q(Max)),
                    Ability(W(Void)),
                    Ability(R(_1)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(R(_1)),
                    Ability(W(Void)),
                    Ability(R(_1)),
                    Ability(Q(Max)),
                    Ability(E(Max)),
                ],
                &[
                    Ability(E(Max)),
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Ability(W(Void)),
                    Attack,
                    Ability(R(_1)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(R(_1)),
                    Ability(Q(Max)),
                    Ability(E(Max)),
                    Ability(W(Void)),
                ],
                &[
                    Ability(E(Max)),
                    Ability(Q(Max)),
                    Ability(R(_1)),
                    Ability(W(Void)),
                    Ability(W(Void)),
                ],
                &[
                    Ability(W(Void)),
                    Ability(R(_1)),
                    Ability(E(Max)),
                    Ability(W(Void)),
                    Ability(R(_1)),
                    Ability(Q(Max)),
                ],
                &[
                    Ability(R(_1)),
                    Ability(E(Max)),
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Max)),
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Attack,
                    Attack,
                    Ability(W(Void)),
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Attack,
                    Attack,
                    Ability(R(_1)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(W(Void)),
                ],
                &[
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Ability(R(_1)),
                    Ability(E(Max)),
                    Ability(W(Void)),
                ],
                &[Ability(W(Void)), Attack, Ability(Q(Max)), Ability(W(Void))],
                &[Ability(Q(Max)), Ability(W(Void)), Attack, Ability(W(Void))],
            ])?
            .end()
    }
}
