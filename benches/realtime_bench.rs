// ! AI GENERATED

use bumpalo::Bump;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::{fs, time::Duration};
use tutorlolv2::__v2::AbilityLevels;
use tutorlolv2::__v2::riot::RiotChampionStats;
use tutorlolv2::__v2::stack::calc::calculator;
use tutorlolv2::__v2::stack::model::*;

/// Capacidade da arena (>= 2 MB). Usei 8 MB para folga nos casos grandes.
const ARENA_CAP: usize = 8 * 1024 * 1024;

fn bench_realtime_for_file(c: &mut Criterion, path: &str, label: &str) {
    let bytes = fs::read(path).unwrap_or_else(|_| panic!("Não consegui ler {path}"));
    // Sem anotar tipo: o compilador infere pelo uso nas funções realtime()
    let de = serde_json::from_slice(&bytes).expect("JSON inválido (de)");
    let de2 = serde_json::from_slice(&bytes).expect("JSON inválido (de2)");

    // -------- stack / stdalloc / old --------
    let mut group = c.benchmark_group(format!("realtime_only/{label}"));
    group.sample_size(80); // mais amostras
    group.measurement_time(Duration::from_secs(15)); // janelas maiores

    group.bench_function(BenchmarkId::new("v2_stack", "compute"), |b| {
        b.iter(|| {
            let rt = tutorlolv2_math::__v2::stack::rt::realtime(black_box(&de))
                .expect("realtime v2 (stack) falhou");
            black_box(rt);
        });
    });

    group.bench_function(BenchmarkId::new("v2_stdalloc", "compute"), |b| {
        b.iter(|| {
            let rt = tutorlolv2_math::__v2::stdalloc::rt::realtime(black_box(&de))
                .expect("realtime v2 (stdalloc) falhou");
            black_box(rt);
        });
    });

    group.bench_function(BenchmarkId::new("old", "compute"), |b| {
        b.iter(|| {
            let rt = tutorlolv2_math::math::realtime::realtime(black_box(&de2))
                .expect("realtime old falhou");
            black_box(rt);
        });
    });
    group.finish();

    // -------- arena --------
    // Parse único (sem type hint) e execução em arena por iteração
    let game = serde_json::from_slice(&bytes).expect("JSON inválido (game)");

    let mut group_arena = c.benchmark_group(format!("realtime_arena_only/{label}"));
    group_arena.sample_size(80);
    group_arena.measurement_time(Duration::from_secs(15));

    group_arena.bench_function(BenchmarkId::new("v2_arena", "compute"), |b| {
        b.iter(|| {
            // Arena com pelo menos 2 MB (usamos 8 MB para exemplo.json)
            let arena = Bump::with_capacity(ARENA_CAP);
            let ptr = tutorlolv2_math::__v2::arena::rt::realtime_arena(&arena, black_box(&game))
                .expect("realtime v2 (arena) falhou");
            black_box(ptr);
        });
    });
    group_arena.finish();
}

fn bench_realtime_small(c: &mut Criterion) {
    bench_realtime_for_file(c, "serde_test.json", "small");
}

fn bench_realtime_medium(c: &mut Criterion) {
    bench_realtime_for_file(c, "example.json", "medium");
}

fn bench_realtime_xxl(c: &mut Criterion) {
    bench_realtime_for_file(c, "example2.json", "xxl");
}

criterion_group!(
    name = benches;
    config = {
        Criterion::default()
            .warm_up_time(Duration::from_secs(3))   // aquece mais
            .measurement_time(Duration::from_secs(15))
            .without_plots()
    };
    targets =
        bench_realtime_small,
        bench_realtime_medium,
        bench_realtime_xxl,
        bench_calculator_stack
);
criterion_main!(benches);

fn make_input_game() -> InputGame {
    InputGame {
        active_player: InputActivePlayer {
            runes: Default::default(),
            abilities: AbilityLevels {
                q: 1,
                w: 2,
                e: 3,
                r: 3,
            },
            data: InputMinData {
                stats: RiotChampionStats {
                    ability_power: 100.0,
                    armor: 1000.0,
                    armor_penetration_flat: 1.0,
                    armor_penetration_percent: 1.0,
                    attack_damage: 1.0,
                    attack_range: 1.0,
                    attack_speed: 1.0,
                    crit_chance: 1.0,
                    crit_damage: 1.0,
                    current_health: 1.0,
                    magic_penetration_flat: 1.0,
                    magic_penetration_percent: 1.0,
                    magic_resist: 1.0,
                    health: 1.0,
                    mana: 1.0,
                    current_mana: 1.0,
                },
                items: Default::default(),
                stacks: 1,
                level: 18,
                infer_stats: false,
                is_mega_gnar: false,
                champion_id: tutorlolv2::ChampionId::Aatrox,
            },
        },
        enemy_earth_dragons: 1,
        stack_exceptions: Default::default(),
        enemy_players: [InputMinData {
            is_mega_gnar: false,
            stacks: 0,
            stats: SimpleStatsF32 {
                armor: 100.0,
                health: 100.0,
                magic_resist: 100.0,
            },
            items: Default::default(),
            infer_stats: false,
            level: 18,
            champion_id: tutorlolv2::ChampionId::Ekko,
        }]
        .into(),
        ally_dragons: Dragons { earth: 1, fire: 1 },
    }
}

fn bench_calculator_stack(c: &mut Criterion) {
    let mut group = c.benchmark_group("calculator_stack");

    // Janela maior p/ reduzir ruído
    group.sample_size(1000);
    group.warm_up_time(Duration::from_secs(3));
    group.measurement_time(Duration::from_secs(20));

    group.bench_function("compute", |b| {
        b.iter_batched(
            make_input_game, // setup (fora do tempo medido)
            |game| {
                let out = calculator(black_box(game)); // rotina medida
                black_box(out);
            },
            criterion::BatchSize::SmallInput,
        );
    });

    group.finish();
}
