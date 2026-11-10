use super::*;

impl Generator for Lucian {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                ("Innate", Void),
                ("Innate - Vigilance", _1),
            ],
        )
        .damage_type(P(_1), Magic)?
        .ability(Key::Q, [("Physical Damage", Void)])
        .ability(Key::W, [("Magic Damage", Void)])
        .ability(Key::R, [("Physical Damage Per Shot", Min)])
        .clone_with(R(Min), R(Max), |dmg| {
            let shots = f![2.2222 * CritChance / 100];
            /* Maybe include IE bonus shots in the future */
            f![(dmg) * shots]
        })?
        .comment(R(Max), "Maximum Damage without IE")?
        .combos([
            &[
                Ability(W(Void)),
                Attack,
                Attack,
                Attack,
                Ability(Q(Void)),
                Attack,
                Attack,
            ],
            &[
                Ability(W(Void)),
                Attack,
                Attack,
                Ability(Q(Void)),
                Attack,
                Attack,
                Attack,
            ],
            &[Ability(W(Void)), Attack, Attack, Ability(Q(Void)), Attack],
            &[
                Ability(W(Void)),
                Ability(R(Max)),
                Attack,
                Ability(Q(Void)),
                Attack,
            ],
            &[
                Attack,
                Ability(W(Void)),
                Ability(Q(Void)),
                Attack,
                Ability(R(Max)),
                Attack,
            ],
            &[Ability(Q(Void)), Attack, Attack, Ability(W(Void)), Attack],
            &[Attack, Ability(Q(Void)), Attack, Ability(W(Void)), Attack],
            &[Ability(Q(Void)), Attack, Attack],
        ])?
        .end()
    }
}
