fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let n: i32 = getline().trim().parse().unwrap();
    let mut ans = 0;
    for i in 1..=n {
        let tmp = i * i * i;
        if i % 2 == 1 {
            ans -= tmp;
        } else {
            ans += tmp;
        }
    }
    println!("{ans}");
}
