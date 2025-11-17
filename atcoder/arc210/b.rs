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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

struct KLargest {
    hi: std::collections::BTreeSet<(i64, usize)>,
    lo: std::collections::BTreeSet<(i64, usize)>,
    k: usize,
    hisum: i64,
}

impl KLargest {
    fn new(k: usize) -> Self {
        KLargest {
            hi: std::collections::BTreeSet::new(),
            lo: std::collections::BTreeSet::new(),
            k: k,
            hisum: 0,
        }
    }
    fn add(&mut self, x: (i64, usize)) {
        self.hi.insert(x);
        self.hisum += x.0;
        self.adjust();
    }
    fn remove(&mut self, x: (i64, usize)) {
        if self.hi.remove(&x) {
            self.hisum -= x.0;
            self.adjust();
            return;
        }
        self.lo.remove(&x);
    }
    fn adjust(&mut self) {
        if self.hi.len() < self.k && !self.lo.is_empty() {
            let &ma = self.lo.iter().rev().next().unwrap();
            self.lo.remove(&ma);
            self.hi.insert(ma);
            self.hisum += ma.0;
        }
        if self.hi.len() > self.k {
            let &mi = self.hi.iter().next().unwrap();
            self.hi.remove(&mi);
            self.hisum -= mi.0;
            self.lo.insert(mi);
        }
    }
    fn hisum(&self) -> i64 {
        self.hisum
    }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize, q: usize,
        a: [i64; n],
        b: [i64; m],
        tix: [(i32, usize1, i64); q],
    }
    let mut a = a;
    a.extend_from_slice(&b);
    let mut set1 = KLargest::new(n / 2);
    let mut set2 = KLargest::new(n / 2 + m);
    let mut s = 0;
    for i in 0..n + m {
        set1.add((a[i], i));
        set2.add((a[i], i));
        s += a[i];
    }

    for (t, i, x) in tix {
        let idx = if t == 1 {
            i
        } else {
            i + n
        };
        s += x - a[idx];
        set1.remove((a[idx], idx));
        set2.remove((a[idx], idx));
        a[idx] = x;
        set1.add((a[idx], idx));
        set2.add((a[idx], idx));
        // eprintln!("s = {s}, {} {}", set1.hisum(), set2.hisum());
        puts!("{}\n", s - set2.hisum() + set1.hisum());
    }
}
