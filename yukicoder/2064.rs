use std::cmp::*;
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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        h: usize, w: usize,
        s: [String; h],
    }
    let mut tmp = vec![];
    for i in 0..h {
        tmp.extend(s[i].bytes());
    }
    let mut que = vec![(0, 0)];
    let mut ans = "".to_string();
    ans.push(tmp[0] as char);
    for _ in 0..h + w - 2 {
        let mut new = vec![];
        let mut mi = b'{';
        for &(x, y) in &que {
            if x + 1 < h {
                mi = min(mi, tmp[(x + 1) * w + y]);
            }
            if y + 1 < w {
                mi = min(mi, tmp[x * w + y + 1]);
            }
        }
        for &(x, y) in &que {
            if x + 1 < h {
                if mi == tmp[(x + 1) * w + y] {
                    new.push((x + 1, y));
                }
            }
            if y + 1 < w {
                if mi == tmp[x * w + y + 1] {
                    new.push((x, y + 1));
                }
            }
        }
        ans.push(mi as char);
        new.sort(); new.dedup();
        que = new;
    }
    println!("{}", ans);
}
