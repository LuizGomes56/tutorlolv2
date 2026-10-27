use super::*;

impl Generator for Ashe {
    fn generate(&mut self) -> MayFail {
        self.insert(
            P(Void),
            Ability {
                name: "Frost Shot".into(),
                damage_type: Physical,
                attributes: Undefined,
                comment: "Bonus damage per basic attack, based on critical strike chance".into(),
                damage: f![AttackDamage * CritChance / 100],
            },
        )
        .ability(
            Key::Q,
            [
                (1, Min), /* Physical Damage Per Arrow */
                (2, Max), /* Total Damage Per Flurry */
            ],
        )
        .ability(Key::W, [(1, Void) /* Physical Damage */])
        .ability(Key::R, [(0, Void) /* Magic Damage */])
        .combo([
            Attack,
            Ability(Q(Max)),
            Ability(R(Void)),
            Attack,
            Ability(W(Void)),
            Attack,
        ])?
        .combo([Ability(W(Void)), Attack])?
        .combo([Ability(R(Void)), Attack])?
        .combo([Attack, Attack, Attack, Attack, Ability(Q(Max))])?
        .combo([
            Ability(R(Void)),
            Attack,
            Ability(W(Void)),
            Attack,
            Attack,
            Attack,
            Ability(Q(Max)),
            Attack,
        ])?
        .combo([Attack, Ability(W(Void)), Attack])?
        .end()
    }
}
