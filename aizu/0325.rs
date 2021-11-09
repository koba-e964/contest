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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        hw: [(i64, i64); 6],
    }
    let mut hm = HashMap::new();
    for (h, w) in hw {
        let mut u = [h, w];
        u.sort();
        *hm.entry(u).or_insert(0) += 1;
    }
    let mut ans = vec![];
    for (k, v) in hm {
        if v % 2 != 0 {
            println!("no");
            return;
        }
        for _ in 0..v / 2 {
            ans.push(k);
        }
    }
    ans.sort();
    println!("{}", if ans[0][0] == ans[1][0] && ans[0][1] == ans[2][0]
             && ans[1][1] == ans[2][1] {
        "yes"
    } else {
        "no"
    });
}
