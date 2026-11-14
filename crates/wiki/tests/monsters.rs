use wiki::{
    monsters::{MonsterData, MonsterStat},
    wikitext::Nodes,
};

fn stat(src: &str) -> MonsterStat {
    MonsterStat::new(&Nodes::parse(src)).unwrap()
}

fn levels(src: &str) -> (u8, Vec<f64>) {
    match stat(src) {
        MonsterStat::Levels { start, values } => (start, values),
        other => panic!("Expected levels, found {other:?}"),
    }
}

#[test]
fn flat_and_expr() {
    assert_eq!(stat("42"), MonsterStat::Flat(42.0));
    assert_eq!(stat("-30"), MonsterStat::Flat(-30.0));
    assert_eq!(
        stat("{{#expr:3625*1.15}}"),
        MonsterStat::Flat(3625.0 * 1.15)
    );
}

#[test]
fn growth_starts_at_its_level() {
    let (start, values) = levels("{{gs|16300|170|11|18}}");
    assert_eq!(start, 11);
    assert_eq!(values.len(), 8);
    assert_eq!(values[0], 16300.0);
    assert_eq!(values[7], 16300.0 + 7.0 * 170.0);

    let (_, values) = levels("{{gs|{{#expr:3625*1.15}}|{{#expr:375*1.15}}|6|18}}");
    assert_eq!(values[0], 3625.0 * 1.15);
}

#[test]
fn jungle_monster_health() {
    let (start, values) = levels("{{Jungle monster stat|2050|hp}}");
    assert_eq!(start, 1);
    assert_eq!(values.len(), 18);
    assert_eq!(values[1], 2050.0);
    assert!((values[2] - 2050.0 * 1.2).abs() < 1e-6);
    assert!((values[10] - 2050.0 * 2.0).abs() < 1e-6);
    assert!((values[17] - 2050.0 * 2.35).abs() < 1e-6);
}

#[test]
fn level_progression_fills_missing_levels() {
    let (start, values) =
        levels("{{pp|2300;2760 to 6210 for 16|1;3 to 18|formula=2300 + 230 per level}}");
    assert_eq!(start, 1);
    assert_eq!(values.len(), 18);
    assert_eq!(&values[..3], &[2300.0, 2300.0, 2760.0]);
    assert!((values[17] - 6210.0).abs() < 1e-6);

    let (start, values) = levels("{{pp|8000;8000*1.15;8000*2.3|6 to 8}}");
    assert_eq!(start, 6);
    assert_eq!(values, [8000.0, 8000.0 * 1.15, 8000.0 * 2.3]);
}

#[test]
fn scuttler_uses_respawn_health() {
    let (start, values) = levels(
        "{{pp|1550*.65 to 1550*1.8*.65 by 1550*.1*.65|formula=x}} (first spawned) / \
         {{pp|1550 to 1550*1.8 by 1550*.1;1550*2 to 1550*3.4 by 1550*.2|formula=x}} (normal form)",
    );
    assert_eq!(start, 1);
    assert_eq!(values.len(), 17);
    assert_eq!(values[0], 1550.0);
    assert!((values[16] - 5270.0).abs() < 1e-6);
}

#[test]
fn minion_upgrades_read_the_formula() {
    assert_eq!(
        stat(
            "{{pp|type=upgrades|430+35*x for 15;...;1550|x for 15;...;32|formula=430 + 35 per upgrade, capped at (430 + 1120) health}}"
        ),
        MonsterStat::Upgrades {
            base: 430.0,
            per_upgrade: 35.0,
            cap: 1550.0
        }
    );
    assert_eq!(
        stat(
            "{{pp|type=upgrades|round=3|0 to 0 for 5; then+0.085*(x-1)/2*x;20|x for 28|formula=0 for upgrades 1–5; then + (0.085*(upgrade-6)/2*(upgrade-5)), capped at 20 armor}}"
        ),
        MonsterStat::TriangularUpgrades {
            coefficient: 0.085,
            after: 5,
            cap: 20.0
        }
    );
}

#[test]
fn camp_pages_hold_every_infobox() {
    let monsters = MonsterData::from_page(
        "{{Infobox unit\n|name = Krug\n|hp = {{Jungle monster stat|650|hp}}\n|armor = 20\n|magicresist = 20\n}}\n\
         {{Infobox unit\n|name = Mini Krug\n|hp = {{Jungle monster stat|60|hp}}\n|armor = 20\n|magicresist = 20\n}}",
    )
    .unwrap();

    assert_eq!(
        monsters
            .iter()
            .map(|(name, _)| name.as_str())
            .collect::<Vec<_>>(),
        ["Krug", "Mini Krug"]
    );
}
