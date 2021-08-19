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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

const INF: i64 = 1 << 60;

// Complexity: O(n log m + m) where n = r - l, m = b - a.
fn monotone_minima<F>(l: usize, r: usize, a: usize, b: usize,
                      frm: &[i64], lat: &mut [i64],
                      cost_fun: &F)
where F: Fn(usize, usize) -> i64 {
    let n = r - l;
    let m = b - a;
    if n == 0 || m == 0 {
        return;
    }
    let mid = (a + b) / 2;
    let mut mi = (INF, n);
    for i in l..r {
        let cost = cost_fun(i, mid);
        mi = std::cmp::min(mi, (cost + frm[i], i));
    }
    let idx = mi.1;
    assert!(l <= idx && idx < r);
    lat[mid] = std::cmp::min(lat[mid], mi.0);
    monotone_minima(l, idx + 1, a, mid, frm, lat, cost_fun);
    monotone_minima(idx, r, mid + 1, b, frm, lat, cost_fun);
}

// Tags: monotone-minima
fn main() {
    input! {
        n: usize,
        h: [i64; n],
    }
    let mut lft = vec![(h[0], 0)];
    for i in 1..n {
        if lft[lft.len() - 1].0 < h[i] {
            lft.push((h[i], i));
        }
    }
    let mut rgt = vec![(h[n - 1], n - 1)];
    for i in (0..n - 1).rev() {
        if rgt[rgt.len() - 1].0 < h[i] {
            rgt.push((h[i], i));
        }
    }
    rgt.reverse();
    let a = lft.len();
    let b = rgt.len();
    let frm = vec![0; a];
    let mut lat = vec![0; b];
    monotone_minima(0, a, 0, b, &frm, &mut lat, &|i: usize, j: usize| {
        let (x, y) = lft[i];
        let (z, w) = rgt[j];
        if y >= w {
            return INF / 2;
        }
        -(z + x) * (w - y) as i64
    });
    let ma = lat.iter().min().unwrap();
    println!("{}", -ma);
}
