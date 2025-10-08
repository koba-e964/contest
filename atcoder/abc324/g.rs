fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

// References:
// - https://scrapbox.io/data-structures/Wavelet_Matrix
// - https://github.com/noshi91/n91lib_rs/blob/master/src/data_structure/wavelet_matrix.rs
// - https://github.com/noshi91/n91lib_rs/blob/master/src/other/bit.rs
// - https://github.com/noshi91/n91lib_rs/blob/master/src/data_structure/bit_vector.rs
// Fine-tuned for machines whose machine word is 64-bit.
// The code is a modified version of code in the references.
const WORD: usize = (!0usize).count_ones() as usize;

pub fn access(bit: usize, index: usize) -> bool {
    bit & 1 << index != 0
}

pub fn rank(bit: usize, end: usize) -> usize {
    (bit & !(!0 << end)).count_ones() as usize
}

pub fn select(bit: usize, k: usize) -> usize {
    macro_rules! select_impl {
        ($k: expr, $({$b: expr, $m: expr, $s: expr}),*) => {
            let mut k = $k;
            let mut r = 0;
            $(
                let b = ($b >> r & $m) as usize;
                if k >= b {
                    k -= b;
                    r += $s;
                }
            )*
            r
        }
    }

    let b0 = bit as u64;
    let b1 = (b0 & 0x5555555555555555) + (b0 >> 1 & 0x5555555555555555);
    let b2 = (b1 & 0x3333333333333333) + (b1 >> 2 & 0x3333333333333333);
    let b3 = b2 + (b2 >> 4) & 0x0F0F0F0F0F0F0F0F;
    let b4 = b3 + (b3 >> 8) & 0x00FF00FF00FF00FF;
    let b5 = b4 + (b4 >> 16) & 0x0000FFFF0000FFFF;
    let b6 = b5 + (b5 >> 32) & 0x00000000FFFFFFFF;
    if k >= b6 as usize {
        return 64;
    }

    #[allow(unused_assignments)]
    {
        select_impl! {
            k,
            {b5, 0xFFFFFFFF, 32},
            {b4, 0xFFFF, 16},
            {b3, 0xFF, 8},
            {b2, 0xF, 4},
            {b1, 0x3, 2},
            {b0, 0x1, 1}
        }
    }
}

pub struct BitVector {
    data: Box<[Node]>,
}

struct Node {
    bit: usize,
    sum: usize,
}

impl BitVector {
    pub fn access(&self, index: usize) -> bool {
        access(self.data[index / WORD].bit, index % WORD)
    }

    pub fn rank0(&self, end: usize) -> usize {
        end - self.rank1(end)
    }

    pub fn rank1(&self, end: usize) -> usize {
        let t = &self.data[end / WORD];
        t.sum + rank(t.bit, end % WORD)
    }

    pub fn select0(&self, k: usize) -> usize {
        let (mut st, mut en) = (0, self.data.len());
        while en - st != 1 {
            let mid = (st + en) / 2;
            if mid * WORD - self.data[mid].sum <= k {
                st = mid;
            } else {
                en = mid;
            }
        }
        let rem = k - (st * WORD - self.data[st].sum);
        st * WORD + select(!self.data[st].bit, rem)
    }

    pub fn select1(&self, k: usize) -> usize {
        let (mut st, mut en) = (0, self.data.len());
        while en - st != 1 {
            let mid = (st + en) / 2;
            if self.data[mid].sum <= k {
                st = mid;
            } else {
                en = mid;
            }
        }
        let rem = k - self.data[st].sum;
        st * WORD + select(self.data[st].bit, rem)
    }
}

impl std::iter::FromIterator<bool> for BitVector {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let mut v = Vec::new();
        let mut sum = 0;
        'base: loop {
            let mut bit = 0;
            for i in 0..WORD {
                match iter.next() {
                    Some(v) => if v {
                        bit |= 1 << i;
                    },
                    None => {
                        v.push(Node { bit: bit, sum: sum });
                        break 'base;
                    }
                }
            }
            v.push(Node { bit: bit, sum: sum });
            sum += bit.count_ones() as usize;
        }
        Self {
            data: v.into_boxed_slice(),
        }
    }
}

pub struct WaveletMatrix {
    data: Box<[(usize, BitVector)]>,
}

