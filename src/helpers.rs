extern crate serde;
use serde_json;
use errors::*;
use std::f64;
use std::fs::File;
use std::io::{Read, Write};
use Segment;


#[derive(Debug,Clone)]
#[derive(Serialize,Deserialize)]
pub struct PtValue {
    lat: f64,
    lon: f64,
    value: f64,
}

impl PtValue {
    #[inline(always)]
    pub fn new(lat: f64, lon: f64, value: f64) -> Self {
        PtValue {
            lat: lat,
            lon: lon,
            value: value,
        }
    }
    pub fn get_triplet(&self) -> (f64, f64, f64) {
        (self.lat, self.lon, self.value)
    }
}

#[derive(Serialize,Deserialize)]
pub struct ValuesJson {
    values: Vec<PtValue>,
}

pub fn save_json_points(path: &str, result_contour: Vec<Segment>) -> Result<()> {
    let encoded = serde_json::to_string(&result_contour)?;
    let mut file = File::create(path)?;
    file.write(encoded.as_bytes())?;
    Ok(())
}

pub fn parse_json_points(path: &str,
                         scale: (usize, usize))
                         -> Result<(Vec<Vec<f64>>, Vec<f64>, Vec<f64>, f64, f64)> {
    let mut file = File::open(path)?;
    let mut raw_json = String::new();
    file.read_to_string(&mut raw_json)?;
    let decoded: serde_json::Value = serde_json::from_str(&raw_json)?;
    let ref arr = if decoded.is_object() && !decoded.get("values").is_none() &&
                     decoded["values"].is_array() {
        decoded["values"].as_array().unwrap()
    } else if decoded.is_array() {
        decoded.as_array().unwrap()
    } else {
        return Err("Invalid datastructure".into());
    };
    let mut res = Vec::with_capacity(scale.0);
    let mut i: i32 = -1;
    let mut j = 0;
    let mut xs = Vec::with_capacity(arr.len());
    let mut ys = Vec::with_capacity(arr.len());
    let mut min = f64::MAX;
    let mut max = f64::MIN;
    for elem in arr.iter() {
        if j % scale.1 == 0 {
            res.push(Vec::new());
            i += 1;
        }
        let value = match elem["value"] {
            serde_json::Value::Number(ref val) => val.as_f64().unwrap(),
            serde_json::Value::String(ref val) => val.to_string().parse::<f64>()?,
            _ => return Err("Invalid datastructure".into()),
        };
        let lat = match elem["lat"] {
            serde_json::Value::Number(ref val) => val.as_f64().unwrap(),
            serde_json::Value::String(ref val) => val.to_string().parse::<f64>()?,
            _ => return Err("Invalid datastructure".into()),
        };
        let lon = match elem["lon"] {
            serde_json::Value::Number(ref val) => val.as_f64().unwrap(),
            serde_json::Value::String(ref val) => val.to_string().parse::<f64>()?,
            _ => return Err("Invalid datastructure".into()),
        };

        res[i as usize].push(value);
        if value < min {
            min = value;
        }
        if value > max {
            max = value;
        }
        xs.push(lon);
        ys.push(lat);
        j += 1;
    }
    xs.dedup();
    ys.dedup();
    Ok((res, xs, ys, min, max))
}
