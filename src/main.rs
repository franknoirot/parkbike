use core::fmt;
use std::fmt::{Display, Formatter};

use clap::Parser;
use geo::{
    point, Closest::SinglePoint, ClosestPoint, GeodesicBearing, GeodesicDistance,
    GeometryCollection,
};
use geojson::{quick_collection, GeoJson};

/// A simple program to get the nearest NYC bike parking location
/// to a given location.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The latitude of the location
    #[clap(short = 'y', long, alias = "lat")]
    latitude: f64,

    /// The longitude of the location
    #[clap(short = 'x', long, alias = "lon")]
    longitude: f64,
}

fn main() {
    let args = Args::parse();
    let location = point!(x: args.longitude, y: args.latitude);
    let data: &str = include_str!("./Bicycle Parking_20241004.geojson");
    let geojson: GeoJson = data.parse().unwrap();

    let collection: GeometryCollection<f64> = quick_collection(&geojson).unwrap();

    let closest = collection.closest_point(&location);

    let point = match closest {
        SinglePoint(point) => point,
        _ => panic!("Expected a single point"),
    };

    let distance = point.geodesic_distance(&location);
    let bearing = location.geodesic_bearing(point);
    let direction = Direction::from(bearing);

    println!(
        "🚲🔒 The closest bicycle parking is {:.2} meters away {} ({:.2} degrees)",
        distance, direction, bearing
    );
    println!(
        "(Because {:?} is the closest point to {:?})",
        point, location
    );
}

enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl From<f64> for Direction {
    fn from(bearing: f64) -> Self {
        match bearing % 360.0 {
            0.0..=22.5 => Direction::North,
            22.5..=67.5 => Direction::NorthEast,
            67.5..=112.5 => Direction::East,
            112.5..=157.5 => Direction::SouthEast,
            157.5..=202.5 => Direction::South,
            202.5..=247.5 => Direction::SouthWest,
            247.5..=292.5 => Direction::West,
            292.5..=337.5 => Direction::NorthWest,
            _ => Direction::North,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::North => "N",
                Direction::NorthEast => "NE",
                Direction::East => "E",
                Direction::SouthEast => "SE",
                Direction::South => "S",
                Direction::SouthWest => "SW",
                Direction::West => "W",
                Direction::NorthWest => "NW",
            }
        )
    }
}
