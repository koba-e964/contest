use std::io::{Write, BufWriter};
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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Ref: <https://qiita.com/Akizuki_/items/ea75a8863bad9e9f4320>
#[derive(Clone, Debug)]
pub struct Predecessor64{
    n: usize,
    d: Vec<Vec<u64>>,
}

impl Predecessor64 {
    // ~2e5: 3, ~1e7: 4, ~1e9: 5
    pub fn new(n: usize)->Self{
        let d = (0..n).into_iter().map(|k| vec![0; 1<<(6*(n-k-1))]).collect::<Vec<Vec<u64>>>();
        Predecessor64{
            n, d
        }
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.d[self.n-1][0]==0
    }

    #[inline(always)]
    pub fn include(&self, p: usize) -> bool {
        self.d[0][p>>6]&1<<(p&63)!=0
    }

    #[inline(always)]
    pub fn insert(&mut self, p: usize){
        for i in 0..self.n{
            if self.d[i][p>>(6*(i+1))]&1<<((p>>(6*i))&63)==0{
                self.d[i][p>>(6*(i+1))] |= 1<<((p>>(6*i))&63);
            } else {
                return;
            }
        }
    }

    #[inline(always)]
    pub fn remove(&mut self, p: usize){
        if self.d[0][p>>6]&1<<(p&63)==0{return;}
        for i in 0..self.n{
            self.d[i][p>>(6*(i+1))] ^= 1<<((p>>(6*i))&63);
            if self.d[i][p>>(6*(i+1))]!=0{
                return;
            } 
        }
    }

    #[inline(always)]
    fn ml(r: usize)->u64{
        (1<<r)-1
    }

    #[inline(always)]
    fn mr(l: usize)->u64{
        if l==63{return 0;}
        !((1<<(l+1))-1)
    }

    #[inline(always)]
    fn msb(bit: u64)->usize{
        63-bit.leading_zeros()as usize
    }

    #[inline(always)]
    fn lsb(bit: u64)->usize{
        bit.trailing_zeros()as usize
    }

    //存在しないは!0
    #[inline(always)]
    pub fn prev(&self, mut p: usize)->usize{
        for i in 0..self.n{
            if Self::ml(p&63)&self.d[i][p>>6]!=0{
                let mut res = ((p>>6)<<6)|Self::msb(self.d[i][p>>6]&Self::ml(p&63));
                for j in (0..i).rev(){
                    res = (res<<6)|Self::msb(self.d[j][res]);
                }
                return res;
            }
            p >>= 6;
        }
        !0
    }

    #[inline(always)]
    pub fn next(&self, mut p: usize)->usize{
        for i in 0..self.n{
            if Self::mr(p&63)&self.d[i][p>>6]!=0{
                let mut res = ((p>>6)<<6)|Self::lsb(self.d[i][p>>6]&Self::mr(p&63));
                for j in (0..i).rev(){
                    res = (res<<6)|Self::lsb(self.d[j][res]);
                }
                return res;
            }
            p >>= 6;
        }
        !0
    }

    #[inline(always)]
    pub fn inprev(&self, p: usize)->usize{
        if self.include(p){p}
        else {self.prev(p)}
    }

    #[inline(always)]
    pub fn innext(&self, p: usize)->usize{
        if self.include(p){p}
        else {self.next(p)}
    }

    #[inline(always)]
    pub fn min(&self)->usize{
        self.innext(0)
    }

    #[inline(always)]
    pub fn max(&self)->usize{
        self.inprev((1<<(6*self.n))-1)
    }
}

struct Multiset {
    inner: Predecessor64,
    count: Vec<i32>,
}

