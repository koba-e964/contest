mod sol {
    include!("b.rs");
}

fn naive(x: i64) -> usize {
    let mut layers = vec![];
    let mut v = vec![0, x + 1];
    while v.len() < x as usize + 2 {
        v.sort();
        let mut new = vec![];
        for i in 0..v.len() - 1 {
            if v[i] + 1 != v[i + 1] {
                new.push((v[i] + v[i + 1]) / 2);
            }
        }
        layers.push(new.clone());
        v.extend(&new);
    }
    let last = &layers[layers.len() - 1];
    let last2 = &layers[layers.len() - 2];
    let mut ma = 0;
    for i in 0..last2.len() {
        let idx = last.binary_search(&last2[i]).unwrap_err();
        ma = core::cmp::max(ma, i + 1 + last.len() - idx);
    }
    ma
}

fn main() {
    for i in 2..33 {
        let x = naive(i);
        eprintln!("{} => {} ({})", i, x, sol::lastlen2(i));
        assert_eq!(sol::lastlen2(i), x as i64);
    }
}
