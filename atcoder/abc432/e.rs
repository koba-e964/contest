fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

// Binary Indexed Tree (Fenwick Tree). Holds an array of type T.
// T is a commutative monoid. Indices are in 0..n.
// Verified by ABC285-F (https://atcoder.jp/contests/abc285/submissions/38329093)
#[derive(Clone, Debug)]
pub struct BIT<T> {
    n: usize,
    ary: Vec<T>,
    e: T,
}

impl<T: Clone + std::ops::AddAssign<T>> BIT<T> {
    pub fn new(n: usize, e: T) -> Self {
        BIT { n: n, ary: vec![e.clone(); n + 1], e: e }
    }
    // Usage: self.accum(..idx)
    fn accum(&self, idx: std::ops::RangeTo<usize>) -> T {
        let mut idx = idx.end;
        let mut sum = self.e.clone();
        while idx > 0 {
            sum += self.ary[idx].clone();
            idx &= idx - 1;
        }
        sum
    }
    // Usage: self.accum(l..r)
    #[inline(always)]
    pub fn range(&self, rng: std::ops::Range<usize>) -> T
        where T: std::ops::Sub<Output = T> {
        self.accum(..rng.end) - self.accum(..rng.start)
    }
    // performs data[idx] += val;
    // 0 <= idx, idx < n
    pub fn add<U: Clone>(&mut self, mut idx: usize, val: U)
        where T: std::ops::AddAssign<U> {
        debug_assert!(idx < self.n);
        idx += 1;
        let n = self.n;
        while idx <= n {
            self.ary[idx] += val.clone();
            idx += idx & idx.wrapping_neg();
        }
    }
    // Make sure that 0 <= idx < n.
    #[allow(unused)]
    #[inline(always)]
    pub fn get(&self, idx: usize) -> T
        where T: std::ops::Sub<Output = T> {
        debug_assert!(idx < self.n);
        self.accum(..idx + 1) - self.accum(..idx)
    }
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let q = ints[1];
    let mut a = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let n = a.len();
    const W: usize = 500_010;
    let mut cnt = BIT::new(W, 0i64);
    let mut sum = BIT::new(W, 0i64);
    for i in 0..n {
        cnt.add(a[i], 1);
        sum.add(a[i], a[i] as i64);
    }
    for _ in 0..q {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        if ints[0] == 1 {
            let x = ints[1] - 1;
            let y = ints[2];
            let old = a[x];
            cnt.add(old, -1);
            sum.add(old, -(old as i64));
            a[x] = y;
            cnt.add(a[x], 1);
            sum.add(a[x], a[x] as i64);
        } else {
            let l = ints[1];
            let r = ints[2];
            if l >= r {
                println!("{}", l as i64 * n as i64);
                continue;
            }
            let mut ans = cnt.accum(..l) * l as i64;
            ans += cnt.range(r..W) * r as i64;
            ans += sum.range(l..r);
            println!("{ans}");
        }
    }
}
