extern crate conrec;
use conrec::*;

fn main() {
    let (data, ys, xs, min_val, max_val) =
        parse_json_points("examples/ra_gaussian_output.json", (160, 80)).unwrap();
    let levels = vec![-1., min_val, 2500., 5000., 7500., 10000., 12000., 13400.0, max_val];
    let res = conrec(data, 0, 159, 0, 79, &xs, &ys, &levels).unwrap();
    save_json_points("ContourResult.json", res).unwrap();
}
