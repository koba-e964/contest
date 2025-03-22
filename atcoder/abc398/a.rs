fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let n: usize = getline().trim().parse().unwrap();
    let mut a = vec!['-'; n];
    if n % 2 == 1 {
        a[n / 2] = '=';
    } else {
        a[n / 2] = '=';
        a[n / 2 - 1] = '=';
    }
    println!("{}", a.into_iter().collect::<String>());
}
