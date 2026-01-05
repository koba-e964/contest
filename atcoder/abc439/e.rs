fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

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

fn main() {
    let n = getline().trim().parse::<usize>().unwrap();
    let mut ab = vec![];
    let mut al = vec![];
    let mut bl = vec![];
    for _ in 0..n {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        ab.push((ints[0], ints[1]));
        al.push(ints[0]);
        bl.push(ints[1]);
    }
    ab.sort();
    al.sort(); al.dedup();
    bl.sort(); bl.dedup();
    let x = al.len();
    let y = bl.len();
    let mut occ = vec![vec![]; x];
    for (a, b) in ab {
        let a = al.binary_search(&a).unwrap();
        let b = bl.binary_search(&b).unwrap();
        occ[a].push(b);
    }
    let mut dp = vec![0; x];
    let mut dp1 = SegTree::new(y, |x, y| x.max(y), 0);
    for i in 0..x {
        let mut ma = 0;
        for &v in occ[i].iter().rev() {
            let tmp = dp1.query(0..v);
            ma = ma.max(tmp + 1);
            let old = dp1.query(v..v + 1);
            dp1.update(v, old.max(tmp + 1));
        }
        dp[i] = ma;
    }
    println!("{}", dp.iter().max().unwrap());
}
