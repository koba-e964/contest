use std::cmp::*;
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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

struct Rng {
    x: u64,
}

impl Rng {
    fn new() -> Self {
        use std::hash::{Hasher, BuildHasher};
        let hm = std::collections::HashMap::<i32, i32>::new();
        let mut hash = hm.hasher().build_hasher();
        hash.write_u32(8128);
        Rng {
            x: hash.finish(),
        }
    }
    fn next(&mut self) -> u64 {
        let a = 0xdead_c0de_0013_3331u64;
        let b = 2457;
        self.x = self.x.wrapping_mul(a).wrapping_add(b);
        let x = self.x;
        x ^ x << 10
    }
}

/// Verified by https://atcoder.jp/contests/abc198/submissions/21774342
mod mod_int {
    use std::ops::*;
    pub trait Mod: Copy { fn m() -> i64; }
    #[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ModInt<M> { pub x: i64, phantom: ::std::marker::PhantomData<M> }
    impl<M: Mod> ModInt<M> {
        // x >= 0
        pub fn new(x: i64) -> Self { ModInt::new_internal(x % M::m()) }
        fn new_internal(x: i64) -> Self {
            ModInt { x: x, phantom: ::std::marker::PhantomData }
        }
        pub fn pow(self, mut e: i64) -> Self {
            debug_assert!(e >= 0);
            let mut sum = ModInt::new_internal(1);
            let mut cur = self;
            while e > 0 {
                if e % 2 != 0 { sum *= cur; }
                cur *= cur;
                e /= 2;
            }
            sum
        }
        #[allow(dead_code)]
        pub fn inv(self) -> Self { self.pow(M::m() - 2) }
    }
    impl<M: Mod, T: Into<ModInt<M>>> Add<T> for ModInt<M> {
        type Output = Self;
        fn add(self, other: T) -> Self {
            let other = other.into();
            let mut sum = self.x + other.x;
            if sum >= M::m() { sum -= M::m(); }
            ModInt::new_internal(sum)
        }
    }
    impl<M: Mod, T: Into<ModInt<M>>> Sub<T> for ModInt<M> {
        type Output = Self;
        fn sub(self, other: T) -> Self {
            let other = other.into();
            let mut sum = self.x - other.x;
            if sum < 0 { sum += M::m(); }
            ModInt::new_internal(sum)
        }
    }
    impl<M: Mod, T: Into<ModInt<M>>> Mul<T> for ModInt<M> {
        type Output = Self;
        fn mul(self, other: T) -> Self { ModInt::new(self.x * other.into().x % M::m()) }
    }
    impl<M: Mod, T: Into<ModInt<M>>> AddAssign<T> for ModInt<M> {
        fn add_assign(&mut self, other: T) { *self = *self + other; }
    }
    impl<M: Mod, T: Into<ModInt<M>>> SubAssign<T> for ModInt<M> {
        fn sub_assign(&mut self, other: T) { *self = *self - other; }
    }
    impl<M: Mod, T: Into<ModInt<M>>> MulAssign<T> for ModInt<M> {
        fn mul_assign(&mut self, other: T) { *self = *self * other; }
    }
    impl<M: Mod> Neg for ModInt<M> {
        type Output = Self;
        fn neg(self) -> Self { ModInt::new(0) - self }
    }
    impl<M> ::std::fmt::Display for ModInt<M> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            self.x.fmt(f)
        }
    }
    impl<M: Mod> ::std::fmt::Debug for ModInt<M> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            let (mut a, mut b, _) = red(self.x, M::m());
            if b < 0 {
                a = -a;
                b = -b;
            }
            write!(f, "{}/{}", a, b)
        }
    }
    impl<M: Mod> From<i64> for ModInt<M> {
        fn from(x: i64) -> Self { Self::new(x) }
    }
    // Finds the simplest fraction x/y congruent to r mod p.
    // The return value (x, y, z) satisfies x = y * r + z * p.
    fn red(r: i64, p: i64) -> (i64, i64, i64) {
        if r.abs() <= 10000 {
            return (r, 1, 0);
        }
        let mut nxt_r = p % r;
        let mut q = p / r;
        if 2 * nxt_r >= r {
            nxt_r -= r;
            q += 1;
        }
        if 2 * nxt_r <= -r {
            nxt_r += r;
            q -= 1;
        }
        let (x, z, y) = red(nxt_r, r);
        (x, y - q * z, z)
    }
} // mod mod_int

macro_rules! define_mod {
    ($struct_name: ident, $modulo: expr) => {
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct $struct_name {}
        impl mod_int::Mod for $struct_name { fn m() -> i64 { $modulo } }
    }
}
const MOD: i64 = 1_000_000_007;
define_mod!(P, MOD);
type MInt = mod_int::ModInt<P>;

