use problem_12787::solve;

#[test]
fn example() {
    let source = include_str!("fixtures/example/in");

    assert_eq!(
        solve(source),
        Ok(include_str!("fixtures/example/out").to_string())
    );
}
