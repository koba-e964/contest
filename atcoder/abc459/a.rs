fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let mut s = "HelloWorld".chars().collect::<Vec<_>>();
    let n = getline().trim().parse::<usize>().unwrap() - 1;
    s.remove(n);
    println!("{}", s.into_iter().collect::<String>());
}
