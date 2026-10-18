fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let n = getline().trim().parse::<i64>().unwrap();
    let mut tot = 0;
    for len in 3..=20 {
        let lim;
        {
            let mut pass = 0;
            let mut fail = 1 << 20;
            while fail - pass > 1 {
                let mid = (pass + fail) / 2;
                let mut x = 1i64;
                for i in 0..len {
                    x = x.saturating_mul(mid + i);
                }
                if x <= n {
                    pass = mid;
                } else {
                    fail = mid;
                }
            }
            lim = pass;
        }
        eprintln!("{len} => {lim}");
        for a in 1..=lim {
            let mut pass = 0;
            let mut fail = 1 << 30;
            while fail - pass > 1 {
                let mid = (pass + fail) / 2;
                let mut x = 1i64;
                for i in 0..len {
                    x = x.saturating_mul(a + i * mid);
                }
                if x <= n {
                    pass = mid;
                } else {
                    fail = mid;
                }
            }
            tot += pass;
        }
    }
    println!("{tot}");
}
