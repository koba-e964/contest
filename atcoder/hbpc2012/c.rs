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

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Dec {
    int: Vec<i32>,
    fin: Vec<i32>,
    rep: Vec<i32>,
}

fn parse(s: &[char]) -> Dec {
    let dot = s.iter().position(|&x| x == '.').unwrap();
    let int = s[..dot].iter().map(|&x| (x as u8 - b'0') as _).collect();
    let par = s.iter().position(|&x| x == '(');
    if let Some(par) = par {
        let fin = s[dot + 1..par].iter().map(|&x| (x as u8 - b'0') as _).collect();
        let rep = s[par + 1..s.len() - 1].iter().map(|&x| (x as u8 - b'0') as _).collect();
        Dec {
            int, fin, rep,
        }
    } else {
        let fin = s[dot + 1..].iter().map(|&x| (x as u8 - b'0') as _).collect();
        Dec {
            int, fin, rep: vec![],
        }
    }
}

fn reduce(d: Dec) -> Dec {
    let r = d.rep.len();
    if r == 0 {
        return remove_zero(d);
    }
    let mut l = 0;
    while l < d.fin.len() {
        if d.fin[d.fin.len() - 1 - l] != d.rep[r - 1 - l % r] {
            break;
        }
        l += 1;
    }
    let mut fin = d.fin[..d.fin.len() - l].to_vec();
    let mut rep = vec![10; r];
    for i in 0..r {
        rep[i] = d.rep[(i + r - l % r) % r];
    }
    let mut per = r;
    for p in 1..r {
        if r % p != 0 {
            continue;
        }
        let mut ok = true;
        for i in 0..r {
            if rep[i] != rep[i % p] {
                ok = false;
                break;
            }
        }
        if ok {
            per = p;
            break;
        }
    }
    rep = rep[..per].to_vec();
    let mut int = d.int;
    if rep == vec![0] {
        rep = vec![];
    }
    if rep == vec![9] {
        rep = vec![];
        // increment
        let mut carry = 1;
        for i in (0..fin.len()).rev() {
            fin[i] += carry;
            carry = fin[i] / 10;
            fin[i] %= 10;
        }
        for i in (0..int.len()).rev() {
            int[i] += carry;
            carry = int[i] / 10;
            int[i] %= 10;
        }
        if carry > 0 {
            int.insert(0, carry);
        }
    }
    remove_zero(Dec {
        int,
        fin,
        rep,
    })
}

fn remove_zero(mut d: Dec) -> Dec {
    if !d.rep.is_empty() {
        return d;
    }
    while d.fin.last() == Some(&0) {
        d.fin.pop();
    }
    d
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        s: [chars],
    }
    let mut set = HashSet::new();
    for s in s {
        let v = parse(&s);
        let v = reduce(v);
        set.insert(v);
    }
    puts!("{}\n", set.len());
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
