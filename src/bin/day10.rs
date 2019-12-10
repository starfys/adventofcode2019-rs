use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, BufRead};

#[derive(Clone, Copy)]
struct Point2<T> {
    x: T,
    y: T,
}
impl<T> Point2<T> {
    fn new(x: T, y: T) -> Point2<T> {
        Point2 { x, y }
    }
}
#[derive(Clone, Copy)]
struct Line2<T> {
    a: Point2<T>,
    b: Point2<T>,
}

fn los(grid: &[Vec<char>], x1: isize, y1: isize, x2: isize, y2: isize) -> bool {
    let start = Point2::new(x1 as isize, y1 as isize);
    let end = Point2::new(x2 as isize, y2 as isize);

    for x in 0..grid[0].len() {
        for y in 0..grid.len() {
            if (x as isize == x1 && y as isize == y1) || (x as isize == x2 && y as isize == y2) {
                continue;
            }
            if grid[y as usize][x as usize] == '#' {
                let x = x as isize;
                let y = y as isize;
                if is_between(start, end, Point2::new(x, y)) {
                    return false;
                }
            }
        }
    }
    return true;
}

fn num_asteroids(grid: &[Vec<char>], x: isize, y: isize) -> isize {
    if grid[y as usize][x as usize] == '#' {
        let mut num = 0;
        for cy in 0..grid.len() {
            for cx in 0..grid[0].len() {
                if cx == x as usize && cy == y as usize {
                    continue;
                }
                if grid[cy][cx] == '#' {
                    if los(grid, x, y, cx as isize, cy as isize) {
                        // determine los
                        num += 1;
                    }
                }
            }
        }
        num
    } else {
        0
    }
}

fn main() {
    let stdin = io::stdin();
    let mut grid: Vec<_> = stdin
        .lock()
        .lines()
        .flatten()
        .map(|line| line.chars().collect::<Vec<_>>())
        //.map(|l| l.parse())
        //.flatten()
        .collect();
    assert!(is_between(
        Point2::new(3, 4),
        Point2::new(1, 0),
        Point2::new(2, 2)
    ));
    assert!(is_between(
        Point2::new(0, 0),
        Point2::new(0, 2),
        Point2::new(0, 1)
    ));
    assert!(is_between(
        Point2::new(0, 0),
        Point2::new(2, 0),
        Point2::new(1, 0)
    ));
    assert!(!los(&grid, 2, 2, 0, 2));
    let mut m = 0;
    let mut mx = 0;
    let mut my = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '#' {
                let n = num_asteroids(&grid, x as isize, y as isize);
                if n > m {
                    m = n;
                    mx = x;
                    my = y;
                }
            }
        }
    }
    println!("{} {} {}", mx, my, m);
    println!();
    let myy = my as f64;
    let mxx = mx as f64;
    use std::ops::Mul;
    let mut last_angle = -0.0001; //std::f64::consts::PI * 2.0 + 0.000000001;
    let mut it = 1;
    loop {
        let res = (0..grid.len())
            // All x,y pairs
            .flat_map(|y| (0..grid[y].len()).map(|x| (y, x)).collect::<Vec<_>>())
            // All meteors that are not ourself
            .filter(|(y, x)| !(*y == my && *x == mx) && grid[*y][*x] == '#')
            .map(|(y, x)| {
                (
                    normalize((y as f64 - myy).atan2(x as f64 - mxx) - 1.5 * std::f64::consts::PI),
                    ((y - my) * (y - my) + (x - mx) * (x - mx)),
                    y,
                    x,
                )
            })
            .filter(|(angle, _, _, _)| *angle > last_angle)
            .min_by(|(a, b, _, _), (a1, b1, _, _)| {
                use std::cmp::Ordering::*;
                match a.partial_cmp(a1).unwrap_or(Equal) {
                    Less => Less,
                    Equal => b.cmp(b1),
                    Greater => Greater
                }
            });
        if let Some((angle, dist, y, x)) = res {
            println!("{}:     {} {} {} {}", it, angle, dist, x, y);
            last_angle = angle;
            grid[y][x] = '.';
            it += 1;
            if it == 201 {
                break;
            }
        }
        else {
            last_angle = -0.00001;//std::f64::consts::PI * 2.0 + 0.000000001;
        }
    }
}
fn normalize(mut angle: f64) -> f64 {
    while angle < 0.0 {
        angle += std::f64::consts::PI * 2.0;
    }
    while angle >= 2.0 * std::f64::consts::PI * 2.0 {
        angle -= std::f64::consts::PI * 2.0;
    }
    angle
}

