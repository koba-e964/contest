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

#[derive(Clone, Copy)]
struct Input<'a> {
    n: usize,
    s: &'a [char],
    a: &'a [i32],
    l: usize,
}

#[derive(Clone, Debug)]
struct State {
    s: Vec<char>,
    l: usize,
    r: usize,
}

#[derive(Clone, Debug)]
enum Tree {
    Num(usize),
    Match(Box<Tree>, Box<Tree>),
}

fn dfs(t: &Tree) -> (usize, i32, Vec<(usize, i32)>) {
    match *t {
        Tree::Num(v) => (v, 0, vec![]),
        Tree::Match(ref win, ref lose) => {
            let (tl, twl, mut losses) = dfs(win);
            let (tr, twr, losses_sub) = dfs(lose);
            losses.extend_from_slice(&losses_sub);
            losses.push((tr, twr));
            (tl, twl + 1, losses)
        }
    }
}

fn check(a: &[i32], t: &Tree) -> bool {
    let (top, topwin, losses) = dfs(t);
    for &(v, w) in &losses {
        if a[v - 1] != w {
            return false;
        }
    }
    a[top - 1] >= topwin
}

fn game_results(s: Input) -> Vec<(State, Tree)> {
    let a = s.a;
    let len = s.s.len();
    let mut res = vec![];
    for i in 0..len + 1 {
        let fst = play_data(Input { s: &s.s[..i], ..s });
        for &(ref f, ref ft, fwin) in &fst {
            let snd = play_data(Input { s: &s.s[i..], l: f.r, ..s });
            for &(ref s, ref st, swin) in &snd {
                if f.r != s.l {
                    continue;
                }
                if fwin == swin {
                    continue;
                }
                let (win, lose) = if fwin {
                    (Box::new(ft.clone()), Box::new(st.clone()))
                } else {
                    (Box::new(st.clone()), Box::new(ft.clone()))
                };
                let tree = Tree::Match(win, lose);
                if check(a, &tree) {
                    let mut news = f.s.clone();
                    news.extend_from_slice(&s.s);
                    res.push((State {
                        s: news,
                        l: f.l,
                        r: s.r,
                    }, tree));
                }
            }
        }
    }
    res
}

fn play_data(s: Input) -> Vec<(State, Tree, bool)> {
    let len = s.s.len();
    if len <= 3 {
        return vec![];
    }
    if s.s[0] != '?' && s.s[0] != '[' {
        return vec![];
    }
    if s.s[len - 1] != '?' && s.s[len - 1] != ']' {
        return vec![];
    }
    if s.s[len - 2] != '?' && s.s[len - 2] != 'o' && s.s[len - 2] != 'x' {
        return vec![];
    }
    let mut win = vec![];
    if s.s[len - 2] != 'x' {
        win.push(true);
    }
    if s.s[len - 2] != 'o' {
        win.push(false);
    }
    let pl = player_data(Input { s: &s.s[1..len - 2], ..s });
    let mut res = vec![];
    for (s, t) in pl {
        for &win in &win {
            let mut s = s.clone();
            s.s.insert(0, '[');
            s.s.push(if win { 'o' } else { 'x' });
            s.s.push(']');
            res.push((s, t.clone(), win));
        }
    }
    res
}

fn player_data(s: Input) -> Vec<(State, Tree)> {
    let mut res = vec![];
    if s.s.len() <= 2 {
        return vec![];
    }
    if s.s[0] != '(' && s.s[0] != '?' {
        return vec![];
    }
    if s.s[s.s.len() - 1] != ')' && s.s[s.s.len() - 1] != '?' {
        return vec![];
    }
    let s = Input { s: &s.s[1..s.s.len() - 1], ..s };
    if s.s.len() <= 2 {
        res.extend_from_slice(&number(s));
    }
    res.extend_from_slice(&game_results(s));
    for &mut (ref mut s, _) in &mut res {
        s.s.insert(0, '(');
        s.s.push(')');
    }
    res
}

fn number(s: Input) -> Vec<(State, Tree)> {
    if s.l > s.n {
        return vec![];
    }
    let mut res = vec![];
    if s.s.len() == 1 && s.l < 10 {
        let i = s.l;
        let c = (b'0' + i as u8) as char;
        if s.s[0] == c || s.s[0] == '?' {
            res.push((State {
                s: vec![c],
                l: i,
                r: i + 1,
            }, Tree::Num(i)));
        }
    }
    if s.s.len() == 2 && s.l >= 10 {
        let i = s.l;
        let c = (b'0' + (i - 10) as u8) as char;
        if (s.s[0] == '1' || s.s[0] == '?') && (s.s[1] == c || s.s[1] == '?') {
            res.push((State {
                s: vec!['1', c],
                l: i,
                r: i + 1,
            }, Tree::Num(i)));
        }
    }
    res
}

// Tags: parsing
fn main() {
    input! {
        s: chars,
        n: usize,
        a: [i32; n],
    }
    let res = game_results(Input { n: n, s: &s, a: &a, l: 1 });
    for (a, _) in res {
        println!("{}", a.s.into_iter().collect::<String>());
    }
}
