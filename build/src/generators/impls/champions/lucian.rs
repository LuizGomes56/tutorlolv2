use super::*;

impl Generator for Lucian {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (1, Void), /* Innate */
                (2, _1),   /* Innate - Vigilance */
            ],
        )
        .damage_type(P(_1), Magic)?
        .ability(Key::Q, [(0, Void) /* Physical Damage */])
        .ability(Key::W, [(1, Void) /* Magic Damage */])
        .ability(Key::R, [(1, Min) /* Physical Damage Per Shot */])
        .clone_with(R(Min), R(Max), |dmg| {
            let shots = f![2.2222 * CritChance / 100];
            /* Maybe include IE bonus shots in the future */
            f![(dmg) * shots]
        })?
        .comment(R(Max), "Maximum Damage without IE")?
        .combo([
            Ability(W(Void)),
            Attack,
            Attack,
            Attack,
            Ability(Q(Void)),
            Attack,
            Attack,
        ])?
        .combo([
            Ability(W(Void)),
            Attack,
            Attack,
            Ability(Q(Void)),
            Attack,
            Attack,
            Attack,
        ])?
        .combo([Ability(W(Void)), Attack, Attack, Ability(Q(Void)), Attack])?
        .combo([
            Ability(W(Void)),
            Ability(R(Max)),
            Attack,
            Ability(Q(Void)),
            Attack,
        ])?
        .combo([
            Attack,
            Ability(W(Void)),
            Ability(Q(Void)),
            Attack,
            Ability(R(Max)),
            Attack,
        ])?
        .combo([Ability(Q(Void)), Attack, Attack, Ability(W(Void)), Attack])?
        .combo([Attack, Ability(Q(Void)), Attack, Ability(W(Void)), Attack])?
        .combo([Ability(Q(Void)), Attack, Attack])?
        .end()
    }
}
