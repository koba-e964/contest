mod inner {
    include!("2489.rs");
    pub type MIntPub = MInt;
}

fn main() {
    let m = 157;
    let mut k = 0;
    while (1 << k) < m {
        k += 1;
    }
    for i in 0..k {
        let mut expected_c = inner::MIntPub::new(0);
        for x in 0..m {
            expected_c += x ^ (1 << i);
            expected_c -= x;
        }
        assert_eq!(expected_c, inner::c(m, i), "m = {}, i = {}", m, i);
        let mut expected_e = inner::MIntPub::new(0);
        for x in 0..m {
            if (x & (1 << i)) != 0 {
                expected_e += x;
            }
        }
        assert_eq!(expected_e, inner::e(m, i), "m = {}, i = {}", m, i);
    }
}
