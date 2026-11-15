use super::*;

/// Aphelios has five weapons, always in this order: Calibrum, Severum, Gravitum,
/// Infernum and Crescendum. Each one's damage is its own ability, numbered by the
/// weapon (`_1` to `_5`), so every weapon is evaluated at once and none reads
/// `stacks`. Weapons with no damage of their own get no ability
impl Generator for Aphelios {
    fn generate(&mut self) -> MayFail {
        // Infernum's secondary bolts deal a share of the triggering attack, which is
        // the 110% AD fire bolt. Of the `description` values of P, Severum has the
        // first two
        let infernum_share = self.value_nth("description", 3, Key::P)?;

        self.take_in("Calibrum", "description2", P(_1))?
            .comment(P(_1), "Calibrum mark bonus damage per mark consumed")
            .take_nth("description", 2, P(_4))?
            .edit(P(_4), |p| f![(p) * (infernum_share) / 100])
            .comment(P(_4), "Infernum secondary bolts, per target")
            // Crescendum's bonus grows with the Chakrams caught, up to 20. It deals
            // nothing at 0 Chakrams, so the minimum is 1
            .take_nth("description3", 0, P(_5Min))?
            .derive(P(_5Max), P(_5Min), |p| {
                p.replace_var(Stacks, &Formula::Const(20.0))
            })?
            .edit(P(_5Min), |p| p.replace_var(Stacks, &Formula::Const(1.0)))
            .comment(P(_5Min), "Crescendum bonus damage with 1 Chakram")
            .comment(P(_5Max), "Crescendum bonus damage with 20 Chakrams")
            // Of the `Active` values of Q: Moonshot, Onslaught, Duskwave (wave, then
            // the off-hand volley) and Sentry (attack, then its range)
            .take_in("Moonshot", "Active", Q(_1))?
            .comment(Q(_1), "Calibrum: Moonshot")
            .take_in("Onslaught", "Active", Q(_2))?
            .comment(Q(_2), "Severum: Onslaught damage per attack")
            .take_in("Binding Eclipse", "Gravitum - Active", Q(_3))?
            .comment(Q(_3), "Gravitum: Binding Eclipse")
            .take_nth("Active", 2, Q(_4))?
            .comment(Q(_4), "Infernum: Duskwave")
            .take_nth("Active", 4, Q(_5))?
            .comment(Q(_5), "Crescendum: Sentry basic attack")
            // Of the `Active` values of R: the smite, the attacks, the critical
            // multiplier, Calibrum's mark, Severum's heal, Infernum's blast bonus, then
            // Infernum's attack and its bolts
            .take_nth("Active", 0, R(Void))?
            .comment(R(Void), "Moonlight Vigil")
            .take_nth("Active", 3, R(_1))?
            .comment(R(_1), "Calibrum: bonus damage per mark consumed")
            .take_nth("Active", 5, R(_4))?
            .comment(R(_4), "Infernum: bonus damage of the blast")
            .combos([
                &[Ability(Q(_1)), Attack, Ability(P(_1))],
                &[
                    Ability(Q(_2)),
                    Ability(Q(_2)),
                    Ability(Q(_2)),
                    Ability(Q(_2)),
                    Ability(Q(_2)),
                    Ability(Q(_2)),
                ],
                &[Ability(Q(_3)), Attack, Attack],
                &[Ability(Q(_4)), Attack, Ability(P(_4))],
                &[Ability(Q(_5)), Ability(Q(_5)), Attack, Ability(P(_5Max))],
                &[Ability(R(Void)), Ability(R(_1)), Attack, Ability(P(_1))],
                &[Ability(R(Void)), Ability(R(_4)), Attack, Ability(P(_4))],
            ])?
            .end()
    }
}