impl WaveletMatrix {
    pub fn new(bitlen: usize, mut seq: Vec<usize>) -> Self {
        for &s in &seq {
            assert!(bitlen == WORD || s < (1 << bitlen));
        }
        let len = seq.len();
        let mut data = Vec::new();
        for l in (0..bitlen).rev() {
            let v = seq.iter().map(|&x| access(x, l)).collect::<BitVector>();
            data.push((v.rank0(len), v));
            let zeros = seq.iter().filter(|&&x| !access(x, l)).cloned();
            let ones = seq.iter().filter(|&&x| access(x, l)).cloned();
            seq = zeros.chain(ones).collect();
        }
        Self {
            data: data
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    pub fn access(&self, mut index: usize) -> usize {
        let mut ret = 0;
        for (l, &(z, ref v)) in self.base_iter().rev() {
            if !v.access(index) {
                index = v.rank0(index);
            } else {
                ret |= 1 << l;
                index = z + v.rank1(index);
            }
        }
        ret
    }

    pub fn rank(&self, value: usize, mut range: std::ops::Range<usize>) -> usize {
        for (l, &(z, ref v)) in self.base_iter().rev() {
            if !access(value, l) {
                range.start = v.rank0(range.start);
                range.end = v.rank0(range.end);
            } else {
                range.start = z + v.rank1(range.start);
                range.end = z + v.rank1(range.end);
            }
        }
        range.end - range.start
    }

    pub fn select(&self, value: usize, k: usize) -> usize {
        let mut index = 0;
        for (l, &(z, ref v)) in self.base_iter().rev() {
            if !access(value, l) {
                index = v.rank0(index);
            } else {
                index = z + v.rank1(index);
            }
        }
        index += k;
        for (_, &(z, ref v)) in self.base_iter() {
            if index < z {
                index = v.select0(index);
            } else {
                index = v.select1(index - z);
            }
        }
        index
    }

    pub fn count(&self, idxrng: std::ops::Range<usize>, valrng: std::ops::Range<usize>) -> usize {
        self.count_to(idxrng.clone(), valrng.end) - self.count_to(idxrng, valrng.start)
    }

    pub fn quantile(&self, mut range: std::ops::Range<usize>, mut k: usize) -> usize {
        let mut ret = 0;
        for (l, &(z, ref v)) in self.base_iter().rev() {
            let zeros = v.rank0(range.end) - v.rank0(range.start);
            if zeros > k {
                range.start = v.rank0(range.start);
                range.end = v.rank0(range.end);
            } else {
                k -= zeros;
                ret |= 1 << l;
                range.start = z + v.rank1(range.start);
                range.end = z + v.rank1(range.end);
            }
        }
        ret
    }

    fn count_to(&self, mut range: std::ops::Range<usize>, val: usize) -> usize {
        let mut ret = 0;
        for (l, &(z, ref v)) in self.base_iter().rev() {
            if !access(val, l) {
                range.start = v.rank0(range.start);
                range.end = v.rank0(range.end);
            } else {
                ret += v.rank0(range.end) - v.rank0(range.start);
                range.start = z + v.rank1(range.start);
                range.end = z + v.rank1(range.end);
            }
        }
        ret
    }

    fn base_iter(&self) -> impl DoubleEndedIterator<Item = (usize, &(usize, BitVector))> {
        self.data.iter().enumerate()
    }
}

// Tags: wavelet-matrix
fn main() {
    getline();
    let a = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let n = a.len();
    let wm = WaveletMatrix::new(18, a);
    let q = getline().trim().parse::<i32>().unwrap();
    let mut arrays = vec![(0, n, 0, n + 1)];
    for _ in 0..q {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let [ty, s, x] = ints[..] else { panic!() };
        let (old, new) = if ty == 1 {
            if x == 0 {
                (
                    (arrays[s].0, arrays[s].0, arrays[s].2, arrays[s].3),
                    arrays[s],
                )
            } else {
                let mut pass = arrays[s].1;
                let mut fail = arrays[s].0;
                while pass - fail > 1 {
                    let mid = (pass + fail) / 2;
                    let c = wm.count(arrays[s].0..mid, arrays[s].2..arrays[s].3);
                    if c >= x {
                        pass = mid;
                    } else {
                        fail = mid;
                    }
                }
                (
                    (arrays[s].0, pass, arrays[s].2, arrays[s].3),
                    (pass, arrays[s].1, arrays[s].2, arrays[s].3),
                )
            }
        } else {
            let thresh = arrays[s].3.min(x + 1).max(arrays[s].2);
            (
                (arrays[s].0, arrays[s].1, arrays[s].2, thresh),
                (arrays[s].0, arrays[s].1, thresh, arrays[s].3),
            )
        };
        arrays[s] = old;
        arrays.push(new);
        println!("{}", wm.count(new.0..new.1, new.2..new.3));
    }
}
