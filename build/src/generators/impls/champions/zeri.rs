use super::*;

impl Generator for Zeri {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, Min), /* Basic Attack */
                (2, Max), /* Description 2 */
                (3, _1),  /* Description 2 [1] */
            ],
        );

        let charged = self.sum([P(Max), P(_1)])?;

        self.set_damage(P(Max), charged)?
            .delete(P(_1))
            .comment(P(Min), "Uncharged basic attack")?
            .comment(P(Max), "Fully charged basic attack")?
            .ability(
                Key::Q,
                [
                    (2, Min), /* Physical Damage per Hit */
                    (5, Max), /* Total Physical Damage */
                    (3, _1),  /* Total Critical Strike Damage */
                ],
            )
            .ability(
                Key::W,
                [
                    (2, Void), /* Physical Damage */
                    (0, _1),   /* Critical Strike Damage */
                ],
            )
            .ability(Key::E, [(0, Void) /* Burst Fire Bonus Magic Damage */])
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            .damage_type(R(Void), Magic)?
            .combos([
                &[
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(R(Void)),
                    Ability(Q(Max)),
                    Ability(Q(Max)),
                    Ability(Q(Max)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(Q(Max)),
                    Ability(Q(Max)),
                    Ability(Q(Max)),
                    Attack,
                ],
                &[
                    Ability(R(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Ability(Q(Max)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(W(Void)),
                ],
                &[
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Ability(Q(Max)),
                    Ability(Q(Max)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(Q(Max)),
                ],
                &[
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Ability(Q(Max)),
                    Ability(Q(Max)),
                    Attack,
                ],
                &[
                    Ability(Q(Max)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                ],
                &[
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(Q(Max)),
                ],
            ])?
            .end()
    }
}
