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

// Complexity: O(n log log n)
fn cube_tbl(n: usize) -> (Vec<usize>, Vec<Vec<(usize, usize)>>) {
    if n <= 2 {
        panic!();
    }
    let mut fac = vec![0; n];
    let mut ps = vec![];
    let mut pi = vec![0; n];
    for i in 2..n {
        pi[i] = pi[i - 1];
        if fac[i] != 0 { continue; }
        fac[i] = i;
        ps.push(i);
        pi[i] += 1;
        for j in 2..(n - 1) / i + 1 {
            fac[i * j] = i;
        }
    }
    let mut ans = vec![vec![]; n];
    for i in 2..n {
        let mut v = i;
        while v > 1 {
            let mut e = 0;
            let p = fac[v];
            while v % p == 0 {
                v /= p;
                e += 1;
            }
            if e % 3 != 0 {
                ans[i].push((pi[p] - 1, e % 3));
            }
        }
    }
    (ps, ans)
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

// (F_4^\times)^32
fn merge(a: [u32; 2], b: [u32; 2]) -> [u32; 2] {
    let mut v = [0; 2];
    for i in 0..2 {
        for j in 0..2 {
            let x = a[i] & b[j];
            if (i, j) == (1, 1) {
                v[0] ^= x;
                v[1] ^= x;
            } else {
                v[i + j] ^= x;
            }
        }
    }
    v
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, q: usize,
        a: [usize; n],
        lr: [(usize1, usize); q],
    }
    const W: usize = 1_000_001;
    let (ps, tbl) = cube_tbl(W);
    let m = ps.len();
    let mut lri = vec![(0, 0, 0); q];
    for i in 0..q {
        let (l, r) = lr[i];
        lri[i] = (l, r, i);
    }
    let mut rng = Rng::new();
    let mut rnd = vec![[0u32; 2]; m];
    for i in 0..m {
        for j in 0..32 {
            let x = (rng.next() >> 16) % 3;
            if x == 1 {
                rnd[i][1] |= 1 << j;
            } else if x == 2 {
                rnd[i][0] |= 1 << j;
                rnd[i][1] |= 1 << j;
            } else {
                rnd[i][0] |= 1 << j;
            }
        }
    }
    let mut val = vec![[0u32; 2]; n];
    let mut invval = vec![[0u32; 2]; n];
    for i in 0..n {
        let v = &tbl[a[i]];
        let mut me = [0; 2];
        me[0] = !0;
        for &(idx, f) in v {
            let mut p = rnd[idx];
            assert!(f == 1 || f == 2);
            if f == 2 {
                p[0] ^= p[1];
            }
            me = merge(me, p);
        }
        assert_eq!(me[0] | me[1], !0);
        val[i] = me;
        invval[i] = [val[i][0] ^ val[i][1], val[i][1]];
    }
    const B: usize = 500;
    lri.sort_by_key(|&(l, r, _)| {
        let q = l / B;
        (q, if q % 2 == 0 {
            r
        } else {
            n - r
        })
    });
    let mut ans = vec![false; q];
    let mut x = 0;
    let mut y = 0;
    let mut tmp = [0; 2];
    tmp[0] = !0;
    for &(l, r, idx) in &lri {
        while y < r {
            tmp = merge(tmp, val[y]);
            y += 1;
        }
        while x > l {
            x -= 1;
            tmp = merge(tmp, val[x]);
        }
        while y > r {
            y -= 1;
            tmp = merge(tmp, invval[y]);
        }
        while x < l {
            tmp = merge(tmp, invval[x]);
            x += 1;
        }
        ans[idx] = tmp == [!0, 0];
    }
    for i in 0..q {
        puts!("{}\n", if ans[i] { "Yes" } else { "No" });
    }
}
