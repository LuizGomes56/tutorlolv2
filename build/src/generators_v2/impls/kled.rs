use super::*;

impl GeneratorV2 for Kled {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Physical Damage", 1, Q(Min))?
            .take("Total Physical Damage", Q(Max))?
            .take_in("Pocket Pistol", "Physical Damage", Q(_1Min))?
            .take("Maximum Damage", Q(_1Max))?
            .comment(Q(_1Min), "Pocket Pistol while dismounted")
            .comment(Q(_1Max), "Pocket Pistol while dismounted, maximum damage")
            .take("Additional Physical Damage", W(Void))?
            .take("Physical Damage", E(Min))?
            .take("Total Physical Damage", E(Max))?
            .damage_type(Q(Min), Physical)
            .damage_type(Q(Max), Physical)
            .damage_type(Q(_1Min), Physical)
            .damage_type(Q(_1Max), Physical)
            .damage_type(W(Void), Physical)
            .damage_type(E(Min), Physical)
            .damage_type(E(Max), Physical)
            .take("Minimum Magic Damage", R(Min))?
            .take("Maximum Magic Damage", R(Max))?
            .damage_type(R(Min), Magic)
            .damage_type(R(Max), Magic)
            .combos([
                &[
                    Ability(R(Max)),
                    Ability(E(Min)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Min)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Min)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(E(Min)),
                    Attack,
                ],
                &[
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Min)),
                    Attack,
                    Attack,
                    Ability(E(Min)),
                    Attack,
                ],
                &[
                    Ability(E(Min)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Attack,
                    Ability(E(Min)),
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                    Ability(E(Min)),
                    Attack,
                    Ability(E(Min)),
                    Attack,
                ],
                &[
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Ability(E(Min)),
                    Attack,
                    Attack,
                    Ability(E(Min)),
                ],
                &[
                    Ability(Q(Max)),
                    Ability(E(Min)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(E(Min)),
                    Attack,
                ],
                &[
                    Ability(E(Max)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                ],
            ])?
            .end()
    }
}
