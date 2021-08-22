use std::cmp::*;
use std::collections::*;
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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn main() {
    input! {
        n: usize,
        ab: [(usize1, usize1); n - 1],
    }
    let mut g = vec![0; n];
    for &(a, b) in &ab {
        g[a] += 1;
        g[b] += 1;
    }
    let mut f = vec![0; n - 1];
    for i in 0..n {
        f[g[i] - 1] += 1;
    }
    // eprintln!("f = {:?}", f);
    // TODO super duper slow
    let mut lo = vec![n as i64; n];
    let mut hi = vec![-(n as i64); n];
    let mut sz = 1;
    lo[0] = 0;
    for d in 0..n - 1 {
        if f[d] == 0 {
            continue;
        }
        let mut nlo = vec![n as i64; n];
        let mut nhi = vec![-(n as i64); n];
        if d == 0 {
            nlo[0].chmin(0);
            nhi[0].chmax(f[0] as i64);
        } else {
            let mut stl = vec![VecDeque::<(i64, usize)>::new(); d];
            let mut sth = vec![VecDeque::<(i64, usize)>::new(); d];
            for i in 0..sz + d * f[d] {
                lo[i] -= (i / d) as i64;
                hi[i] -= (i / d) as i64;
            }
            for j in 0..sz + d * f[d] {
                let r = j % d;
                if let Some(&(_, idx)) = stl[r].front() {
                    if idx + (f[d] + 1) * d == j {
                        stl[r].pop_front();
                    }
                }
                if let Some(&(_, idx)) = sth[r].front() {
                    if idx + (f[d] + 1) * d == j {
                        sth[r].pop_front();
                    }
                }
                while let Some((v, idx)) = stl[r].pop_back() {
                    if v >= lo[j] {
                        continue;
                    }
                    stl[r].push_back((v, idx));
                    break;
                }
                while let Some((v, idx)) = sth[r].pop_back() {
                    if v <= hi[j] {
                        continue;
                    }
                    sth[r].push_back((v, idx));
                    break;
                }
                stl[r].push_back((lo[j], j));
                sth[r].push_back((hi[j], j));
                if false {
                    eprintln!("stl = {:?}", stl);
                    eprintln!("sth = {:?}", sth);
                }
                let &(v, _) = stl[r].front().unwrap();
                nlo[j] = v + (j / d) as i64;
                let &(v, _) = sth[r].front().unwrap();
                nhi[j] = v + (j / d) as i64;
            }
            sz += d * f[d];
        }
        lo = nlo;
        hi = nhi;
    }
    let mut tot = 0i64;
    for j in 0..n {
        if lo[j] <= hi[j] {
            tot += (hi[j] - lo[j] + 1) as i64;
        }
    }
    println!("{}", tot);
}
