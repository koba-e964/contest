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
        n: usize, m: usize,
        s: [i64; n - 1],
        x: [i64; m],
    }
    let mut v = vec![vec![]; n];
    v[0] = x.clone();
    let mut coo = x.clone();
    for i in 0..m {
        let mut a = 1;
        let mut b = 0;
        for j in 0..n - 1 {
            b = s[j] - b;
            a *= -1;
            let c = (x[i] - b) / a;
            v[j + 1].push(c);
            coo.push(c);
        }
    }
    coo.sort(); coo.dedup();
    let k = coo.len();
    let mut freq = vec![0; k];
    for i in 0..n {
        for v in &v[i] {
            let v = coo.binary_search(v).unwrap();
            freq[v] += 1;
        }
    }
    println!("{}", freq.iter().max().unwrap());
}
