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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        h: usize, w: usize,
        s: [chars; h],
        m: usize,
        tn: [(i32, usize); m],
    }
    let mut basis = vec![vec![1; h * w]];
    for (t, n) in tn {
        let mut x = vec![0; h * w];
        if t == 1 {
            for j in 0..n * w { x[j] = 1; }
        } else {
            for j in 0..n {
                for k in 0..h { x[k * w + j] = 1; }
            }
        }
        for b in &basis {
            let mut y = x.clone();
            for i in 0..h * w { y[i] ^= b[i]; }
            if y < x { x = y; }
        }
        if x.iter().any(|&x| x != 0) {
            basis.push(x);
        }
    }
    let mut x = vec![0; h * w];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' { x[i * w + j] = 1; }
            if (i + j) % 2 == 0 { x[i * w + j] ^= 1; }
        }
    }
    for b in &basis {
        let mut y = x.clone();
        for i in 0..h * w { y[i] ^= b[i]; }
        if y < x { x = y; }
    }
    println!("{}", if x.iter().any(|&x| x != 0) {
        "No"
    } else {
        "Yes"
    });
}
