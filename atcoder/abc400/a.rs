fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let n = getline().trim().parse::<usize>().unwrap();
    if 400 % n == 0 {
        println!("{}", 400 / n);
    } else {
        println!("-1");
    }
}
