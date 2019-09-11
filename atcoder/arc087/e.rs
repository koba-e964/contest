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

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

fn ex(n: usize) {
    let mut dp = vec![0; n + 1];
    dp[0] = 0;
    for i in 1..n + 1 {
        let mut hs = HashSet::new();
        let mut cur = 0;
        for j in 0..i {
            hs.insert(cur);
            cur ^= dp[i - 1 - j];
        }
        let mut mex = 0;
        while hs.contains(&mex) {
            mex += 1;
        }
        dp[i] = mex;
    }
    debugln!("{:?}", dp);
}

struct Trie {
    occ: bool,
    l: Option<Box<Trie>>,
    r: Option<Box<Trie>>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            occ: false,
            l: None,
            r: None,
        }
    }
    fn add(&mut self, s: &[char]) {
        if s.is_empty() {
            self.occ = true;
            return;
        }
        let idx = s[0] as usize - 48;
        if idx == 0 {
            if self.l.is_none() {
                self.l = Some(Box::new(Trie::new()));
            }
            self.l.as_mut().unwrap().add(&s[1..]);
            return;
        }
        if self.r.is_none() {
            self.r = Some(Box::new(Trie::new()));
        }
        self.r.as_mut().unwrap().add(&s[1..]);
    }
    fn lowest(x: i64) -> i64 {
        x & (-x)
    }
    fn grundy(&self, len: i64) -> i64 {
        let mut sum = 0;
        sum ^= match &self.l {
            &Some(ref ch) => ch.grundy(len - 1),
            &None => Self::lowest(len),
        };
        sum ^= match &self.r {
            &Some(ref ch) => ch.grundy(len - 1),
            &None => Self::lowest(len),
        };
        sum
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize, l: i64,
        s: [chars; n],
    }
    let mut trie = Trie::new();
    for s in s {
        trie.add(&s);
    }
    let g = trie.grundy(l);
    puts!("{}\n", if g == 0 { "Bob" } else { "Alice" });
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
