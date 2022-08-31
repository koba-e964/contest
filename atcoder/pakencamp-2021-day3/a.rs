fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut a = 0;
    for _ in 0..4 {
        if getline() == "1111\n" {
            a += 1;
        }
    }
    println!("{}", a);
}
