use aayojak::structures::todo::Todo;

#[test]
fn test_new_todo() {
    let todo = Todo::new("Test Todo").expect("error creating Todo");
    assert_eq!(todo.title(), "Test Todo");
    assert_eq!(todo.completion_status(), false);
    assert_eq!(todo.description().is_none(), true);
}

#[test]
fn test_status_toggling() {
    let mut todo = Todo::new("Test Todo").expect("error creating Todo");
    assert_eq!(todo.completion_status(), false);
    todo.toggle_completion_status();
    assert_eq!(todo.completion_status(), true);
    todo.toggle_completion_status();
    assert_eq!(todo.completion_status(), false);
}
