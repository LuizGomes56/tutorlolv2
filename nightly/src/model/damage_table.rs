//! Borrowed damage storage and the owned table's accessors.
use super::*;

/// Mutable view over caller-owned damage and context buffers.
///
/// Arrays can back this view during constant evaluation; at runtime the same
/// operations work on the slices owned by [`DamageTable`]. Contexts are shared
/// across rows: attacker `r % attackers.len()` and target `r / attackers.len()`.
#[derive(Debug)]
pub struct DamageTableMut<'a> {
    layout: DamageLayout,
    values: &'a mut [i32],
    attackers: &'a mut [CtxAttacker],
    targets: &'a mut [CtxTarget],
}

impl<'a> DamageTableMut<'a> {
    /// Borrows initialized buffers with exactly one complete row per
    /// attacker/target pair. At least one attacker must be provided.
    ///
    /// # Panics
    /// Panics if there are no attackers, the dimensions overflow, or the values
    /// buffer does not match the layout and context buffers.
    pub const fn new(
        layout: DamageLayout,
        values: &'a mut [i32],
        attackers: &'a mut [CtxAttacker],
        targets: &'a mut [CtxTarget],
    ) -> Self {
        assert!(!attackers.is_empty(), "at least one attacker is required");

        let rows = targets
            .len()
            .checked_mul(attackers.len())
            .expect("damage row count overflow");

        let len = rows
            .checked_mul(layout.stride())
            .expect("damage values length overflow");

        assert!(values.len() == len, "damage values do not match the layout");

        Self {
            layout,
            values,
            attackers,
            targets,
        }
    }

    /// Reborrows the buffers for a shorter-lived mutable view.
    pub const fn as_mut(&mut self) -> DamageTableMut<'_> {
        DamageTableMut {
            layout: self.layout,
            values: self.values,
            attackers: self.attackers,
            targets: self.targets,
        }
    }

    pub const fn rows(&self) -> usize {
        self.targets.len() * self.attackers.len()
    }

    /// Indexes of the attacker state and the target of a row
    #[cfg(feature = "engine")]
    const fn parts(&self, r: usize) -> (usize, usize) {
        let attackers = self.attackers.len();
        (r % attackers, r / attackers)
    }

    /// Context the row was evaluated with
    pub const fn ctx(&self, r: usize) -> Ctx {
        Self::read_ctx(self.attackers, self.targets, r)
    }

    pub(super) const fn read_ctx(
        attackers: &[CtxAttacker],
        targets: &[CtxTarget],
        r: usize,
    ) -> Ctx {
        Ctx::new(
            &attackers[r % attackers.len()],
            &targets[r / attackers.len()],
        )
    }

    /// Stores the parts of the context of a row. Rows must be filled with the
    /// first target first, and each target with its first attacker state
    /// first: each part is written at its first row only
    #[cfg(feature = "engine")]
    pub(crate) const fn set_ctx(&mut self, r: usize, ctx: &Ctx) {
        let (attacker, target) = self.parts(r);

        if target == 0 {
            self.attackers[attacker] = ctx.attacker();
        }

        if attacker == 0 {
            self.targets[target] = ctx.target();
        }
    }

    pub const fn layout(&self) -> DamageLayout {
        self.layout
    }

    pub const fn row(&self, r: usize) -> Damages<'_> {
        Self::read_row(self.layout, self.values, r, self.ctx(r))
    }

    pub(super) const fn read_row(
        layout: DamageLayout,
        values: &[i32],
        r: usize,
        ctx: Ctx,
    ) -> Damages<'_> {
        let s = layout.stride();
        let (_, rest) = values.split_at(r * s);
        let (row, _) = rest.split_at(s);
        let (attacks, rest) = row.split_at(DamageLayout::ATTACKS);
        let (abilities, rest) = rest.split_at(layout.abilities as usize);
        let (items, runes) = rest.split_at(layout.items as usize);

        Damages {
            attacks: Attacks {
                basic_attack: attacks[0],
                critical_strike: attacks[1],
                onhit_damage: RangeDamage {
                    minimum_damage: attacks[2],
                    maximum_damage: attacks[3],
                },
            },
            abilities,
            items,
            runes,
            ctx,
        }
    }

    pub const fn row_mut(&mut self, r: usize) -> DamagesMut<'_> {
        DamageTableMut {
            layout: self.layout,
            values: self.values,
            attackers: self.attackers,
            targets: self.targets,
        }
        .into_row_mut(r)
    }

    pub(crate) const fn into_row_mut(self, r: usize) -> DamagesMut<'a> {
        assert!(r < self.rows(), "damage row out of bounds");

        let s = self.layout.stride();
        let (_, rest) = self.values.split_at_mut(r * s);
        let (row, _) = rest.split_at_mut(s);
        let (attacks, rest) = row.split_at_mut(DamageLayout::ATTACKS);
        let (abilities, rest) = rest.split_at_mut(self.layout.abilities as usize);
        let (items, runes) = rest.split_at_mut(self.layout.items as usize);

        DamagesMut {
            // We're sure that the length is always 4 in this case
            attacks: attacks.first_chunk_mut::<4>().unwrap(),
            abilities,
            items,
            runes,
        }
    }
}

