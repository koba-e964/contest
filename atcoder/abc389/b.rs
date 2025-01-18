fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let n: i64 = getline().trim().parse().unwrap();
    let mut cur = 1;
    let mut i = 0;
    loop {
        i += 1;
        cur *= i;
        if cur == n {
            println!("{i}");
            return;
        }
    }
}
