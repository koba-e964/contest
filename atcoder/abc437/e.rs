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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn alloc(gl: &mut Vec<Trie>) -> usize {
    let c = gl.len();
    gl.push(Trie { ch: Default::default(), idx: vec![] });
    c
}

struct Trie {
    ch: BTreeMap<i64, usize>,
    idx: Vec<usize>,
}

fn add(
    me: usize,
    gl: &mut Vec<Trie>,
    val: i64, idx: usize,
) -> usize {
    let to = if let Some(&to) = gl[me].ch.get(&val) {
        to
    } else {
        let tmp = alloc(gl);
        gl[me].ch.insert(val, tmp);
        tmp
    };
    gl[to].idx.push(idx);
    to
}

fn dfs(gl: &[Trie], idx: usize, res: &mut Vec<usize>) {
    res.extend_from_slice(&gl[idx].idx);
    for (_, &to) in &gl[idx].ch {
        dfs(gl, to, res);
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize,
        xy: [(usize, i64); n],
    }
    let mut gl: Vec<Trie> = vec![];
    alloc(&mut gl);
    let mut re = vec![0; n + 1];
    for i in 0..n {
        let (x, y) = xy[i];
        let to = add(re[x], &mut gl, y, i + 1);
        re[i + 1] = to;
    }
    let mut res = vec![];
    dfs(&gl, 0, &mut res);
    putvec!(res);
}
