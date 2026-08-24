fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

#[allow(unused)]
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
/// This implementation of AddAssign is useful when you want to make a 2D BIT.
impl<T: Clone, U: Clone> std::ops::AddAssign<(usize, U)> for BIT<T>
    where T: std::ops::AddAssign<U>,
          T: std::ops::AddAssign<T> {
    fn add_assign(&mut self, (idx, val): (usize, U)) {
        self.add(idx, val);
    }
}

struct Seq {
    a: Vec<i64>,
    coo: Vec<i64>,
    ind: Vec<usize>,
    pos: usize,
    st: BIT<i64>,
    cnt: BIT<i64>,
}

impl Seq {
    fn new(a: &[i64]) -> Self {
        let n = a.len();
        let mut coo = a.to_vec();
        coo.sort(); coo.dedup();
        let mut ind = vec![0; n];
        for i in 0..n {
            ind[i] = coo.binary_search(&a[i]).unwrap();
        }
        let m = coo.len();
        Self {
            a: a.to_vec(), coo, ind,
            pos: 0,
            st: BIT::new(m + 1, 0),
            cnt: BIT::new(m + 1, 0),
        }
    }
    fn mv_right(&mut self) -> i64 {
        let x = self.a[self.pos];
        let ind = self.ind[self.pos];
        self.st.add(ind + 1, x);
        self.cnt.add(ind + 1, 1);
        self.pos += 1;
        x
    }
    fn mv_left(&mut self) -> i64 {
        self.pos -= 1;
        let x = self.a[self.pos];
        let ind = self.ind[self.pos];
        self.st.add(ind + 1, -x);
        self.cnt.add(ind + 1, -1);
        x
    }
    fn calc(&self, x: i64) -> i64 {
        let m = self.coo.len();
        let ind = self.coo.lower_bound(&x);
        let fst = self.cnt.range(1..ind + 1) * x -  self.st.range(1..ind + 1);
        let snd = self.st.range(ind + 1..m + 1) - self.cnt.range(ind + 1..m + 1) * x;
        fst + snd
    }
}

fn main() {
    let n = getline().trim().parse::<usize>().unwrap();
    let a = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let b = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let q = getline().trim().parse::<usize>().unwrap();
    let mut xy = vec![];
    for _ in 0..q {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let [x, y] = ints[..] else { panic!() };
        xy.push((x, y));
    }
    const B: usize = 1000;
    let mut lri: Vec<_> = (0..q).map(|i| {
        let (l, r) = xy[i];
        (l, r, i)
    }).collect();
    lri.sort_by_key(|&(l, r, _idx)| {
        let q = l / B;
        if q % 2 == 1 {
            (q, n - r)
        } else {
            (q, r)
        }
    });
    let mut ans = vec![0; q];
    
    // pointer
    let mut cl = 0;
    let mut cr = 0;
    
    // state
    let mut seqa = Seq::new(&a);
    let mut seqb = Seq::new(&b);
    let mut val = 0;

    for &(l, r, idx) in &lri {
        while cr < r {
            // add cr
            let x = seqb.mv_right();
            val += seqa.calc(x);
            cr += 1;
        }
        while cl > l {
            cl -= 1;
            // add cl
            let x = seqa.mv_left();
            val -= seqb.calc(x);
        }
        while cr > r {
            cr -= 1;
            // del cr
            let x = seqb.mv_left();
            val -= seqa.calc(x);
        }
        while cl < l {
            // del cl
            let x = seqa.mv_right();
            val += seqb.calc(x);
            cl += 1;
        }
        ans[idx] = val;
    }
    for a in ans {
        println!("{a}");
    }
}
