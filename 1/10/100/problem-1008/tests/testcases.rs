use problem_1008::solve;

#[test]
fn example1() {
    let source = include_str!("fixtures/example1/in");

    assert_eq!(
        solve(source),
        Ok(include_str!("fixtures/example1/out").to_string())
    );
}

#[test]
fn example2() {
    let source = include_str!("fixtures/example2/in");

    assert_eq!(
        solve(source),
        Ok(include_str!("fixtures/example2/out").to_string())
    );
}
