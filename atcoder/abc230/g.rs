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
    let mut fac = vec![0; n + 1];
    let mut moe = vec![1; n + 1];
    for i in 2..n + 1 {
        if fac[i] != 0 { continue; }
        for j in 1..n / i + 1 {
            fac[i * j] = i;
            moe[i * j] *= -1;
        }
        for j in 1..n / i / i + 1 {
            moe[i * i * j] = 0;
        }
    }
    let mut divs = vec![vec![]; n + 1];
    for i in 2..n + 1 {
        if moe[i] == 0 { continue; }
        for j in 1..n / i + 1 {
            divs[i * j].push(i);
        }
    }
    let mut tmp = vec![0; n + 1];
    let mut tot = 0i64;
    for g in 2..n + 1 {
        if moe[g] == 0 { continue; }
        let mut me = 0i64;
        for j in 1..n / g + 1 {
            let v = p[j * g - 1];
            for &d in &divs[v] {
                tmp[d] += 1;
                me -= tmp[d] * moe[d];
            }
        }
        tot -= me * moe[g];
        for j in 1..n / g + 1 {
            let v = p[j * g - 1];
            for &d in &divs[v] {
                tmp[d] -= 1;
            }
        }
    }
    println!("{}", tot);
}
