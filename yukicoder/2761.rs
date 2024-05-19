use std::io::Read;

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

// Segment Tree. This data structure is useful for fast folding on intervals of an array
// whose elements are elements of monoid I. Note that constructing this tree requires the identity
// element of I and the operation of I.
// Verified by: yukicoder No. 2220 (https://yukicoder.me/submissions/841554)
#[derive(Clone)]
struct SegTree<I, BiOp> {
    n: usize,
    orign: usize,
    dat: Vec<I>,
    op: BiOp,
    e: I,
}

impl<I, BiOp> SegTree<I, BiOp>
    where BiOp: Fn(I, I) -> I,
          I: Copy {
    pub fn new(n_: usize, op: BiOp, e: I) -> Self {
        let mut n = 1;
        while n < n_ { n *= 2; } // n is a power of 2
        SegTree {n: n, orign: n_, dat: vec![e; 2 * n - 1], op: op, e: e}
    }
    // ary[k] <- v
    pub fn update(&mut self, idx: usize, v: I) {
        debug_assert!(idx < self.orign);
        let mut k = idx + self.n - 1;
        self.dat[k] = v;
        while k > 0 {
            k = (k - 1) / 2;
            self.dat[k] = (self.op)(self.dat[2 * k + 1], self.dat[2 * k + 2]);
        }
    }
    // [a, b) (half-inclusive)
    // http://proc-cpuinfo.fixstars.com/2017/07/optimize-segment-tree/
    #[allow(unused)]
    pub fn query(&self, rng: std::ops::Range<usize>) -> I {
        let (mut a, mut b) = (rng.start, rng.end);
        debug_assert!(a <= b);
        debug_assert!(b <= self.orign);
        let mut left = self.e;
        let mut right = self.e;
        a += self.n - 1;
        b += self.n - 1;
        while a < b {
            if (a & 1) == 0 {
                left = (self.op)(left, self.dat[a]);
            }
            if (b & 1) == 0 {
                right = (self.op)(self.dat[b - 1], right);
            }
            a = a / 2;
            b = (b - 1) / 2;
        }
        (self.op)(left, right)
    }
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

// https://yukicoder.me/problems/no/2761 (3)
// 文字列の位置が固定 (シフトしない) であるため、特にローリングハッシュである必要はなく、位置ごとに乱数を生成すれば良い。
// また ModInt である必要もないので xor で演算している (zobrist hash と同様)。これで解説より定数倍高速である。
fn main() {
    let mut rng = Rng::new();
    let n: usize = get();
    let l: usize = get();
    let q: usize = get();
    let mut s = vec![];
    for _ in 0..n {
        s.push(get_word().bytes().collect::<Vec<_>>());
    }
    let mut val = vec![[0u64; 26]; l];
    for i in 0..l {
        for j in 0..26 {
            val[i][j] = rng.next() as u64;
            val[i][j] |= (rng.next() as u64) << 32;
        }
    }
    let mut st = vec![SegTree::new(l, |x, y| x ^ y, 0); n];
    for i in 0..n {
        for j in 0..l {
            let idx = (s[i][j] - b'a') as usize;
            st[i].update(j, val[j][idx]);
        }
    }
    for _ in 0..q {
        let ty: i32 = get();
        if ty == 1 {
            let k = get::<usize>() - 1;
            let c: char = get();
            let d: char = get();
            let v = val[k][(d as u8 - b'a') as usize];
            for i in 0..n {
                if s[i][k] == c as u8 {
                    s[i][k] = d as u8;
                    st[i].update(k, v);
                }
            }
        } else {
            let mut sum = 0;
            let t = get_word().bytes().collect::<Vec<_>>();
            for i in 0..t.len() {
                sum ^= val[i][(t[i] - b'a') as usize];
            }
            let mut ans = 0;
            for i in 0..n {
                if st[i].query(0..t.len()) == sum {
                    ans += 1;
                }
            }
            println!("{}", ans);
        }
    }
}
