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

fn rec(a: [usize; 9], n: usize, sum: i64, p2: i64, p10: i64) -> i64 {
    if n == 0 {
        return sum & -sum;
    }
    if sum & (p2 - 1) != 0 {
        let r = sum & (p2 - 1);
        return r & -r;
    }
    let mut ans = 0;
    let mut b = a;
    for i in 0..9 {
        if a[i] > 0 {
            b[i] -= 1;
            let val = rec(b, n - 1, sum + p10 * (i as i64 + 1), p2 * 2, p10 * 10);
            if ans < val {
                ans = val;
            }
            b[i] += 1;
        }
    }
    ans
}

// https://yukicoder.me/problems/no/1633 (3)
// 全探索すると 14!/2!^5 ~= 2 * 10^9 通りを試す必要があるが、枝刈りを行えばもっと減るはず。
fn main() {
    input! {
        n: usize,
        a: [usize; 9],
    }
    let mut aa = [0; 9];
    for i in 0..9 {
        aa[i] = a[i];
    }
    let ans = rec(aa, n, 0, 1, 1);
    println!("{}", (ans - 1).count_ones());
}
