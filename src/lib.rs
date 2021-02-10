use geo::bounding_rect::BoundingRect;
use geo::contains::Contains;
use geo_types::{MultiPolygon, Point};
use shapefile::dbase::{FieldValue, Record};
use shapefile::Polygon;
use std::rc::Rc;

struct Timezone {
    // The timezone ID ("America/New_York")
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

pub trait TimezoneFinder {
    // Look up a a timezone ID ("America/New_York") from a lon/lat coordinate.
    fn find(&self, lon: f64, lat: f64) -> Option<String>;
}

pub struct SimpleTimezoneFinder {
    timezones: Vec<Timezone>,
}

impl SimpleTimezoneFinder {
    pub fn new() -> SimpleTimezoneFinder {
        SimpleTimezoneFinder::from_path("tzdata/combined-shapefile.shp".into())
    }

    pub fn from_path(path: String) -> SimpleTimezoneFinder {
        let reader = shapefile::Reader::from_path(path).unwrap();
        let iter = reader.iter_shapes_and_records_as::<Polygon>().unwrap();
        let timezones: Vec<Timezone> = iter
            .map(|tuple| Timezone::from_shape_record_tuple(tuple.unwrap()))
            .collect();

        SimpleTimezoneFinder { timezones }
    }
}

impl TimezoneFinder for SimpleTimezoneFinder {
    fn find(&self, lon: f64, lat: f64) -> Option<String> {
        let point: Point<f64> = (lon, lat).into();
        for tz in self.timezones.iter() {
            if tz.shape.contains(&point) {
                return Some(tz.id.clone());
            }
        }
        return None;
    }
}

pub struct BucketedTimezoneFinder {
    timezones: Vec<Vec<Bucket>>,
}

type Bucket = Vec<Rc<Timezone>>;

impl BucketedTimezoneFinder {
    const LON_BUCKETS: usize = 361;
    const LAT_BUCKETS: usize = 181;

    fn bucket(lon: f64, lat: f64) -> (usize, usize) {
        let normalized_lon = (lon + 180.0) as usize;
        let normalized_lat = (lat + 90.0) as usize;
        (normalized_lon, normalized_lat)
    }

    pub fn new() -> BucketedTimezoneFinder {
        BucketedTimezoneFinder::from_path("tzdata/combined-shapefile.shp".into())
    }

    pub fn from_path(path: String) -> BucketedTimezoneFinder {
        let mut buckets: Vec<Vec<Bucket>> = Vec::new();
        for _ in 0..BucketedTimezoneFinder::LON_BUCKETS{
            let mut bucket = Vec::new();
            for _ in 0..BucketedTimezoneFinder::LAT_BUCKETS{
                 bucket.push(Vec::new())
            }
            buckets.push(bucket);
        }
        let reader = shapefile::Reader::from_path(path).unwrap();
        let iter = reader.iter_shapes_and_records_as::<Polygon>().unwrap();

        let timezones: Vec<Rc<Timezone>> = iter
            .map(|tuple| Rc::new(Timezone::from_shape_record_tuple(tuple.unwrap())))
            .collect();

        for timezone in timezones {
            let bbox = timezone.shape.bounding_rect().unwrap();
            let (min_lon, min_lat) = BucketedTimezoneFinder::bucket(bbox.min().x, bbox.min().y);
            let (max_lon, max_lat) = BucketedTimezoneFinder::bucket(bbox.max().x, bbox.max().y);
            for lat in min_lon..=max_lon {
                for lon in min_lat..=max_lat{
                    buckets.get_mut(lat).unwrap().get_mut(lon).unwrap().push(timezone.clone())

                }
            }
        }

        BucketedTimezoneFinder { timezones: buckets }
    }
}

impl TimezoneFinder for BucketedTimezoneFinder {
    fn find(&self, lon: f64, lat: f64) -> Option<String> {
        let point: Point<f64> = (lon, lat).into();
        let (lon_bucket, lat_bucket) = BucketedTimezoneFinder::bucket(lon, lat);
        for tz in self.timezones[lon_bucket][lat_bucket].iter() {
            if tz.shape.contains(&point) {
                return Some(tz.id.clone());
            }
        }
        return None;
    }
}
