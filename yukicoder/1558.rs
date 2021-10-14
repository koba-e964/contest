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

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

/**
 * Segment Tree. This data structure is useful for fast folding on intervals of an array
 * whose elements are elements of monoid I. Note that constructing this tree requires the identity
 * element of I and the operation of I.
 * Verified by: yukicoder No. 259 (http://yukicoder.me/submissions/100581)
 *              AGC015-E (http://agc015.contest.atcoder.jp/submissions/1461001)
 *              yukicoder No. 833 (https://yukicoder.me/submissions/703521)
 */
struct SegTree<I, BiOp> {
    n: usize,
    dat: Vec<I>,
    op: BiOp,
    e: I,
}

impl<I, BiOp> SegTree<I, BiOp>
    where BiOp: Fn(I, I) -> I,
          I: Clone {
    pub fn new(n_: usize, op: BiOp, e: I) -> Self {
        let mut n = 1;
        while n < n_ { n *= 2; } // n is a power of 2
        SegTree {n: n, dat: vec![e.clone(); 2 * n - 1], op: op, e: e}
    }
    /* ary[k] <- v */
    pub fn update(&mut self, idx: usize, v: I) {
        let mut k = idx + self.n - 1;
        self.dat[k] = v;
        while k > 0 {
            k = (k - 1) / 2;
            self.dat[k] = (self.op)(self.dat[2 * k + 1].clone(), self.dat[2 * k + 2].clone());
        }
    }
    /* [a, b) (note: half-inclusive)
     * http://proc-cpuinfo.fixstars.com/2017/07/optimize-segment-tree/ */
    #[allow(unused)]
    pub fn query(&self, mut a: usize, mut b: usize) -> I {
        let mut left = self.e.clone();
        let mut right = self.e.clone();
        a += self.n - 1;
        b += self.n - 1;
        while a < b {
            if (a & 1) == 0 {
                left = (self.op)(left, self.dat[a].clone());
            }
            if (b & 1) == 0 {
                right = (self.op)(self.dat[b - 1].clone(), right);
            }
            a = a / 2;
            b = (b - 1) / 2;
        }
        (self.op)(left, right)
    }
}

fn main() {
    let n: usize = get();
    let m: usize = get();
    let q: usize = get();
    let mut st = SegTree::new(m, |a, b| {
        let mut c = vec![0; n];
        for i in 0..n {
            c[i] = b[a[i]];
        }
        c
    }, (0..n).collect::<Vec<_>>());
    for _ in 0..q {
        let ty: i32 = get();
        if ty == 1 {
            let d = get::<usize>() - 1;
            let p: Vec<_> = (0..n).map(|_| get::<usize>() - 1).collect();
            st.update(d, p);
        } else if ty == 2 {
            let s: usize = get();
            let k = st.query(0, s);
            let mut v = vec![0; n];
            for i in 0..n {
                v[k[i]] = i;
            }
            for i in 0..n {
                print!("{}{}", v[i] + 1, if i + 1 == n { "\n" } else { " " });
            }
        } else {
            let l = get::<usize>() - 1;
            let r: usize = get();
            let k = st.query(l, r);
            let mut ans = 0;
            for i in 0..n {
                ans += (i as i32 - k[i] as i32).abs();
            }
            println!("{}", ans);
        }
    }
}
