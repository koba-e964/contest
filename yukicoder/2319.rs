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
            for i in 0..count - sh0 {
                ans.buf[i + sh0] = self.buf[i];
            }
        } else {
            ans.buf[sh0] = self.buf[0] << sh1;
            for i in 1..count - sh0 {
                ans.buf[i + sh0] = self.buf[i] << sh1
                    | self.buf[i - 1] >> (w - sh1);
            }
        }
        ans
    }
    // Verified by: https://www.hackerrank.com/contests/yfkpo4/challenges/e-strange-clock/submissions/code/1357435235
    #[allow(unused)]
    fn shr(&self, val: usize) -> Self {
        if val >= self.size { return Self::new(self.size); }
        let w = 8 * std::mem::size_of::<usize>();
        let count = self.size / w;
        let sh0 = val / w;
        let sh1 = val & (w - 1);
        let mut ans = Self::new(self.size);
        if sh1 == 0 {
            for i in 0..count - sh0 {
                ans.buf[i] = self.buf[i + sh0];
            }
        } else {
            for i in 0..count - sh0 - 1 {
                ans.buf[i] = self.buf[i + sh0] >> sh1
                    | self.buf[i + sh0 + 1] << (w - sh1);
            }
            ans.buf[count - sh0 - 1] = self.buf[count - 1] >> sh1;
        }
        ans
    }
    #[allow(unused)]
    fn msb(&self) -> Option<usize> {
        let w = 8 * std::mem::size_of::<usize>();
        let count = self.size / w;
        for i in (0..count).rev() {
            let v = self.buf[i];
            if v != 0 {
                return Some(w * i + w - 1 - v.leading_zeros() as usize);
            }
        }
        None
    }
}

// https://yukicoder.me/problems/no/2319 (3.5)
// 次数の大きい頂点を偉い頂点と呼ぶことにする。辺それぞれにたいして、偉くない側が偉い側に情報を push することにする。
// 拡張点に対して自分より偉い頂点は高々 sqrt(2M) 個であることに注意すると、各操作について以下のようにデータが移動することになる。
// 位置の変更: 自分の位置を変更後、自分より偉い頂点に情報を push
// フレンドがいるかどうかの取得: 自分が持っている偉くない頂点の情報と、自分より偉い頂点の情報を取得
// 計算量は O(Q sqrt(M) log(N)) である。
// -> TLE。各頂点が持つデータを木構造のマップからハッシュマップにしても TLE。
// -> bitset を使うことで計算量が O(QN/w) になり AC。
// 各エトワーニュくんに対するフレンドの biset と、各ワールドにいるエトワーニュくんの bitset を持っておく。
// これにより集合の共通部分が N/w 回の AND で計算できる。
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize,
        p: [usize1; n],
        ab: [(usize1, usize1); m],
        q: usize,
        xy: [(usize1, usize1); q],
    }
    let mut p = p;
    const W: usize = 20_032;
    let mut pb = vec![BitSet::new(W); n];
    for i in 0..n {
        pb[p[i]].set(i, true);
    }
    let mut g = vec![vec![]; n];
    let mut gb = vec![BitSet::new(W); n];
    for &(a, b) in &ab {
        if a != b {
            g[a].push(b as u32);
            g[b].push(a as u32);
            gb[a].set(b, true);
            gb[b].set(a, true);
        }
    }
    for (x, y) in xy {
        let mut found = false;
        for i in 0..gb[x].buf.len() {
            if (gb[x].buf[i] & pb[p[y]].buf[i]) != 0 {
                found = true;
                break;
            }
        }
        if !found || p[x] == p[y] {
            puts!("No\n");
            continue;
        }
        puts!("Yes\n");
        pb[p[x]].set(x, false);
        p[x] = p[y];
        pb[p[x]].set(x, true);
    }
}
