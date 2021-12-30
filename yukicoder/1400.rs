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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
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

// TODO reference is not allowed as rhs
impl std::ops::BitXorAssign for BitSet {
    fn bitxor_assign(&mut self, other: BitSet) {
        debug_assert_eq!(self.size, other.size);
        for i in 0 .. self.buf.len() {
            unsafe {
                *self.buf.get_unchecked_mut(i) ^= *other.buf.get_unchecked(i);
            }
        }
    }
}
impl std::ops::BitOrAssign for BitSet {
    fn bitor_assign(&mut self, other: BitSet) {
        debug_assert_eq!(self.size, other.size);
        for i in 0 .. self.buf.len() {
            unsafe {
                *self.buf.get_unchecked_mut(i) |= *other.buf.get_unchecked(i);
            }
        }
    }
}
impl<'a> std::ops::BitOrAssign<&'a BitSet> for BitSet {
    fn bitor_assign(&mut self, other: &BitSet) {
        debug_assert_eq!(self.size, other.size);
        for i in 0 .. self.buf.len() {
            unsafe {
                *self.buf.get_unchecked_mut(i) |= *other.buf.get_unchecked(i);
            }
        }
    }
}

// https://yukicoder.me/problems/no/1400 (3.5)
// O(N^3/64) で ok。
// -> 新しく追加された頂点だけを見るところで実装を間違えた。
// Tags: diameter-of-graphs, dijkstra-like, difference-of-bitsets
// The author read the editorial before implementing this.
fn main() {
    input! {
        n: usize, d: i64,
        e: [chars; n],
    }
    const W: usize = 2048;
    let mut bs = vec![BitSet::new(W); n];
    for i in 0..n {
        for j in 0..n {
            if e[i][j] == '1' {
                bs[i].set(j, true);
            }
        }
    }
    let d = std::cmp::min(n as i64, d / 2) * 2 + (d % 2);
    let mut mat2 = vec![BitSet::new(W); n];
    for i in 0..n {
        for j in 0..n {
            if bs[i].get(j) {
                mat2[i] |= &bs[j];
            }
        }
    }
    // O(n^3/64)
    for i in 0..n {
        let mut r = BitSet::new(W);
        r.set(i, true);
        let mut v = vec![i];
        for _ in 0..d / 2 {
            if v.is_empty() { break; }
            let mut nr = r.clone();
            for k in v.drain(..) {
                nr |= &mat2[k];
            }
            let mut diff = nr.clone();
            diff ^= r;
            r = nr;
            for k in 0..diff.buf.len() {
                if diff.buf[k] != 0 {
                    for l in 0..64 { // only for 64-bit env
                        if (diff.buf[k] & 1 << l) != 0 {
                            v.push(64 * k + l);
                        }
                    }
                }
            }
        }
        if d % 2 == 1 {
            let mut nr = BitSet::new(W);
            for k in 0..n {
                if r.get(k) {
                    nr |= &bs[k];
                }
            }
            r = nr;
        }
        if !(0..n).all(|p| r.get(p)) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
