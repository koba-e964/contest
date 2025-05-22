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

// https://yukicoder.me/problems/no/3130 (3)
fn main() {
    input! {
        n: usize,
        a: [i64; n],
        s: [String; n],
    }
    let mut max = 0;
    let mut add = 0;
    let mut min = 0;
    for s in s {
        if s == "add" {
            add += 1;
        }
        if s == "max" {
            max += 1;
        }
        if s == "min" {
            min += 1;
        }
    }
    let mut a = a;
    a.sort();
    let mut last = 1 << 60;
    if min > 0 {
        last = a[n - min];
    }
    let mut ans = if add > 0 {
        a[0..add - 1].iter().sum::<i64>() + a[add + max - 1]
    } else if max > 0{
        a[max - 1]
    } else {
        0
    };
    ans = ans.min(last);
    println!("{ans}");
}
