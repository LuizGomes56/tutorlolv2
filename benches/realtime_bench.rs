// ! AI GENERATED

use criterion::{BatchSize, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;
use std::{fs, time::Duration};
use tutorlolv2::__v2::riot::RiotRealtime;

// NEW: bumpalo para a arena
use bumpalo::Bump;

fn bench_realtime_only(c: &mut Criterion) {
    let bytes = fs::read("serde_test.json").expect("Não consegui ler serde_test.json");
    let de = serde_json::from_slice(&bytes).expect("JSON inválido em serde_test.json");
    let de2 = serde_json::from_slice(&bytes).expect("JSON inválido em serde_test.json");

    let mut group = c.benchmark_group("realtime_only");
    group.sample_size(50);
    group.measurement_time(Duration::from_secs(5));

    group.bench_function(BenchmarkId::new("v2", "compute"), |b| {
        b.iter(|| {
            let rt = tutorlolv2_math::__v2::rt_stack::realtime(black_box(&de))
                .expect("realtime v2 falhou");
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
}

// NEW: apenas compute com arena
fn bench_realtime_arena_only(c: &mut Criterion) {
    let bytes = fs::read("serde_test.json").expect("Não consegui ler serde_test.json");
    // Sem anotar tipo: o compilador infere para RiotRealtime pelo uso em realtime_arena
    let game = serde_json::from_slice(&bytes).expect("JSON inválido em serde_test.json");

    let mut group = c.benchmark_group("realtime_arena_only");
    group.sample_size(50);
    group.measurement_time(Duration::from_secs(5));

    group.bench_function(BenchmarkId::new("v2_arena", "compute"), |b| {
        b.iter(|| {
            // Arena com pelo menos 2 MB
            let arena = Bump::with_capacity(1024 * 1024);
            let ptr =
                tutorlolv2_math::__v2::rt_arena::realtime_arena(&arena, black_box(&game)).unwrap();
            black_box(ptr);
        });
    });

    group.finish();
}

fn bench_realtime_then_serialize(c: &mut Criterion) {
    let bytes = fs::read("serde_test.json").expect("Não consegui ler serde_test.json");
    let de = serde_json::from_slice(&bytes).expect("JSON inválido em serde_test.json");
    let de2 = serde_json::from_slice(&bytes).expect("JSON inválido em serde_test.json");

    let mut group = c.benchmark_group("realtime_then_serialize");
    group.sample_size(40);
    group.measurement_time(Duration::from_secs(5));

    group.bench_function(BenchmarkId::new("v2", "serialize"), |b| {
        b.iter_batched(
            || {
                tutorlolv2_math::__v2::rt_stack::realtime(black_box(&de))
                    .expect("realtime v2 falhou")
            },
            |rt| {
                let ser = bincode::encode_to_vec(&rt, bincode::config::standard())
                    .expect("serialize v2 falhou");
                black_box(ser.len());
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function(BenchmarkId::new("old", "serialize"), |b| {
        b.iter_batched(
            || {
                tutorlolv2_math::math::realtime::realtime(black_box(&de2))
                    .expect("realtime old falhou")
            },
            |rt| {
                let ser = bincode::encode_to_vec(&rt, bincode::config::standard())
                    .expect("serialize old falhou");
                black_box(ser.len());
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

// NEW: compute→serialize usando a arena
fn bench_realtime_arena_then_serialize(c: &mut Criterion) {
    let bytes = fs::read("serde_test.json").expect("Não consegui ler serde_test.json");
    // Sem anotar tipo – inferido como RiotRealtime
    let game = serde_json::from_slice(&bytes).expect("JSON inválido em serde_test.json");

    let mut group = c.benchmark_group("realtime_arena_then_serialize");
    group.sample_size(40);
    group.measurement_time(Duration::from_secs(5));

    use std::mem;

    group.bench_function(BenchmarkId::new("v2_arena", "serialize"), |b| {
        b.iter_batched(
            || {
                // Arena ≥ 2MB no HEAP
                let arena_box = Box::new(bumpalo::Bump::with_capacity(2 * 1024 * 1024));
                let arena_ptr: *mut bumpalo::Bump = Box::into_raw(arena_box);

                // Recria referência só para chamar o realtime_arena
                let arena_ref: &bumpalo::Bump = unsafe { &*arena_ptr };

                // `game` inferido de serde_json::from_slice sem anotar tipo
                let ptr =
                    tutorlolv2_math::__v2::rt_arena::realtime_arena(arena_ref, black_box(&game))
                        .unwrap();

                // Opcional: “elevar” o lifetime do ponteiro para facilitar o tipo (somente porque a arena vai viver até o routine)
                let ptr_static: *const tutorlolv2_math::__v2::model_arena::Realtime<'static> =
                    unsafe { mem::transmute(ptr) };

                // Passamos só ponteiros crus; nada aqui cai fora de escopo
                (arena_ptr, ptr_static)
            },
            |(arena_ptr, rt_ptr)| {
                // Arena volta a ser Box para liberar no fim do routine
                let arena_box: Box<bumpalo::Bump> = unsafe { Box::from_raw(arena_ptr) };

                // Usa o ponteiro para serializar (enquanto a arena está viva)
                let rt_ref = unsafe { &*rt_ptr };
                let ser = bincode::encode_to_vec(rt_ref, bincode::config::standard())
                    .expect("serialize v2_arena falhou");

                // usa arena_box para não sobrar warning de “unused”
                black_box(ser.len());
                black_box(&*arena_box as *const _);

                // `arena_box` é dropado aqui → memória da arena liberada
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

fn bench_parse_only(c: &mut Criterion) {
    let bytes = fs::read("serde_test.json").expect("read serde_test.json");
    let mut group = c.benchmark_group("parse_only");
    group.sample_size(40);
    group.measurement_time(Duration::from_secs(5));
    group.throughput(Throughput::Bytes(bytes.len() as u64));

    group.bench_function("serde_json_from_slice", |b| {
        b.iter(|| {
            // Sem anotar tipo aqui também
            let v: RiotRealtime = serde_json::from_slice(black_box(&bytes)).unwrap();
            black_box(v);
        });
    });

    group.finish();
}

criterion_group!(
    name = benches;
    config = {
        Criterion::default()
            .warm_up_time(Duration::from_secs(3))
            .measurement_time(Duration::from_secs(5))
            .without_plots()
    };
    targets =
        bench_realtime_only,
        bench_realtime_arena_only,            // NEW
        bench_realtime_then_serialize,
        bench_realtime_arena_then_serialize,  // NEW
        bench_parse_only
);
criterion_main!(benches);
