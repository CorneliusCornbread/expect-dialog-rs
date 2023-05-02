# expect-dialog-rs
A simple utility for error handling that uses a dialog for it's notification as opposed to only having terminal output.

Great for graphical applications which need to crash but want to inform the user of the crash.

# Install
```
cargo add expect-dialog
```

# Example
```rust
use expect-dialog::ExpectDialog

fn main() {
    let f: Option<bool> = None;
    f.expect_dialog("Value not present");
}
```

![image](https://user-images.githubusercontent.com/8294697/235587004-b86da48c-2253-4667-8497-6063c1a489ac.png)
