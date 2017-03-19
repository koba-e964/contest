#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Read;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}
fn get_word() -> String {
    let mut stdin = std::io::stdin();
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

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }


/**
 * Differential Segment Tree. This data structure is useful for fast folding 
 * a[l upto r].map(f) where f: M -> V, M and V are monoids.
 * Note that constructing this tree requires the identity
 * element of M.
 * Verified by: 
 */
struct DiffSegTree<I> {
    n: usize,
    dat: Vec<I>,
    e: I,
}

impl<I: Clone> DiffSegTree<I> {
    pub fn new(n_: usize, e: I) -> Self {
        let n = n_.next_power_of_two(); // n >= n_, n is a power of 2
        DiffSegTree {n: n, dat: vec![e.clone(); 2 * n - 1], e: e}
    }
    /* upd(&mut ary[k], v) */
    pub fn update<U: Clone, F>(&mut self, idx: usize, v: U, mut upd: F)
        where F: FnMut(I, U) -> I {
        let mut k = idx + self.n - 1;
        while k > 0 {
            let mut tmp = self.e.clone();
            std::mem::swap(&mut tmp, &mut self.dat[k]);
            self.dat[k] = upd(tmp, v.clone());
            k = (k - 1) / 2;
        }
    }
    /* l,r are for simplicity */
    fn query_sub<V, F, Fold>(&self, a: usize, b: usize, k: usize,
                             l: usize, r: usize,
                             f: &mut F, fold: &Fold) -> V
        where F: FnMut(&I) -> V, Fold: Fn(V, V) -> V {
        // [a,b) and  [l,r) intersects?
        if r <= a || b <= l { return f(&self.e); }
        if a <= l && r <= b { return f(&self.dat[k]); }
        let vl = self.query_sub(a, b, 2 * k + 1, l, (l + r) / 2, f, fold);
        let vr = self.query_sub(a, b, 2 * k + 2, (l + r) / 2, r, f, fold);
        fold(vl, vr)
    }
    /* [a, b] (note: inclusive) */
    pub fn query_map<V, F, Fold>(&self, a: usize, b: usize, mut f: F, fold: Fold)
                                 -> V
        where F: FnMut(&I) -> V, Fold: Fn(V, V) -> V {
        self.query_sub(a, b + 1, 0, 0, self.n, &mut f, &fold)
    }
}

/// Treap (balanced binary search tree)
/// Reference: https://www.slideshare.net/iwiwi/2-12188757
#[derive(Clone, Debug)]
enum Treap<T> {
    Bin(
        usize, // size
        i64, // priority
        T, // value
        Box<Treap<T>>, // left
        Box<Treap<T>>, // right
    ),
    Tip,
}

