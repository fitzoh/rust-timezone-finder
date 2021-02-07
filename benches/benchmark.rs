#[path = "../src/lib.rs"]
mod timezone_finder;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

struct Input {
    location: String,
    lon: f64,
    lat: f64,
    tz: String,
}

fn criterion_benchmark(c: &mut Criterion) {
    let tz = timezone_finder::TimezoneFinder::new();

    let inputs = vec![
        Input {
            location: "New York".into(),
            lon: -74.0060,
            lat: 40.7128,
            tz: "America/New_York".into(),
        },
        Input {
            location: "Rio de Janerio".to_string(),
            lon: -43.1729,
            lat: -22.9068,
            tz: "America/Sao_Paulo".to_string(),
        },
        Input {
            location: "Beijing".into(),
            lon: 116.4074,
            lat: 39.9042,
            tz: "Asia/Shanghai".into(),
        },
        Input {
            location: "Sidney".to_string(),
            lon: 151.2093,
            lat: -33.8688,
            tz: "Australia/Sydney".to_string(),
        },
    ];
    let mut group = c.benchmark_group("lookups");
    for item in inputs {
        assert_eq!(tz.find(item.lon, item.lat).unwrap(), item.tz);
        group.bench_with_input(
            BenchmarkId::from_parameter(item.location.clone()),
            &item,
            |b, item| b.iter(|| tz.find(item.lon, item.lat)),
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
