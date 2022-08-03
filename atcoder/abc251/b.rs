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
        n: usize, w: usize,
        a: [usize; n],
    }
    let mut has = vec![false; w + 1];
    for i in 0..n {
        if a[i] > w { continue; }
        has[a[i]] = true;
        for j in 0..i {
            if a[i] + a[j] > w { continue; }
            has[a[i] + a[j]] = true;
            for k in 0..j {
                if a[i] + a[j] + a[k] <= w {
                    has[a[i] + a[j] + a[k]] = true;
                }
            }
        }
    }
    println!("{}", has.iter().filter(|&&v| v).count());
}
