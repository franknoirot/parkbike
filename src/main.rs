use geo::{ point, ClosestPoint, GeometryCollection };
use geojson::{ quick_collection, GeoJson };

fn main() {
    let data: &str = include_str!("./Bicycle Parking_20241004.geojson");
    let geojson: GeoJson = data.parse().unwrap();

    let collection: GeometryCollection<f64> = quick_collection(&geojson).unwrap();

    let location = point! {
        x: -73.91326042038499, y: 40.68723037854061 
    };

    let closest = collection.closest_point(&location);

    println!("{:?} is the closest point to {:?}", closest, location);
}
