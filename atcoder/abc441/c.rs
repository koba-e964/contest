fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let [n, k, x] = ints[..] else { panic!() };
    let mut a = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let n = n as usize;
    let k = k as usize;
    a.sort();
    let mut y = 0;
    for i in (0..k).rev() {
        y += a[i];
        if y >= x {
            println!("{}", n - i);
            return;
        }
    }
    println!("-1");
}
