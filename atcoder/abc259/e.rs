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
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        n: usize,
        pe: [[(u32, i64)]; n],
    }
    let mut coo = vec![];
    for pe in &pe {
        for &(p, _) in pe {
            coo.push(p);
        }
    }
    coo.sort(); coo.dedup();
    let m = coo.len();
    let mut snd = vec![[0; 2]; m];
    for pe in &pe {
        for &(p, e) in pe {
            let p = coo.binary_search(&p).unwrap();
            if snd[p][0] < e {
                snd[p].swap(0, 1);
                snd[p][0] = e;
            } else if snd[p][1] < e {
                snd[p][1] = e;
            }
        }
    }
    let mut set = HashSet::new();
    for pe in &pe {
        let mut abs = vec![];
        for &(p, e) in pe {
            let p = coo.binary_search(&p).unwrap();
            if snd[p][0] == e && snd[p][1] != e {
                abs.push(p);
            }
        }
        set.insert(abs);
    }
    println!("{}", set.len());
}
