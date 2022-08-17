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
        h1: usize, w1: usize,
        a: [[i64; w1]; h1],
        h2: usize, w2: usize,
        b: [[i64; w2]; h2],
    }
    for bits in 0usize..1 << w1 {
        if bits.count_ones() as usize != w2 {
            continue;
        }
        let mut pos = 0;
        for i in 0..h1 {
            if pos < h2 {
                let mut tmp = vec![];
                for j in 0..w1 {
                    if (bits & 1 << j) != 0 {
                        tmp.push(a[i][j]);
                    }
                }
                if b[pos] == tmp {
                    pos += 1;
                }
            }
        }
        if pos == h2 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
