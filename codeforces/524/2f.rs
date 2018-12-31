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

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

// Verified by https://codeforces.com/contest/1080/submission/47790998
trait Monoid: Clone {
    type T: Clone;
    fn e() -> Self::T;
    fn op(&Self::T, &Self::T) -> Self::T;
}

#[derive(Clone, Debug)]
struct PersistentSegTree<M: Monoid> {
    size: usize,
    val: M::T,
    left: Option<std::rc::Rc<PersistentSegTree<M>>>,
    right: Option<std::rc::Rc<PersistentSegTree<M>>>,
}

impl<M: Monoid> PersistentSegTree<M> {
    pub fn new(n: usize) -> Self {
        use std::rc::Rc;
        assert!(n >= 1);
        if n == 1 {
            PersistentSegTree {
                size: 1,
                val: M::e(),
                left: None,
                right: None,
            }
        } else {
            let l = Self::new(n / 2);
            let r = Self::new(n - n / 2);
            PersistentSegTree {
                size: n,
                val: M::e(),
                left: Some(Rc::new(l)),
                right: Some(Rc::new(r)),
            }
        }
    }
    // This function breaks the immutability of self.
    pub fn inplace_update(&mut self, v: &M::T) {
        self.val = v.clone();
        if let Some(ref mut left) = self.left {
            std::rc::Rc::make_mut(left).inplace_update(v);
        }
        if let Some(ref mut right) = self.right {
            std::rc::Rc::make_mut(right).inplace_update(v);
        }
    }
    pub fn update(&self, k: usize, val: M::T) -> Self {
        use std::rc::Rc;
        let size = self.size;
        assert!(k < size);
        if size == 1 {
            return PersistentSegTree {
                size: 1,
                val: val,
                left: None,
                right: None,
            };
        }
        let mut l = Rc::clone(self.left.as_ref().unwrap());
        let mut r = Rc::clone(self.right.as_ref().unwrap());
        if k < size / 2 {
            l = Rc::new(l.update(k, val));
        } else {
            r = Rc::new(r.update(k - size / 2, val));
        }
        PersistentSegTree {
            size: self.size,
            val: M::op(&l.val, &r.val),
            left: Some(l),
            right: Some(r),
        }
    }
    pub fn query(&self, a: usize, b: usize) -> M::T {
        let size = self.size;
        assert!(b <= size);
        if a >= b { return M::e(); }
        if a == 0 && b == size { return self.val.clone(); }
        debug_assert!(size >= 2);
        let (lo, hi) = (a, min(b, size / 2));
        let left = self.left.as_ref().unwrap().query(lo, hi);
        let (lo, hi) = (max(a, size / 2) - size / 2, max(b, size / 2) - size / 2);
        let right = self.right.as_ref().unwrap().query(lo, hi);
        M::op(&left, &right)
    }
}

const INF: i64 = 1 << 50;

#[derive(Clone)]
struct PMin;
impl Monoid for PMin {
    type T = i64;
    fn e() -> i64 { INF }
    fn op(&a: &i64, &b: &i64) -> i64 { min(a, b) }
}

fn solve() {
    let n: usize = get();
    let m: usize = get();
    let k: usize = get();
    let mut events = Vec::new();
    for _ in 0 .. k {
        let l: i64 = get();
        let r: i64 = get();
        let p: usize = get::<usize>() - 1;
        events.push((r, l, p));
    }
    events.sort_unstable();
    let mut chrono = Vec::new();
    chrono.push(PersistentSegTree::<PMin>::new(n));
    chrono[0].inplace_update(&(-1 << 20));
    for pos in 0 .. events.len() {
        let (_, l, p) = events[pos];
        let old = chrono[pos].query(p, p + 1);
        let next = chrono[pos].update(p, max(old, l));
        chrono.push(next);
    }
    for _ in 0 .. m {
        let a: usize = get::<usize>() - 1;
        let b: usize = get::<usize>();
        let x: i64 = get();
        let y: i64 = get();
        let idx = events.binary_search(&(y, i64::max_value(), 0)).unwrap_err();
        let res = chrono[idx].query(a, b);
        if res >= x {
            println!("yes");
        } else {
            println!("no");
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
