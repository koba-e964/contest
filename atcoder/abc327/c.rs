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
    input!(a: [[usize; 9]; 9]);
    for i in 0..9 {
        let mut s = 0;
        for j in 0..9 {
            s |= 1 << a[i][j];
        }
        if s != 0x3fe {
            println!("No");
            return;
        }
    }
    for i in 0..9 {
        let mut s = 0;
        for j in 0..9 {
            s |= 1 << a[j][i];
        }
        if s != 0x3fe {
            println!("No");
            return;
        }
    }
    for i in 0..9 {
        let mut s = 0;
        for j in 0..9 {
            s |= 1 << a[(i / 3) * 3 + j / 3][(i % 3) * 3 + j % 3];
        }
        if s != 0x3fe {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