impl<T: Ord> Treap<T> {
    pub fn new() -> Self { Treap::Tip }
    pub fn singleton(v: T, pri: i64) -> Self {
        use Treap::*;
        Bin(1, pri, v, Box::new(Tip), Box::new(Tip))
    }
    pub fn size(&self) -> usize {
        use Treap::*;
        match *self {
            Tip => 0,
            Bin(t, _,  _, _, _) => t,
        }
    }
    // Merges two BST. Their ownership is taken.
    pub fn merge(left: Self, right: Self) -> Self {
        use Treap::*;
        match (left, right) {
            (Tip, Tip) => Tip,
            (Tip, x) => x,
            (x, Tip) => x,
            (Bin(lsize, lpri, lelem, lleft, lright),
             Bin(rsize, rpri, relem, rleft, rright)) => {
                if lpri > rpri {
                    let right = Bin(rsize, rpri, relem, rleft, rright);
                    let mut ret = Bin(lsize, lpri, lelem, lleft,
                                  Box::new(Self::merge(*lright, right)));
                    ret.update();
                    ret
                } else {
                    let left = Bin(lsize, lpri, lelem, lleft, lright);
                    let mut ret = Bin(rsize, rpri, relem,
                                      Box::new(Self::merge(left, *rleft)),
                                      rright);
                    ret.update();
                    ret
                }
            }
        }
    }
    pub fn split(self, k: usize) -> (Self, Self) {
        use Treap::*;
        match self {
            Tip => (Tip, Tip),
            Bin(size, pri, elem, left, right) => {
                if k <= left.size() {
                    let (sl, sr) = Self::split(*left, k);
                    let mut ret = Bin(size, pri, elem, Box::new(sr), right);
                    ret.update();
                    (sl, ret)
                } else {
                    let (sl, sr) = Self::split(*right, k - left.size() - 1);
                    let mut ret = Bin(size, pri, elem, left, Box::new(sl));
                    ret.update();
                    (ret, sr)
                }
            }
        }
    }
    fn update(&mut self) {
        use Treap::*;
        match *self {
            Tip => (),
            Bin(ref mut lsize, ref _pri, ref _elem, ref left, ref right) => {
                *lsize = left.size() + right.size() + 1;
            },
        }
    }
    fn insert_at(self, v: T, pri: i64, k: usize) -> Self {
        use Treap::*;
        // Speed up: compare the priority
        match self {
            Tip => Self::singleton(v, pri),
            Bin(size, spri, elem, left, right) => {
                let lsize = left.size();
                if spri <= pri {
                    let cself = Bin(size, spri, elem, left, right);
                    let (left, right) = cself.split(k);
                    return Bin(size + 1, pri, v,
                               Box::new(left), Box::new(right));
                }
                if k < lsize {
                    return Bin(size + 1, spri, elem,
                               Box::new((*left).insert_at(v, pri, k)),
                               right);
                }
                if k >= lsize + 1 {
                    return Bin(size + 1, spri, elem,
                               left,
                               Box::new((*right)
                                        .insert_at(v, pri, k - lsize - 1)));
                }
                let cself = Bin(size, spri, elem, left, right);
                let sing = Self::singleton(v, pri);
                let (left, right) = cself.split(k);
                let tmp = Self::merge(left, sing);
                Self::merge(tmp, right)
            }
        }
    }
    fn erase_at(self, k: usize) -> Self {
        use Treap::*;
        match self {
            Tip => Tip,
            Bin(size, pri, elem, left, right) => {
                if k < left.size() {
                    return Bin(size - 1, pri, elem,
                               Box::new((*left).erase_at(k)), right);
                }
                if k == left.size() {
                    return Self::merge(*left, *right); // hit
                }
                let lsize = left.size();
                return Bin(size - 1, pri, elem,
                           left,
                           Box::new((*right).erase_at(k - lsize - 1)));
            }
        }
    }
    fn find_index(&self, t: &T) -> (usize, bool) {
        use Treap::*;
        use std::cmp::Ordering;
        let mut offset = 0;
        let mut tap = self;
        loop {
            match *tap {
                Tip => return (offset, false),
                Bin(_, _, ref elem, ref left, ref right) => {
                    match elem.cmp(t) {
                        Ordering::Equal => return (offset + left.size(), true),
                        Ordering::Greater =>
                            tap = left,
                        Ordering::Less => {
                            offset += left.size() + 1;
                            tap = right;
                        },
                    }
                }
            }
        }
    }
    #[allow(dead_code)]
    fn insert(self, v: T, pri: i64) -> Self {
        let (idx, found) = self.find_index(&v);
        if found {
            self
        } else {
            self.insert_at(v, pri, idx)
        }
    }
    #[allow(dead_code)]
    fn erase(self, v: &T) -> Self {
        let (idx, found) = self.find_index(v);
        if found {
            self.erase_at(idx)
        } else {
            self
        }
    }
}

fn read_two_usize() -> (usize, usize) {
    let str = getline();
    let a: Vec<&str> = str.trim().split(" ").collect();
    (a[0].parse().unwrap(), a[1].parse().unwrap())
}

fn main() {
    let mut seed: i64 = 0x114514;
    let mut next = || {
        seed = seed.wrapping_mul(0x12345678deadc0d1).wrapping_add(0x1551);
        seed
    };
    next();
    let (n, q) = read_two_usize();
    let mut st = DiffSegTree::new(n, Treap::<usize>::new());
    let mut ary = vec![0; n];
    // Ugly hack
    for i in 0 .. n {
        st.dat[st.n - 1 + i] = Treap::new().insert(i + 1, next());
        ary[i] = i + 1;
    }
    for i in (0 .. st.n - 1).rev() {
        st.dat[i] = Treap::merge(st.dat[2 * i + 1].clone(), st.dat[2 * i + 2].clone());
    }
    let mut inv: i64 = 0;
    let adder = |x, y| x + y;
    for _ in 0 .. q {
        let (mut l, mut r) = read_two_usize();
        l -= 1;
        r -= 1;
        if l == r {
            println!("{}", inv);
            continue;
        }
        if l > r {
            let tmp = l;
            l = r;
            r = tmp;
        }
        inv -= st.query_map(l + 1, r - 1, |treap| treap.find_index(&ary[l]).0 as i32,
                            &adder) as i64;
        inv += st.query_map(l + 1, r - 1, |treap| treap.find_index(&ary[r]).0 as i32,
                            &adder) as i64;
        st.update(l, ary[l], |t, v| t.erase(&v));
        st.update(r, ary[r], |t, v| t.erase(&v));
        ary.swap(l, r);
        st.update(l, ary[l], |t, v| t.insert(v, next()));
        st.update(r, ary[r], |t, v| t.insert(v, next()));
        inv += st.query_map(l + 1, r - 1, |treap| treap.find_index(&ary[l]).0 as i32,
                            &adder) as i64;
        inv -= st.query_map(l + 1, r - 1, |treap| treap.find_index(&ary[r]).0 as i32,
                            &adder) as i64;
        if ary[l] > ary[r] {
            inv += 1;
        } else {
            inv -= 1;
        }
        println!("{}", inv);
    }
}
