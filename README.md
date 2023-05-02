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