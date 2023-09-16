# winmsg
Feature rich `rust` `crate` for [message box](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw) in `windows` 

# Install
```shell
cargo add winmsg
```

# Basic usage
```rust
use winmsg::message_box;

fn main() {
    // empty message box
    message_box(None, None, None, None, None);

    // simple message box with title and description
    // anything passed as Some<T> since anything optional
    message_box(Some("Title"), Some("Some description"), None, None, None);
}

```

# Advanced usage
```rust
use winmsg::{
    message_box, MessageBoxFlags, MessageBoxIconType, MessageBoxReturnCode, MessageBoxType,
};

fn main() {
    // empty message box
    message_box(None, None, None, None, None);

    // simple message box with title and description
    message_box(Some("Title"), Some("Some description"), None, None, None);

    // yes no question
    let ret = message_box(
        Some("Question"),
        Some("Are you sure?"),
        Some(MessageBoxType::YesNo),
        Some(MessageBoxIconType::QUESTION),
        None,
    );
    match ret {
        MessageBoxReturnCode::YES => println!("Yes..."),
        _ => println!("No..."), // anything else
    }

    // ... special flags
    message_box(
        Some("RTL aligned message"),
        Some("שלום וברכה"),
        None,
        None,
        Some(vec![MessageBoxFlags::TextAlignRight]),
    );

    // ... and many more
}

```