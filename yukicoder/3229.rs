fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

// Verified by https://atcoder.jp/contests/arc084/submissions/3935443
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
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
    fn is_empty(&self) -> bool {
        self.buf.iter().all(|&x| x == 0)
    }
}

impl std::ops::BitXorAssign for BitSet {
    fn bitxor_assign(&mut self, other: BitSet) {
        *self ^= &other;
    }
}
impl std::ops::BitXorAssign<&BitSet> for BitSet {
    fn bitxor_assign(&mut self, other: &BitSet) {
        debug_assert_eq!(self.size, other.size);
        for i in 0..self.buf.len() {
            self.buf[i] ^= other.buf[i];
        }
    }
}
impl std::ops::BitOrAssign for BitSet {
    #[inline(always)]
    fn bitor_assign(&mut self, other: BitSet) {
        *self |= &other;
    }
}
impl std::ops::BitOrAssign<&BitSet> for BitSet {
    fn bitor_assign(&mut self, other: &BitSet) {
        debug_assert_eq!(self.size, other.size);
        for i in 0..self.buf.len() {
            self.buf[i] |= other.buf[i];
        }
    }
}
impl std::ops::BitOr for BitSet {
    type Output = Self;
    fn bitor(self, other: BitSet) -> Self {
        debug_assert_eq!(self.size, other.size);
        let mut ans = BitSet::new(self.size);
        for i in 0..self.buf.len() {
            ans.buf[i] = self.buf[i] | other.buf[i];
        }
        ans
    }
}
impl std::ops::BitAndAssign for BitSet {
    #[inline(always)]
    fn bitand_assign(&mut self, other: BitSet) {
        *self &= &other;
    }
}
impl std::ops::BitAndAssign<&BitSet> for BitSet {
    fn bitand_assign(&mut self, other: &BitSet) {
        debug_assert_eq!(self.size, other.size);
        for i in 0..self.buf.len() {
            self.buf[i] &= other.buf[i];
        }
    }
}

// Tags: noshi-basis, rank-nullity-theorem
fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let [n, m, k] = ints[..] else { panic!() };
    const W: usize = 2560;
    let mut basis = vec![];
    for _ in 0..m {
        let l = getline().trim().chars().collect::<Vec<_>>();
        let mut cur = BitSet::new(W);
        for i in 0..n {
            if l[i] == '1' {
                cur.set(i, true);
            }
        }
        for b in &basis {
            let mut cp = cur.clone();
            cp ^= b;
            if cur > cp {
                cur = cp;
            }
        }
        if !cur.is_empty() {
            basis.push(cur);
        }
    }
    let dim = n - basis.len();
    let mut ans = 1;
    for _ in 0..dim {
        ans = ans * 2 % k;
    }
    println!("{ans}");
}
