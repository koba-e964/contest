fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn f(n: usize, k: usize) -> Vec<usize> {
    if n >= 32 {
        let mut v = vec![];
        let mut c = k + 2;
        while c > 2 {
            v.push(c);
            c = c / 2 + 1;
        }
        v.push(2);
        v.resize(n, 1);
        v.reverse();
        return v;
    }
    let div = (1 << (n - 1)) - 1;
    let q = (k + div - 1) / div + 1;
    let mut v = vec![];
    let mut c = k + q;
    while c > q {
        v.push(c);
        c = c / 2 + 1;
    }
    v.push(q);
    v.resize(n, 1);
    v.reverse();
    v
}

fn main() {
    let t: i32 = getline().trim().parse().unwrap();
    for _ in 0..t {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let [n, k] = ints[..] else { panic!() };
        let a = f(n, k);
        for i in 0..a.len() {
            print!("{}{}", a[i], if i + 1 < n { " " } else { "\n" });
        }
    }
}
