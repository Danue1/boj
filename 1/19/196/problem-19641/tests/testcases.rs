use problem_19641::solve;

#[test]
fn example() {
    let source = include_str!("fixtures/example/in");

    assert_eq!(
        solve(source),
        Ok(include_str!("fixtures/example/out").to_string())
    );
}

#[test]
fn myself() {
    let source = include_str!("fixtures/myself/in");

    assert_eq!(
        solve(source),
        Ok(include_str!("fixtures/myself/out").to_string())
    );
}

#[test]
fn description() {
    let source = include_str!("fixtures/description/in");

    assert_eq!(
        solve(source),
        Ok(include_str!("fixtures/description/out").to_string())
    );
}
