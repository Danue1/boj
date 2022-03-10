use problem_5430::solve;

#[test]
fn myself() {
    let source = include_str!("fixtures/myself/in");

    assert_eq!(
        solve(source),
        Ok(include_str!("fixtures/myself/out").to_string())
    );
}
