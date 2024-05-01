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
    let n: usize = get();
    let l: usize = get();
    let d: usize = get();
    let mut deal_imos = SegTree::new(n + 3, |x, y| x + y, 0.0);
    deal_imos.update(0, 1.0);
    deal_imos.update(1, -1.0);
    for i in 0..l {
        let rg = deal_imos.query(0..i + 1);
        let val = rg / d as f64;
        // deal[i + 1..i + 1 + d] += val
        let targ = (n + 2).min(i + d + 1);
        let rem = i + d + 1 - targ;
        let tmp = deal_imos.query(targ..targ + 1);
        deal_imos.update(targ, tmp - val);
        let tmp = deal_imos.query(i + 1..i + 2);
        deal_imos.update(i + 1, tmp + val);
        if rem > 0 {
            let tmp = deal_imos.query(n + 1..n + 2);
            deal_imos.update(n + 1, tmp + val * rem as f64);
        }
    }
    let mut deal = vec![0.0; n + 2];
    for i in l..n + 2 {
        deal[i] = deal_imos.query(0..i + 1);
    }
    let mut deal_acc = vec![0.0; n + 3];
    for i in 0..n + 2 {
        deal_acc[i + 1] = deal_acc[i] + deal[i];
    }

    let mut dp = SegTree::new(n + 2, |x, y| x + y, 0.0);
    for i in (0..n + 1).rev() {
        let mut me = deal_acc[i] + deal[n + 1];
        let tmp = dp.query(i + 1..(i + d + 1).min(n + 2)) / d as f64;
        me = me.max(tmp);
        dp.update(i, me);
    }
    println!("{}", dp.query(0..1));
}