// http://www.prefield.com/algorithm/container/avl_tree.html
// https://atcoder.jp/contests/code-festival-2014-exhibition-open/tasks/code_festival_exhibition_b
// https://atcoder.jp/contests/code-festival-2014-exhibition-open/submissions/23724473
#[derive(Clone)]
struct Node<T, F> {
    val: T,
    sum: T,
    f: F,
    e: T,
    size: usize,
    height: i32,
    ch: [Option<Box<Self>>; 2],
}

impl<T: Default + Copy, F: Fn(T, T) -> T + Copy> Node<T, F> {
    fn new(val: T, f: F, e: T) -> Self {
        Node {
            val: val,
            sum: val,
            f: f,
            e: e,
            size: 1,
            height: 1,
            ch: [None, None],
        }
    }
    // Insert a single element represented as a leaf node.
    fn insert(l: Option<Box<Self>>, x: Self, pos: usize) -> Option<Box<Self>> {
        let mut t = match l {
            None => return Some(Box::new(x)),
            Some(l) => l
        };
        let llen = Self::len(&t.ch[0]);
        if pos <= llen {
            t.ch[0] = Node::insert(t.ch[0].take(), x, pos);
        } else {
            t.ch[1] = Node::insert(t.ch[1].take(), x, pos - 1 - llen);
        }
        Node::cons(&mut t);
        Some(Node::balance(t))
    }
    fn erase(t: Option<Box<Self>>, pos: usize) -> Option<Box<Self>> {
        let mut t = match t {
            None => return None,
            Some(l) => l
        };
        let llen = Self::len(&t.ch[0]);
        if pos < llen {
            t.ch[0] = Self::erase(t.ch[0].take(), pos);
        } else if pos == llen {
            return Self::move_down(t.ch[0].take(), t.ch[1].take());
        } else {
            t.ch[1] = Node::erase(t.ch[1].take(), pos - 1 - llen);
        }
        Node::cons(&mut t);
        Some(Node::balance(t))
        
    }
    fn move_down(l: Option<Box<Self>>, r: Option<Box<Self>>) -> Option<Box<Self>> {
        let mut t = match l {
            Some(t) => t,
            None => return r,
        };
        t.ch[1] = Self::move_down(t.ch[1].take(), r);
        Some(Self::balance(t))
    }
    fn len(t: &Option<Box<Self>>) -> usize {
        if let &Some(ref x) = t {
            x.size
        } else {
            0
        }
    }
    fn ht(t: &Option<Box<Self>>) -> i32 {
        if let &Some(ref x) = t {
            x.height
        } else {
            0
        }
    }
    // Make t consistent.
    fn cons(t: &mut Box<Self>) {
        t.sum = t.val;
        if let Some(ref ch) = t.ch[0] {
            t.sum = (t.f)(ch.sum, t.sum);
        }
        if let Some(ref ch) = t.ch[1] {
            t.sum = (t.f)(t.sum, ch.sum);
        }
        t.height = std::cmp::max(Node::ht(&t.ch[0]), Node::ht(&t.ch[1])) + 1;
        t.size = 1 + Node::len(&t.ch[0]) + Node::len(&t.ch[1]);
    }
    fn rotate(mut t: Box<Self>, l: usize, r: usize) -> Box<Self> {
        let mut s = t.ch[r].take().unwrap();
        t.ch[r] = s.ch[l].take();
        Node::cons(&mut t);
        s.ch[l] = Some(Self::balance(t));
        Node::cons(&mut s);
        Self::balance(s)
    }
    fn balance(mut t: Box<Self>) -> Box<Self> {
        for i in 0..2 {
            if Self::ht(&t.ch[1 - i]) - Self::ht(&t.ch[i]) < -1 {
                let tmp = t.ch[i].as_mut().unwrap();
                if Self::ht(&tmp.ch[1 - i]) - Self::ht(&tmp.ch[i]) > 0 {
                    *tmp = 
                        Self::rotate(std::mem::replace(tmp, Box::new(Node::new(T::default(), t.f, t.e))), i, 1 - i);
                }
                return Self::rotate(t, 1 - i , i);
            }
        }
        Node::cons(&mut t);
        t
    }
    fn each<F1: FnMut(T)>(t: Option<Box<Self>>, f: &mut F1) {
        let mut t = match t {
            None => return,
            Some(t) => t,
        };
        Node::each(t.ch[0].take(), f);
        f(t.val);
        Node::each(t.ch[1].take(), f);
    }
    fn range(t: &Box<Self>, l: usize, r: usize) -> T {
        if l == 0 && r >= t.size {
            return t.sum;
        }
        let llen = Self::len(&t.ch[0]);
        let mut left = t.e;
        let mut me = t.e;
        let mut right = t.e;
        if l < llen {
            left = Self::range(t.ch[0].as_ref().unwrap(), l, min(r, llen));
        }
        if l <= llen && r >= llen + 1 {
            me = t.val;
        }
        if r > llen + 1 {
            right = Self::range(t.ch[1].as_ref().unwrap(),
                                max(llen + 1, l) - llen - 1, r - llen - 1);
        }
        (t.f)((t.f)(left, me), right)
    }
    // Find min x s.t. p(t.range(0..x)), or |t| + 1 if there is no such x
    fn binary_search<M: Fn(T) -> bool>(t: &Option<Box<Self>>, cur: T, p: &M) -> usize {
        if p(cur) {
            return 0;
        }
        let t = if let Some(ref t) = t {
            t
        } else {
            return 1;
        };
        if !p((t.f)(cur, t.sum)) {
            return t.size + 1;
        }
        let sub = Node::binary_search(&t.ch[0], cur, p);
        let llen = Self::len(&t.ch[0]);
        if sub <= llen {
            return sub;
        }
        let lsum = if let Some(ref l) = t.ch[0] {
            l.sum
        } else {
            t.e
        };
        let tmp = (t.f)(lsum, t.val);
        if p(tmp) {
            return llen + 1;
        }
        llen + 1 + Self::binary_search(&t.ch[1], (t.f)(cur, tmp), p)
    }
}

