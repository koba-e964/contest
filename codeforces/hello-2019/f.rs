#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Read;
use std::io::{Write, BufWriter};

#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

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
            self.buf[i] ^= other.buf[i];
        }
    }
}

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    const W: usize = (7001 + 63) / 64 * 64;
    let n: usize = get();
    let q: usize = get();
    let mut bit = vec![BitSet::new(W); n];
    let uw = 8 * std::mem::size_of::<usize>();
    let mut pool = vec![BitSet::new(W); W];
    let mut moebius = vec![BitSet::new(W); W];
    for i in 1 .. W {
        for j in 1 .. (W - 1) / i + 1 {
            pool[i * j].set(i, true);
        }
    }
    {
        let mut m = vec![0; W];
        for i in 2 .. W {
            if m[i] != 0 { continue; }
            for j in 1 .. (W - 1) / i + 1 {
                m[i * j] = max(m[i * j], 1);
            }
            for j in 1 .. (W - 1) / i / i + 1 {
                m[i * i * j] = 2;
            }
        }
        for i in 1 .. W {
            for j in 0 .. (W - 1) / i + 1 {
                if m[j] != 2 {
                    moebius[i].set(i * j, true);
                }
            }
        }
    }
    for _ in 0 .. q {
        let ty: i32 = get();
        let x: usize = get::<usize>() - 1;
        match ty {
            1 => {
                let v: usize = get();
                for i in 0 .. W / uw {
                    bit[x].buf[i] = pool[v].buf[i];
                }
            },
            2 => {
                let y: usize = get::<usize>() - 1;
                let z: usize = get::<usize>() - 1;
                for i in 0 .. W / uw {
                    bit[x].buf[i] = bit[y].buf[i] ^ bit[z].buf[i];
                }
            },
            3 => {
                let y: usize = get::<usize>() - 1;
                let z: usize = get::<usize>() - 1;
                for i in 0 .. W / uw {
                    bit[x].buf[i] = bit[y].buf[i] & bit[z].buf[i];
                }
            },
            4 => {
                let v: usize = get();
                let mut ans = 0;
                for i in 0 .. W / uw {
                    ans += (bit[x].buf[i] & moebius[v].buf[i]).count_ones();
                }
                puts!("{}", ans % 2);
            },
            _ => unreachable!(),
        }
    }
    puts!("\n");
}

fn main() {
    solve();
}
