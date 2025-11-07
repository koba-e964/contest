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

const INF: i32 = 1 << 27;
type O = [[i32; 7]; 7];

fn op(a: O, b: O) -> O {
    let mut res = [[INF; 7]; 7];
    for i in 0..res.len() {
        for j in 4.min(i)..res.len() {
            for k in 4.min(j)..res.len() {
                res[i][k] = res[i][k].min(a[i][j] + b[j][k]);
            }
        }
    }
    res
}

fn get(a: i32) -> O {
    let mut res = [[INF; 7]; 7];
    if a == 0 {
        res[0][2] = 1;
        res[1][4] = 1;
        res[2][3] = 1;
        res[3][3] = 0;
        res[4][5] = 1;
        res[5][5] = 0;
        res[6][4] = 1;
    } else {
        res[0][1] = 1;
        res[1][1] = 0;
        res[2][6] = 1;
        res[3][6] = 1;
        res[4][6] = -1;
        res[5][6] = 1;
        res[6][6] = 0;
    }
    res
}

fn main() {
    getline();
    let mut a = getline().trim().split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let n = a.len();
    let mut init = [[INF; 7]; 7];
    for i in 0..init.len() {
        init[i][i] = 0;
    }
    let mut st = SegTree::new(n, op, init);
    for i in 0..n {
        st.update(i, get(a[i]));
    }
    let mut s: i32 = a.iter().sum();
    let q = getline().trim().parse::<i32>().unwrap();
    for _ in 0..q {
        let t = getline().trim().parse::<usize>().unwrap() - 1;
        s += 1 - 2 * a[t];
        a[t] = 1 - a[t];
        st.update(t, get(a[t]));
        let all = st.query(0..n);
        let ans = if s == n as i32 {
            s
        } else {
            let mut mi = INF;
            for i in 2..all.len() {
                mi = mi.min(all[0][i]);
            }
            mi.max(2)
        };
        println!("{ans}");
        if n <= 8 && q <= 5 && 1 == 2 {
            eprintln!("a = {a:?}");
            for i in 0..n - 1 {
                eprintln!("{i} => {:?}", st.query(0..i));
            }
        }
    }
}
