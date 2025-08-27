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

fn find_medians(qs: &[(usize, usize)], acc: &[i64]) -> Vec<i64> {
    let q = qs.len();
    let n = acc.len();
    let mut pass = vec![1 << 18; q];
    let mut fail = vec![-1 << 18; q];
    for _ in 0..19 {
        let mut st = SegTree::new(n, |a, b| a + b, 0usize);
        let mut ev = vec![];
        for i in 0..q {
            ev.push(((pass[i] + fail[i]) / 2, 1, i));
        }
        for i in 0..n {
            ev.push((acc[i], 0, i));
        }
        ev.sort_unstable();
        for (x, ty, i) in ev {
            if ty == 0 {
                let old = st.query(i..i + 1);
                st.update(i, old + 1);
            } else {
                let (l, r) = qs[i];
                let c = st.query(l..r);
                if c < (r - l) / 2 {
                    fail[i] = x;
                } else {
                    pass[i] = x;
                }
            }
        }
    }
    pass
}

fn calc_lower(acc: &[i64], qs: &[(i64, usize, usize)]) -> Vec<i64> {
    let mut st = SegTree::new(acc.len(), |a, b| a + b, 0i64);
    let mut stc = SegTree::new(acc.len(), |a, b| a + b, 0i64);
    let q = qs.len();
    let mut ev = vec![];
    let mut res = vec![0; q];
    for i in 0..acc.len() {
        ev.push((acc[i], 0, i));
    }
    for i in 0..q {
        let (x, _, _r) = qs[i];
        ev.push((x, 1, i));
    }
    ev.sort();
    for (x, ty, i) in ev {
        if ty == 0 {
            let old = st.query(i..i + 1);
            st.update(i, old + x);
            let oldc = stc.query(i..i + 1);
            stc.update(i, oldc + 1);
        } else {
            let (_, l, r) = qs[i];
            let v = st.query(l..r);
            let c = stc.query(l..r);
            res[i] = v - x * c;
        }
    }
    res
}

// Tags: linear-programming, dual
fn main() {
    let ints: Vec<usize> = getline()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let [n, q] = ints[..] else { panic!() };
    let s = getline().trim().chars().collect::<Vec<_>>();
    let mut qs = vec![];
    for _ in 0..q {
        let ints: Vec<usize> = getline()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        qs.push((ints[0] - 1, ints[1]));
    }
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + if s[i] == '1' { 1 } else { -1 };
    }

    // Find medians
    let med = find_medians(&qs, &acc);
    for i in 0..0 {
        let (l, r) = qs[i];
        eprintln!("{:?} {:?} {:?}", qs[i], &acc[l..r], med[i]);
    }

    // find sum
    let qsm = (0..q).map(|i| (med[i], qs[i].0, qs[i].1)).collect::<Vec<_>>();
    let lower = calc_lower(&acc, &qsm);
    let neg_acc = acc.into_iter().map(|x| -x).collect::<Vec<_>>();
    let neg_qsm = qsm.into_iter().map(|(x, l, r)| (-x, l, r)).collect::<Vec<_>>();
    let upper = calc_lower(&neg_acc, &neg_qsm);
    for i in 0..q {
        println!("{}", -upper[i] - lower[i]);
    }
    for i in 0..0 {
        eprintln!("{i}: {:?} {} {}", qs[i], lower[i], -upper[i]);
    }
}
