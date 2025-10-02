// ! AI GENERATED

use criterion::{BatchSize, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use serde_json::Value;
use std::hint::black_box;
use std::{fs, time::Duration};

fn bench_realtime_only(c: &mut Criterion) {
    let bytes = fs::read("example.json").expect("Não consegui ler example.json");
    let de = serde_json::from_slice(&bytes).expect("JSON inválido em example.json");
    let de2 = serde_json::from_slice(&bytes).expect("JSON inválido em example.json");

    let mut group = c.benchmark_group("realtime_only");
    group.sample_size(50);
    group.measurement_time(Duration::from_secs(5));

    group.bench_function(BenchmarkId::new("v2", "compute"), |b| {
        b.iter(|| {
            let rt =
                tutorlolv2_math::__v2::rt::realtime(black_box(&de)).expect("realtime v2 falhou");
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

fn bench_realtime_then_serialize(c: &mut Criterion) {
    let bytes = fs::read("example.json").expect("Não consegui ler example.json");
    let de = serde_json::from_slice(&bytes).expect("JSON inválido em example.json");
    let de2 = serde_json::from_slice(&bytes).expect("JSON inválido em example.json");

    let mut group = c.benchmark_group("realtime_then_serialize");
    group.sample_size(40);
    group.measurement_time(Duration::from_secs(5));

    group.bench_function(BenchmarkId::new("v2", "serialize"), |b| {
        b.iter_batched(
            || tutorlolv2_math::__v2::rt::realtime(black_box(&de)).expect("realtime v2 falhou"),
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

fn bench_parse_only(c: &mut Criterion) {
    let bytes = fs::read("example.json").expect("read example.json");
    let mut group = c.benchmark_group("parse_only");
    group.sample_size(40);
    group.measurement_time(Duration::from_secs(5));
    group.throughput(Throughput::Bytes(bytes.len() as u64));

    group.bench_function("serde_json_from_slice", |b| {
        b.iter(|| {
            let v: Value = serde_json::from_slice(black_box(&bytes)).unwrap();
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
    targets = bench_realtime_only, bench_realtime_then_serialize, bench_parse_only
);
criterion_main!(benches);