#[derive(Clone)]
struct SeqAVLTree<T, F> {
    root: Option<Box<Node<T, F>>>,
    f: F,
    e: T,
}

impl<T: Default + Copy, F: Fn(T, T) -> T + Copy> SeqAVLTree<T, F> {
    fn new(f: F, e: T) -> Self {
        SeqAVLTree { root: None, f: f, e: e }
    }
    #[allow(unused)]
    fn len(&self) -> usize {
        Node::len(&self.root)
    }
    fn insert(&mut self, val: T, pos: usize) {
        self.root = Node::insert(self.root.take(), Node::new(val, self.f, self.e), pos);
    }
    #[allow(unused)]
    fn erase(&mut self, pos: usize) {
        self.root = Node::erase(self.root.take(), pos);
    }
    #[allow(unused)]
    fn each<F1: FnMut(T)>(self, mut f: F1) {
        Node::each(self.root, &mut f)
    }
    fn range(&self, rng: std::ops::Range<usize>) -> T {
        let l = rng.start;
        let r = rng.end;
        if let Some(ref x) = self.root {
            Node::range(x, l, r)
        } else {
            self.e
        }
    }
    fn binary_search<M: Fn(T) -> bool>(&self, f: M) -> usize {
        Node::binary_search(&self.root, self.e, &f)
    }
}

/*
 * Suffix Array by Manber & Myers.
 * Verified by: AtCoder ARC050 (http://arc050.contest.atcoder.jp/submissions/818912)
 * Reference: http://mayokoex.hatenablog.com/entry/2016/04/03/145845
 */
fn create_sa<T: Ord + Clone>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut sa: Vec<usize> = (0 .. n + 1).collect();
    let mut rank: Vec<usize> = vec![0; n + 1];
    let mut tmp = vec![0; n + 1];

    let mut coord = s.to_vec();
    coord.sort();
    coord.dedup();

    for i in 0..n + 1 {
        rank[i] = if i < n { coord.binary_search(&s[i]).unwrap() + 1 } else { 0_usize };
    }
    let mut k = 1;
    while k <= n {
        {
            let key = |i: &usize| {
                let ri = if i + k <= n { rank[i + k] as i32 } else { -1 };
                (rank[*i], ri)
            };
            sa.sort_by_key(&key);
            tmp[sa[0]] = 0;
            for i in 1 .. n + 1 {
                tmp[sa[i]] = tmp[sa[i - 1]]
                    + if key(&sa[i - 1]) < key(&sa[i]) { 1 } else { 0 };
            }
        }
        rank.clone_from_slice(&tmp);
        k *= 2;
    }
    return sa;
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

const B: usize = 2;

fn get_hash(hash: &[[MInt; B]], pw: &[[MInt; B]], l: usize, r: usize) -> [MInt; B] {
    let mut ans = [MInt::new(0); B];
    unsafe {
        for i in 0..B {
            *ans.get_unchecked_mut(i) =
                *hash.get_unchecked(r).get_unchecked(i)
                - *hash.get_unchecked(l).get_unchecked(i) *
                *pw.get_unchecked(r - l).get_unchecked(i);
        }
    }
    ans
}

