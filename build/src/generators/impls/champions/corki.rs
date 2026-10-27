use super::*;

impl Generator for Corki {
    fn generate(&mut self) -> MayFail {
        self.insert(
            P(Void),
            Ability {
                name: "Hextech Munitions".into(),
                damage_type: True,
                attributes: Undefined,
                comment: "Bonus true damage per basic attack".into(),
                damage: f![0.2 * AttackDamage],
            },
        )
        .ability(Key::Q, [(0, Void) /* Magic Damage */])
        .ability(
            Key::W,
            [
                (0, Min), /* Magic Damage Per Tick */
                (1, Max), /* Total Magic Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (0, Min), /* Physical Damage Per Tick */
                (2, Max), /* Total Physical Damage */
            ],
        )
        .damage_types([E(Min), E(Max)], Physical)?
        .ability(
            Key::R,
            [
                (1, Min), /* Physical Damage */
                (0, Max), /* Big One Physical Damage */
            ],
        )
        .combo([
            Ability(W(Max)),
            Ability(E(Max)),
            Attack,
            Ability(Q(Void)),
            Ability(R(Min)),
            Ability(W(Max)),
            Attack,
            Ability(R(Min)),
            Attack,
        ])?
        .combo([Ability(Q(Void)), Ability(R(Max)), Attack])?
        .combo([
            Ability(Q(Void)),
            Ability(W(Max)),
            Ability(E(Max)),
            Attack,
            Attack,
        ])?
        .combo([
            Ability(E(Max)),
            Ability(Q(Void)),
            Ability(R(Min)),
            Attack,
            Attack,
            Ability(R(Min)),
            Attack,
            Attack,
            Ability(R(Min)),
        ])?
        .combo([
            Ability(W(Max)),
            Ability(E(Max)),
            Attack,
            Ability(R(Min)),
            Attack,
            Ability(Q(Void)),
            Attack,
            Ability(R(Min)),
            Attack,
        ])?
        .combo([
            Ability(E(Max)),
            Attack,
            Ability(Q(Void)),
            Attack,
            Attack,
            Attack,
        ])?
        .combo([Ability(Q(Void)), Ability(E(Max)), Attack, Attack])?
        .end()
    }
}