impl DamageTable {
    /// Table with `rows_per_target` rows for each target, one per attacker state.
    /// `rows_per_target` must not be zero
    pub fn zeroed(layout: DamageLayout, targets: usize, rows_per_target: usize) -> Self {
        let rows = targets * rows_per_target;

        Self {
            layout,
            values: unsafe { Box::new_zeroed_slice(rows * layout.stride()).assume_init() },
            attackers: unsafe { Box::new_zeroed_slice(rows_per_target).assume_init() },
            targets: unsafe { Box::new_zeroed_slice(targets).assume_init() },
        }
    }

    /// Borrows this table's buffers without allocating or copying them.
    pub const fn as_mut(&mut self) -> DamageTableMut<'_> {
        DamageTableMut {
            layout: self.layout,
            values: &mut self.values,
            attackers: &mut self.attackers,
            targets: &mut self.targets,
        }
    }

    pub const fn rows(&self) -> usize {
        self.targets.len() * self.attackers.len()
    }

    pub const fn layout(&self) -> DamageLayout {
        self.layout
    }

    /// Context the row was evaluated with.
    pub const fn ctx(&self, r: usize) -> Ctx {
        DamageTableMut::read_ctx(&self.attackers, &self.targets, r)
    }

    pub const fn row(&self, r: usize) -> Damages<'_> {
        DamageTableMut::read_row(self.layout, &self.values, r, self.ctx(r))
    }

    pub const fn row_mut(&mut self, r: usize) -> DamagesMut<'_> {
        self.as_mut().into_row_mut(r)
    }
}

/// The same damage layout as `DamageTable`, stored in compile-time-sized arrays.
/// One attacker state is shared by every monster and enemy row.
#[derive(Clone, Debug)]
pub struct ArrayDamageTable<const L: DamageLayout, const E: usize>
where
    [(); (L_MSTR + E) * L.stride()]:,
    [(); L_MSTR + E]:,
{
    values: [i32; (L_MSTR + E) * L.stride()],
    attackers: [CtxAttacker; 1],
    targets: [CtxTarget; L_MSTR + E],
}

impl<const L: DamageLayout, const E: usize> ArrayDamageTable<L, E>
where
    [(); (L_MSTR + E) * L.stride()]:,
    [(); L_MSTR + E]:,
{
    pub const fn zeroed() -> Self {
        Self {
            values: [0; (L_MSTR + E) * L.stride()],
            // Context parts consist only of floats, levels and flags, all zero-valid.
            attackers: unsafe { core::mem::zeroed() },
            targets: unsafe { core::mem::zeroed() },
        }
    }
    pub const fn as_mut(&mut self) -> DamageTableMut<'_> {
        DamageTableMut::new(L, &mut self.values, &mut self.attackers, &mut self.targets)
    }
    pub const fn rows(&self) -> usize {
        L_MSTR + E
    }
    pub const fn layout(&self) -> DamageLayout {
        L
    }
    pub const fn ctx(&self, r: usize) -> Ctx {
        DamageTableMut::read_ctx(&self.attackers, &self.targets, r)
    }
    pub const fn row(&self, r: usize) -> Damages<'_> {
        DamageTableMut::read_row(L, &self.values, r, self.ctx(r))
    }
    pub const fn row_mut(&mut self, r: usize) -> DamagesMut<'_> {
        self.as_mut().into_row_mut(r)
    }
}

#[cfg(all(test, feature = "engine"))]
mod tests {
    use super::*;

    // These assertions are evaluated by rustc, including set_ctx's writes.
    const _: () = {
        let layout = DamageLayout {
            abilities: 0,
            items: 0,
            runes: 0,
        };
        let mut values = [0; 16];
        // All fields in the context parts are numeric and accept zero.
        let mut attackers: [CtxAttacker; 2] = unsafe { core::mem::zeroed() };
        let mut targets: [CtxTarget; 2] = unsafe { core::mem::zeroed() };
        let mut table = DamageTableMut::new(layout, &mut values, &mut attackers, &mut targets);
        let mut ctx: Ctx = unsafe { core::mem::zeroed() };
        ctx.attack_damage = 100.0;
        ctx.enemy_armor = 30.0;
        table.set_ctx(0, &ctx);
        ctx.attack_damage = 200.0;
        table.set_ctx(1, &ctx);
        ctx.attack_damage = 100.0;
        ctx.enemy_armor = 60.0;
        table.set_ctx(2, &ctx);
        ctx.attack_damage = 200.0;
        table.set_ctx(3, &ctx);
        assert!(table.ctx(0).attack_damage == 100.0);
        assert!(table.ctx(1).attack_damage == 200.0);
        assert!(table.ctx(2).attack_damage == 100.0);
        assert!(table.ctx(3).attack_damage == 200.0);
        assert!(table.ctx(0).enemy_armor == 30.0);
        assert!(table.ctx(1).enemy_armor == 30.0);
        assert!(table.ctx(2).enemy_armor == 60.0);
        assert!(table.ctx(3).enemy_armor == 60.0);
    };
}
