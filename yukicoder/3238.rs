// Segment Tree. This data structure is useful for fast folding on intervals of an array
// whose elements are elements of monoid I. Note that constructing this tree requires the identity
// element of I and the operation of I.
// Verified by: yukicoder No. 2220 (https://yukicoder.me/submissions/841554)
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

fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

// https://yukicoder.me/problems/no/3238 (3)
// 重みを、左下の重みの max + 1 (ただしそのような点がない場合は 0) と定義する。
// 重みの小さい順にまとめて削除されるので、重みを計算すれば良い。
// 重みは平面走査 + セグメント木 (max) で計算できる。
fn main() {
    getline();
    let p = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let n = p.len();
    let mut seg = SegTree::new(n, |x, y| x.max(y), -1);
    let mut w = vec![vec![]; n];
    for i in 0..n {
        let val = seg.query(0..p[i] - 1);
        seg.update(p[i] - 1, val + 1);
        w[(val + 1) as usize].push(i);
    }
    for i in 0..n {
        if w[i].is_empty() {
            continue;
        }
        let mut x = n;
        let mut y = n;
        for &idx in &w[i] {
            x = x.min(idx + 1);
            y = y.min(p[idx]);
        }
        println!("{x} {y}");
    }
}
