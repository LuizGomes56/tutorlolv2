use super::*;

impl Generator for Fiora {
    fn generate(&mut self) -> MayFail {
        let vital = f![(0.03 + 0.0004 * BonusAd) * EnemyMaxHealth];

        self.ability(Key::P, [(0, Void) /* Description 1 */])
            .set_damage(P(Void), vital.clone())?
            .comment(P(Void), "Vital true damage")?
            .ability(Key::Q, [(0, Void) /* Physical Damage */])
            .ability(Key::W, [(0, Void) /* Magic Damage */])
            .ability(Key::E, [(1, Void) /* Critical damage */])
            .modify(E(Void), |_| f![(1.6 + 0.1 * (ELevel - 1)) * AttackDamage])?
            .comment(E(Void), "Second basic attack critical strike")?
            .clone_with(P(Void), R(Void), |p| f![4 * (p)])?
            .comment(R(Void), "All 4 Vitals")?
            .combo([
                Ability(R(Void)),
                Attack,
                Ability(E(Void)),
                Ability(Q(Void)),
                Ability(E(Void)),
                Attack,
            ])?
            .combo([
                Ability(R(Void)),
                Ability(W(Void)),
                Attack,
                Ability(E(Void)),
                Ability(Q(Void)),
            ])?
            .combo([
                Ability(Q(Void)),
                Ability(R(Void)),
                Ability(W(Void)),
                Attack,
                Ability(E(Void)),
                Ability(E(Void)),
                Attack,
                Attack,
                Ability(Q(Void)),
                Attack,
            ])?
            .combo([
                Attack,
                Ability(E(Void)),
                Ability(E(Void)),
                Attack,
                Ability(R(Void)),
                Ability(W(Void)),
                Attack,
                Ability(Q(Void)),
                Attack,
            ])?
            .combo([
                Ability(R(Void)),
                Ability(Q(Void)),
                Attack,
                Attack,
                Ability(E(Void)),
                Ability(E(Void)),
                Attack,
            ])?
            .combo([
                Attack,
                Ability(E(Void)),
                Ability(Q(Void)),
                Ability(W(Void)),
                Ability(E(Void)),
                Attack,
            ])?
            .combo([
                Attack,
                Ability(Q(Void)),
                Ability(E(Void)),
                Ability(E(Void)),
                Attack,
            ])?
            .combo([
                Ability(Q(Void)),
                Attack,
                Ability(W(Void)),
                Attack,
                Ability(E(Void)),
                Ability(E(Void)),
                Attack,
            ])?
            .combo([
                Ability(Q(Void)),
                Attack,
                Ability(E(Void)),
                Ability(W(Void)),
                Ability(E(Void)),
                Attack,
            ])?
            .end()
    }
}
