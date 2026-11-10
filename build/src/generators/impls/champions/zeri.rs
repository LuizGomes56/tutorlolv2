use super::*;

impl Generator for Zeri {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                ("Basic Attack", Min),
                ("Description 2", Max),
                ("Description 2 [1]", _1),
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
                    ("Physical Damage per Hit", Min),
                    ("Total Physical Damage", Max),
                    ("Total Critical Strike Damage", _1),
                ],
            )
            .ability(
                Key::W,
                [
                    ("Physical Damage", Void),
                    ("Critical Strike Damage", _1),
                ],
            )
            .ability(Key::E, [("Burst Fire Bonus Magic Damage", Void)])
            .ability(Key::R, [("Magic Damage", Void)])
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
