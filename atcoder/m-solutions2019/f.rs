#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

// Verified by https://atcoder.jp/contests/arc084/submissions/3935443
#[derive(Clone)]
struct BitSet {
    size: usize,
    buf: Vec<usize>,
}

impl BitSet {
    // size should be a multiple of bit-size of usize.
    fn new(size: usize) -> Self {
        let w = 8 * std::mem::size_of::<usize>();
        assert_eq!(size & (w - 1), 0);
        let count = size / w;
        BitSet {
            size: size,
            buf: vec![0; count],
        }
    }
    #[allow(unused)]
    fn set(&mut self, idx: usize, val: bool) {
        debug_assert!(idx < self.size);
        let w = 8 * std::mem::size_of::<usize>();
        let idx0 = idx / w;
        let idx1 = idx & (w - 1);
        if val {
            self.buf[idx0] |= 1 << idx1;
        } else {
            self.buf[idx0] &= !(1 << idx1);
        }
    }
    #[allow(unused)]
    fn get(&self, idx: usize) -> bool {
        let w = 8 * std::mem::size_of::<usize>();
        debug_assert!(idx < self.size);
        let idx0 = idx / w;
        let idx1 = idx & (w - 1);
        (self.buf[idx0] >> idx1 & 1) == 1
    }
    #[allow(unused)]
    fn shl(&self, val: usize) -> Self {
        if val >= self.size { return Self::new(self.size); }
        let w = 8 * std::mem::size_of::<usize>();
        let count = self.size / w;
        let sh0 = val / w;
        let sh1 = val & (w - 1);
        let mut ans = Self::new(self.size);
        if sh1 == 0 {
            for i in 0 .. count - sh0 {
                ans.buf[i + sh0] = self.buf[i];
            }
        } else {
            ans.buf[sh0] = self.buf[0] << sh1;
            for i in 1 .. count - sh0 {
                ans.buf[i + sh0] = self.buf[i] << sh1
                    | self.buf[i - 1] >> (w - sh1);
            }
        }
        ans
    }
    // Not verified
    #[allow(unused)]
    fn shr(&self, val: usize) -> Self {
        if val >= self.size { return Self::new(self.size); }
        let w = 8 * std::mem::size_of::<usize>();
        let count = self.size / w;
        let sh0 = val / w;
        let sh1 = val & (w - 1);
        let mut ans = Self::new(self.size);
        if sh1 == 0 {
            for i in 0 .. count - sh0 {
                ans.buf[i] = self.buf[i + sh0];
            }
        } else {
            for i in 0 .. count - sh0 - 1 {
                ans.buf[i] = self.buf[i + sh0] >> sh1
                    | self.buf[i + sh0 + 1] << (w - sh1);
            }
            ans.buf[self.size - sh0 - 1] = self.buf[self.size - 1] >> sh1;
        }
        ans
    }
    #[allow(unused)]
    fn msb(&self) -> Option<usize> {
        let w = 8 * std::mem::size_of::<usize>();
        let count = self.size / w;
        for i in (0 .. count).rev() {
            let v = self.buf[i];
            if v != 0 {
                return Some(w * i + w - 1 - v.leading_zeros() as usize);
            }
        }
        None
    }
}

// Verified by https://atcoder.jp/contests/m-solutions2019/submissions/5746770
trait BitSetAdapter: Sized {
    fn word_size(&self) -> usize;
    fn at(&self, word_pos: usize) -> usize;
    // short-circuit operations
    fn any(&self) -> bool {
        let sz = self.word_size();
        for i in 0..sz {
            if self.at(i) != 0 {
                return true;
            }
        }
        false
    }
    fn all(&self) -> bool {
        let sz = self.word_size();
        for i in 0..sz {
            if self.at(i) != 1usize.wrapping_neg() {
                return false;
            }
        }
        true
    }
    fn and<U: BitSetAdapter>(self, other: U) -> AndAdapter<Self, U> {
        AndAdapter {
            t: self,
            u: other,
        }
    }
}

struct AndAdapter<T, U> {
    t: T,
    u: U,
}

impl<T: BitSetAdapter, U: BitSetAdapter> BitSetAdapter for AndAdapter<T, U> {
    fn word_size(&self) -> usize {
        self.t.word_size()
    }
    fn at(&self, word_pos: usize) -> usize {
        self.t.at(word_pos) & self.u.at(word_pos)
    }
}

/*
impl BitSetAdapter for BitSet {
    fn word_size(&self) -> usize {
        let w = 8 * std::mem::size_of::<usize>();
        assert_eq!(self.size & (w - 1), 0);
        self.size / w
    }
    fn at(&self, word_pos: usize) -> usize {
        self.buf[word_pos]
    }
}*/

impl<'a> BitSetAdapter for &'a BitSet {
    fn word_size(&self) -> usize {
        let w = 8 * std::mem::size_of::<usize>();
        assert_eq!(self.size & (w - 1), 0);
        self.size / w
    }
    fn at(&self, word_pos: usize) -> usize {
        self.buf[word_pos]
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        aa: [chars; n - 1],
    }
    let mut a = vec![BitSet::new(2048); n];
    for i in 1..n {
        for j in 0..i {
            a[i].set(j, aa[i - 1][j] == '1');
            a[j].set(i, aa[i - 1][j] == '0');
        }
    }
    drop(aa);
    let mut dp = vec![BitSet::new(2048); n];
    for i in 0..n {
        dp[i].set(i, true);
    }
    for s in 1..n {
        for i in 0..n - s {
            let j = i + s;
            let res = dp[i].and(&dp[j - 1]).and(&a[j]).any();
            dp[i].set(j, res);
            let res = dp[i + 1].and(&dp[j]).and(&a[i]).any();
            dp[j].set(i, res);
        }
    }
    let ans = (0..n)
        .filter(|&i|
                dp[0].get(i) && dp[n - 1].get(i))
        .count();
    puts!("{}\n", ans);
}

fn main() {
    solve();
}
