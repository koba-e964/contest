use std::collections::*;

fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let n: usize = getline().trim().parse().unwrap();
    let s: Vec<Vec<_>> = (0..n).map(|_| getline().trim().chars().collect()).collect();
    let t: Vec<Vec<_>> = (0..5).map(|_| getline().trim().chars().collect()).collect();
    let mut fst = vec![];
    let mut snd = vec![];
    if t[1][0] == '↑' {
        fst.push(t[0][0]);
    } else {
        snd.push(t[0][0]);
    }
    if t[3][0] == '↑' {
        snd.push(t[4][0]);
    } else {
        fst.push(t[4][0]);
    }
    if t[2][1] == '←' {
        fst.push(t[2][0]);
    } else {
        snd.push(t[2][0]);
    }
    if t[2][3] == '←' {
        snd.push(t[2][4]);
    } else {
        fst.push(t[2][4]);
    }
    let mut hs = HashSet::new();
    let mut used = HashSet::new();
    for s in &s {
        for &c in s {
            used.insert(c);
        }
        hs.insert(s.clone());
    }
    for c in used {
        let mut ok = true;
        for &f in &fst {
            ok &= hs.contains(&[c, f] as &[char]);
        }
        for &f in &snd {
            ok &= hs.contains(&[f, c] as &[char]);
        }
        if ok {
            println!("{}", c);
            return;
        }
    }
}
