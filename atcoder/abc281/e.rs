use std::collections::*;
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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

struct Data {
    fst: BTreeSet<(i64, usize)>,
    snd: BTreeSet<(i64, usize)>,
    k: usize,
    sum: i64,
}

impl Data {
    fn add(&mut self, p: (i64, usize)) {
        self.sum += p.0;
        self.fst.insert(p);
        if self.fst.len() > self.k {
            let &ma = self.fst.iter().rev().next().unwrap();
            self.fst.remove(&ma);
            self.sum -= ma.0;
            self.snd.insert(ma);
        }
    }

    fn del(&mut self, p: (i64, usize)) {
        if self.fst.remove(&p) {
            self.sum -= p.0;
        } else {
            self.snd.remove(&p);
        }
        if self.fst.len() < self.k && !self.snd.is_empty() {
            let &mi = self.snd.iter().next().unwrap();
            self.snd.remove(&mi);
            self.sum += mi.0;
            self.fst.insert(mi);
        }
    }
}

// Tags: k-smallest-elements
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize, m: usize, k: usize,
        a: [i64; n],
    }
    let mut dat = Data {
        fst: BTreeSet::new(),
        snd: BTreeSet::new(),
        k,
        sum: 0,
    };
    for i in 0..m {
        dat.add((a[i], i));
    }
    let mut res = vec![dat.sum];
    for i in m..n {
        dat.add((a[i], i));
        dat.del((a[i - m], i - m));
        res.push(dat.sum);
    }
    putvec!(res);
}
