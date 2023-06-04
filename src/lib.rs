///! It's just like `.expect()` except you get a dialog instead of only terminal output


/// Expect dialog trait, implemented on Option and Result out of the box
pub trait ExpectDialog<T> {
    fn expect_dialog(self, msg: &str) -> T;
}

impl<T, E: std::fmt::Debug> ExpectDialog<T> for Result<T, E> {
    fn expect_dialog(self, msg: &str) -> T {
        match self {
            Ok(value) => return value,
            Err(e) => {
                panic_dialog!("{msg}: {e:?}");
            }
        }
    }
}

impl<T> ExpectDialog<T> for Option<T> {
    fn expect_dialog(self, msg: &str) -> T {
        match self {
            Some(value) => return value,
            None => {
                panic_dialog!("{}", msg);
            }
        }
    }
}

#[macro_export]
macro_rules! panic_dialog {
    ($($arg:tt)*) => { 
        let msg = format!($($arg)*);

        native_dialog::MessageDialog::new()
                    .set_type(native_dialog::MessageType::Error)
                    .set_title("Fatal Error")
                    .set_text(&msg)
                    .show_alert()
                    .expect("Could not display dialog box");
        core::panic!($($arg)*);
    }
}
