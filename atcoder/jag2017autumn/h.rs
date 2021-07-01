#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
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
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
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

// Ref: http://algoogle.hadrori.jp/algorithm/aho-corasick.html
// Verified by: https://atcoder.jp/contests/jsc2019-final/submissions/23661893
// Verified by: https://atcoder.jp/contests/joisc2010/submissions/23693164
// If no reference to the root remains, it does not work correctly.
struct PMA<T> {
    len: usize,
    next: Vec<Option<std::rc::Rc<std::cell::RefCell<Self>>>>,
    dp: T,
    back: std::rc::Weak<std::cell::RefCell<Self>>
}

impl<T: Clone> PMA<T> {
    fn new(len: usize, e: T) -> std::rc::Rc<std::cell::RefCell<Self>> {
        use std::rc::{Rc, Weak};
        use std::cell::RefCell;
        Rc::new(RefCell::new(PMA {
            len: len,
            next: vec![None; len],
            dp: e,
            back: Weak::new(),
        }))
    }
    #[allow(unused)]
    pub fn with_lower_strings<F: Fn(&[usize], usize) -> T, M: Fn(T, T) -> T>(pat: &[Vec<char>], f: F, m: M, e: T) -> std::rc::Rc<std::cell::RefCell<Self>> {
        let len = 26;
        let pat: Vec<Vec<_>> = pat.iter().map(|pat| pat.iter().map(|&x| (x as u8 - b'a') as _).collect()).collect();
        Self::with_arrays(len, &pat, f, m, e)
    }
    pub fn with_arrays<F: Fn(&[usize], usize) -> T, M: Fn(T, T) -> T>(len: usize, pat: &[Vec<usize>], f: F, m: M, e: T) -> std::rc::Rc<std::cell::RefCell<Self>> {
        use std::rc::{Rc, Weak};
        let root = Self::new(len, e.clone());
        let root_cp = Rc::clone(&root);
        let root_weak = Rc::downgrade(&root);
        root.borrow_mut().back = Weak::clone(&root_weak);
        for pat in pat {
            Self::add_pattern(root.clone(), &pat, &f, &m, e.clone());
        }
        let mut que = std::collections::VecDeque::new();
        for i in 0..len {
            if root.borrow().next[i].is_none() {
                root.borrow_mut().next[i] = Some(Rc::clone(&root_cp));
            } else {
                let tmp = root.borrow().next[i].clone().unwrap();
                tmp.borrow_mut().back = Weak::clone(&root_weak);
                que.push_back(tmp);
            }
        }
        while let Some(now) = que.pop_front() {
            for i in 0..len {
                if let Some(tmp) = now.borrow().next[i].clone() {
                    let mut nxt = Weak::upgrade(&now.borrow().back).unwrap();
                    while nxt.borrow().next[i].is_none() {
                        let val = Weak::upgrade(&nxt.borrow().back).unwrap();
                        nxt = val;
                    }
                    let to = nxt.borrow().next[i].clone().unwrap();
                    tmp.borrow_mut().back = Rc::downgrade(&to);
                    let newdp = m(tmp.borrow().dp.clone(), to.borrow().dp.clone());
                    tmp.borrow_mut().dp = newdp;
                    que.push_back(tmp);
                }
            }
        }
        root
    }
    fn add_pattern<F: Fn(&[usize], usize) -> T, M: Fn(T, T) -> T>(root: std::rc::Rc<std::cell::RefCell<Self>>, pat: &[usize], f: &F, m: &M, e: T) {
        let len = root.borrow().len;
        let mut now = root;
        for i in 0..pat.len() {
            let c = pat[i];
            if now.borrow().next[c].is_none() {
                now.borrow_mut().next[c] = Some(Self::new(len, e.clone()));
            }
            let val = now.borrow().next[c].clone().unwrap();
            now = val;
            let newdp = m(now.borrow().dp.clone(), f(&pat, i + 1));
            now.borrow_mut().dp = newdp;
        }
    }
    fn progress(mut pma: std::rc::Rc<std::cell::RefCell<Self>>, idx: usize)
                -> std::rc::Rc<std::cell::RefCell<Self>> {
        while pma.borrow().next[idx].is_none() {
            let val = std::rc::Weak::upgrade(&pma.borrow().back).unwrap();
            pma = val;
        }
        pma.borrow().next[idx].clone().unwrap()
    }
}

// Tags: aho-corasick
fn main() {
    input! {
        n: usize,
        s: [chars; n],
        t: chars,
    }
    let root = PMA::with_lower_strings(&s, |s, pos| if pos == s.len() {
        vec![pos]
    } else {
        vec![]
    }, |mut a, b| {
        a.extend(&b);
        a
    }, vec![]);
    let mut cur = root.clone();
    let l = t.len();
    let mut dp = vec![MInt::new(0); l + 1];
    dp[0] += 1;
    for i in 0..l {
        let c = t[i];
        let idx = (c as u8 - b'a') as usize;
        cur = PMA::progress(cur, idx);
        let mut me = MInt::new(0);
        for &v in &cur.borrow().dp {
            me += dp[i + 1 - v];
        }
        dp[i + 1] = me;
    }
    println!("{}", dp[l]);
}
