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
    let s = (0..h).map(|_| getline().trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut ok = true;
    let dxy = [(1i32, 0i32), (0, 1), (-1, 0), (0, -1)];
    for i in 0..h {
        for j in 0..w {
            let mut c = 0;
            if s[i][j] != '#' { continue; }
            for &(dx, dy) in &dxy {
                let nx = i.wrapping_add(dx as usize);
                let ny = j.wrapping_add(dy as usize);
                if nx >= h || ny >= w {
                    continue;
                }
                if s[nx][ny] == '#' {
                    c += 1;
                }
            }
            if c != 2 && c != 4 {
                ok = false;
            }
        }
    }
    println!("{}", if ok {
        "Yes"
    } else {
        "No"
    });
}
