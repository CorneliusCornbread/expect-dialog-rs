# expect-dialog-rs
A simple utility for error handling that uses a dialog for it's notification as opposed to only having terminal output.

Great for graphical applications which need to crash but want to inform the user of the crash.

## Note:
For tests expect-dialog will revert to their non dialog counterparts and will function without creating dialogs.

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

![image](https://user-images.githubusercontent.com/8294697/235587567-ca6667b7-7920-43a6-b168-eeaff99c1114.png)

