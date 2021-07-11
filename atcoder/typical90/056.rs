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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
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
}
impl std::ops::BitOrAssign for BitSet {
    fn bitor_assign(&mut self, other: BitSet) {
        debug_assert_eq!(self.size, other.size);
        for i in 0 .. self.buf.len() {
            self.buf[i] |= other.buf[i];
        }
    }
}

fn main() {
    input! {
        n: usize, s: usize,
        ab: [(usize, usize); n],
    }
    const W: usize = 100_032;
    let mut dp = vec![BitSet::new(0); n + 1];
    dp[0] = BitSet::new(W);
    dp[0].set(0, true);
    for i in 0..n {
        let (a, b) = ab[i];
        let mut me = dp[i].shl(a);
        me |= dp[i].shl(b);
        dp[i + 1] = me;
    }
    if !dp[n].get(s) {
        println!("Impossible");
        return;
    }
    let mut ans = vec!['+'; n];
    let mut cur = s;
    for i in (0..n).rev() {
        let (a, b) = ab[i];
        assert!(dp[i + 1].get(cur));
        if cur >= a && dp[i].get(cur - a) {
            ans[i] = 'A';
            cur -= a;
        } else {
            ans[i] = 'B';
            cur -= b;
        }
    }
    assert_eq!(cur, 0);
    println!("{}", ans.into_iter().collect::<String>());
}
