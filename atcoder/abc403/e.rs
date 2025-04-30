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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

#[derive(Clone)]
struct Trie {
    tot: i64,
    annihil: bool,
    ch: Vec<Option<Box<Trie>>>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            tot: 0,
            annihil: false,
            ch: vec![None; 26],
        }
    }
    fn add(&mut self, s: &[char]) {
        if s.is_empty() {
            if self.annihil {
                return;
            }
            self.tot += 1;
            return;
        }
        let idx = s[0] as usize - 97;
        if self.ch[idx].is_none() {
            self.ch[idx] = Some(Box::new(Trie::new()));
        }
        let old = self.ch[idx].as_ref().unwrap().get_tot();
        self.ch[idx].as_mut().unwrap().add(&s[1..]);
        if self.annihil {
            return;
        }
        let new = self.ch[idx].as_ref().unwrap().get_tot();
        self.tot += new - old;
    }
    fn ban(&mut self, s: &[char]) {
        if s.is_empty() {
            self.annihil = true;
            return;
        }
        let idx = s[0] as usize - 97;
        if self.ch[idx].is_none() {
            self.ch[idx] = Some(Box::new(Trie::new()));
        }
        let old = self.ch[idx].as_ref().unwrap().get_tot();
        self.ch[idx].as_mut().unwrap().ban(&s[1..]);
        if self.annihil {
            return;
        }
        let new = self.ch[idx].as_ref().unwrap().get_tot();
        self.tot += new - old;
    }
    fn get_tot(&self) -> i64 {
        if self.annihil {
            return 0;
        }
        self.tot
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

// Tags: trie
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        q: usize,
        ts: [(i32, chars); q],
    }
    let mut trie = Trie::new();
    for (t, s) in ts {
        if t == 1 {
            trie.ban(&s);
        } else {
            trie.add(&s);
        }
        puts!("{}\n", trie.get_tot());
    }
}
