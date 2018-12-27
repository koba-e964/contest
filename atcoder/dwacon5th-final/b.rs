#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

#[derive(Clone)]
struct Trie {
    ch: Vec<Option<Box<Trie>>>,
    count: i32,
    level: usize,
}

impl Trie {
    fn new(level: usize) -> Trie {
        Trie {
            ch: vec![None; 2],
            count: 0,
            level: level,
        }
    }
    fn add(&mut self, a: i32) {
        self.count += 1;
        let level = self.level;
        if level == 0 {
            return;
        }
        let bit = (a >> (level - 1) & 1) as usize;
        if self.ch[bit].is_none() {
            self.ch[bit] = Some(Box::new(Trie::new(level - 1)));
        }
        self.ch[bit].as_mut().unwrap().add(a);
    }
    fn min(&self, a: i32, acc: i32) -> i32 {
        assert!(self.count > 0);
        let level = self.level;
        if level == 0 {
            return acc;
        }
        let mut bit = (a >> (level - 1) & 1) as usize;
        let first_count = match self.ch[bit] {
            None => 0,
            Some(ref ch) => ch.count,
        };
        if first_count == 0 {
            bit = 1 - bit;
        }
        self.ch[bit].as_ref().unwrap().min(a, acc ^ (bit as i32) << (level - 1))
    }
    fn remove(&mut self, a: i32) {
        self.count -= 1;
        let level = self.level;
        if level == 0 {
            return;
        }
        let bit = (a >> (level - 1) & 1) as usize;
        assert!(self.ch[bit].is_some());
        self.ch[bit].as_mut().unwrap().remove(a);
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut b = vec![0; n + 1];
    for i in 0 .. n {
        b[i + 1] = b[i] ^ a[i];
    }
    let mut trie = Trie::new(31);
    for i in 1 .. n {
        trie.add(b[i]);
    }
    let mut ans = vec![0; n + 1];
    for i in 1 .. n {
        let next = trie.min(ans[i - 1], 0);
        ans[i] = next;
        trie.remove(next);
    }
    ans[n] = b[n];
    for i in (0 .. n).rev() {
        ans[i + 1] ^= ans[i];
    }
    for i in 0 .. n {
        puts!("{}{}", ans[i + 1], if i == n - 1 { "\n" } else { " " });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
