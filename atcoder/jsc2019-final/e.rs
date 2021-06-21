#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
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

// http://algoogle.hadrori.jp/algorithm/aho-corasick.html
struct PMA {
    len: usize,
    next: Vec<Option<std::rc::Rc<std::cell::RefCell<PMA>>>>,
    dp: i64,
    back: std::rc::Weak<std::cell::RefCell<PMA>>
}

impl PMA {
    fn new(len: usize) -> std::rc::Rc<std::cell::RefCell<PMA>> {
        use std::rc::{Rc, Weak};
        use std::cell::RefCell;
        Rc::new(RefCell::new(PMA {
            len: len,
            next: vec![None; len],
            dp: 1 << 60,
            back: Weak::new(),
        }))
    }
    pub fn with_lower_strings<F: Fn(&[usize], usize) -> i64>(pat: &[Vec<char>], f: F) -> std::rc::Rc<std::cell::RefCell<PMA>> {
        use std::rc::{Rc, Weak};
        let len = 26;
        let root = Self::new(len);
        let root_cp = Rc::clone(&root);
        let root_weak = Rc::downgrade(&root);
        root.borrow_mut().back = Weak::clone(&root_weak);
        for pat in pat {
            let pat: Vec<_> = pat.iter().map(|&x| (x as u8 - b'a') as _).collect();
            Self::add_pattern(root.clone(), &pat, &f);
        }
        let mut que = VecDeque::new();
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
                    let newdp = min(tmp.borrow().dp, to.borrow().dp);
                    tmp.borrow_mut().dp = newdp;
                    que.push_back(tmp);
                }
            }
        }
        root
    }
    fn add_pattern<F: Fn(&[usize], usize) -> i64>(root: std::rc::Rc<std::cell::RefCell<PMA>>, pat: &[usize], f: &F) {
        let len = root.borrow().len;
        let mut now = root;
        for i in 0..pat.len() {
            let c = pat[i];
            if now.borrow().next[c].is_none() {
                now.borrow_mut().next[c] = Some(Self::new(len));
            }
            let val = now.borrow().next[c].clone().unwrap();
            now = val;
            let newdp = min(now.borrow().dp, f(&pat, i + 1));
            now.borrow_mut().dp = newdp;
        }
    }
    fn progress(mut pma: std::rc::Rc<std::cell::RefCell<PMA>>, idx: usize)
                -> std::rc::Rc<std::cell::RefCell<PMA>> {
        while pma.borrow().next[idx].is_none() {
            let val = std::rc::Weak::upgrade(&pma.borrow().back).unwrap();
            pma = val;
        }
        pma.borrow().next[idx].clone().unwrap()
    }
}

// Tags: aho-corasick, string-algorithms, automaton
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, q: usize, x: i64, y: i64,
        s: [chars; n],
        t: [chars; q],
    }
    let root = PMA::with_lower_strings(&s, |s, idx| (s.len() - idx) as i64 * y - idx as i64 * x);
    let mil = s.iter().map(|s| s.len()).min().unwrap();
    for t in t {
        let mut cur = root.clone();
        let l = t.len();
        let mut mi = mil as i64 * y;
        for c in t {
            let idx = (c as u8 - b'a') as usize;
            cur = PMA::progress(cur, idx);
            mi.chmin(cur.borrow().dp);
        }
        puts!("{}\n", l as i64 * x + mi);
    }
}
