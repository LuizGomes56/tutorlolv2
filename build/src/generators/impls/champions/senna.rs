use super::*;

impl Generator for Senna {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                ("Innate - Relic Cannon", _1),
                ("Innate - Weakened Soul", Void),
            ],
        )
        .modify(P(_1), |_| f![0.2 * AttackDamage])?
        .modify(P(Void), |dmg| f![(dmg) / 100 * EnemyCurrentHealth])?
        .ability(Key::Q, [("Physical Damage", Void)])
        .ability(Key::W, [("Physical Damage", Void)])
        .ability(Key::R, [("Physical Damage", Void)])
        .damage_types([P(Void), P(_1), R(Void)], Physical)?
        .combos([
            &[
                Ability(W(Void)),
                Attack,
                Ability(Q(Void)),
                Ability(R(Void)),
                Attack,
            ],
            &[Ability(R(Void)), Ability(Q(Void))],
            &[Ability(W(Void)), Ability(Q(Void)), Ability(R(Void))],
            &[Ability(Q(Void)), Attack],
            &[
                Ability(Q(Void)),
                Attack,
                Ability(W(Void)),
                Attack,
                Ability(R(Void)),
                Attack,
            ],
            &[Ability(W(Void)), Attack, Ability(Q(Void)), Attack],
        ])?
        .end()
    }
}
