use rand::Rng;
use timezonefinder;
use timezonefinder::TimezoneFinder;

struct Input {
    location: String,
    lon: f64,
    lat: f64,
    tz: String,
}

fn inputs() -> Vec<Input> {
    vec![
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
        Input {
            // Discovered via fuzz testing, time zone has a very small bounding box and only shows up in one bucket.
            location: "Gaza".to_string(),
            lon: 34.28092229445389,
            lat: 31.665431986413495,
            tz: "Asia/Gaza".to_string(),
        },
    ]
}

#[test]
fn simple_tz() {
    let tz = timezonefinder::SimpleTimezoneFinder::new();

    for item in inputs() {
        assert_eq!(
            tz.find(item.lon, item.lat),
            Some(item.tz),
            "wrong time zone for {}",
            item.location
        );
    }
}

#[test]
fn bucketed_tz() {
    let tz = timezonefinder::BucketedTimezoneFinder::new();

    for item in inputs() {
        assert_eq!(
            tz.find(item.lon, item.lat),
            Some(item.tz),
            "wrong time zone for {}",
            item.location
        );
    }
}

// Fuzz test for the (fast/complicated) BucketedTimezoneFinder that ensures it has the same results as the (slow/simple) reference implementation.
// If you want to run this, remove the #[ignore] directive and make sure to test in --release mode.
#[test]
#[ignore]
fn fuzz_bucketed() {
    let simple = timezonefinder::SimpleTimezoneFinder::new();
    let bucketed = timezonefinder::BucketedTimezoneFinder::new();
    let mut rng = rand::thread_rng();
    let mut i: i64 = 0;

    loop {
        let lon = rng.gen_range(-180.0..180.0);
        let lat = rng.gen_range(-90.0..90.0);
        assert_eq!(
            simple.find(lon, lat),
            bucketed.find(lon, lat),
            "wrong time zone for ({}, {}) ({} tests complete)",
            lon,
            lat,
            i
        );
        i += 1;
        if i % 1000 == 0 {
            println!("executed {} fuzz tests", i)
        }
    }
}
