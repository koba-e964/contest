use std::cmp::*;
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes.by_ref().map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Bisect<T> {
    fn lower_bound(&self, val: &T) -> usize;
    fn upper_bound(&self, val: &T) -> usize;
}

impl<T: Ord> Bisect<T> for [T] {
    fn lower_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] >= val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
    fn upper_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] > val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
}

// Tags: implementation, perimeter
fn main() {
    input! {
        n: usize, t: i64,
        be: [((i64, i64), (i64, i64)); n],
    }
    let mut vert = vec![false; n];
    for i in 0..n {
        vert[i] = (be[i].0).0 == (be[i].1).0;
    }
    let swap = |(x, y)| if x < y {
        (x, y)
    } else {
        (y, x)
    };
    let mut cont = vec![(0, (0, 0)); n];
    for i in 0..n {
        let xlr = if vert[i] {
            ((be[i].0).0, swap(((be[i].0).1, (be[i].1).1)))
        } else {
            ((be[i].0).1, swap(((be[i].0).0, (be[i].1).0)))
        };
        cont[i] = xlr;
    }
    let mut ints = vec![vec![]; n];
    for i in 0..n {
        let (xi, (li, ri)) = cont[i];
        for j in 0..n {
            if vert[i] == vert[j] {
                continue;
            }
            let (xj, (lj, rj)) = cont[j];
            if xj < li || ri < xj || xi < lj || rj < xi {
                continue;
            }
            ints[i].push((xj, j));
        }
        ints[i].push((li, i));
        ints[i].push((ri, i));
    }
    for i in 0..n {
        ints[i].sort_unstable();
    }
    // eprintln!("{:?}", ints);
    let mut path = vec![];
    let (mut x, mut y) = be[0].0;
    let mut dir = 1;
    let mut idx = 0;
    if vert[0] {
        if y > (be[0].1).1 {
            dir = -1;
        }
        std::mem::swap(&mut x, &mut y);
    } else {
        if x > (be[0].1).0 {
            dir = -1;
        }
    }
    let init = (x, y);
    loop {
        let to = if dir == 1 {
            ints[idx].lower_bound(&(x + 1, 0))
        } else {
            ints[idx].lower_bound(&(x, 0)) - 1
        };
        let (tov, toidx) = ints[idx][to];
        // eprintln!("x = {}, y = {}, {} ({}) => {} ({})", x, y, idx, x, toidx, tov);
        // transition
        let oldidx = idx;
        idx = toidx;
        path.push(((tov - x).abs(), (x, y), tov, oldidx));
        if idx == oldidx {
            // bounced
            dir = -dir;
            x = tov;
        } else {
            x = y;
            y = tov;
            if vert[oldidx] {
                dir = -dir;
            }
        }
        // break?
        if (x, y) == init && idx == 0 {
            break;
        }
    }
    let m = path.len();
    let mut acc = vec![0; m + 1];
    for i in 0..m {
        acc[i + 1] = acc[i] + path[i].0;
    }
    let r = t % acc[m];
    let ii = acc.lower_bound(&r);
    let (_, (x, y), z, idx) = path[max(ii, 1) - 1];
    let mut diff = r - acc[max(ii, 1) - 1];
    if x > z {
        diff = -diff;
    }
    eprintln!("r = {}, ii = {}, diff = {}, x = {}, y = {}", r, ii, diff, x, y);
    let (mut a, mut b) = (x + diff, y);
    if vert[idx] {
        std::mem::swap(&mut a, &mut b);
    }
    println!("{} {}", a, b);
}
