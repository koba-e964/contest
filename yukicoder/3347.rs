fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn ask(v: &[usize]) -> bool {
    print!("?");
    print!(" {}", v.len());
    for &v in v {
        print!(" {}", v + 1);
    }
    println!();
    let x = getline().trim().to_string();
    x == "Yes"
}

pub fn calc(n: usize, ask: impl Fn(&[usize]) -> bool) -> Vec<usize> {
    let mut f = vec![0; n];
    let mut qs = 0;
    let mut howmany = 0;
    for i in 0..n - 1 {
        let mut x = 0;
        while howmany < n {
            qs += 1;
            if !ask(&vec![i; x + 1]) {
                break;
            }
            x += 1;
            howmany += 1;
        }
        f[i] = x;
    }
    f[n - 1] = n - howmany;
    // eprintln!("f = {:?}", f);
    let mut init = vec![0; f[0]];
    for i in 1..n {
        let mut last = 0;
        for sofar in 0..f[i] {
            let mut fail = last;
            let mut pass = init.len() + 1;
            while pass - fail > 1 {
                let mid = (fail + pass) / 2 - 1;
                let mut tmp = init[mid..].to_vec();
                tmp.insert(0, i);
                for _ in 0..sofar {
                    tmp.insert(0, i);
                }
                qs += 1;
                if ask(&tmp) {
                    pass = mid + 1;
                } else {
                    fail = mid + 1;
                }
            }
            init.insert(pass - 1, i);
            last = pass;
        }
    }
    eprintln!("qs = {qs}");
    assert!(qs <= 5000);
    init
}

fn main() {
    let n: usize = getline().trim().parse().unwrap();
    let init = calc(n, ask);
    print!("!");
    for v in init {
        print!(" {}", v + 1);
    }
    println!();
}
