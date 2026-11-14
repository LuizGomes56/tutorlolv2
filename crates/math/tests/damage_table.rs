use math::{Ctx, CtxAttacker, CtxTarget, model::*};

const ROW_VALUES: [i32; 8] = {
    let layout = DamageLayout {
        abilities: 1,
        items: 2,
        runes: 1,
    };
    let mut values = [0; 8];
    let mut attackers = [CtxAttacker {
        attack_damage: 123.0,
        ..unsafe { core::mem::zeroed() }
    }];
    let mut targets = [CtxTarget {
        enemy_armor: 45.0,
        ..unsafe { core::mem::zeroed() }
    }];
    let mut table = DamageTableMut::new(layout, &mut values, &mut attackers, &mut targets);
    assert!(table.rows() == 1);
    assert!(table.layout().stride() == 8);
    assert!(table.ctx(0).attack_damage == 123.0);
    assert!(table.ctx(0).enemy_armor == 45.0);
    let row = table.row_mut(0);
    *row.attacks = [10, 20, 30, 40];
    row.abilities[0] = 50;
    row.items[0] = 60;
    row.items[1] = 70;
    row.runes[0] = 80;
    assert!(table.row(0).sum() == 330);
    values
};

#[test]
fn borrowed_rows_can_be_written_and_read_at_compile_time() {
    assert_eq!(ROW_VALUES, [10, 20, 30, 40, 50, 60, 70, 80]);
}

#[test]
fn owned_table_exposes_the_same_storage_through_its_view() {
    let mut table = DamageTable::zeroed(
        DamageLayout {
            abilities: 1,
            items: 0,
            runes: 0,
        },
        2,
        2,
    );
    {
        let mut view = table.as_mut();
        assert_eq!(view.rows(), 4);
        view.row_mut(3).abilities[0] = 91;
    }
    assert_eq!(table.row(3).abilities, &[91]);
    table.row_mut(2).abilities[0] = 27;
    assert_eq!(table.as_mut().row(2).abilities, &[27]);
    assert_eq!(
        table.ctx(0),
        Ctx::new(&CtxAttacker::default(), &CtxTarget::default())
    );
}

#[test]
#[should_panic(expected = "damage values do not match the layout")]
fn borrowed_table_rejects_inconsistent_storage() {
    DamageTableMut::new(
        DamageLayout {
            abilities: 0,
            items: 0,
            runes: 0,
        },
        &mut [0; 3],
        &mut [CtxAttacker::default()],
        &mut [CtxTarget::default()],
    );
}
