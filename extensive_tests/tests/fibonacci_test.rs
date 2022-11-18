use extensive_tests::fibonacci;
use rstest::rstest;

#[rstest]
#[case(0, 0)]
#[case(1, 1)]
#[case(2, 1)]
#[case(3, 2)]
#[case(4, 3)]
#[case(5, 5)]
#[case(6, 8)]
fn fibonacci_fails(#[case] index: usize, #[case] expected: usize) {
    assert_eq!(fibonacci::solve(index), expected)
}
