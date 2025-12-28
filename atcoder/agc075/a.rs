use std::io::{Write, BufWriter};

fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    let n = getline().trim().parse::<usize>().unwrap();
    let mut ans = vec![vec!['+'; n]; n];
    if n % 2 == 0 {
        for i in 0..n {
            for j in 0..n {
                ans[i][j] = if i < n / 2 { '0' } else { '1' };
            }
        }
    } else {
        for i in 0..n {
            for j in 0..n - i - 1 {
                ans[i][j] = '0';
                ans[n - 1 - i][n - 1 - j] = '1';
            }
        }
        for i in 0..n {
            ans[i][n - i - 1] = if i % 2 == 0 { '1' } else { '0' };
        }
    }
    for i in 0..n {
        let s = ans[i].iter().cloned().collect::<String>();
        puts!("{s}\n");
    }
}
