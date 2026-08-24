fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let [h, w] = ints[..] else { panic!() };
    for i in 0..h {
        for j in 0..w {
            let mut x = 4;
            if i == 0 {
                x -= 1;
            }
            if i == h - 1 {
                x -= 1;
            }
            if j == 0 {
                x -= 1;
            }
            if j == w - 1 {
                x -= 1;
            }
            print!("{x}{}", if j == w - 1 { "\n" } else { " " });
        }
    }
}
