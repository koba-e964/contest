fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    getline();
    let s = getline().trim().split_whitespace()
        .map(|x| x.trim().chars().collect::<Vec<_>>()[0])
        .collect::<Vec<_>>();
    fn f(c: char) -> u8 {
        let i = c as u8 - b'a';
        if c <= 'o' {
            return i / 3 + 2;
        }
        if c <= 's' {
            return 7;
        }
        if c <= 'v' {
            return 8;
        }
        9
    }
    for c in s {
        print!("{}", f(c));
    }
    println!();
}
