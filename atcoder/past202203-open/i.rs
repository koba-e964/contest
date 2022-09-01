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

fn eq(a: &[Vec<i64>], b: &[Vec<i64>]) -> bool {
    let mut a = a.to_vec();
    let mut b = b.to_vec();
    a.sort();
    b.sort();
    a == b
}

fn main() {
    input! {
        n: usize,
        ab: [[i64; 2]; n],
        xy: [[i64; 2]; n],
    }
    if eq(&ab, &xy) {
        println!("Yes");
        return;
    }
    for i in 0..2 {
        let ami = ab.iter().map(|x| x[i]).min().unwrap();
        let ama = ab.iter().map(|x| x[i]).max().unwrap();
        let xmi = xy.iter().map(|x| x[i]).min().unwrap();
        let xma = xy.iter().map(|x| x[i]).max().unwrap();
        if ama - ami != xma - xmi { continue; }
        let mut cd = ab.clone();
        for x in &mut cd {
            x[i] = xma + ami - x[i];
        }
        if eq(&cd, &xy) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
