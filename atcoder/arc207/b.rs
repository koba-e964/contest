fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn even(n: usize) -> Vec<(usize, usize)> {
    let mut e = vec![];
    for i in 0..n / 2 {
        for j in 0..n / 2 {
            if i + j != n / 2 - 1 {
                e.push((i, n / 2 + j));
            }
        }
    }
    e
}

fn main() {
    let n = getline().trim().parse::<usize>().unwrap();
    if n <= 5 {
        println!("-1");
        return;
    }
    let e = if n % 2 == 0 {
        even(n)
    } else {
        let mut tmp = even(n - 1);
        for i in 0..n / 2 {
            tmp.push((n - 1, i));
        }
        tmp
    };
    println!("{}", e.len());
    for (u, v) in e {
        println!("{} {}", u + 1, v + 1);
    }
}