// hash is reversed
fn get_lcp(hash: &[[MInt; B]], pw: &[[MInt; B]], a: usize, b: usize) -> usize {
    let len = hash.len() - 1;
    let mut lpass = 0;
    let mut lfail = len - max(a, b) + 1;
    while lfail - lpass > 1 {
        let lmid = (lpass + lfail) / 2;
        let hmid = get_hash(&hash, &pw, len - a - lmid, len - a);
        let hpos = get_hash(&hash, &pw, len - b - lmid, len - b);
        if hmid == hpos {
            lpass = lmid;
        } else {
            lfail = lmid;
        }
    }
    lpass
}

// Tags: string-reversal, incremental-suffix-array, constant-factor-optimization
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        s: chars,
        u: chars,
    }
    let n = s.len();
    let q = u.len();
    let mut t = vec!['+'; n + q]; // reversed
    t[q..].copy_from_slice(&s);
    t[q..].reverse();
    let sa_a = create_sa(&t[q..]);
    let mut sa = SeqAVLTree::new(min, 1 << 30);
    for i in 0..n + 1 {
        sa.insert(sa_a[i] + q, i);
    }
    let mut rng = Rng::new();
    let mut base = [MInt::new(0); B];
    for i in 0..B {
        base[i] = MInt::new((rng.next() >> 32) as i64);
    }
    let mut hash = vec![[MInt::new(0); B]; n + q + 1]; // NOT reversed
    for i in 0..n {
        for j in 0..B {
            hash[i + 1][j] = hash[i][j] * base[j] + (s[i] as u8 - b'A') as i64;
        }
    }
    let mut pw = vec![[MInt::new(0); B]; n + q + 1];
    for j in 0..B {
        pw[0][j] = 1.into();
    }
    for i in 0..n {
        for j in 0..B {
            pw[i + 1][j] = pw[i][j] * base[j];
        }
    }
    let mut tpos = q;
    let mut cur = 0;
    // calculate cur
    macro_rules! find_delta {
        ($pos:expr) => {{
            let pos = $pos;
            let tpos = sa.range(pos..pos + 1);
            let len = n + q - tpos;
            let l0 = if pos > 0 {
                let idx0 = sa.range(pos - 1..pos);
                get_lcp(&hash, &pw, idx0, tpos)
            } else {
                0
            };
            let l1 = if pos + 1 <= len {
                let idx1 = sa.range(pos + 1..pos + 2);
                get_lcp(&hash, &pw, idx1, tpos)
            } else {
                0
            };
            let mut lo = min(l0, l1);
            let hi = max(l0, l1);
            let l2 = if pos >= 2 {
                let idx0 = sa.range(pos - 2..pos - 1);
                get_lcp(&hash, &pw, idx0, tpos)
            } else {
                0
            };
            let l3 = if pos + 2 <= len {
                let idx1 = sa.range(pos + 2..pos + 3);
                get_lcp(&hash, &pw, idx1, tpos)
            } else {
                0
            };
            lo.chmax(l2);
            lo.chmax(l3);
            (hi - lo) as i64
        }}
    }
    for i in 1..n + 1 {
        let l0 = get_lcp(&hash, &pw, sa_a[i - 1] + q, sa_a[i] + q);
        let l2 = if i >= 2 {
            get_lcp(&hash, &pw, sa_a[i - 2] + q, sa_a[i] + q)
        } else {
            0
        };
        cur += (l0 - l2) as i64;
    }
    puts!("{}\n", cur);
    for c in u {
        if c == '-' {
            // Remove tpos-1 from sa
            let pos = sa.binary_search(|x| x == tpos) - 1;
            let del = find_delta!(pos);
            sa.erase(pos);
            // update cur
            cur -= del;
            puts!("{}\n", cur);
            tpos += 1;
            continue;
        }
        tpos -= 1;
        t[tpos] = c;
        for j in 0..B {
            hash[n + q - tpos][j] = hash[n + q - tpos - 1][j] * base[j]
                + (c as u8 - b'A') as i64;
        }
        let len = n + q - (tpos + 1);
        assert_eq!(sa.len(), len + 1);
        let mut pass = 1;
        let mut fail = len + 2;
        while fail - pass > 1 {
            let mid = (pass + fail) / 2;
            // t[sa[mid - 1]..] <= t[tpos..]?
            let samid = sa.range(mid - 1..mid);
            let lpass = get_lcp(&hash, &pw, samid, tpos);
            if lpass == n + q - samid || t[samid + lpass] < t[tpos + lpass] {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        sa.insert(tpos, pass);
        let del = find_delta!(pass);
        cur += del;
        puts!("{}\n", cur);
    }
}
