include!("../MInt.rs");
include!("determinant.rs");

fn main() {
    // test 1, ordinary case
    let mat = vec![
        vec![MInt::new(2), MInt::new(1)],
        vec![MInt::new(1), MInt::new(2)],
    ];
    assert_eq!(determinant(&mat), 3.into());
    // test 2, needs row swapping
    let mat = vec![
        vec![MInt::new(0), MInt::new(1)],
        vec![-MInt::new(1), MInt::new(1)],
    ];
    assert_eq!(determinant(&mat), 1.into());
    // test 3, not full rank
    let mat = vec![
        vec![MInt::new(1), -MInt::new(1)],
        vec![-MInt::new(1), MInt::new(1)],
    ];
    assert_eq!(determinant(&mat), 0.into());
}
