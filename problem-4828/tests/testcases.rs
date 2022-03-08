use problem_4828::solve;

#[test]
fn example() {
    let source = include_str!("fixtures/example/in").trim();

    assert_eq!(
        solve(source),
        Ok(include_str!("fixtures/example/out").trim().to_string())
    );
}

#[test]
fn test1() {
    let source = include_str!("fixtures/test1/in").trim();

    assert_eq!(
        solve(source),
        Ok(include_str!("fixtures/test1/out").trim().to_string())
    );
}

#[test]
fn test2() {
    let source = include_str!("fixtures/test2/in").trim();

    assert_eq!(
        solve(source),
        Ok(include_str!("fixtures/test2/out").trim().to_string())
    );
}

#[test]
fn test3() {
    let source = include_str!("fixtures/test3/in").trim();

    assert_eq!(
        solve(source),
        Ok(include_str!("fixtures/test3/out").trim().to_string())
    );
}
