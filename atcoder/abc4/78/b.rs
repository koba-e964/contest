fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let [n, v] = ints[..] else { panic!() };
    let a = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut ma = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if i + j + k <= v - 3 {
                    ma = ma.max(a[i] + a[j] + a[k]);
                }
            }
        }
    }
    println!("{ma}");
}
