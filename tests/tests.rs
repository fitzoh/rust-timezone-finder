use timezonefinder;

struct Input {
    location: String,
    lon: f64,
    lat: f64,
    tz: String,
}

#[test]
fn it_works() {
    let tz = timezonefinder::TimezoneFinder::new();

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
    for item in inputs {
        assert_eq!(
            tz.find(item.lon, item.lat),
            Some(item.tz),
            "wrong time zone for {}",
            item.location
        );
    }
}
