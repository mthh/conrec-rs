extern crate conrec;
use conrec::*;

fn main() {
    let data = vec![vec![0., 1., 0.], vec![1., 2., 1.], vec![0., 1., 0.]];
    let levels = vec![0., 1., 2.];
    let res = conrec(data,
                     0,
                     2,
                     0,
                     2,
                     &vec![1., 2., 3.],
                     &vec![1., 2., 3.],
                     &levels);
    println!("{:?}", res);
}
