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

trait Bisect<T> {
    fn lower_bound(&self, val: &T) -> usize;
    fn upper_bound(&self, val: &T) -> usize;
}

impl<T: Ord> Bisect<T> for [T] {
    fn lower_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] >= val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
    fn upper_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] > val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
}

fn calc(ab: &[(i64, i64)]) -> Vec<Vec<(i64, i64)>> {
    let n = ab.len();
    let mut ans = vec![vec![]; n + 1];
    for bits in 0usize..1 << n {
        let mut x = 0;
        let mut y = 0;
        for i in 0..n {
            if (bits & 1 << i) != 0 {
                x += ab[i].0;
                y += ab[i].1;
            }
        }
        let bc = bits.count_ones() as usize;
        ans[bc].push((x, y));
    }
    ans
}

// https://yukicoder.me/problems/no/2161 (3)
// 半分全列挙 + 前半 x 個、後半 y 個 (x + y <= k) であるものを平面走査で数える
fn main() {
    input! {
        n: usize, k: usize, l: i64, p: i64,
        ab: [(i64, i64); n],
    }
    let fst = calc(&ab[..n / 2]);
    let snd = calc(&ab[n / 2..]);
    let mut ans = 0i64;
    for i in 0..k + 1 {
        if k - i > n - n / 2 {
            continue;
        }
        let mut coo = vec![];
        for &(_, y) in &snd[k - i] {
            coo.push(y);
        }
        coo.sort(); coo.dedup();
        let mut ev = vec![];
        for j in 0..std::cmp::min(i, n / 2) + 1 {
            for &(x, y) in &fst[j] {
                ev.push((l - x, 1, coo.lower_bound(&(p - y))));
            }
        }
        for &(x, y) in &snd[k - i] {
            ev.push((x, 0, coo.lower_bound(&y)));
        }
        ev.sort();
        let m = coo.len();
        let mut st = SegTree::new(m, |x, y| x + y, 0i64);
        for &(_, ty, idx) in &ev {
            if ty == 1 {
                ans += st.query(idx..m);
            } else {
                let old = st.query(idx..idx + 1);
                st.update(idx, old + 1);
            }
        }
    }
    println!("{}", ans);
}
