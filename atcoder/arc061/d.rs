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
    fn into_vec_sub(self, vec: &mut Vec<T>) {
        use Treap::*;
        match self {
            Tip => (),
            Bin(_, _, elem, left, right) => {
                left.into_vec_sub(vec);
                vec.push(elem);
                right.into_vec_sub(vec);
            }
        }
    }
    #[allow(dead_code)]
    pub fn into_vec(self) -> Vec<T> {
        let mut ret = Vec::new();
        self.into_vec_sub(&mut ret);
        ret
    }
}



fn solve() {
    let mut seed: i64 = 0x114514;
    let mut next = || {
        seed = seed.wrapping_mul(0x12345678deadc0d1).wrapping_add(0x1551);
        seed
    };
    next();
    let h: i32 = get();
    let w: i32 = get();
    let n = get();
    let mut pts = Treap::new();
    let mut nbr = Treap::new();
    for _ in 0 .. n {
        let a: i32 = get();
        let b: i32 = get();
        pts = pts.insert((a, b), next());
        for dx in -1 .. 2 {
            for dy in -1 .. 2 {
                let nx = a + dx;
                let ny = b + dy;
                if nx >= 2 && nx <= h - 1 && ny >= 2 && ny <= w - 1 {
                    nbr = nbr.insert((nx, ny), next());
                }
            }
        }
    }
    let mut a = [0i64; 10];
    for (cx, cy) in nbr.into_vec() {
        let mut cnt = 0;
        for dx in -1 .. 2 {
            for dy in -1 .. 2 {
                if pts.find_index(&(cx + dx, cy + dy)).1 {
                    cnt += 1;
                }
            }
        }
        a[cnt] += 1;
    }
    a[0] = (h as i64 - 2) * (w as i64 - 2);
    for i in 1 .. 10 {
        a[0] -= a[i];
    }
    for i in 0 .. 10 {
        println!("{}", a[i]);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
