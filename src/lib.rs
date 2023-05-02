///! It's just like `.expect()` except you get a dialog instead of only terminal output

use native_dialog::{MessageDialog, MessageType};

/// Expect dialog trait, implemented on Option and Result out of the box
pub trait ExpectDialog<T> {
    fn expect_dialog(self, msg: &str) -> T;
}

impl<T, E: std::fmt::Debug> ExpectDialog<T> for Result<T, E> {
    fn expect_dialog(self, msg: &str) -> T {
        match self {
            Ok(value) => return value,
            Err(_) => {
                MessageDialog::new()
                    .set_type(MessageType::Error)
                    .set_title("Fatal Error")
                    .set_text(msg)
                    .show_confirm()
                    .expect("Could not display dialog box");
                panic!("{}", msg)
            }
        }
    }
}

impl<T> ExpectDialog<T> for Option<T> {
    fn expect_dialog(self, msg: &str) -> T {
        match self {
            Some(value) => return value,
            None => {
                MessageDialog::new()
                    .set_type(MessageType::Error)
                    .set_title("Fatal Error")
                    .set_text(msg)
                    .show_confirm()
                    .expect("Could not display dialog box");
                panic!("{}", msg)
            }
        }
    }
}
