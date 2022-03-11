use problem_3447::solve;

#[test]
fn example() {
    let source = include_str!("fixtures/example/in");

    assert_eq!(
        solve(source),
        Ok(include_str!("fixtures/example/out").to_string())
    );
}

#[test]
fn official() {
    let source = include_str!("fixtures/official/in");

    assert_eq!(
        solve(source),
        Ok(include_str!("fixtures/official/out").to_string())
    );
}
