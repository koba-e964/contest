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
        n: usize,
        p: [usize; n],
    }
    let mut imos = vec![0i64; 2 * n + 1];
    for i in 0..n {
        let diff = (p[i] + n - i) % n;
        // 1..=n/2 => 1 to n/2
        imos[diff + 1] += 1;
        if n % 2 == 0 {
            imos[diff + n / 2 + 1] -= 2;
        } else {
            imos[diff + n / 2 + 1] -= 1;
            imos[diff + n / 2 + 2] -= 1;
        }
        imos[diff + n + 1] += 1;
    }
    for i in 1..2 * n {
        imos[i] += imos[i - 1];
    }
    for i in 1..2 * n {
        imos[i] += imos[i - 1];
    }
    for i in n..2 * n {
        imos[i - n] += imos[i];
    }
    println!("{}", imos[..n].iter().min().unwrap());
}
