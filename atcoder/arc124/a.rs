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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        n: usize, k: usize,
        ck: [(char, usize1); k],
    }
    let mut eq = vec![vec![]; n];
    let mut ne = vec![vec![]; n];
    for i in 0..k {
        let (c, pos) = ck[i];
        eq[pos].push(i);
        if c == 'L' {
            for j in 0..pos {
                ne[j].push(i);
            }
        } else {
            for j in pos + 1..n {
                ne[j].push(i);
            }
        }
    }
    const MOD: i64 = 998_244_353;
    let mut prod = 1;
    for i in 0..n {
        if eq[i].len() >= 2 {
            prod = 0;
        }
        if eq[i].len() == 1 {
            continue;
        }
        prod = prod * (k - ne[i].len()) as i64 % MOD;
    }
    println!("{}", prod);
}
