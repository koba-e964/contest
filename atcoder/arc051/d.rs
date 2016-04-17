#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::*;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}
fn get_word() -> String {
    let mut stdin = std::io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.is_err() || u8b[0] <= ' ' as u8 {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = std::string::String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}
fn parse<T: std::str::FromStr>(s: &str) -> T { s.parse::<T>().ok().unwrap() }

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { parse(&get_word()) }


/*
 * Manages multiple linear graphs.
 * Lines that are not necessary to calculate maximum values are deleted.
 */
struct ConvexHullTrick {
    dat: Vec<(i64, i64)> // (a,b) -> y = a * x + b
}

impl ConvexHullTrick {
    fn new() -> Self {
        ConvexHullTrick { dat: Vec::new() }
    }
    fn check(a: (i64, i64), b: (i64, i64), c: (i64, i64)) -> bool {
        (b.0 - a.0) * (c.1 - b.1) >= (b.1 - a.1) * (c.0 - b.0)
    }
    /*
     * added.0 must be the largest.
     */
    fn add(&mut self, added: (i64, i64)) {
        while self.dat.len() >= 2 {
            let l = self.dat.len();
            if Self::check(self.dat[l - 2], self.dat[l - 1], added) {
                self.dat.pop().unwrap();
            } else {
                break;
            }
        }
        self.dat.push(added);
    }
    fn get(&self) -> Vec<(i64, i64)> {
        self.dat.clone()
    }
}

const MINF: i64 = -1_i64 << 60;

/*
 * tbl.len() = n + 1
 * tbl[i].len() = i + 1
 * tbl[i][j]:= max { a_m + a_{m + 1} + ... + a_{m + j - 1} | m + j <= i }
 *  (0 <= j <= i)
 */
fn make_table(a: &[i64]) -> Vec<Vec<i64>> {
    let n = a.len();
    let mut tbl = vec![Vec::new(); n + 1];
    for i in 0 .. (n + 1) {
        tbl[i] = vec![MINF; i + 1];
    }
    tbl[0][0] = 0;
    for i in 0 .. n {
        let mut cur = 0;
        for j in i .. n {
            cur += a[j];
            tbl[j + 1][j - i + 1] = max(tbl[j + 1][j - i + 1], cur);
        }
    }

    for j in 0 .. (n + 1) {
        for i in (j + 1) .. (n + 1) {
            tbl[i][j] = max(tbl[i][j], tbl[i - 1][j]);
        }
    }

    return tbl;
}


fn query(maxw: &[Vec<i64>], maxh: &[Vec<i64>], qa: usize, qb: usize) -> i64 {
    let ra = &maxw[qa];
    let rb = &maxh[qb];
    let mut ma = MINF;

    let mut xcoords: Vec<(f64, f64)> = vec![(0.0, 0.0); qa];
    for k1 in 1 .. (qa + 1) {
        xcoords[k1 - 1] = ((ra[k1] as f64) / (k1 as f64), k1 as f64);
    }
    xcoords.sort_by(|&a, b| a.0.partial_cmp(&b.0).unwrap());


    let mut cht_obj = ConvexHullTrick::new();
    for k2 in 1 .. (qb + 1) {
        cht_obj.add((k2 as i64, rb[k2]));
    }
    let cht = cht_obj.get();

    let app = |(a, b), x| (a as f64) * x + (b as f64);

    let mut cur = 0;
    
    for (x, k1) in xcoords.iter().cloned() {
        while cur < cht.len() - 1 && app(cht[cur], x) < app(cht[cur + 1], x) {
            cur += 1;
        }
        ma = max(ma, (app(cht[cur], x) * k1).round() as i64);
    }
    return ma;
}

fn main() {
    let w: usize = get();
    let h: usize = get();
    let mut a: Vec<i64> = vec![0; w];
    for i in 0 .. w {
        a[i] = get();
    }
    let mut b: Vec<i64> = vec![0; h];
    for i in 0 .. h {
        b[i] = get();
    }
    
    let q: usize = get();


    let maxw: Vec<Vec<i64>> = make_table(&a);
    let maxh: Vec<Vec<i64>> = make_table(&b);

    for _ in 0 .. q {
        let qa: usize = get::<usize>();
        let qb: usize = get::<usize>();
        println!("{}", query(&maxw, &maxh, qa, qb));
    }
}
