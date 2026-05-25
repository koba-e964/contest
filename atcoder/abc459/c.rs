fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let [n, q] = ints[..] else { panic!() };
    let mut a = vec![0; n];
    let mut f = vec![0; q + 1];
    f[0] = n;
    let mut mi = 0;
    for _ in 0..q {
        let s = getline().trim().split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let ty = s[0];
        if ty == 1 {
            let x = s[1] - 1;
            a[x] += 1;
            f[a[x]] += 1;
            while mi < q && f[mi + 1] == n {
                mi += 1;
            }
        } else {
            let y = s[1];
            let f = |y: usize| if y + mi <= q { f[y + mi] } else { 0 };
            println!("{}", f(y));
        }
    }
}
