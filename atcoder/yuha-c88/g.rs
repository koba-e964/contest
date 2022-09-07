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

type Coord = i64; // the type of coordinates
type P = (Coord, Coord);

fn det((ax, ay): P, (bx, by): P) -> Coord {
    ax * by - ay * bx
}

fn sub((ax, ay): P, (bx, by): P) -> P {
    (ax - bx, ay - by)
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        sxy: [(String, (i64, i64)); n],
        m: usize,
        t: [String; m],
    }
    let mut map = HashMap::new();
    for i in 0..n {
        map.insert(sxy[i].0.clone(), i);
    }
    let mut idx = vec![0; m];
    for i in 0..m {
        idx[i] = map[&t[i]];
    }
    let dir = if det(sub(sxy[idx[1]].1, sxy[idx[0]].1), sub(sxy[idx[2]].1, sxy[idx[1]].1)) >= 0 {
        1
    } else {
        -1
    };
    let mut ins = vec![];
    for i in 0..n {
        let (ref name, me) = sxy[i];
        let mut ok = true;
        for j in 0..m {
            let (_, p1) = sxy[idx[j]];
            let (_, p2) = sxy[idx[(j + 1) % m]];
            if det(sub(p2, p1), sub(me, p1)) * dir <= 0 {
                ok = false;
                break;
            }
        }
        if ok {
            ins.push((me.0, name.clone()));
        }
    }
    ins.sort();
    for (_, name) in ins {
        puts!("{}\n", name);
    }
}
