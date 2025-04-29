/// References: http://noshi91.hatenablog.com/entry/2018/06/19/192741
///             https://github.com/noshi91/NoshiMochiLibrary/blob/master/WaveletMatrix/WaveletMatrix.noshi.cpp
/// Fine-tuned for machines whose machine word is 64-bit.
/// Not on Codeforces.
/// Verified:
/// at_least, range_freq (by https://beta.atcoder.jp/contests/bitflyer2018-final/submissions/2972019)
/// at_least, range_freq (by https://beta.atcoder.jp/contests/abc106/submissions/3066825)
type Word = u64;
struct BitVector {
    vec: Vec<Word>,
    dict: Vec<u32>,
}

impl BitVector {
    fn divide_digits(index: usize) -> (usize, usize) {
        (index >> 6, index & 0x3f)
    }
    pub fn build(a: &[bool]) -> Self {
        let n = a.len();
        let size = (n + 63) >> 6;
        let mut vec = vec![0; size];
        for i in 0 .. n {
            let (block, bit) = BitVector::divide_digits(i);
            if a[i] {
                vec[block] |= 1 << bit;
            }
        }
        let dict = BitVector::make_dict(&vec);
        BitVector {
            vec: vec,
            dict: dict,
        }
    }
    fn make_dict(a: &[Word]) -> Vec<u32> {
        let size = a.len();
        let mut dict = vec![0; size + 1];
        for i in 0 .. size {
            dict[i + 1] = dict[i] + a[i].count_ones();
        }
        dict
    }
    pub fn len(&self) -> u32 {
        self.vec.len() as u32 * 8
    }
    pub fn rank(&self, last: usize) -> u32 {
        let (block, bits) = BitVector::divide_digits(last);
        self.dict[block] + if bits == 0 { 0 } else { (self.vec[block] & ((1 << bits) - 1)).count_ones() }
    }
    pub fn access(&self, x: usize) -> bool {
        let (block, bit) = BitVector::divide_digits(x);
        (self.vec[block] >> bit & 1) != 0
    }
}

const BITS: usize = 64;
type ValueType = u64;
struct WaveletMatrix {
    len: usize,
    matrix: [BitVector; BITS],
    cnt: [u32; BITS],
}

impl WaveletMatrix {
    fn assert_valid_range(&self, first: usize, last: usize) {
        assert!(first <= self.len);
        assert!(last <= self.len);
        assert!(first <= last);
    }
    pub fn new(a: &[ValueType]) -> Self {
        let n = a.len();
        let mut matrix: [BitVector; BITS];
        let mut vec0 = a.to_vec();
        let mut bit_vec = vec![false; n];
        let mut tmp = 1 << (BITS - 1);
        let mut vec_l = vec![0; n].into_boxed_slice();
        let mut vec_r = vec![0; n].into_boxed_slice();
        let mut cnt = [0; BITS];
        unsafe {
            matrix = std::mem::uninitialized();
            for (i, elem) in matrix.iter_mut().enumerate() {
                let cur = tmp;
                tmp >>= 1;
                let mut l = 0;
                let mut r = 0;
                for j in 0 .. n {
                    if (vec0[j] & cur) != 0 {
                        vec_r[r] = vec0[j];
                        r += 1;
                        bit_vec[j] = true;
                    } else {
                        vec_l[l] = vec0[j];
                        l += 1;
                        bit_vec[j] = false;
                    }
                }
                let bv = BitVector::build(&bit_vec);
                std::ptr::write(elem, bv);
                vec0[..l].copy_from_slice(&vec_l[..l]);
                vec0[l..].copy_from_slice(&vec_r[..r]);
                cnt[i] = l as u32;
            }
        }
        WaveletMatrix { len: n, matrix: matrix, cnt: cnt }
    }
    #[allow(unused)]
    pub fn len(&self) -> usize { self.len }
    #[allow(unused)]
    pub fn access(&self, mut index: usize) -> ValueType {
        assert!(index < self.len);
        let mut ret = 0;
        let mut tmp = 1 << (BITS - 1);
        for (i, v) in self.matrix.iter().enumerate() {
            if v.access(index) {
                ret |= tmp;
                index = (v.rank(index) + self.cnt[i]) as usize;
            } else {
                index -= v.rank(index) as usize;
            }
            tmp >>= 1;
        }
        ret
    }
    #[allow(unused)]
    pub fn rank(&self, mut first: usize, mut last: usize, x: ValueType)
                -> usize {
        self.assert_valid_range(first, last);
        let mut tmp = 1 << (BITS - 1);
        for (i, v) in self.matrix.iter().enumerate() {
            let l = v.rank(first) as usize;
            let r = v.rank(last) as usize;
            if (x & tmp) != 0 {
                first = l + self.cnt[i] as usize;
                last = r + self.cnt[i] as usize;
            } else {
                first -= l;
                last -= r;
            }
            tmp >>= 1;
        }
        last - first
    }
    #[allow(unused)]
    pub fn quantile(&self, mut first: usize, mut last: usize, mut k: usize)
                    -> ValueType {
        assert!(first < self.len);
        assert!(last <= self.len);
        assert!(first < last);
        assert!(last - first > k);
        let mut ret = 0;
        let mut tmp = 1 << (BITS - 1);
        for (i, v) in self.matrix.iter().enumerate() {
            let l = v.rank(first) as usize;
            let r = v.rank(last) as usize;
            if r - l > k {
                first = l + self.cnt[i] as usize;
                last = r + self.cnt[i] as usize;
                ret |= tmp;
            } else {
                first -= l;
                last -= r;
                k -= r - l;
            }
            tmp >>= 1;
        }
        ret
    }
    pub fn at_least(&self, mut first: usize, mut last: usize, x: ValueType)
                    -> usize {
        self.assert_valid_range(first, last);
        let mut ret = 0;
        let mut tmp = 1 << (BITS - 1);
        for (i, v) in self.matrix.iter().enumerate() {
            let l = v.rank(first) as usize;
            let r = v.rank(last) as usize;
            if (x & tmp) != 0 {
                first = l + self.cnt[i] as usize;
                last = r + self.cnt[i] as usize;
            } else {
                ret += r - l;
                first -= l;
                last -= r;
            }
            tmp >>= 1;
        }
        ret + (last - first)
    }
    #[allow(unused)]
    pub fn range_freq(&self, first: usize, last: usize, x: ValueType, y: ValueType)
                      -> usize {
        self.at_least(first, last, x) - self.at_least(first, last, y)
    }
}
