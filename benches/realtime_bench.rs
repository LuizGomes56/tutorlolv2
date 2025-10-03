// ! AI GENERATED

use bumpalo::Bump;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::{fs, time::Duration};

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
            .warm_up_time(Duration::from_secs(5))   // aquece mais
            .measurement_time(Duration::from_secs(15))
            .without_plots()
    };
    targets =
        bench_realtime_small,
        bench_realtime_medium,
        bench_realtime_xxl
);
criterion_main!(benches);
