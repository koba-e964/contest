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

fn main() {
    input!(c: chars);
    let mut t = c.clone();
    t.sort(); t.dedup();
    if t.len() >= 2 {
        let mut c = c;
        'outer:
        for i in 0..c.len() {
            for j in 0..i {
                if c[i] != c[j] {
                    c.swap(i, j);
                    break 'outer;
                }
            }
        }
        println!("{}", c.into_iter().collect::<String>());
        return;
    }
    if c.len() == 1 {
        if c[0] == 'a' {
            println!("NO");
            return;
        }
        let mut c = c;
        c[0] = ((c[0] as u8) - 1) as char;
        c.push('a');
        println!("{}", c.into_iter().collect::<String>());
        return;
    }
    if c.len() == 20 {
        if c[0] == 'z' {
            println!("NO");
            return;
        }
        let mut c = c;
        c[0] = ((c[0] as u8) + 1) as char;
        if c[19] == 'a' {
            c.pop();
        } else {
            c[1] = ((c[1] as u8) - 1) as char;
        }
        println!("{}", c.into_iter().collect::<String>());
        return;
    }
    let mut c = c;
    if c[0] == 'z' {
        c[0] = ((c[0] as u8) - 1) as char;
        c.push('a');
    } else {
        c[0] = ((c[0] as u8) + 1) as char;
        if c[1] == 'a' {
            c.pop();
        } else {
            c[1] = ((c[1] as u8) - 1) as char;
        }
    }
    println!("{}", c.into_iter().collect::<String>());
}