impl Multiset {
    fn new(n: usize, lim: usize) -> Self {
        Self {
            inner: Predecessor64::new(n),
            count: vec![0; lim],
        }
    }
    pub fn insert(&mut self, p: usize) {
        if self.count[p] == 0 {
            self.inner.insert(p);
        }
        self.count[p] += 1;
    }
    pub fn remove(&mut self, p: usize) {
        if self.count[p] <= 0 {
            panic!("{:?} {}", &self.count[..10], p);
        }
        if self.count[p] == 1 {
            self.inner.remove(p);
        }
        self.count[p] -= 1;
    }
    pub fn prev(&self, p: usize) -> usize {
        if self.count[p] >= 2 {
            return p;
        }
        self.inner.prev(p)
    }
    pub fn next(&self, p: usize) -> usize {
        if self.count[p] >= 2 {
            return p;
        }
        self.inner.next(p)
    }
}

struct Yuki3346State<'a> {
    bs: Multiset,
    diff: Multiset,
    coo: &'a [usize],
}

impl<'a> Yuki3346State<'a> {
    fn new(a: &'a [usize], coo: &'a [usize]) -> Self {
        Self {
            bs: Multiset::new(3, a.len()),
            diff: Multiset::new(4, 10_000_001),
            coo,
        }
    }
    fn add(&mut self, x: usize) {
        let cont = self.bs.count[x] > 0;
        if cont {
            self.diff.insert(0);
        } else {
            let b = self.bs.next(x);
            let a = self.bs.prev(x);
            let b = if b == !0 { !0 } else { self.coo[b] };
            let a = if a == !0 { !0 } else { self.coo[a] };
            let x = self.coo[x];
            match (a != !0, b != !0) {
                (true, true) => {
                    self.diff.remove(b - a);
                    self.diff.insert(x - a);
                    self.diff.insert(b - x);
                }
                (true, false) => {
                    self.diff.insert(x - a);
                }
                (false, true) => {
                    self.diff.insert(b - x);
                }
                (false, false) => {}
            }
        }
        self.bs.insert(x);
    }
    fn del(&mut self, x: usize) {
        if self.bs.count[x] >= 2 {
            self.diff.remove(0);
        } else {
            let b = self.bs.next(x);
            let a = self.bs.prev(x);
            let b = if b == !0 { !0 } else { self.coo[b] };
            let a = if a == !0 { !0 } else { self.coo[a] };
            let x = self.coo[x];
            match (a != !0, b != !0) {
                (true, true) => {
                    self.diff.insert(b - a);
                    self.diff.remove(x - a);
                    self.diff.remove(b - x);
                }
                (true, false) => {
                    self.diff.remove(x - a);
                }
                (false, true) => {
                    self.diff.remove(b - x);
                }
                (false, false) => {}
            }
        }
        self.bs.remove(x);
    }
    fn query(&self, x: usize, c: char) -> i64 {
        match c {
            'L' => {
                let a = if self.diff.count[x] > 0 { x } else { self.diff.prev(x) };
                if a == !0 { -1 } else { a as i64 }
            }
            'R' => {
                let a = if self.diff.count[x] > 0 { x } else { self.diff.next(x) };
                if a == !0 { -1 } else { a as i64 }
            }
            _ => panic!(),
        }
    }
}

// https://yukicoder.me/problems/no/3446 (3.5)
// Mo's algorithm を使う。
// Tags: mos-algorithm, fast-tree
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, q: usize,
        a: [usize; n],
        lrxc: [(usize1, usize, usize, char); q],
    }

    const B: usize = 330;
    let mut lri: Vec<_> = (0..q).map(|i| {
        let (l, r, _, _) = lrxc[i];
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
    let mut coo = a.clone();
    coo.sort(); coo.dedup();
    let mut st = Yuki3346State::new(&a, &coo);
    let mut cp = vec![0; n];
    for i in 0..n {
        cp[i] = coo.binary_search(&a[i]).unwrap();
    }

    for &(l, r, idx) in &lri {
        while cr < r {
            st.add(cp[cr]);
            cr += 1;
        }
        while cl > l {
            cl -= 1;
            st.add(cp[cl]);
        }
        while cr > r {
            cr -= 1;
            st.del(cp[cr]);
        }
        while cl < l {
            st.del(cp[cl]);
            cl += 1;
        }
        let (_, _, x, c) = lrxc[idx];
        ans[idx] = st.query(x, c);
    }

    for a in ans {
        puts!("{a}\n");
    }
}
