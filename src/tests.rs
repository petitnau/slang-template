use slang_ui::prelude::*;

use crate::App;

#[test]
fn trivial() -> Result<()> {
    let result = slang_ui::test(
        App,
        r#"
method m() {
    assert true
}
    "#,
    );

    assert!(result.has_message("asserting true is a bit silly, no?"));
    assert_eq!(result.reports().len(), 1);

    Ok(())
}

#[test]
fn test_fa_1() -> Result<()> {
    let result = slang_ui::test(App, include_str!("./tests/FA-1.slang"));

    assert!(!result.has_errors());
    assert_eq!(result.reports().len(), 0);

    Ok(())
}
