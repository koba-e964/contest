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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
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
    fn next(&mut self) -> u32 {
        let a = 0xdead_c0de_0013_3331u64;
        let b = 2457;
        self.x = self.x.wrapping_mul(a).wrapping_add(b);
        let x = self.x;
        ((x ^ x << 10) >> 32) as _
    }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
        q: usize,
        xy: [(usize, usize); q],
    }
    let mut coo = a.clone();
    coo.extend(&b);
    coo.sort(); coo.dedup();
    let a: Vec<_> = a.iter().map(|v| coo.binary_search(v).unwrap()).collect();
    let b: Vec<_> = b.iter().map(|v| coo.binary_search(v).unwrap()).collect();
    let m = coo.len();
    let mut rng = Rng::new();
    let mut val = vec![0u64; m];
    for i in 0..m {
        let x = rng.next();
        let y = rng.next();
        val[i] = (x as u64) << 32 | y as u64;
    }
    let mut acc_a = vec![0; n + 1];
    let mut acc_b = vec![0; n + 1];
    let mut set = HashSet::new();
    for i in 0..n {
        acc_a[i + 1] = acc_a[i];
        if !set.contains(&a[i]) {
            set.insert(a[i]);
            acc_a[i + 1] ^= val[a[i]];
        }
    }
    set.clear();
    for i in 0..n {
        acc_b[i + 1] = acc_b[i];
        if !set.contains(&b[i]) {
            set.insert(b[i]);
            acc_b[i + 1] ^= val[b[i]];
        }
    }
    for (x, y) in xy {
        puts!("{}\n", if acc_a[x] == acc_b[y] { "Yes" } else { "No" });
    }
}
