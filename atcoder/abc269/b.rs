use std::cmp::*;

fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut lx = 10;
    let mut rx = 0;
    let mut ly = 10;
    let mut ry = 0;
    for i in 0..10 {
        for (j, c) in getline().trim().chars().enumerate() {
            if c == '#' {
                lx = min(lx, i);
                rx = max(rx, i);
                ly = min(ly, j);
                ry = max(ry, j);
            }
        }
    }
    println!("{} {}\n{} {}", lx + 1, rx + 1, ly + 1, ry + 1);
}
