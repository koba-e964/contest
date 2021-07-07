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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        a: usize, b: usize, c: usize,
        ch: [(usize1, usize1, usize1, i32)],
    }
    let mut ans = vec![2; a + b + c];
    for &(i, j, k, x) in &ch {
        if x == 0 {
            continue;
        }
        ans[i] = 1;
        ans[j] = 1;
        ans[k] = 1;
    }
    for &(i, j, k, x) in &ch {
        if x == 1 {
            continue;
        }
        let mut s = [(ans[i], i), (ans[j], j), (ans[k], k)];
        s.sort();
        if s[0].0 == 1 && s[1].0 == 1 && s[2].0 == 2 {
            ans[s[2].1] = 0;
        }
    }
    for v in ans {
        println!("{}", v);
    }
}
