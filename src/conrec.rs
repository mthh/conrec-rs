use errors::*;
use {Point, Segment};

#[inline(always)]
fn sect(h: [f64; 5], p: [f64; 5], v1: usize, v2: usize) -> f64 {
    (h[v2] * p[v1] - h[v1] * p[v2]) / (h[v2] - h[v1])
}

pub fn conrec(d: Vec<Vec<f64>>,
              ilb: usize,
              iub: usize,
              jlb: usize,
              jub: usize,
              x: &Vec<f64>,
              y: &Vec<f64>,
              z: &Vec<f64>)
              -> Result<Vec<Segment>> {
    let mut levels = z.clone();
    levels.sort_by(|a, b| a.partial_cmp(&b).unwrap());
    let (mut h, mut xh, mut yh, mut sh): ([f64; 5], [f64; 5], [f64; 5], [i32; 5]) =
        ([0.0, 0.0, 0.0, 0.0, 0.0],
         [0.0, 0.0, 0.0, 0.0, 0.0],
         [0.0, 0.0, 0.0, 0.0, 0.0],
         [0, 0, 0, 0, 0]);
    let im: [usize; 4] = [0, 0, 1, 1];
    let jm: [usize; 4] = [0, 1, 1, 0];
    let n_contour = levels.len();
    let cases = vec![vec![vec![0, 0, 8], vec![0, 2, 5], vec![7, 6, 9]],
                     vec![vec![0, 3, 4], vec![1, 3, 1], vec![4, 3, 0]],
                     vec![vec![9, 6, 7], vec![5, 2, 0], vec![8, 0, 0]]];
    let mut res = Vec::new();
    for j in (jlb..jub).rev() {
        for i in ilb..iub {
            let dmin = (d[i][j].min(d[i][j + 1])).min(d[i + 1][j].min(d[i + 1][j + 1]));
            let dmax = (d[i][j].max(d[i][j + 1])).max(d[i + 1][j].max(d[i + 1][j + 1]));
            if dmax < levels[0] || *levels.last().unwrap() < dmin {
                continue;
            }
            for k in 0..n_contour {
                if levels[k] < dmin || levels[k] > dmax {
                    continue;
                }
                for m in (0..5).rev() {
                    if m > 0 {
                        h[m] = d[i + im[m - 1]][j + jm[m - 1]] - levels[k];
                        xh[m] = x[i + im[m - 1]];
                        yh[m] = y[j + jm[m - 1]];
                    } else {
                        h[0] = 0.25 * (h[1] + h[2] + h[3] + h[4]);
                        xh[0] = 0.50 * (x[i] + x[i + 1]);
                        yh[0] = 0.50 * (y[j] + y[j + 1]);
                    }
                    sh[m] = if h[m] > 0.0 {
                        1
                    } else if h[m] < 0.0 {
                        -1
                    } else {
                        0
                    };
                }
                /*
				   Note: at this stage the relative heights of the corners and the
				   centre are in the h array, and the corresponding coordinates are
				   in the xh and yh arrays. The centre of the box is indexed by 0
				   and the 4 corners by 1 to 4 as shown below.
				   Each triangle is then indexed by the parameter m, and the 3
				   vertices of each triangle are indexed by parameters m1,m2,and m3.
				   It is assumed that the centre of the box is always vertex 2
				   though this isimportant only when all 3 vertices lie exactly on
				   the same contour level, in which case only the side of the box
				   is drawn.

				      vertex 4 +-------------------+ vertex 3
				               | \               / |
				               |   \    m-3    /   |
				               |     \       /     |
				               |       \   /       |
				               |  m=2    X   m=2   |       the centre is vertex 0
				               |       /   \       |
				               |     /       \     |
				               |   /    m=1    \   |
				               | /               \ |
				      vertex 1 +-------------------+ vertex 2
				*/

                /* Scan each triangle in the box */
                for m in 1..5 {
                    let (m1, m2): (usize, usize) = (m, 0);
                    let m3: usize = if m != 4 { m + 1 } else { 1 };
                    let case_value = cases[(sh[m1] + 1) as usize][(sh[m2] + 1) as usize][(sh[m3] + 1) as
                    usize];
                    if case_value == 0 {
                        continue;
                    }
                    let (p1, p2): (Point, Point) = match case_value {
                        // Line between vertices 1 and 2
                        1 => {
                            (Point {
                                 x: xh[m1],
                                 y: yh[m1],
                             },
                             Point {
                                 x: xh[m2],
                                 y: yh[m2],
                             })
                        }
                        // Line between vertices 2 and 3
                        2 => {
                            (Point {
                                 x: xh[m2],
                                 y: yh[m2],
                             },
                             Point {
                                 x: xh[m3],
                                 y: yh[m3],
                             })
                        }
                        // Line between vertices 3 and 1
                        3 => {
                            (Point {
                                 x: xh[m3],
                                 y: yh[m3],
                             },
                             Point {
                                 x: xh[m1],
                                 y: yh[m1],
                             })
                        }
                        // Line between vertex 1 and side 2-3
                        4 => {
                            (Point {
                                 x: xh[m1],
                                 y: yh[m1],
                             },
                             Point {
                                 x: sect(h, xh, m2, m3),
                                 y: sect(h, yh, m2, m3),
                             })
                        }
                        // Line between vertex 2 and side 3-1
                        5 => {
                            (Point {
                                 x: xh[m2],
                                 y: yh[m2],
                             },
                             Point {
                                 x: sect(h, xh, m3, m1),
                                 y: sect(h, yh, m3, m1),
                             })
                        }
                        // Line between vertex 3 and side 1-2
                        6 => {
                            (Point {
                                 x: xh[m3],
                                 y: yh[m3],
                             },
                             Point {
                                 x: sect(h, xh, m1, m2),
                                 y: sect(h, yh, m1, m2),
                             })
                        }
                        // Line between sides 1-2 and 2-3
                        7 => {
                            (Point {
                                 x: sect(h, xh, m1, m2),
                                 y: sect(h, yh, m1, m2),
                             },
                             Point {
                                 x: sect(h, xh, m2, m3),
                                 y: sect(h, yh, m2, m3),
                             })
                        }
                        // Line between sides 2-3 and 3-1
                        8 => {
                            (Point {
                                 x: sect(h, xh, m2, m3),
                                 y: sect(h, yh, m2, m3),
                             },
                             Point {
                                 x: sect(h, xh, m3, m1),
                                 y: sect(h, yh, m3, m1),
                             })
                        }
                        // Line between sides 3-1 and 1-2
                        9 => {
                            (Point {
                                 x: sect(h, xh, m3, m1),
                                 y: sect(h, yh, m3, m1),
                             },
                             Point {
                                 x: sect(h, xh, m1, m2),
                                 y: sect(h, yh, m1, m2),
                             })
                        }
                        // Unexpected !
                        _ => panic!("Unexpected case"),
                    };
                    // drawContour(...)
                    res.push(Segment::new(p1, p2, levels[k]));
                }
            }

        }
    }
    Ok(res)
}
