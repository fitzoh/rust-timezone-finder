use geo::contains::Contains;
use geo_types::{MultiPolygon, Point};
use shapefile::dbase::{FieldValue, Record};
use shapefile::Polygon;

struct Timezone {
    id: String,
    shape: MultiPolygon<f64>,
}

impl Timezone {
    fn from_shape_record_tuple((shape, record): (Polygon, Record)) -> Timezone {
        fn tz_id_from_record(record: Record) -> String {
            match record.get("tzid").unwrap() {
                FieldValue::Character(Some(tzid)) => tzid.clone(),
                _ => panic!("tzid not found"),
            }
        }
        Timezone {
            id: tz_id_from_record(record),
            shape: shape.into(),
        }
    }
}

pub struct TimezoneFinder {
    timezones: Vec<Timezone>,
}

impl TimezoneFinder {
    pub fn new() -> TimezoneFinder {
        TimezoneFinder::from_path("tzdata/combined-shapefile.shp".into())
    }

    pub fn from_path(path: String) -> TimezoneFinder {
        let reader = shapefile::Reader::from_path(path).unwrap();
        let iter = reader.iter_shapes_and_records_as::<Polygon>().unwrap();
        let timezones: Vec<Timezone> = iter
            .map(|tuple| Timezone::from_shape_record_tuple(tuple.unwrap()))
            .collect();

        TimezoneFinder { timezones }
    }

    pub fn find(&self, lon: f64, lat: f64) -> Option<String> {
        let point: Point<f64> = (lon, lat).into();
        for tz in self.timezones.iter() {
            if tz.shape.contains(&point) {
                return Some(tz.id.clone());
            }
        }
        return None;
    }
}
