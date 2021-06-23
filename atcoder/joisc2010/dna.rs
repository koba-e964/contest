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

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

// Ref: http://algoogle.hadrori.jp/algorithm/aho-corasick.html
// Verified by: https://atcoder.jp/contests/jsc2019-final/submissions/23661893
// If no reference to the root remains, it does not work correctly.
struct PMA<T> {
    len: usize,
    next: Vec<Option<std::rc::Rc<std::cell::RefCell<Self>>>>,
    dp: T,
    back: std::rc::Weak<std::cell::RefCell<Self>>
}

impl<T: Copy> PMA<T> {
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
        let root = Self::new(len, e);
        let root_cp = Rc::clone(&root);
        let root_weak = Rc::downgrade(&root);
        root.borrow_mut().back = Weak::clone(&root_weak);
        for pat in pat {
            Self::add_pattern(root.clone(), &pat, &f, &m, e);
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
                    let newdp = m(tmp.borrow().dp, to.borrow().dp);
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
                now.borrow_mut().next[c] = Some(Self::new(len, e));
            }
            let val = now.borrow().next[c].clone().unwrap();
            now = val;
            let newdp = m(now.borrow().dp, f(&pat, i + 1));
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

// Tags: aho-corasick, string-algorithms, automaton
fn main() {
    input! {
        n: usize,
        s: chars,
        t: [chars; n],
    }
    let conv = |t: &[char]| {
        let a = ['A', 'G', 'C', 'T'];
        t.iter().map(|&c| a.iter().position(|&x| x == c).unwrap())
            .collect::<Vec<_>>()
    };
    let t: Vec<_> = t.iter().map(|t| conv(t)).collect();
    let root = PMA::with_arrays(4, &t, |s, x| if s.len() == x { x } else { 0 }, max, 0);
    let mut cur = root.clone();
    const INF: i32 = 1 << 20;
    let mut dp = vec![INF; s.len()];
    let s = conv(&s);
    dp[0] = 0;
    for i in 0..s.len() {
        cur = PMA::progress(cur, s[i]);
        let mut me = INF;
        let l = cur.borrow().dp;
        for j in i + 1 - l..i {
            me.chmin(dp[j]);
        }
        dp[i].chmin(me + 1);
    }
    println!("{}", max(1, dp[s.len() - 1]));
}
