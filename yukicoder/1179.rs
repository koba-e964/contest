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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// ax^2 + bx + c = 0
// None: infinitely many solutions
fn quad(a: i64, b: i64, c: i64) -> Option<Vec<f64>> {
    if a == 0 {
        if b == 0 {
            if c == 0 {
                return None;
            } else {
                return Some(vec![]);
            }
        }
        return Some(vec![-c as f64 / b as f64]);
    }
    let mut ans = vec![];
    if b * b / 4 >= a * c {
        let cent = -b as f64 / 2.0 / a as f64;
        let delta = ((b as f64 * b as f64 - 4.0 * a as f64 * c as f64) as f64).sqrt() / 2.0 / a.abs() as f64;
        ans.push(cent - delta);
        if b * b % 4 != 0 || b * b / 4 > a * c {
            ans.push(cent + delta);
            if ans[0].abs() < ans[1].abs() {
                ans[0] = c as f64 / a as f64 / ans[1];
            } else {
                ans[1] = c as f64 / a as f64 / ans[0];
            }
        }
    }
    Some(ans)
}

// Tags: quadratic-equation
fn main() {
    input!(a: i64, b: i64, c: i64);
    let v = quad(a, b, c).unwrap();
    if v.is_empty() {
        println!("imaginary");
        return;
    }
    for i in 0..v.len() {
        print!("{}{}", v[i], if i + 1 == v.len() { "\n" } else { " " });
    }
}
