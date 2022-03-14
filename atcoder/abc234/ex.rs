use std::io::{Write, BufWriter};
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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

struct Rng {
    x: u64,
}

impl Rng {
    fn new() -> Self {
        use std::hash::{Hasher, BuildHasher};
        let hm = std::collections::HashMap::<i32, i32>::new();
        let mut hash = hm.hasher().build_hasher();
        hash.write_u32(8128);
        Rng {
            x: hash.finish(),
        }
    }
    fn next(&mut self) -> u64 {
        let a = 0xdead_c0de_0013_3331u64;
        let b = 2457;
        self.x = self.x.wrapping_mul(a).wrapping_add(b);
        let x = self.x;
        x ^ x << 10
    }
}

fn rot((x, y): (f64, f64), theta: f64) -> (f64, f64) {
    let c = theta.cos();
    let s = theta.sin();
    (x * c - y * s, x * s + y * c)
}

trait Bisect<T> {
    fn lower_bound(&self, val: &T) -> usize;
    fn upper_bound(&self, val: &T) -> usize;
}

impl<T: PartialOrd> Bisect<T> for [T] {
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

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, k: i64,
        xy: [(i64, i64); n],
    }
    let mut rng = Rng::new();
    const INF: f64 = 1.0e18;
    const LIM: i64 = 1 << 29; // ~= (n + m)^1.5
    let mut ans = vec![];
    loop {
        let mut pq = vec![(0.0, 0.0, 0); n];
        let s = (rng.next() >> 32 & 0xffff) as f64;
        let theta = s * std::f64::consts::PI / 32768.0;
        for i in 0..n {
            let (x, y) = xy[i];
            let (p, q) = rot((x as _, y as _), theta);
            pq[i] = (p, q, i);
        }
        let mut v = pq.clone();
        v.sort_by(|&p1, &p2| p1.partial_cmp(&p2).unwrap());
        let mut sum = 0;
        for i in 0..n {
            let (p, _q, _) = pq[i];
            let lo = v.lower_bound(&(p - k as f64 - 0.1, -INF, 0));
            let hi = v.upper_bound(&(p + k as f64 + 0.1, INF, 0));
            sum += (hi - lo) as i64;
            if sum > LIM { break; }
        }
        if sum > LIM { continue; }
        for i in 0..n {
            let (p, _, _) = pq[i];
            let lo = v.lower_bound(&(p - k as f64 - 0.1, -INF, 0));
            let hi = v.upper_bound(&(p + k as f64 + 0.1, INF, 0));
            for j in lo..hi {
                let (_, _, idx) = v[j];
                if i >= idx { continue; }
                let (p, q) = xy[i];
                let (x, y) = xy[idx];
                if (x - p) * (x - p) + (y - q) * (y - q) <= k * k {
                    ans.push((i, idx));
                }
            }
        }
        break;
    }
    ans.sort();
    puts!("{}\n", ans.len());
    for (x, y) in ans {
        puts!("{} {}\n", x + 1, y + 1);
    }
}
