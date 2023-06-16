#[cfg(test)]

use crate::ExpectDialog;

#[test]
#[should_panic]
fn test_compile() {
    let opt: Option<i32> = None;
    opt.expect_dialog("This should not make a dialog, but still print to stderr");
}