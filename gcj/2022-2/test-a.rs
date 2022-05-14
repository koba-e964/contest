mod inner {
    include!("a.rs");
}

fn main() {
    let n = 101;
    for k in n - 1..n * n {
        if k % 2 != 0 { continue; }
        let mut raw = vec![];
        let ans = inner::calc(n, k, &mut raw, 0).unwrap();
        raw.extend(ans);
        assert_eq!(raw.len(), k as usize + 1);
        for i in 0..k as usize {
            let dx = raw[i].0 - raw[i + 1].0;
            let dy = raw[i].1 - raw[i + 1].1;
            assert!((dx == 0 && dy.abs() == 1) || (dx.abs() == 1 && dy == 0));
            let (x1, y1) = raw[i];
            let (x2, y2) = raw[i + 1];
            if inner::pos(n, x1, y1) > inner::pos(n, x2, y2) {
                eprintln!("k = {}, {} < {}?", k, inner::pos(n, x1, y1) , inner::pos(n, x2, y2));
                assert!(inner::pos(n, x1, y1) < inner::pos(n, x2, y2));
            }
        }
    }
}
