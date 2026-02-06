use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum CoordinateType {
  GEO,
  XY,
}

pub fn get_distance(
  coordinate_type: CoordinateType,
  origin: (f64, f64),
  destination: (f64, f64),
) -> Result<f64, Box<dyn Error>> {
  if coordinate_type == CoordinateType::XY {
    Ok(get_euclidian_distance(origin, destination))
  } else if coordinate_type == CoordinateType::GEO {
    Ok(get_geo_distance(origin, destination))
  } else {
    Err("Unresolved coordinate type".into())
  }
}

fn get_geo_distance(origin: (f64, f64), destination: (f64, f64)) -> f64 {
  let r = 6371.0; // Earth's radius in km

  let lat1 = origin.0.to_radians();
  let lat2 = destination.0.to_radians();
  let delta_lat = (destination.0 - origin.0).to_radians();
  let delta_lon = (destination.1 - origin.1).to_radians();

  let a =
    (delta_lat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (delta_lon / 2.0).sin().powi(2);
  let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

  r * c * 100.00
}

fn get_euclidian_distance(origin: (f64, f64), destination: (f64, f64)) -> f64 {
  let dx = destination.0 - origin.0;
  let dy = destination.1 - origin.1;
  (dx.powi(2) + dy.powi(2)).sqrt()
}
