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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        n: usize, m: usize,
        a: [chars; 2 * n],
    }
    let kk = ['G', 'C', 'P'];
    let mut b: Vec<i64> = vec![0; 2 * n];
    for i in 0..m {
        let mut c = vec![(0, 0); 2 * n];
        for j in 0..2 * n {
            c[j] = (-b[j], j);
        }
        c.sort();
        for j in 0..n {
            let p = c[2 * j].1;
            let q = c[2 * j + 1].1;
            let x = kk.iter().position(|&c| c == a[p][i]).unwrap();
            let y = kk.iter().position(|&c| c == a[q][i]).unwrap();
            if (x + 2 * y) % 3 == 1 {
                b[q] += 1;
            }
            if (x + 2 * y) % 3 == 2 {
                b[p] += 1;
            }
        }
    }
    let mut c = vec![(0, 0); 2 * n];
    for j in 0..2 * n {
        c[j] = (-b[j], j);
    }
    c.sort();
    for i in 0..2 * n {
        println!("{}", c[i].1 + 1);
    }
}
