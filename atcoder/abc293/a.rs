fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut s: Vec<_> = getline().chars().collect();
    for i in 0..s.len() / 2 {
        s.swap(2 * i, 2 * i + 1);
    }
    print!("{}", s.into_iter().collect::<String>());
}
