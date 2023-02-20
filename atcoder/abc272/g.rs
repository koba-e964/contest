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

// (3/4)^41 ~= 7.54 * 10^-6 < 10^-5
// Tags: probabilistic-algorithm
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut rng = Rng::new();
    let mut div = vec![];
    for _ in 0..41 {
        let x = rng.next() as usize % n;
        let dif = rng.next() as usize % (n - 1);
        let y = (x + 1 + dif) % n;
        let u = (a[x] - a[y]).abs();
        let mut d = 1;
        while d * d <= u {
            if u % d == 0 {
                div.push(d);
                div.push(u / d);
            }
            d += 1;
        }
    }
    div.sort(); div.dedup();
    for d in div {
        if d < 3 { continue; }
        let mut hm = std::collections::HashMap::new();
        for &a in &a {
            *hm.entry(a % d).or_insert(0usize) += 1;
        }
        if hm.values().any(|v| 2 * v > n) {
            println!("{}", d);
            return;
        }
    }
    println!("-1");
}

