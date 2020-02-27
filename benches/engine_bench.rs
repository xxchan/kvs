use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rand::prelude::*;
use tempfile::TempDir;

use kvs::{KvStore, KvsEngine, SledKvsEngine};

pub fn write_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Write Group");

    let store_size = 1 << 20;

    group.bench_function("kvs_write", |b| {
        b.iter_batched(
            || {
                let temp_dir = TempDir::new().unwrap();
                KvStore::open(temp_dir.path()).unwrap()
            },
            |mut engine| {
                for i in 1..store_size {
                    engine
                        .set(format!("key{}", i), format!("value{}", i))
                        .unwrap();
                }
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function("sled_write", |b| {
        b.iter_batched(
            || {
                let temp_dir = TempDir::new().unwrap();
                SledKvsEngine::open(temp_dir).unwrap()
            },
            |mut engine| {
                for i in 1..store_size {
                    engine
                        .set(format!("key{}", i), format!("value{}", i))
                        .unwrap();
                }
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

pub fn read_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Read Group");

    let store_size = 1 << 20;

    group.bench_function("kvs_read", |b| {
        let temp_dir = TempDir::new().unwrap();
        let mut engine = KvStore::open(temp_dir.path()).unwrap();
        for i in 1..store_size {
            engine
                .set(format!("key{}", i), format!("value{}", i))
                .unwrap();
        }
        let mut rng = thread_rng();
        b.iter(|| {
            engine
                .get(format!("key{}", rng.gen_range(1, store_size)))
                .unwrap()
        })
    });
    group.bench_function("sled_read", |b| {
        let temp_dir = TempDir::new().unwrap();
        let mut engine = SledKvsEngine::open(temp_dir).unwrap();
        for i in 1..store_size {
            engine
                .set(format!("key{}", i), format!("value{}", i))
                .unwrap();
        }
        let mut rng = thread_rng();
        b.iter(|| {
            engine
                .get(format!("key{}", rng.gen_range(1, store_size)))
                .unwrap()
        })
    });

    group.finish();
}

criterion_group!(benches, read_bench, write_bench);
criterion_main!(benches);