// Given three colinear points p, q, r, the function checks if
// point q lies on line segment 'pr'
fn on_segment(p: Point2<isize>, q: Point2<isize>, r: Point2<isize>) -> bool {
    q.x <= isize::max(p.x, r.x)
        && q.x >= isize::min(p.x, r.x)
        && q.y <= isize::max(p.y, r.y)
        && q.y >= isize::min(p.y, r.y)
}
fn is_between(start: Point2<isize>, end: Point2<isize>, point: Point2<isize>) -> bool {
    let crossproduct = (point.y as f64 - start.y as f64) * (end.x as f64 - start.x as f64)
        - (point.x as f64 - start.x as f64) * (end.y as f64 - start.y as f64);


    if crossproduct.abs() > 0.00001 {
        return false;
    }

    let dotproduct = (point.x as f64 - start.x as f64) * (end.x as f64 - start.x as f64)
        + (point.y as f64 - start.y as f64) * (end.y as f64 - start.y as f64);
    if dotproduct < 0.0 {
        return false;
    }

    let squaredlengthba = (end.x as f64 - start.x as f64) * (end.x as f64 - start.x as f64)
        + (end.y as f64 - start.y as f64) * (end.y as f64 - start.y as f64);
    if dotproduct > squaredlengthba {
        return false;
    }

    return true;
}
// To find orientation of ordered triplet (p, q, r).
// The function returns following values
// 0 --> p, q and r are colinear
// 1 --> Clockwise
// 2 --> Counterclockwise
#[derive(PartialEq)]
enum Orientation {
    Colinear,
    Clockwise,
    CounterClockwise,
}
fn orientation(p: Point2<isize>, q: Point2<isize>, r: Point2<isize>) -> Orientation {
    // See https://www.geeksforgeeks.org/orientation-3-ordered-points/
    // for details of below formula.
    let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);

    if val == 0 {
        Orientation::Colinear
    } else if val > 0 {
        Orientation::Clockwise
    } else if val < 0 {
        Orientation::CounterClockwise
    } else {
        unreachable!()
    }
}

// The main function that returns true if line segment 'p1q1'
// and 'p2q2' intersect.
fn do_intersect(
    p1: Point2<isize>,
    q1: Point2<isize>,
    p2: Point2<isize>,
    q2: Point2<isize>,
) -> bool {
    // Find the four orientations needed for general and
    // special cases
    let o1 = orientation(p1, q1, p2);
    let o2 = orientation(p1, q1, q2);
    let o3 = orientation(p2, q2, p1);
    let o4 = orientation(p2, q2, q1);

    // General case
    if o1 != o2 && o3 != o4 {
        return true;
    }

    // Special Cases
    // p1, q1 and p2 are colinear and p2 lies on segment p1q1
    if o1 == Orientation::Colinear && on_segment(p1, p2, q1) {
        return true;
    }

    // p1, q1 and q2 are colinear and q2 lies on segment p1q1
    if o2 == Orientation::Colinear && on_segment(p1, q2, q1) {
        return true;
    }

    // p2, q2 and p1 are colinear and p1 lies on segment p2q2
    if o3 == Orientation::Colinear && on_segment(p2, p1, q2) {
        return true;
    }

    // p2, q2 and q1 are colinear and q1 lies on segment p2q2
    if o4 == Orientation::Colinear && on_segment(p2, q1, q2) {
        return true;
    }

    return false; // Doesn't fall in any of the above cases
}
